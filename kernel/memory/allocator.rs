use core::fail::{abort, out_of_memory};
use core::ptr::offset;
use core::ptr::set_memory;
use core::i32::ctlz32;
use kernel::ptr::mut_offset;

pub trait Allocator {
    unsafe fn alloc(&mut self, size: uint) -> (*mut u8, uint);
    unsafe fn zero_alloc(&mut self, size: uint) -> (*mut u8, uint);
    unsafe fn realloc(&mut self, ptr: *mut u8, size: uint) -> (*mut u8, uint);
    unsafe fn free(&mut self, ptr: *mut u8);
}

trait BitvTrait {
    fn get(&self, i: uint) -> uint;
    fn set(&self, i: uint, x: uint);
    fn to_bytes(&self) -> *mut u8;
    fn size(&self) -> uint;
}

static BITV_SIZE: uint = 0x10_000;
pub type BitvStorage = *mut [u32, ..BITV_SIZE / 4];

// vector of 2-bit
pub struct Bitv {
    storage: BitvStorage
}

impl BitvTrait for Bitv {
    #[inline]
    fn get(&self, i: uint) -> uint {
        unsafe { 3 & ((*self.storage)[i / 16] as uint >> ((i % 16) * 2)) }
    }

    #[inline]
    fn set(&self, i: uint, x: uint) {
        let w = i / 16;
        let b = (i % 16) * 2;
        unsafe { (*self.storage)[w] = (((*self.storage)[w] & !(3 << b)) | (x as u32 << b)); }
    }

    #[inline]
    fn to_bytes(&self) -> *mut u8 {
        self.storage as *mut u8
    }

    #[inline]
    fn size(&self) -> uint {
        BITV_SIZE
    }
}

pub static UNUSED: uint = 0;
pub static USED:   uint = 1;
pub static SPLIT:  uint = 2;
pub static FULL:   uint = 3;

pub struct BuddyAlloc {
    base: *mut u8,
    order: uint,
    tree: Bitv
}

impl BuddyAlloc {
    pub fn new(base: *mut u8, order: uint, storage: Bitv) -> BuddyAlloc {
        unsafe { set_memory(storage.to_bytes(), 0, storage.size()); }

        BuddyAlloc { base: base, order: order, tree: storage }
    }

    pub unsafe fn combine(&self, mut index: uint) {
        loop {
            let buddy = index + (index & 1) * 2;
            if buddy < 1 || self.tree.get(buddy - 1) != UNUSED {
                self.tree.set(index, UNUSED);
                while index >= 1 && self.tree.get(index) == FULL {
                    index = (index + 1) / 2 - 1;
                    self.tree.set(index, SPLIT);
                }
            }
        }
    }
}

impl Allocator for BuddyAlloc {
    fn alloc(&mut self, mut size: uint) -> (*mut u8, uint) {
        if size == 0 {
            size = 1;
        }
        // smallest power of 2 >= size
        let lg2_size = 32 - unsafe { ctlz32(size as i32 - 1) };
        size = 1 << lg2_size;

        let mut index = 0; // points to current tree node
        let mut level = self.order; // current height

        loop {
            match (self.tree.get(index), level == lg2_size as uint) {
                (UNUSED, true) => {
                    // Found appropriate unused node
                    self.tree.set(index, USED); // use
                    return unsafe {(
                        mut_offset(self.base, (index + 1 - (1 << (self.order - level))) as int << level),
                        size
                    )};
                }
                (UNUSED, false) => {
                    // This large node is unused, split it!
                    self.tree.set(index, SPLIT);
                    self.tree.set(index*2 + 1, UNUSED);
                    self.tree.set(index*2 + 2, UNUSED);
                    index = index * 2 + 1; // left child
                    level -= 1;
                },
                (SPLIT, false) => {
                    // Traverse
                    index = index * 2 + 1; // left child
                    level -= 1;
                },
                _ => loop {
                    // Get back
                    if index & 1 == 1 {
                        index += 1;
                        break;
                    }

                    level += 1;

                    if index == 0 {
                        out_of_memory();
                    }

                    index = (index + 1) / 2 - 1; // parent
                }
            }
        }
    }

    fn zero_alloc(&mut self, s: uint) -> (*mut u8, uint) {
        let (ptr, size) = self.alloc(s);
        unsafe { set_memory(ptr, 0, size); }
        (ptr, size)
    }

    fn realloc(&mut self, _: *mut u8, _: uint) -> (*mut u8, uint) {
        abort();
    }

    unsafe fn free(&mut self, ptr: *mut u8) {
        let mut length = 1 << self.order;
        let mut left = 0;
        let mut index = 0;
        // TODO: offset

        loop {
            match self.tree.get(index) {
                USED => {
                    // offset == left
                    self.combine(index);
                    return
                },
                UNUSED => { return },
                _ => {
                    length /= 2;
                    if (ptr as uint) < left + length {
                        index += index + 1;
                    }
                    else {
                        left += length;
                        index += index + 2;
                    }
                }
            }
        }
    }
}
