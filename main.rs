#[allow(ctypes)];
#[no_std];
#[no_core];

mod zero;

#[inline]
pub fn size_of_val<T>(_val: *mut T) -> uint {
    unsafe { zero::size_of::<T>() }
}

#[packed]
struct idt_reg {
    size: u16,
    addr: *mut [idt_entry, ..256],
}

static Present: u8 = 1 << 7;
static PM32Bit: u8 = 1 << 3;

#[packed]
struct idt_entry {
    addr_lo: u16,
    sel: u16,
    zero: u8,
    flags: u8,
    addr_hi: u16
}

fn idt_entry(proc: u32, sel: u16, flags: u8) -> idt_entry {
    idt_entry {
        addr_lo: (proc & 0xffff) as u16,
        sel: sel,
        zero: 0,
        flags: flags | 0b110,
        addr_hi: (proc >> 16) as u16
    }
}

enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Pink        = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    LightPink   = 13,
    Yellow      = 14,
    White       = 15,
}

fn range(lo: uint, hi: uint, it: &fn(uint)) {
    let mut iter = lo;
    while iter < hi {
        it(iter);
        iter += 1;
    }
}

unsafe fn clear_screen(background: Color) {
    range(0, 80*25, |i| {
        *((0xb8000 + i * 2) as *mut u16) = (background as u16) << 12;
    });
}

static KeyboardIRQ: u8 = 0x20 + 1;

unsafe fn pic_remap() {
    asm!("
        mov al, 0x11
        out 0x20, al
        out 0xa0, al

        mov al, 0x20
        out 0x21, al
        mov al, 0x28
        out 0xa1, al

        mov al, 4
        out 0x21, al
        mov al, 2
        out 0xa1, al

        mov al, 1
        out 0x21, al
        out 0xa1, al

        mov al, 0xff
        out 0x21, al
        out 0xa1, al"
        :::: "intel");
}

unsafe fn pic_enable(irq: u8) {
    let port: u16 = if (irq & 0b1000) == 0 { 0x21 } else { 0xa1 };
    let mask: u8 = !(1u8 << (irq & 0b111));

    asm!("
        mov dx, $0
        in al, dx
        and al, $1
        out dx, al"
        :: "r"(port), "r"(mask) : "al", "dx" : "intel")
}

#[no_mangle]
extern "C" fn keyup(code: u32) { }

#[no_mangle]
extern "C" fn keydown(code: u32) {
    // wtf?
    let char: u8 = "\
\x00\x1B1234567890-=\x08\
\tqwertyuiop[]\n\
\x00asdfghjkl;'`\
\x00\\zxcvbnm,./\x00\
*\x00 "[code];

    let screen = 0xb8000 as *mut [u16, ..2000];
    // mutable statics are incorrectly dereferenced!
    static mut pos: u32 = 0x10000;

    unsafe {
        if char == 8 && pos > 0 {
            pos -= 1;
            (*screen)[pos] &= 0xff00;
        } else if char == '\n' as u8 {
            pos += 80 - pos % 80;
        } else {
            (*screen)[pos] |= char as u16;
            pos += 1;
        }
    }
}

#[no_mangle]
pub unsafe fn main() {
    clear_screen(LightRed);

    let idt = 0x100000 as *mut [idt_entry, ..256];

    let mut ptr: u32 = 0;

    asm!("
        call n
    n:  pop eax
        jmp skip

        push gs
        push fs
        push es
        push ds
        pusha

        xor eax, eax
        in al, 60h
        bt ax, 7
        jb up

    down:
        push eax
        call keydown
        add esp, 4
        jmp end

    up:
        push eax
        call keyup
        add esp, 4

    end:
        mov al, 20h
        out 0x20, al

        popa
        pop ds
        pop es
        pop fs
        pop gs
        iretd

    skip:
        add eax, 3"
        : "=A"(ptr) ::: "intel");

    (*idt)[KeyboardIRQ] = idt_entry(ptr, 1 << 3, PM32Bit | Present);

    let idt_table = 0x100800 as *mut idt_reg;
    *idt_table = idt_reg {
        addr: idt,
        size: size_of_val(idt) as u16
    };

    pic_remap();
    pic_enable(KeyboardIRQ);

    asm!("
        lidt [$0]
        sti"
        :: "n"(idt_table) :: "intel");
}
