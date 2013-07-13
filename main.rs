#[link(name = "rustboot",
       vers = "0.1",
       license = "MIT")];

#[allow(ctypes)];
#[no_std];
#[no_core];

use rust::option::*;
use kernel::idt;
use drivers::cga;
use drivers::keyboard;
use drivers::pic;

mod rust {
    pub mod zero;
    pub mod option;
}

mod kernel {
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
    static mut pos: u32 = 0;

    if(code & (1 << 7) == 0) {
        unsafe {
            let char = ASCII_TABLE[code];
            if char == 8 && pos > 0 {
                pos -= 1;
                (*cga::SCREEN)[pos] &= 0xff00;
            } else if char == '\n' as u8 {
                pos += 80 - pos % 80;
            } else {
                (*cga::SCREEN)[pos] |= char as u16;
                pos += 1;
            }
        }
    }
}

#[lang="start"]
#[no_mangle]
pub unsafe fn main() {
    cga::clear_screen(cga::LightRed);
    // invalid deref when &fn?
    keyboard::callback = Some(keydown);

    let idt = 0x100000 as *mut idt::table;
    (*idt)[keyboard::IRQ] = idt::entry(keyboard::isr_addr(), 1 << 3, idt::PM_32 | idt::PRESENT);

    let idt_reg = 0x100800 as *mut idt::reg;
    *idt_reg = idt::reg::new(idt);
    idt::load(idt_reg);

    pic::remap();
    pic::enable(keyboard::IRQ);

    asm!("sti" :::: "intel");
}
