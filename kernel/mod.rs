use core::option::{Option, Some, None};

use platform::{cpu, io, drivers};
use cpu::interrupt;
pub use cpu::interrupt::Table;

pub mod util;
pub mod mm;
pub mod heap;
#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod elf;

pub static mut int_table: Option<Table> = None;

#[lang="start"]
#[no_mangle]
pub fn main() {
    heap::init();
    mm::physical::init();

    let table = interrupt::Table::new();
    table.load();
    unsafe {
        int_table = Some(table);
        drivers::keydown = Some(io::putc);
    }
    cpu::init();

    drivers::init();
    elf::exec(&_binary_initram_elf_start);
    extern { static _binary_initram_elf_start: u8; }
}
