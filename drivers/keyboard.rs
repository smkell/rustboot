use rust::option::*;

pub static IRQ: u8 = 0x20 + 1;

pub static LAYOUT: &'static str = "\
\x00\x1B1234567890-=\x08\
\tqwertyuiop[]\n\
\x00asdfghjkl;'`\
\x00\\zxcvbnm,./\x00\
*\x00 ";

pub static mut keydown: Option<extern fn(u8)> = None;

#[inline(never)]
#[no_mangle]
pub extern "C" fn keypress(code: u32) {
    unsafe {
        if(code & (1 << 7) == 0 && keydown.is_some()) {
            keydown.get()(LAYOUT[code]);
        }
    }
}

#[inline(never)]
pub unsafe fn isr_addr() -> u32 {
	let mut ptr: u32 = 0;

    asm!("call n
      n:  pop eax
          jmp skip

          .word 0xa80f
          .word 0xa00f
          .byte 0x06
          .byte 0x1e
          pusha

          xor eax, eax
          in al, 60h

          push eax
          call keypress
          add esp, 4
  
          mov dx, 20h
          mov al, dl
          out dx, al

          popa
          .byte 0x1f
          .byte 0x07
          .word 0xa10f
          .word 0xa90f
          iretd
      skip:
          add eax, 6"
        : "=A"(ptr) ::: "intel");

    ptr
}
