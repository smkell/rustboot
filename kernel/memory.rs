use core::fail::abort;

extern "C" {
    pub fn memset(s: *mut u8, c: i32, n: u32);
}

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

// a special kind of bit vector...
pub struct Bitv {
    storage: *mut [u32, ..0x8_000 / 4]
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
        0x8_000
    }
}

pub static UNUSED: uint = 0;
pub static USED:   uint = 1;
pub static SPLIT:  uint = 2;
pub static FULL:   uint = 3;

pub struct BuddyAlloc {
    base: uint,
    order: uint,
    storage: Bitv
}

impl BuddyAlloc {
    pub unsafe fn new(base: uint, order: uint, storage: Bitv) -> BuddyAlloc {
        memset(storage.to_bytes(), 0, storage.size() as u32);

        let this = BuddyAlloc {
            base: base,
            order: order,
            storage: storage
        };

        this
    }

    pub unsafe fn combine(&self, mut index: uint) {
        loop {
            let buddy = index + (index & 1) * 2;
            if buddy < 1 || self.storage.get(buddy - 1) != UNUSED {
                self.storage.set(index, UNUSED);
                while index >= 1 && self.storage.get(index) == FULL {
                    index = (index + 1) / 2 - 1;
                    self.storage.set(index, SPLIT);
                }
            }
        }
    }
}

impl Allocator for BuddyAlloc {
    fn alloc(&mut self, s: uint) -> (*mut u8, uint) {
        let mut size = s;

        if size == 0 {
            size = 1;
        }
        else if size & (size-1) != 0 {
            size |= size >> 1;
            size |= size >> 2;
            size |= size >> 4;
            size |= size >> 8;
            size |= size >> 16;
            size += 1;
        }

        let mut length = 1 << self.order;
        let mut index = 0;
        let mut level = 0;

        loop {
            if length == size {
                if self.storage.get(index) == UNUSED { // if unused
                    self.storage.set(index, USED); // use
                    return (
                        (self.base + ((index + 1 - (1 << level)) << (self.order - level))) as *mut u8,
                        size
                    );
                }
            }
            else {
                match self.storage.get(index) {
                    UNUSED => {
                        self.storage.set(index, SPLIT);
                        self.storage.set(index*2 + 1, UNUSED);
                        self.storage.set(index*2 + 2, UNUSED);
                        index = index * 2 + 1;
                        length /= 2;
                        level += 1;
                        continue
                    },
                    SPLIT => {
                        index = index * 2 + 1;
                        length /= 2;
                        level += 1;
                        continue
                    },
                    _ => ()
                }
            }

            if index & 1 == 1 {
                index += 1;
            }
            else {
                loop {
                    level -= 1;
                    length *= 2;

                    if index == 0 { abort(); }

                    index = (index + 1) / 2 - 1;
                    if index & 1 == 1 {
                        index += 1;
                        break;
                    }
                }
            }
        }
    }

    unsafe fn zero_alloc(&mut self, s: uint) -> (*mut u8, uint) {
        let (ptr, size) = self.alloc(s);
        memset(ptr, 0, size as u32);
        (ptr, size)
    }

    fn realloc(&mut self, _: *mut u8, _: uint) -> (*mut u8, uint) {
        abort();
    }

    unsafe fn free(&mut self, offset: *mut u8) {
        let mut length = 1 << self.order;
        let mut left = 0;
        let mut index = 0;

        loop {
            match self.storage.get(index) {
                USED => {
                    // offset == left
                    self.combine(index);
                    return
                },
                UNUSED => { return },
                _ => {
                    length /= 2;
                    if (offset as uint) < left + length {
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
