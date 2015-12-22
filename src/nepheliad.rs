#![no_std]

pub mod vga;

const VERSION: &'static str = "0.2.0";

#[no_mangle]
pub fn main() {
    vga::clear_screen(vga::Color::LightCyan as u16);
    vga::write_str("nepheliad v");
    vga::write_str(VERSION);
    vga::write_char('\n');
}

