pub mod pic;
pub mod vga;
pub mod keyboard;

pub unsafe fn init() {
    vga::clear_screen(vga::LightRed);
    vga::cursor_at(0);
}
