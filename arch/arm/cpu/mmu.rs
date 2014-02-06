use core::mem::size_of;
use core::ptr::set_memory;
use core::option::Some;
use core;

use kernel::memory::physical;

// kinda clever
define_flags!(Descriptor: u32 {
    SECTION = 0b10010,

    BUFFER = 1 << 2,
    CACHE  = 1 << 3,
    RW     = 1 << 10,
    USER   = 1 << 11
})

#[packed]
struct PageTableCoarse {
    pages: [Descriptor, ..256]
}

#[packed]
pub struct PageDirectory {
    tables: [Descriptor, ..4096]
}

pub unsafe fn init() {
    let dir = physical::zero_alloc_frames(4) as *mut PageDirectory;

    (*dir).tables[0] = Descriptor::section(0, RW);
    (*dir).enable();
}

pub unsafe fn map(page_ptr: *mut u8, flags: Descriptor) {
    // TODO
}

impl Descriptor {
    fn section(base: u32, flags: Descriptor) -> Descriptor {
        // make a section descriptor
        Descriptor(base) | flags | SECTION
    }
}

impl PageDirectory {
    pub unsafe fn enable(&self) {
        asm!("mov ip, 0
              mcr p15, 0, ip, c7, c5, 0     // invalidate I cache
              mcr p15, 0, ip, c7, c10, 4    // drain WB
              mcr p15, 0, r0, c2, c0, 0     // load page table pointer
              mcr p15, 0, ip, c8, c7, 0     // invalidate I & D TLBs"
            :: "{r0}"(self) : "ip")
    }
}
