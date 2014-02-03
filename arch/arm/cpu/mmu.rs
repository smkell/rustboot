use core::mem::size_of;
use core::ptr::set_memory;
use core::option::Some;

use kernel::heap;
use kernel::memory::Allocator;
use kernel;

static CACHE:  u32 = 1 << 3;
static BUFFER: u32 = 1 << 2;

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
    let (kernel_dir, _) = heap.alloc(size_of::<PageDirectory>());
    let dir = kernel_dir as *mut PageDirectory;
    set_memory(dir as *mut u8, 0, size_of::<PageDirectory>());

    let (table_ptr, _) = heap.alloc(size_of::<PageTableCoarse>());
    let table = table_ptr as *mut PageTableCoarse;

    (*dir).tables[0] = Descriptor::section(0);

    kernel::page_dir = Some(dir);
    (*dir).enable();
}

pub unsafe fn map(page_ptr: *mut u8) {
    // TODO
}

impl Descriptor {
    fn section(base: u32) -> Descriptor {
        // make a section descriptor
        //                /permissions
        Descriptor(base | (3 << 10) | 0b10010)
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
