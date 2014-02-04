use core::mem::size_of;
use core::option::Some;
use kernel::heap;
use kernel::memory::Allocator;
use kernel::int;
use kernel::rt::memset;
use kernel;
use cpu::idt;

static PRESENT: u32 = 1 << 0;
static RW:      u32 = 1 << 1;
static USER:    u32 = 1 << 2;

static CR0_PG: u32 = 1 << 31;

static PAGE_SIZE: u32 = 0x1000;

#[packed]
struct Page(u32);

#[packed]
struct PageTable {
    pages: [Page, ..1024]
}

#[packed]
pub struct PageDirectory {
    tables: [u32, ..1024]
}

pub unsafe fn init() {
    let (kernel_dir, _) = heap.alloc(0x1000);
    let dir = kernel_dir as *mut PageDirectory;
    memset(dir as *mut u8, 0, size_of::<PageDirectory>() as u32);

    let (table_ptr, _) = heap.alloc(0x1000);
    let table = table_ptr as *mut PageTable;

    int::range(0, 1024, |i| {
        (*table).pages[i] = Page(((i as u32) * PAGE_SIZE) | PRESENT | RW | USER);
    });

    (*dir).tables[0] = table as u32 | PRESENT | RW | USER;

    kernel::int_table.map(|t| {
        use cpu::interrupt::{Isr, Fault};
        use cpu::exception::{PAGE_FAULT, exception_handler};
        (*t.table)[PAGE_FAULT as u8] = Isr::new(Fault(PAGE_FAULT), true).idt_entry(exception_handler());
    });

    kernel::page_dir = Some(dir);
    (*dir).enable();
}

pub unsafe fn map(page_ptr: *mut u8) {
    let (phys_ptr, _) = heap.alloc(0x1000);
    let vaddr = page_ptr as u32;

    let (table_ptr, _) = heap.alloc(0x1000);
    let table = table_ptr as *mut PageTable;

    (*table).pages[(vaddr >> 12) & 0x3ff] = Page(phys_ptr as u32 | PRESENT | RW | USER);
    kernel::page_dir.map(|p| {
        (*p).tables[vaddr >> 22] = table as u32 | PRESENT | RW | USER;
    });
}

impl PageDirectory {
    pub unsafe fn enable(&self) {
        asm!("mov cr3, $0

              mov eax, cr0
              or eax, $1
              mov cr0, eax"
            :: "{eax}"(self), "n"(CR0_PG)
            :: "intel")
    }
}
