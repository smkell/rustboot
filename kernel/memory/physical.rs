use core::fail::abort;
use core::mem::transmute;

use kernel::heap;
use kernel::memory;
use kernel::memory::Allocator;
use cpu::mmu::Frame;

pub static mut frames: memory::Alloc = memory::Alloc {
    base: 0x200_000 as *mut u8,
    el_size: 12,
    parent: memory::BuddyAlloc {
        order: 13,
        tree: memory::Bitv { storage: 0 as memory::BitvStorage }
    }
};

pub struct Phys<T> {
    priv ptr: *mut T
}

impl<T> Phys<T> {
    pub fn at(offset: u32) -> Phys<T> {
        Phys { ptr: offset as *mut T }
    }

    /*unsafe*/ pub fn as_ptr(&self) -> *mut T {
        match *self {
            Phys { ptr: p } => p
        }
    }

    pub fn offset(&self) -> u32 {
        unsafe {
            transmute(*self)
        }
        // match *self {
        //     Phys { ptr: p } => unsafe { transmute(p) }
        // }
    }
}

pub fn init() {
    unsafe {
        frames.parent.tree.storage = heap::zero_alloc::<u32>(1024) as memory::BitvStorage;
    }
}

pub unsafe fn alloc_frames<T = Frame>(count: uint) -> Phys<T> {
    match frames.alloc(count) {
        (_, 0) => abort(),
        (ptr, _) => Phys { ptr: ptr as *mut T }
    }
}

pub unsafe fn zero_alloc_frames<T = Frame>(count: uint) -> Phys<T> {
    match frames.zero_alloc(count) {
        (_, 0) => abort(),
        (ptr, _) => Phys { ptr: ptr as *mut T }
    }
}
