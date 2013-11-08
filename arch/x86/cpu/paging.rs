use kernel::memory;
use kernel::int;
use core::mem::Allocator;

type page_dir = [u32, ..1024];

static PRESENT: u32 = 1 << 0;
static RW:      u32 = 1 << 1;
static USER:    u32 = 1 << 2;

static CR0_PG: u32 = 1 << 31;

pub unsafe fn identity() {
    let (dir_ptr, _) = memory::allocator.alloc(0x1000);
    let dir = dir_ptr as *mut page_dir;
    let (table_ptr, _) = memory::allocator.alloc(0x1000);
    let table = table_ptr as *mut page_dir;

    int::range(0, 1024, |i| {
        (*table)[i] = ((i as u32) * 4096) | PRESENT | RW | USER;
    });

    int::range(0, 1024, |i| {
        (*dir)[i] = 0;
    });

    (*dir)[0] = table as u32 | PRESENT | RW | USER;
    enable(dir);
}

pub unsafe fn enable(dir: *mut page_dir) {
    asm!("mov eax, $1
          mov cr3, eax

          mov eax, cr0
          or eax, $0
          mov cr0, eax"
        :: "n"(CR0_PG), "n"(dir)
        : "eax" : "intel")
}
