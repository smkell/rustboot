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
    pub mod cpu;
    pub mod idt;
    pub mod exception;
    pub mod paging;
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
fn keydown(key: u8) {
    // mutable statics are incorrectly dereferenced in PIC!
    static mut pos: uint = 0;

#[cfg(target_arch = "x86")]
    unsafe {
        if key == 8 {
            if pos > 0 { pos -= 1; }
            (*cga::SCREEN)[pos].char = 0;
        } else if key == '\n' as u8 {
            pos += 80 - pos % 80;
        } else {
            (*cga::SCREEN)[pos].char = key;
            pos += 1;
        }
        cga::cursor_at(pos);
    }
}

#[cfg(target_arch = "x86")]
#[lang="start"]
#[no_mangle]
pub unsafe fn main() {
    cga::clear_screen(cga::LightRed);
    cga::cursor_at(0);
    // invalid deref when &fn?
    keyboard::keydown = Some(keydown);

    let vendor = cpu::info();
    int::range(0, 12, |i| {
        (*cga::SCREEN)[i].char = vendor[i];
    });

    let mut i = 80;
    int::to_str_bytes(cpu::max as int, 10, |n| {
        (*cga::SCREEN)[i].char = n;
        i += 1;
    });

    let idt = 0x100000 as *mut idt::table;
    (*idt)[keyboard::IRQ] = idt::entry(keyboard::isr_addr(), 1 << 3, idt::PM_32 | idt::PRESENT);
    (*idt)[exception::PF] = idt::entry(exception::page_fault(), 1 << 3, idt::PM_32 | idt::PRESENT);

    let idt_reg = 0x100800 as *mut idt::reg;
    *idt_reg = idt::reg::new(idt);
    idt::load(idt_reg);

    pic::remap();
    pic::enable(keyboard::IRQ);

    paging::identity();
    paging::enable();

    asm!("sti" :::: "intel");
}

#[cfg(target_arch = "arm")]
#[lang="start"]
#[no_mangle]
pub unsafe fn main() {
    io::write_char('a');
    io::write_char('r');
    io::write_char('m');
}
