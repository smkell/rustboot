use core::option::{Option, Some, None};
use platform::{cpu, io, drivers};
use cpu::interrupt;

use self::memory::virtual::PageDirectory;

pub mod int;
pub mod ptr;
pub mod memory;
pub mod elf;

#[cfg(target_word_size = "32")]
pub mod rt;

pub static mut heap: memory::BuddyAlloc = memory::BuddyAlloc {
    base: 0x110_000,
    order: 15,
    storage: memory::Bitv { storage: 0x100_000 as memory::BitvStorage }
};

pub static mut int_table: Option<interrupt::Table> = None;
pub static mut page_dir: Option<*mut PageDirectory> = None;

pub fn keydown(key: char) {
    unsafe {
        io::write_char(key);
    }
}

#[lang="start"]
#[no_mangle]
pub fn main() {
    let table = interrupt::Table::new();
    unsafe {
        int_table = Some(table);
    }
    cpu::init();
    io::keydown(keydown);

    table.load();
    drivers::init();
    elf::exec(&initram);
}

extern { static initram: u8; }
