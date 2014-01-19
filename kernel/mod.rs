use core::option::{Option, None};
use platform::cpu;
use platform::io;

pub mod int;
pub mod memory;

#[cfg(target_word_size = "32")]
pub mod rt;

pub static mut allocator: memory::BuddyAlloc = memory::BuddyAlloc {
    base: 0x110_000,
    order: 15,
    storage: memory::Bitv { storage: 0x100_000 as memory::BitvStorage }
};

pub static mut int_table: Option<cpu::interrupt::Table> = None;

pub fn keydown(key: char) {
    unsafe {
        io::write_char(key);
    }
}
