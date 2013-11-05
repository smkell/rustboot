use kernel::memory;
use kernel::int;

type page_dir = [u32, ..1024];
pub static PAGE_DIR: *mut page_dir = 0x102000 as *mut page_dir;
pub static PAGE_TABLE0: *mut page_dir = 0x103000 as *mut page_dir;

static PRESENT: u32 = 1 << 0;
static RW:      u32 = 1 << 1;
static USER:    u32 = 1 << 2;

static CR0_PG: u32 = 1 << 31;

pub unsafe fn identity() {
    int::range(0, 1024, |i| {
        (*PAGE_TABLE0)[i] = ((i as u32) * 4096) | PRESENT | RW | USER;
    });

    int::range(0, 1024, |i| {
        (*PAGE_DIR)[i] = 0;
    });

    (*PAGE_DIR)[0] = PAGE_TABLE0 as u32 | PRESENT | RW | USER;
}

pub unsafe fn enable() {
    asm!("mov eax, $1
          mov cr3, eax

          mov eax, cr0
          or eax, $0
          mov cr0, eax"
        :: "n"(CR0_PG), "n"(PAGE_DIR)
        : "eax" : "intel")
}
