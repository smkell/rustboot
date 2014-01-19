use super::cpu::interrupt;
use super::io;
use core::option::{Option, None};
use kernel;

pub fn init() {
    unsafe {
        kernel::int_table.map(|t| {
            t.enable(interrupt::IRQ, keypress as u32);
        });
    }
}

pub static mut keydown: Option<extern fn(char)> = None;

#[no_mangle]
pub unsafe fn keypress() {
    keydown.map(|f| {
        f(*io::UART0 as u8 as char);
    });

    asm!("pop {r11, lr}
          subs pc, lr, #4");
}
