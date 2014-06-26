use cpu::io;

pub fn remap() {
    io::out(0x20, 0x11);
    io::out(0xA0, 0x11);

    io::out(0x21, 0x20);
    io::out(0xA1, 0x28);

    io::out(0x21, 4);
    io::out(0xA1, 2);

    io::out(0x21, 1);
    io::out(0xA1, 1);
}

pub fn enable(irq: u8) {
    let port: u16 = if (irq & 0b1000) == 0 { 0x21 } else { 0xa1 };
    let mask: u8 = !(1u8 << (irq & 0b111) as uint);

    io::out(port, io::inb(port) & mask);
}

pub fn mask(mask: u16) {
    io::out(0x21, (mask & 0xFF) as u8);
    io::out(0xA1, ((mask >> 8) & 0xFF) as u8);
}
