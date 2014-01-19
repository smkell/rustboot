use core::option::{Option, None};
use platform::cpu;
pub mod int;
pub mod memory;

#[cfg(target_word_size = "32")]
pub mod rt;

pub static mut allocator: memory::BuddyAlloc = memory::BuddyAlloc {
    base: 0x110_000,
    order: 14,
    storage: memory::Bitv { storage: 0x100_000 as *mut [u32, ..0x8_000 / 4] }
};

pub static mut int_table: Option<cpu::interrupt::Table> = None;
