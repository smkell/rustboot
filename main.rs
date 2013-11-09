#[link(name = "rustboot",
       vers = "0.1",
       license = "MIT")];

#[no_std];
#[feature(asm)];
#[feature(globs)];
#[feature(macro_rules)];

#[cfg(target_arch = "x86")]
use x86::*;
#[cfg(target_arch = "arm")]
use arm::*;

#[path = "rust-core/core/mod.rs"]
mod core;

mod kernel {
    pub mod int;
    pub mod memory;
}

#[cfg(target_arch = "x86")]
#[path = "arch/x86/"]
mod x86 {
    pub mod cpu;
    pub mod io;
    pub mod drivers;
}

#[cfg(target_arch = "arm")]
#[path = "arch/arm/"]
mod arm {
    pub mod cpu;
    pub mod io;
}

#[cfg(target_arch = "arm")]
#[path = "rust-core/support.rs"]
mod support;

#[cfg(target_arch = "x86")]
fn keydown(key: char) {
    unsafe {
        if key == '\n' {
            io::seek(80 - io::pos % 80);
        } else {
            io::write_char(key);
        }
    }
}

#[cfg(target_arch = "x86")]
#[lang="start"]
#[no_mangle]
pub unsafe fn main() {
    drivers::init();

    io::keydown(keydown);

    let table = cpu::interrupt::table::new();
    table.load();
    table.enable(drivers::keyboard::IRQ, drivers::keyboard::isr_addr());
}

#[cfg(target_arch = "arm")]
#[lang="start"]
#[no_mangle]
pub unsafe fn main() {
    let table = cpu::interrupt::table::new();
    table.load();
    table.enable(6, irq as u32);
}

#[cfg(target_arch = "arm")]
#[no_mangle]
pub unsafe fn irq() {
    io::write_char(*io::UART0 as u8 as char);
}
