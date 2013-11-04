#[link(name = "rustboot",
       vers = "0.1",
       license = "MIT")];

#[no_std];
#[feature(asm)];
#[feature(globs)];
#[feature(macro_rules)];

use kernel::*;
#[cfg(target_arch = "x86")]
use drivers::*;

#[path = "rust-core/core/mod.rs"]
mod core;

mod rust {
    pub mod int;
}

#[cfg(target_arch = "x86")]
mod kernel {
    pub mod interrupt;
    pub mod exception;
    pub mod idt;
    pub mod io;
    pub mod memory;
}

#[cfg(target_arch = "x86")]
mod drivers {
    pub mod vga;
    pub mod keyboard;
    pub mod pic;
}

#[cfg(target_arch = "arm")]
#[path = "arch/arm"]
mod kernel {
    pub mod interrupt;
    pub mod io;
}

#[cfg(target_arch = "x86")]
fn keydown(key: char) {
    unsafe {
        if key == '\n' {
            io::seek(80 - io::pos % 80);
        } else {
            io::write_char(key);
        }
        vga::cursor_at(io::pos as uint);
    }
}

#[cfg(target_arch = "x86")]
#[lang="start"]
#[no_mangle]
pub unsafe fn main() {
    vga::clear_screen(vga::LightRed);
    vga::cursor_at(0);

    io::keydown(keydown);

    let table = interrupt::table::new();
    table.load();
    table.enable(keyboard::IRQ, keyboard::isr_addr());
}

#[cfg(target_arch = "arm")]
#[lang="start"]
#[no_mangle]
pub unsafe fn main() {
    asm!("mov r2, sp
          mrs r0, cpsr
          bic r1, r0, #0x1F
          orr r1, r1, #0x12
          msr cpsr, r1
          mov sp, 0x19000
          bic r0, r0, #0x80
          msr cpsr, r0
          mov sp, r2"
        ::: "r0", "r1", "r2", "cpsr");

    interrupt::enable();
}

#[cfg(target_arch = "arm")]
#[no_mangle]
pub unsafe fn irq() {
    io::write_char(*io::UART0 as char);
}
