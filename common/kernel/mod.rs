use core::option::{Option, Some, None};

use platform::{cpu, io, drivers};
use cpu::interrupt;
pub use cpu::interrupt::Table;

pub mod util;
pub mod mm;
pub mod heap;
mod process;
#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod elf;

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

pub static mut int_table: Option<Table> = None;

/// Called at the end of the bootloader. Starts in protected mode
/// with the following memory layout:
///
/// | Physical memory ranges | Size     | Description |
/// | ---------------------- | -------- | ----------- |
/// | 0x0000 ... 0x7BFF      | 31 KiB   | Stack       |
/// | 0x7C00 ... 0x7DFF      | 0.5 KiB  | Bootloader  |
/// | 0x07E00 ... 0x0FFFF    | 32.5 KiB | _unused_    |
/// | 0x10000 ... 0x1FFFF    | 64 KiB   | Kernel      |
#[lang="start"]
#[no_mangle]
pub fn main() {
    heap::init();
    mm::physical::init();

    let table = interrupt::Table::new();
    unsafe {
        table.load();
        int_table = Some(table);
        drivers::keydown = Some(io::putc);
    }
    cpu::init();

    drivers::init();
    elf::exec(&_binary_initram_elf_start);
    extern { static _binary_initram_elf_start: u8; }
}
