#[link(name = "rustboot",
       vers = "0.1",
       license = "MIT")];

#[allow(ctypes)];
#[no_std];
#[no_core];

use rust::int;
use rust::option::*;
use kernel::*;
use drivers::*;

mod rust {
    pub mod zero;
    pub mod int;
    pub mod option;
}

mod kernel {
    pub mod cpu;
    pub mod idt;
}

mod drivers {
    pub mod cga;
    pub mod keyboard;
    pub mod pic;
}

pub static ASCII_TABLE: &'static str = "\
\x00\x1B1234567890-=\x08\
\tqwertyuiop[]\n\
\x00asdfghjkl;'`\
\x00\\zxcvbnm,./\x00\
*\x00 ";

fn keydown(code: u32) {
    // mutable statics are incorrectly dereferenced in PIC!
    static mut pos: uint = 0;

    if(code & (1 << 7) == 0) {
        unsafe {
            let char = ASCII_TABLE[code];
            if char == 8 {
                if pos > 0 { pos -= 1; }
                (*cga::SCREEN)[pos].char = 0;
            } else if char == '\n' as u8 {
                pos += 80 - pos % 80;
            } else {
                (*cga::SCREEN)[pos].char = char;
                pos += 1;
            }
            cga::cursor_at(pos);
        }
    }
}

#[lang="start"]
#[no_mangle]
pub unsafe fn main() {
    cga::clear_screen(cga::LightRed);
    cga::cursor_at(0);
    // invalid deref when &fn?
    keyboard::callback = Some(keydown);

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

    let idt_reg = 0x100800 as *mut idt::reg;
    *idt_reg = idt::reg::new(idt);
    idt::load(idt_reg);

    pic::remap();
    pic::enable(keyboard::IRQ);

    asm!("sti" :::: "intel");
}
