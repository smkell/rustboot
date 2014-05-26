// bits of things mined from rust-core

mod c_types;
mod fail;

#[cfg(target_arch = "x86")]
#[macro_escape]
mod macros;

#[cfg(target_arch = "arm")]
mod support;
