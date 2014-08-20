#![no_std]
#![allow(ctypes)]

enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}

fn clear_screen(background: Color) {
    let mut i = 0u16;
    let max = 80 * 25;

    while i < max {
        unsafe {
            *((0xb8000 + i * 2) as *mut u16) = (background as u16) << 12;
        }
        i = i + 1;
    }
}

#[no_mangle]
#[no_split_stack]
pub fn main() {
    clear_screen(LightRed);
}
