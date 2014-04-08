#![crate_id = "main#0.2.2"]
#![crate_type = "lib"]
#![no_std]
#![feature(asm, macro_rules, default_type_params, phase)]

#[phase(syntax, link)]
extern crate core;

#[cfg(target_arch = "x86")]
pub use platform::runtime::{memset, memcpy, memmove};
#[cfg(target_arch = "arm")]
pub use support::{memcpy, memmove};

use platform::cpu;
use kernel::util;

mod macros;

pub mod kernel;

#[cfg(target_arch = "arm")]
#[path = "rust-core/support.rs"]
mod support;

#[allow(dead_code)]
#[cfg(target_arch = "x86")]
#[path = "arch/x86/"]
mod platform {
    pub mod cpu;
    pub mod io;
    pub mod drivers;
    pub mod runtime;
}

#[allow(dead_code)]
#[cfg(target_arch = "arm")]
#[path = "arch/arm/"]
mod platform {
    pub mod cpu;
    pub mod io;
    pub mod drivers;
}
