use core::mem::{transmute, size_of};
use core;

use kernel::memory::physical;
use kernel::int;
use kernel;

define_flags!(Flags: u32 {
    PRESENT  = 1 << 0,
    RW       = 1 << 1,
    USER     = 1 << 2,
    ACCESSED = 1 << 5,
    HUGE     = 1 << 7
})

#[packed]
struct Page(u32);

static CR0_PG: u32 = 1 << 31;

static PAGE_SIZE: uint = 0x1000;
static ENTRIES:   uint = 1024;

static DIRECTORY_VADDR: u32 = 0xFFFFF000;
static directory: *mut PageDirectory = DIRECTORY_VADDR as *mut PageDirectory;

// U: underlying element type
#[packed]
struct Table<U> {
    entries: [Page, ..ENTRIES]
}

pub type PageTable = Table<Page>;
pub type PageDirectory = Table<Table<Page>>;

pub unsafe fn init() {
    let dir = physical::zero_alloc_frames(1) as *mut PageDirectory;
    let table = physical::alloc_frames(1) as *mut PageTable;

    (*table).identity_map(0, PRESENT | RW);
    (*dir).set(transmute(0), table, PRESENT | RW);

    // Map the directory as its own last table.
    // When accessing its virtual address
    (*dir).set(directory, dir, PRESENT | RW);

    kernel::int_table.map(|t| {
        use cpu::interrupt::{Isr, Fault};
        use cpu::exception::{PAGE_FAULT, exception_handler};
        (*t.table)[PAGE_FAULT as u8] = Isr::new(Fault(PAGE_FAULT), true).idt_entry(exception_handler());
    });

    (*dir).switch();
}

pub unsafe fn map(page_ptr: *mut u8, flags: Flags) {
    (*directory).map_frame(page_ptr, flags);
    flush_tlb(page_ptr);
}

fn flush_tlb<T>(addr: T) {
    unsafe {
        asm!("invlpg [$0]" :: "r"(addr) : "memory" : "volatile", "intel")
    }
}

impl Page {
    fn new(addr: u32, flags: Flags) -> Page {
        Page(addr) | flags
    }

    fn at_frame(i: uint, flags: Flags) -> Page {
        Page::new((i * PAGE_SIZE) as u32, flags)
    }

    fn present(self) -> bool {
        self & PRESENT
    }
}

impl core::ops::BitOr<Flags, Page> for Page {
    #[inline(always)]
    fn bitor(&self, other: &Flags) -> Page {
        match (self, other) {
            (&Page(p), &Flags(f)) => Page(p | f)
        }
    }
}

impl core::ops::BitAnd<Flags, bool> for Page {
    #[inline(always)]
    fn bitand(&self, other: &Flags) -> bool {
        match (self, other) {
            (&Page(p), &Flags(f)) => p & f != 0
        }
    }
}

impl<U> Table<U> {
    fn set<T>(&mut self, addr: *mut T, entry: *mut T, flags: Flags) {
        // update entry, based on the underlying type (page, table)
        let len = size_of::<U>() / size_of::<Page>();
        let index = (addr as uint / PAGE_SIZE / len) % ENTRIES;
        self.entries[index] = Page::new(entry as u32, flags);
    }
}

impl Table<Page> {
    fn identity_map(&mut self, start: uint, flags: Flags) {
        int::range(0, ENTRIES, |i| {
            self.entries[i] = Page::at_frame(start + i, flags);
        });
    }
}

// Can't impl on typedefs. Rust #9767
impl Table<Table<Page>> {
    fn fetch_table(&mut self, addr: u32, flags: Flags) -> *mut PageTable {
        let index = addr as uint / (PAGE_SIZE * ENTRIES);
        match self.entries[index] {
            table @ Page(p) if table.present() => {
                (p & 0xFFFFF000) as *mut PageTable
            }
            _ => unsafe { // allocate table
                let table = physical::zero_alloc_frames(1) as *mut PageTable;
                (*directory).set(addr as *mut PageTable, table, flags);
                flush_tlb(table);
                table
            }
        }
    }

    pub unsafe fn map_frame(&mut self, page_ptr: *mut u8, flags: Flags) {
        let table = self.fetch_table(page_ptr as u32, flags | PRESENT);
        (*table).set(page_ptr, physical::alloc_frames(1), flags | PRESENT);
    }

    unsafe fn switch(&self) {
        asm!("mov cr3, $0

              mov eax, cr0
              or eax, $1
              mov cr0, eax"
            :: "{eax}"(self), "n"(CR0_PG)
            :: "intel")
    }
}
