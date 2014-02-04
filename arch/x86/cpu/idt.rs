use core::mem::size_of;

pub static PRESENT:   u8 = 1 << 7;
pub static INTR_GATE: u8 = 0b1110;
pub static TRAP_GATE: u8 = 0b1111;

pub type Idt = [IdtEntry, ..256];

#[packed]
pub struct IdtEntry {
    addr_lo: u16,
    sel: u16,
    zero: u8,
    flags: u8,
    addr_hi: u16
}

#[packed]
pub struct IdtReg {
    size: u16,
    addr: *Idt,
}

impl IdtEntry {
    pub fn new(func: extern "C" unsafe fn(), sel: u16, flags: u8) -> IdtEntry {
        let base = func as u32;
        IdtEntry {
            addr_lo: (base & 0xffff) as u16,
            addr_hi: (base >> 16) as u16,
            sel: sel,
            zero: 0,
            flags: flags
        }
    }
}

impl IdtReg {
    pub unsafe fn new(idt: *Idt) -> IdtReg {
        IdtReg {
            size: size_of::<Idt>() as u16,
            addr: idt,
        }
    }

    #[inline]
    pub fn load(&self) {
        unsafe {
            asm!("lidt [$0]" :: "A"(self) :: "intel");
        }
    }
}
