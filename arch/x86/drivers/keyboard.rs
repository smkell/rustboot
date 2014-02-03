use cpu::io;
use super::keydown;

pub static IRQ: u8 = 0x20 + 1;

pub static Layout: &'static str = "\
\x00\x1B1234567890-=\x08\
\tqwertyuiop[]\n\
\x00asdfghjkl;'`\
\x00\\zxcvbnm,./\x00\
*\x00 ";

static LayoutShift: &'static str = "\
\x00\x1B!@#$%^&*()_+\x08\
\tQWERTYUIOP{}\n\
\x00ASDFGHJKL:\"~\
\x00|ZXCVBNM<>?\x00\
*\x00 ";

static mut shift: bool = false;
static mut caps_lock: bool = false;
static mut led_state: u8 = 0;

fn led(state: u8) {
    io::wait(0x64, 2);
    io::out(0x60, 0xEDu8);
    io::wait(0x64, 2);
    unsafe {
        led_state ^= state;
        io::out(0x60, led_state);
    }
}

fn isalpha(c: u8) -> bool {
    ((c | 0x20) - 'a' as u8) < 26
}

#[no_split_stack]
fn keypress(code: u8) {
    match (code & 0x7f, code & 0x80 == 0) {
        (0x2A, down) | (0x36, down) => unsafe { shift = down },
        (0x3A, true) => unsafe { // Caps lock
            caps_lock = !caps_lock;
            led(0b100)
        },
        (0x45, true) => led(0b010), // Number lock
        (0x46, true) => led(0b001), // Scroll lock
        (c, true) if c < 0x3A => unsafe {
            // handle character
            keydown.map(|f| {
                let mut ch = if shift { LayoutShift[c] } else { Layout[c] };
                if ch != 0 {
                    if caps_lock && isalpha(ch) {
                        ch ^= 1 << 5;
                    }
                    f(ch);
                }
            });
        },
        _ => {}
    }
}

#[no_split_stack]
#[inline(never)]
pub unsafe fn isr_addr() -> extern "C" unsafe fn() {
    asm!("jmp skip_isr_addr
      isr_addr_asm:
          .word 0xa80f
          .word 0xa00f
          .byte 0x06
          .byte 0x1e
          pusha"
        :::: "intel");

          keypress(io::inb(0x60));
          io::out(0x20, 0x20u8);

    asm!("popa
          .byte 0x1f
          .byte 0x07
          .word 0xa10f
          .word 0xa90f
          iretd
      skip_isr_addr:"
        :::: "intel");

    // it must be referenced in code
    isr_addr_asm
}

extern "C" { fn isr_addr_asm(); }
