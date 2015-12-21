#![no_std]

mod vga;

#[no_mangle]
pub fn main() {
    vga::clear_screen(vga::Color::LightCyan as u16);
}
