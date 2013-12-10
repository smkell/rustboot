use super::drivers::vga;
use super::drivers::keyboard;
use kernel::int;
use core::option::Some;

static mut pos: int = 0;

unsafe fn seek(offset: int) {
    pos += offset;
    vga::cursor_at(pos as uint);
}

pub unsafe fn write_char(c: char) {
    if c == '\x08' {
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
    else if c == '\n' {
        seek(80 - pos % 80);
    }
    else {
        (*vga::SCREEN)[pos].char = c as u8;
        pos += 1;
    }

    vga::cursor_at(pos as uint);
}

pub fn keydown(f: extern fn(char)) {
    unsafe {
        keyboard::keydown = Some(f);
    }
}

pub unsafe fn puts(j: int, buf: *u8) {
    let mut i = j;
    let mut curr = buf;
    while *curr != 0 {
        (*vga::SCREEN)[i] = vga::character { char: *curr, attr: 16 };
        i += 1;
        curr = (curr as uint + 1) as *u8;
    }
}

pub unsafe fn puti(j: uint, num: int) {
    let mut i = j;
    int::to_str_bytes(num, 10, |n| {
        (*vga::SCREEN)[i] = vga::character { char: n, attr: 16 };
        i += 1;
    });
}
