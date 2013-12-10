use kernel::int;
use platform::cpu::io;

pub enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Pink        = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    LightPink   = 13,
    Yellow      = 14,
    White       = 15,
}

#[packed]
pub struct character {
    char: u8,
    attr: u8,
}

pub static SCREEN_SIZE: uint = 80*25;
type screen = [character, ..SCREEN_SIZE];
pub static SCREEN: *mut screen = 0xb8000 as *mut screen;

pub unsafe fn clear_screen(background: Color) {
    int::range(0, 80*25, |i| {
        (*SCREEN)[i].attr = (background as u8) << 4;
    });
}

pub fn cursor_at(pos: uint) {
    io::out(0x3D4, 15);
    io::out(0x3D5, pos as u8);
    io::out(0x3D4, 14);
    io::out(0x3D5, (pos >> 8) as u8);
}
