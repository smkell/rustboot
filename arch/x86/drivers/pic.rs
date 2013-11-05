pub unsafe fn remap() {
    asm!("mov al, 0x11
          outb 0x20
          outb 0xA0

          mov al, 0x20
          outb 0x21
          mov al, 0x28
          outb 0xA1

          mov al, 4
          outb 0x21
          mov al, 2
          outb 0xA1

          mov al, 1
          outb 0x21
          outb 0xA1

          mov al, 0xff
          outb 0x21
          outb 0xA1"
        ::: "al" : "volatile", "intel");
}

#[inline(never)]
pub unsafe fn enable(irq: u8) {
    let port: u16 = if (irq & 0b1000) == 0 { 0x21 } else { 0xa1 };
    let mask: u8 = !(1u8 << (irq & 0b111));

    asm!("mov dx, $0
          in al, dx
          and al, $1
          out dx, al"
        : : "r"(port), "r"(mask)
        : "al", "dx" : "intel")
}
