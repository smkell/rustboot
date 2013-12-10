#[link(name = "main",
       package_id = "rustboot",
       vers = "0.1",
       license = "MIT")];
#[crate_type = "lib"];
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

// do we already need memset? TODO: own implementation
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
pub fn main() {
    cpu::init();
    io::keydown(keydown);

    unsafe {
        let table = cpu::interrupt::table::new();
        table.load();
        drivers::init(table);
    }
}
