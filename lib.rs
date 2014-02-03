#[crate_id = "main#0.2.1"];
#[crate_type = "lib"];
#[no_std];
#[feature(asm, macro_rules)];

extern mod core;

#[cfg(target_arch = "x86")]
pub use platform::runtime::{memset, memcpy, memmove};
#[cfg(target_arch = "arm")]
pub use support::{memcpy, memmove};

use platform::{cpu, io, drivers};

pub mod kernel;

#[cfg(target_arch = "arm")]
#[path = "rust-core/support.rs"]
mod support;

#[cfg(target_arch = "x86")]
#[path = "arch/x86/"]
mod platform {
    pub mod cpu;
    pub mod io;
    pub mod drivers;
    pub mod runtime;
}

#[cfg(target_arch = "arm")]
#[path = "arch/arm/"]
mod platform {
    pub mod cpu;
    pub mod io;
    pub mod drivers;
}
