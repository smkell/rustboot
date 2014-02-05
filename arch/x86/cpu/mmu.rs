use core::mem::transmute;
use core::option::Some;

use kernel::heap;
use kernel::memory::Allocator;
use kernel::memory::physical::{alloc_frames, zero_alloc_frames};
use kernel::int;
use kernel::rt::memset;
use kernel;
use cpu::idt;
use cpu::interrupt::IsrStack;
use platform::io;

pub static PRESENT: u32 = 1 << 0;
pub static RW:      u32 = 1 << 1;
pub static USER:    u32 = 1 << 2;

static CR0_PG: u32 = 1 << 31;

static PAGE_SIZE: u32 = 0x1000;
static ENTRIES:   u32 = 1024;

static DIRECTORY_VADDR: u32 = 0xFFFFF000;
static directory: *mut PageDirectory = DIRECTORY_VADDR as *mut PageDirectory;

#[packed]
struct Page(u32);

#[packed]
struct PageTable {
    pages: [Page, ..1024]
}

// // U: underlying element type
// #[packed]
// struct Table<U> {
//     pages: [Page, ..1024]
// }

// type PageTable = Table<Page>;
// type PageDirectory = Table<Table<Page>>;

#[packed]
pub struct PageDirectory {
    tables: [Page, ..1024]
}

pub unsafe fn init() {
    let dir = zero_alloc_frames(1) as *mut PageDirectory;
    let table = alloc_frames(1) as *mut PageTable;

    (*table).identity_map(0, PRESENT | RW);
    // (*dir).tables[0] = Page::new(table as u32, PRESENT | RW);
    (*dir).set(transmute(0), table, PRESENT | RW);

    // Map the directory as its own last table.
    // (phys *directory)Dir last table => (phys *table)Dir, last page => virt *last_page
    // When accessing its virtual address
    // (*dir).tables[DIRECTORY_VADDR >> 22] = Page::new(dir as u32, PRESENT | RW);
    (*dir).set(directory, dir, PRESENT | RW);

    kernel::int_table.map(|t| {
        use cpu::interrupt::{Isr, IsrStack, Fault};
        use cpu::exception::{PAGE_FAULT, exception_handler};
        (*t.table)[PAGE_FAULT as u8] = Isr::new(Fault(PAGE_FAULT), true).idt_entry(exception_handler());
    });

    (*dir).switch();
}

pub unsafe fn map(page_ptr: *mut u8, flags: u32) {
    /*let vaddr = page_ptr as u32;
    let phys_ptr = alloc_frames(1);

    let table = alloc_frames(1) as *mut PageTable;

    (*table).pages[(vaddr >> 12) & 0x3ff] = Page(phys_ptr as u32 | PRESENT | RW | USER);
    kernel::page_dir.map(|p| {
        // TODO: set page; directory vaddr
        if !(*directory).tables[vaddr >> 22].present() {
            
        }
        (*directory).tables[vaddr >> 22] = Page::new(table as u32, PRESENT | RW);
    });*/
    // io::puti(page_ptr as int);
    let table = (*directory).fetch_table(page_ptr as u32, flags | PRESENT);
    (*table).set(page_ptr, alloc_frames(1), flags | PRESENT);
    flush_tlb(page_ptr);
}

fn flush_tlb<T>(addr: T) {
    unsafe {
        asm!("invlpg ($0)" :: "r"(addr) : "volatile", "memory");
    }
}

impl PageDirectory {
    fn fetch_table(&mut self, addr: u32, flags: u32) -> *mut PageTable {
        let index = addr / (PAGE_SIZE * ENTRIES);
        if self.tables[index].present() {
            match self.tables[index] {
                Page(v) => (v & !(0b111 as u32)) as *mut PageTable
            }
        }
        else {
            // allocate table
            unsafe {
                let table = zero_alloc_frames(1) as *mut PageTable;
                (*directory).set(addr as *mut PageTable, table, flags);
                table
            }
        }
    }

    unsafe fn set<T>(&mut self, addr: *mut T, entry: *mut T, flags: u32) {
        let index = addr as u32 / (PAGE_SIZE * ENTRIES);
        self.tables[index] = Page(entry as u32 | flags);
    }

    unsafe fn get<T>(&mut self, addr: *mut T) -> u32 {
        let index = addr as u32 / (PAGE_SIZE * ENTRIES);
        match self.tables[index] { Page(entry) => entry }
    }

    pub unsafe fn switch(&self) {
        asm!("mov cr3, $0

              mov eax, cr0
              or eax, $1
              mov cr0, eax"
            :: "{eax}"(self), "n"(CR0_PG)
            :: "intel")
    }
}

impl Page {
    fn new(addr: u32, flags: u32) -> Page {
        Page(addr | flags)
    }

    fn at_frame(i: uint, flags: u32) -> Page {
        Page::new((i as u32) * PAGE_SIZE, flags)
    }

    fn present(self) -> bool {
        match self {
            Page(v) => (v & PRESENT) != 0
        }
    }
}

impl PageTable {
    fn identity_map(&mut self, start: uint, flags: u32) {
        // TODO: use dmemset. zero_alloc_frames use sse.
        int::range(0, 1024, |i| {
            self.pages[i] = Page::at_frame(start + i, flags);
        });
    }

    unsafe fn set<T>(&mut self, addr: *mut T, entry: *mut T, flags: u32) {
        let index = (addr as u32 / PAGE_SIZE) % ENTRIES;
        self.pages[index] = Page(entry as u32 | flags);
    }
}
