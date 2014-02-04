use core::option::{Option, None};

use super::cpu::interrupt;
use super::io;
use kernel;

pub static mut keydown: Option<extern fn(u32)> = None;

pub fn init() {
    unsafe {
        kernel::int_table.map(|t| {
            t.enable(interrupt::IRQ, keypress);
        });
    }
}

#[no_mangle]
pub unsafe fn keypress() {
    keydown.map(|f| {
        f(*io::UART0);
    });
}
