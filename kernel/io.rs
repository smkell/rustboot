use drivers::vga;
use drivers::keyboard;
use rust::option::Some;

pub static mut pos: int = 0;

pub unsafe fn seek(offset: int) {
    pos += offset;
}

pub unsafe fn write_char(c: char) {
    if c as u8 == 8 {
        if pos > 0 {
            if pos % 80 == 0 {
                while (*vga::SCREEN)[pos-1].char == 0 {
                    pos -= 1;
                }
            }
            else if pos > 0 {
                if pos > 0 { pos -= 1; }
                (*vga::SCREEN)[pos].char = 0;
            }
        }
    }
    else {
        (*vga::SCREEN)[pos].char = c as u8;
        pos += 1;
    }
}

pub unsafe fn keydown(f: extern fn(char)) {
    keyboard::keydown = Some(f);
}
