use core::option::Some;
use core::mem::volatile_store;
use super::drivers;

pub static UART0: *mut u32 = 0x101f1000 as *mut u32;
pub static UART0_IMSC: *mut u32 = (0x101f1000 + 0x038) as *mut u32;

pub static VIC_INTENABLE: *mut u32 = (0x10140000 + 0x010) as *mut u32;

pub unsafe fn write_word(c: u32) {
    volatile_store(UART0, c);
}

pub unsafe fn write_char(c: char) {
    volatile_store(UART0, c as u32);
}

pub fn keydown(f: extern fn(char)) {
    unsafe {
        drivers::keydown = Some(f);
    }
}
