use core::fail::out_of_memory;
use core::mem::size_of;
use core::uint::mul_with_overflow;

use kernel::mm::{Allocator, Alloc, BuddyAlloc};
use util::bitv;

pub static mut heap: Alloc = Alloc {
    base: 0x110_000 as *mut u8,
    el_size: 0,
    parent: BuddyAlloc {
        order: 17,
        tree: bitv::Bitv { storage: 0x100_000 as *mut u32 }
    }
};

pub fn init() {
    unsafe {
        heap.parent.tree.clear(1 << (heap.parent.order + 1));
    }
}

#[lang = "exchange_malloc"]
#[inline]
pub unsafe fn malloc_raw(size: uint) -> *mut u8 {
    match heap.alloc(size) {
        (_, 0) => out_of_memory(),
        (ptr, _) => ptr
    }
}

#[lang = "exchange_free"]
#[inline]
pub unsafe fn free<T>(ptr: *mut T) {
    heap.free(ptr as *mut u8);
}

#[inline]
pub unsafe fn alloc<T = u8>(count: uint) -> *mut T {
    match mul_with_overflow(count, size_of::<T>()) {
        (_, true) => out_of_memory(),
        (size, _) => malloc_raw(size) as *mut T
    }
}

#[inline]
pub unsafe fn zero_alloc<T = u8>(count: uint) -> *mut T {
    match mul_with_overflow(count, size_of::<T>()) {
        (_, true) => out_of_memory(),
        (size, _) => match heap.zero_alloc(size) {
            (_, 0) => out_of_memory(),
            (ptr, _) => ptr as *mut T
        }
    }
}

#[inline]
pub unsafe fn realloc_raw<T>(ptr: *mut T, count: uint) -> *mut T {
    match mul_with_overflow(count, size_of::<T>()) {
        (_, true) => out_of_memory(),
        (0, _) => {
            free(ptr);
            0 as *mut T
        }
        (size, _) => match heap.realloc(ptr as *mut u8, size) {
            (_, 0) => out_of_memory(),
            (ptr, _) => ptr as *mut T
        }
    }
}
