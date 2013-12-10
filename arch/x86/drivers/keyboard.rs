use core::option::{Option, None};

pub static IRQ: u8 = 0x20 + 1;

pub static LAYOUT: &'static str = "\
\x00\x1B1234567890-=\x08\
\tqwertyuiop[]\n\
\x00asdfghjkl;'`\
\x00\\zxcvbnm,./\x00\
*\x00 ";

pub static mut keydown: Option<extern fn(char)> = None;

#[no_split_stack]
#[inline(never)]
unsafe fn keypress(code: u32) {
    if(code & (1 << 7) == 0) {
        keydown.map(|f| {
            f(LAYOUT[code] as char);
        });
    }
}

#[no_split_stack]
#[inline(never)]
pub unsafe fn isr_addr() -> extern "C" unsafe fn() {
    let mut code: u32;

    asm!("jmp skip_isr_addr
      isr_addr_asm:
          .word 0xa80f
          .word 0xa00f
          .byte 0x06
          .byte 0x1e
          pusha

          xor eax, eax
          in al, 60h"
        : "=A"(code) ::: "intel");
          keypress(code);
    asm!("mov dx, 20h
          mov al, dl
          out dx, al

          popa
          .byte 0x1f
          .byte 0x07
          .word 0xa10f
          .word 0xa90f
          iretd
      skip_isr_addr:"
        :::: "intel");

    isr_addr_asm
}

extern "C" { pub fn isr_addr_asm(); }
