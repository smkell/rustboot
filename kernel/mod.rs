use core::option::{Option, Some, None};
use core::fail::out_of_memory;

use platform::{cpu, io, drivers};
use cpu::interrupt;

use self::memory::virtual::PageDirectory;
use self::memory::Allocator;

pub mod int;
pub mod ptr;
pub mod memory;
pub mod elf;

#[cfg(target_word_size = "32")]
pub mod rt;

pub static mut heap: memory::BuddyAlloc = memory::BuddyAlloc {
    base: 0x110_000 as *mut u8,
    order: 17,
    tree: memory::Bitv { storage: 0x100_000 as memory::BitvStorage }
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
    memory::BuddyAlloc::new(0x110_000 as *mut u8, 17, memory::Bitv { storage: 0x100_000 as memory::BitvStorage });
    let table = interrupt::Table::new();
    unsafe {
        int_table = Some(table);
    }
    cpu::init();
    io::keydown(keydown);

    table.load();
    drivers::init();
    elf::exec(&_binary_boot_initram_elf_start);
}

extern { static _binary_boot_initram_elf_start: u8; }

#[lang = "exchange_malloc"]
pub unsafe fn malloc(size: uint) -> *mut u8 {
    if size == 0 {
        0 as *mut u8
    }
    else {
        let (ptr, sz) = heap.alloc(size);
        if sz == 0 {
            out_of_memory();
        }
        ptr
    }
}

#[lang = "exchange_free"]
pub unsafe fn free(ptr: *mut u8) {
    heap.free(ptr);
}
