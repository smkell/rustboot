#[link(name = "main",
       package_id = "rustboot",
       vers = "0.2",
       license = "MIT")];
#[crate_type = "lib"];
#[no_std];
#[feature(asm, globs, macro_rules)];

extern mod core;

use platform::{cpu, io, drivers};
use kernel::memory::Allocator;

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
