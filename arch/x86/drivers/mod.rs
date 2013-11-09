use cpu::interrupt;
pub mod pic;
pub mod vga;
pub mod keyboard;

pub unsafe fn init(table: interrupt::table) {
    vga::clear_screen(vga::LightRed);
    vga::cursor_at(0);

    table.enable(keyboard::IRQ, keyboard::isr_addr());
}
