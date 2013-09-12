#[link(name = "rustboot",
       vers = "0.1",
       license = "MIT")];

#[allow(ctypes)];
#[no_std];
#[no_core];

use rust::int;
use rust::option::*;
use kernel::*;
#[cfg(target_arch = "x86")]
use drivers::*;

mod rust {
    pub mod zero;
    pub mod int;
    pub mod option;
}

#[cfg(target_arch = "x86")]
mod kernel {
    pub mod interrupt;
    pub mod exception;
    pub mod idt;
    pub mod io;
}

#[cfg(target_arch = "x86")]
mod drivers {
    pub mod cga;
    pub mod keyboard;
    pub mod pic;
}

#[cfg(target_arch = "arm")]
#[path = "arch/arm"]
mod kernel {
    pub mod interrupt;
    pub mod io;
}

#[cfg(target_arch = "x86")]
fn keydown(key: char) {
    unsafe {
        if key == '\n' {
            io::seek(80 - io::pos % 80);
        } else {
            io::write_char(key);
        }
        cga::cursor_at(io::pos as uint);
    }
}

#[cfg(target_arch = "x86")]
#[lang="start"]
#[no_mangle]
pub unsafe fn main() {
    cga::clear_screen(cga::LightRed);
    cga::cursor_at(0);

    io::keydown(keydown);

    keyboard::enable();
    interrupt::enable();
}

#[cfg(target_arch = "arm")]
#[lang="start"]
#[no_mangle]
pub unsafe fn main() {
    io::write_char('a');
    io::write_char('r');
    io::write_char('m');

    *io::VIC_INTENABLE = 1 << 12;
    *io::UART0_IMSC = 1 << 4;
}

extern {
    static vectors: [u32, ..8];
}

#[cfg(target_arch = "arm")]
#[no_mangle]
pub unsafe fn copy_vectors() {
    let mut i = 0;
    while i < 8 {
        *((i*4) as *mut u32) = vectors[i];
        i += 1;
    }
}

#[cfg(target_arch = "arm")]
#[no_mangle]
pub unsafe fn irq() {
    io::write_char(*io::UART0 as char);
}
