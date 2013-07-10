pub type table = [entry, ..256];

#[packed]
pub struct reg {
    size: u16,
    addr: *mut table,
}

#[packed]
pub struct entry {
    addr_lo: u16,
    sel: u16,
    zero: u8,
    flags: u8,
    addr_hi: u16
}

pub static Present: u8 = 1 << 7;
pub static PM32Bit: u8 = 1 << 3;

pub fn entry(proc: u32, sel: u16, flags: u8) -> entry {
    entry {
        addr_lo: (proc & 0xffff) as u16,
        sel: sel,
        zero: 0,
        flags: flags | 0b110,
        addr_hi: (proc >> 16) as u16
    }
}
