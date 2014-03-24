use core::mem::transmute;
use core::ptr::set_memory;
use util::ptr::mut_offset;

// vector of 2-bit
pub struct Bitv {
    storage: *mut u32
}

impl Bitv {
    #[inline]
    pub fn get(&self, i: uint) -> u8 {
        let w = (i / 16) as int;
        let b = (i % 16) * 2;
        unsafe {
            transmute((*mut_offset(self.storage, w) as uint >> b) as u8 & 3)
        }
    }

    #[inline]
    pub fn set(&self, i: uint, x: u8) {
        let w = (i / 16) as int;
        let b = (i % 16) * 2;
        unsafe {
            *mut_offset(self.storage, w) &=  !(3 << b) | (x as u32 << b)
        }
    }

    #[inline]
    fn as_mut_ptr(&self) -> *mut u8 {
        self.storage as *mut u8
    }

    pub fn clear(&self, capacity: uint) {
        unsafe {
            set_memory(self.as_mut_ptr(), 0, capacity / 4);
        }
    }
}
