use cpu::interrupt;
use kernel;
pub mod pic;
pub mod vga;
pub mod keyboard;

pub fn init() {
    vga::clear_screen(vga::LightRed);
    vga::cursor_at(0);

    unsafe {
        kernel::int_table.map(|t| {
            t.enable(keyboard::IRQ, keyboard::isr_addr());
        });
    }
}
