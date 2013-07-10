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

pub static screen: *mut [u16, ..2000] = 0xb8000 as *mut [u16, ..2000];

pub unsafe fn clear_screen(background: Color) {
    range(0, 80*25, |i| {
        (*screen)[i] = (background as u16) << 12;
    });
}