#![no_std]

mod vga;

#[no_mangle]
pub fn main() {
    vga::clear_screen(vga::Color::LightCyan as u16);
    vga::write_char('n', 0, 0);
    vga::write_char('e', 1, 0);
}
