use rust::int;

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
struct character {
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

pub unsafe fn cursor_at(pos: uint) {
    asm!("mov al, 15
          mov dx, 0x3D4
          out dx, al
          mov al, bl
          inc dx
          out dx, al

          mov al, 14
          dec dx
          out dx, al
          mov al, bh
          inc dx
          out dx, al"
        :: "{bx}"(pos)
        : "al", "dx" : "intel")
}
