use core::mem::size_of;
use kernel::allocator;
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
struct PageDirectory {
    tables: [u32, ..1024]
}

pub unsafe fn init() {
    let (kernel_dir, _) = allocator.alloc(0x1000);
    let dir = kernel_dir as *mut PageDirectory;
    memset(dir as *mut u8, 0, size_of::<PageDirectory>() as u32);

    let (table_ptr, _) = allocator.alloc(0x1000);
    let table = table_ptr as *mut PageTable;

    int::range(0, 1024, |i| {
        (*table).pages[i] = Page(((i as u32) * PAGE_SIZE) | PRESENT | RW | USER);
    });

    (*dir).tables[0] = table as u32 | PRESENT | RW | USER;

    kernel::int_table.map(|t| {
        use cpu::exception::{Isr, PF};
        (*t.table)[PF as u8] = Isr::new(PF, true);
    });

    (*dir).enable();
}

impl PageDirectory {
    pub unsafe fn enable(&mut self) {
        asm!("mov cr3, $0

              mov eax, cr0
              or eax, $1
              mov cr0, eax"
            :: "{eax}"(self), "n"(CR0_PG)
            :: "intel")
    }
}
