use core::mem::size_of;

pub type table = [entry, ..256];

#[packed]
pub struct reg {
    size: u16,
    addr: *table,
}

impl reg {
    pub unsafe fn new(idt: *table) -> reg {
        reg {
            addr: idt,
            size: size_of::<table>() as u16
        }
    }
}

#[inline]
pub unsafe fn load(reg: *mut reg) {
    asm!("lidt [$0]" :: "A"(reg) :: "intel");
}

#[packed]
pub struct entry {
    addr_lo: u16,
    sel: u16,
    zero: u8,
    flags: u8,
    addr_hi: u16
}

pub static PRESENT:   u8 = 1 << 7;
pub static INTR_GATE: u8 = 0b1110;

impl entry {
    pub fn new(func: extern "C" unsafe fn(), sel: u16, flags: u8) -> entry {
        let base = func as u32;
        entry {
            addr_lo: (base & 0xffff) as u16,
            sel: sel,
            zero: 0,
            flags: flags,
            addr_hi: (base >> 16) as u16
        }
    }
}
