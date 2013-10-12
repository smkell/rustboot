use rust::int;
use drivers::vga;
use rust::zero;
use kernel::io;

pub static UNUSED: uint = 0;
pub static USED:   uint = 1;
pub static SPLIT:  uint = 2;
pub static FULL:   uint = 3;

struct BuddyAlloc {
    base: uint,
    order: uint,
    storage: *mut [u32, ..2048]
}

impl BuddyAlloc {
    pub unsafe fn new(base: uint, order: uint, storage: *mut [u32, ..2048]) -> BuddyAlloc {
        let this = BuddyAlloc {
            base: base,
            order: order,
            storage: storage
        };

        int::range(0, 2048, |i| {
            this.set(i, 0);
        });

        this
    }

    #[inline]
    unsafe fn get(&self, i: uint) -> uint {
        3 & ((*self.storage)[i / 16] as uint >> ((i % 16) * 2))
    }

    #[inline]
    unsafe fn set(&self, i: uint, x: uint) {
        let w = i / 16;
        let b = (i % 16) * 2;
        (*self.storage)[w] = (((*self.storage)[w] & !(3 << b)) | (x as u32 << b));
    }

    #[fixed_stack_segment]
    pub unsafe fn alloc(&self, s: uint) -> *u8 {
        let mut scr = 0;
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

        while index >= 0 {
            let mut cont: bool = true;

            if size == length {
                if self.get(index) == UNUSED { // if unused
                    self.set(index, 1); // use
                    return (self.base + ((index + 1 - (1 << level)) << (self.order - level))) as *u8;
                }
            }
            else {
                match self.get(index) {
                    UNUSED => {
                        self.set(index, SPLIT);
                        self.set(index*2 + 1, UNUSED);
                        self.set(index*2 + 2, UNUSED);
                        index = index * 2 + 1;
                        length /= 2;
                        level += 1;
                        cont = false;
                    },
                    SPLIT => {
                        index = index * 2 + 1;
                        length /= 2;
                        level += 1;
                        cont = false;
                    },
                    _ => ()
                }
            }

            if index & 1 == 1 && cont {
                index += 1;
                cont = false;
            }

            if cont {
                loop {
                    level -= 1;
                    length *= 2;
                    if index == 0 { zero::abort(); }
                    index = (index + 1) / 2 - 1;
                    if index & 1 == 1 { index += 1; break; }
                }
            }
        }

        zero::abort();
    }

    pub unsafe fn combine(&self, mut index: uint) {
        loop {
            let buddy = index + (index & 1) * 2;
            if buddy < 1 || self.get(buddy - 1) != UNUSED {
                self.set(index, UNUSED);
                while index >= 1 && self.get(index) == FULL {
                    index = (index + 1) / 2 - 1;
                    self.set(index, SPLIT);
                }
            }
        }
    }

    pub unsafe fn free(&self, offset: *u8) {
        let mut length = 1 << self.order;
        let mut left = 0;
        let mut index = 0;

        loop {
            match self.get(index) {
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

pub static mut MM: BuddyAlloc = BuddyAlloc { base: 0x10000, order: 12, storage: 0x105000 as *mut [u32, ..2048] };

pub unsafe fn malloc(size: uint) -> *u8 {
    MM.alloc(size)
}
