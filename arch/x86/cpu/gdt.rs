use core::mem::size_of;
use kernel::allocator;

type entries = [entry, ..256];

#[packed]
pub struct reg {
    size: u16,
    addr: *entries,
}

impl reg {
    pub unsafe fn new(idt: *entries) -> reg {
        reg {
            addr: idt,
            size: size_of::<entries>() as u16
        }
    }
}

#[packed]
pub struct entry {
    limit_lo: u16,
    base_lo: u16,
    base_mid: u8,
    access: u8,
    limit_hi_flags: u8,
    base_hi: u8
}

pub static SIZE_32: u16 = 1 << 14;
pub static PAGES:   u16 = 1 << 15;
pub static ACCESSED:   u16 = 1 << 0;
pub static EXTEND:     u16 = 1 << 1;
pub static CONFORM:    u16 = 1 << 2;
pub static CODE:       u16 = 1 << 3;
pub static STORAGE:    u16 = 1 << 4;
pub static PRESENT:    u8  = 1 << 7;
pub static CODE_READ:  u16 = CODE | EXTEND;
pub static DATA_WRITE: u16 = EXTEND;

impl entry {
    pub fn new(base: u32, limit: u32, other: u16, dpl: u8) -> entry {
        entry {
            limit_lo: (limit & 0xffff) as u16,
            base_lo: (base & 0xffff) as u16,
            base_mid: (base >> 16) as u8,
            access: other as u8,
            limit_hi_flags: (dpl << 5) | ((limit >> 16) & 0xf) as u8 | ((other >> 8) & 0xf0) as u8,
            base_hi: (base >> 24) as u8
        }
    }
}

pub struct table {
    reg: *mut reg,
    table: *mut entries
}

impl table {
    pub fn new() -> table {
        unsafe {
            let (table, _) = allocator.alloc(0x800);
            let (reg, _) = allocator.alloc(6);
            *(reg as *mut reg) = reg::new(table as *entries);
            *(table as *mut u64) = 0;

            table {
                reg: reg as *mut reg,
                table: table as *mut entries
            }
        }
    }

    pub fn enable(&self, n: u8, e: entry) {
        let mut m = e;
        m.access |= PRESENT;
        unsafe {
            (*self.table)[n] = m;
        }
    }

    pub unsafe fn disable(&self, n: u8) {
        (*self.table)[n].access &= !PRESENT;
    }

    pub fn load(&self) {
        unsafe {
            asm!("lgdt [$0]" :: "A"(self.reg) :: "intel");
        }
    }
}
