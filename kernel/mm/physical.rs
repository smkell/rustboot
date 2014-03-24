use core::fail::abort;
use core::mem::transmute;

use kernel::heap;
use kernel::mm;
use kernel::mm::Allocator;
use cpu::mmu::Frame;
use util::bitv;

pub static mut frames: mm::Alloc = mm::Alloc {
    base: 0x200_000 as *mut u8,
    el_size: 12,
    parent: mm::BuddyAlloc {
        order: 13,
        tree: bitv::Bitv { storage: 0 as *mut u32 }
    }
};

pub struct Phys<T> {
    priv ptr: *mut T
}

impl<T> Phys<T> {
    pub fn at(offset: u32) -> Phys<T> {
        Phys { ptr: offset as *mut T }
    }

    pub fn as_ptr(&self) -> *mut T {
        match *self {
            Phys { ptr: p } => p
        }
    }

    pub fn offset(&self) -> u32 {
        unsafe {
            transmute(*self)
        }
    }
}

pub fn init() {
    unsafe {
        frames.parent.tree.storage = heap::zero_alloc::<u32>(1024);
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

#[inline]
pub unsafe fn free_frames<T>(ptr: Phys<T>) {
    frames.free(ptr.offset() as *mut u8);
}
