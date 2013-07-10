pub unsafe fn remap() {
    asm!("
        mov al, 0x11
        mov dx, 0x20
        out dx, al
        mov dx, 0xA0
        out dx, al

        mov al, 0x20
        mov dx, 0x21
        mov bx, 0xA1
        out dx, al
        mov al, 0x28
        xchg bx, dx
        out dx, al

        mov al, 4
        xchg bx, dx
        out dx, al
        mov al, 2
        xchg bx, dx
        out dx, al

        mov al, 1
        xchg bx, dx
        out dx, al
        xchg bx, dx
        out dx, al

        mov al, 0xff
        xchg bx, dx
        out dx, al
        xchg bx, dx
        out dx, al"
        ::: "al", "bx", "dx" : "volatile", "intel");
}

#[inline(never)]
pub unsafe fn enable(irq: u8) {
    let port: u16 = if (irq & 0b1000) == 0 { 0x21 } else { 0xa1 };
    let mask: u8 = !(1u8 << (irq & 0b111));

    asm!("
        mov dx, $0
        in al, dx
        and al, $1
        out dx, al"
        :: "r"(port), "r"(mask) : "al", "dx" : "intel")
}
