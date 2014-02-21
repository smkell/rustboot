use core::mem::{size_of, transmute};

use cpu::DtReg;
use kernel;

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

type Table = [GdtEntry, ..256];
pub type GdtReg = DtReg<Table>;

#[packed]
pub struct GdtEntry {
    limit_lo: u16,
    base_lo: u16,
    base_mid: u8,
    access: u8,
    limit_hi_flags: u8,
    base_hi: u8
}

impl GdtEntry {
    pub fn new(base: u32, limit: u32, access: u16, dpl: u8) -> GdtEntry {
        GdtEntry {
            limit_lo: (limit & 0xffff) as u16,
            base_lo: (base & 0xffff) as u16,
            base_mid: ((base >> 16) & 0xff) as u8,
            base_hi: ((base >> 24) & 0xff) as u8,
            access: access as u8,
            limit_hi_flags: (dpl << 5) | ((limit >> 16) & 0xf) as u8 | ((access >> 8) & 0xf0) as u8
        }
    }

    pub fn seg<T>(data: *mut T, access: u16, dpl: u8) -> GdtEntry {
        GdtEntry::new(data as u32, size_of::<T>() as u32, access, dpl)
    }

    pub fn flat(access: u16, dpl: u8) -> GdtEntry {
        GdtEntry::new(0, 0xFFFFF, access | PAGES, dpl)
    }
}

pub struct Gdt {
    reg: *GdtReg,
    table: *mut Table
}

impl Gdt {
    pub fn new() -> Gdt {
        unsafe {
            let table_ptr = kernel::zero_alloc(size_of::<Table>()) as *Table;
            let reg_ptr = kernel::malloc_raw(size_of::<GdtReg>());

            let reg: &mut GdtReg = transmute(reg_ptr);
            *reg = DtReg::new(table_ptr);

            Gdt { reg: transmute(reg_ptr), table: transmute(table_ptr) }
        }
    }

    pub fn enable(&self, n: u8, entry: GdtEntry) {
        unsafe {
            (*self.table)[n] = entry;
            (*self.table)[n].access |= PRESENT;
        }
    }

    pub unsafe fn disable(&self, n: u8) {
        (*self.table)[n].access &= !PRESENT;
    }

    pub fn load(&self, code: u16, data: u16, local: u16) {
        unsafe {
            asm!("lgdt [$0]
                  mov ds, $1
                  mov ss, $1
                  mov fs, $2
                  mov gs, $2"
                :: "r"(self.reg), "r"(data), "r"(local)
                :: "volatile", "intel");
            asm!("jmp $0, $$.flush; .flush:" :: "Ir"(code) :: "volatile")
        }
    }
}
