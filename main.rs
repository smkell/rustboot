#[link(name = "rustboot",
       vers = "0.1",
       license = "MIT")];

#[no_std];
#[feature(asm, globs, macro_rules)];

use platform::*;

#[path = "rust-core/core/mod.rs"]
mod core;

mod kernel {
    pub mod int;
    pub mod memory;
}

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

#[cfg(target_arch = "arm")]
#[path = "rust-core/support.rs"]
mod support;

fn keydown(key: char) {
    unsafe {
        io::write_char(key);
    }
}

#[lang="start"]
#[no_mangle]
pub unsafe fn main() {
    io::keydown(keydown);

    let table = cpu::interrupt::table::new();
    table.load();
    drivers::init(table);
}
