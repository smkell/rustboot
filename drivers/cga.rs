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

fn range(lo: uint, hi: uint, it: &fn(uint)) {
    let mut iter = lo;
    while iter < hi {
        it(iter);
        iter += 1;
    }
}

pub static SCREEN: *mut [u16, ..2000] = 0xb8000 as *mut [u16, ..2000];

pub unsafe fn clear_screen(background: Color) {
    int::range(0, 80*25, |i| {
        (*SCREEN)[i] = (background as u16) << 12;
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
