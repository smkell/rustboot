#[crate_id = "main#0.2.1"];
#[crate_type = "lib"];
#[no_std];
#[feature(asm, globs, macro_rules)];

extern mod core;

use platform::{cpu, io, drivers};

pub mod kernel;

#[cfg(target_arch = "x86")]
#[path = "arch/x86/"]
mod platform {
    pub mod cpu;
    pub mod io;
    pub mod drivers;
}

#[cfg(target_arch = "arm")]
#[path = "arch/arm/"]
mod platform {
    pub mod cpu;
    pub mod io;
    pub mod drivers;
}

#[lang="start"]
#[no_mangle]
pub fn main() {
    let table = cpu::interrupt::Table::new();
    unsafe {
        kernel::int_table = core::option::Some(table);
    }
    cpu::init();
    io::keydown(kernel::keydown);

    table.load();
    drivers::init();
    kernel::elf::exec();
}
