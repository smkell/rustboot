use core::mem::size_of;
use core::ptr::set_memory;
use core::option::Some;
use core;

use kernel::memory::physical;

// kinda clever
define_flags!(Flags: u32 {
    SECTION = 0b10010,

    BUFFER = 1 << 2,
    CACHE,
    RW     = 1 << 10,
    USER
})

#[packed]
struct Descriptor(u32);

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

pub unsafe fn map(page_ptr: *mut u8, flags: Flags) {
    // TODO
}

impl Descriptor {
    fn section(base: u32, flags: Flags) -> Descriptor {
        // make a section descriptor
        Descriptor(base) | flags | SECTION
    }
}

impl core::ops::BitOr<Flags, Descriptor> for Descriptor {
    #[inline(always)]
    fn bitor(&self, other: &Flags) -> Descriptor {
        match (self, other) {
            (&Descriptor(p), &Flags(f)) => Descriptor(p | f)
        }
    }
}

impl core::ops::BitAnd<Flags, bool> for Descriptor {
    #[inline(always)]
    fn bitand(&self, other: &Flags) -> bool {
        match (self, other) {
            (&Descriptor(p), &Flags(f)) => p & f != 0
        }
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
