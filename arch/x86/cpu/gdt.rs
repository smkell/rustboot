//! Global descriptor table

use core::ptr::RawPtr;
use core::mem::{size_of, transmute, uninitialized};
use core;

use cpu::DtReg;
use kernel::heap;

bitflags!(flags GdtAccess: u8 {
    static ACCESSED = 1 << 0,
    static EXTEND   = 1 << 1,
    static CONFORM  = 1 << 2,
    static CODE     = 1 << 3,
    static STORAGE  = 1 << 4, // not TSS

    static DPL0 = 0 << 5,
    static DPL1 = 1 << 5,
    static DPL2 = 2 << 5,
    static DPL3 = 3 << 5,

    static PRESENT  = 1 << 7,

    static DATA_WRITE = EXTEND.bits,
    static CODE_READ  = CODE.bits
                      | EXTEND.bits,
    static TSS        = 0b1001
})

bitflags!(flags GdtFlags: u8 {
    static SIZE_32  = 1 << 6,
    static PAGES    = 1 << 7
})

pub type GdtReg = DtReg<GdtEntry>;

#[repr(packed)]
pub struct GdtEntry {
    limit_lo: u16,
    base_lo: u16,
    base_hl: u8,
    access: GdtAccess,
    limit_hi_flags: u8,
    base_hh: u8
}

impl GdtEntry {
    pub fn new(base: u32, limit: u32, access: GdtAccess, flags: GdtFlags) -> GdtEntry {
        let (base_hh, base_hl, base_lo) = (
            (base  & 0xFF000000) >> 24,
            (base  & 0x__FF0000) >> 16,
            (base  & 0x____FFFF)
        );
        let (limit_hi, limit_lo) = (
            (limit & 0x___F0000) >> 16,
            (limit & 0x____FFFF)
        );
        GdtEntry {
            limit_lo: limit_lo as u16,
            base_lo:  base_lo as u16,
            base_hl: base_hl as u8,
            base_hh:  base_hh as u8,
            access: access,
            limit_hi_flags: flags.bits() | limit_hi as u8
        }
    }

    pub fn seg<T>(data: *mut T, access: GdtAccess, flags: GdtFlags) -> GdtEntry {
        assert!(size_of::<T>() < (1u << 20));
        GdtEntry::new(data as u32, size_of::<T>() as u32, access, flags)
    }

    pub fn flat(access: GdtAccess, flags: GdtFlags) -> GdtEntry {
        GdtEntry::new(0, 0xFFFFF, access, flags | PAGES)
    }
}

pub struct Gdt {
    reg: *mut GdtReg, // WARNING verify should be mutable.
    table: *mut GdtEntry
}

impl Gdt {
    pub fn new() -> Gdt {
        unsafe {
            let table_ptr = heap::zero_alloc::<GdtEntry>(256);
            let reg_ptr: *mut GdtReg = heap::alloc(1);

            let reg: &mut GdtReg = transmute(reg_ptr);
            *reg = DtReg::new(table_ptr, 256);

            Gdt { reg: transmute(reg_ptr), table: table_ptr }
        }
    }

    pub fn enable(&self, n: uint, mut entry: GdtEntry) {
        unsafe {
            entry.access = entry.access | PRESENT;
            *self.table.offset(n as int) = entry;
        }
    }

    pub unsafe fn disable(&self, n: uint) {
        let entry = self.table.offset(n as int);
        (*entry).access = (*entry).access & !PRESENT;
    }

    pub fn load(&self, code: u16, data: u16, local: u16) {
        unsafe {
            (*self.reg).load();
            asm!("mov ds, $0
                  mov ss, $0
                  mov fs, $1
                  mov gs, $1"
                :: "r"(data), "r"(local)
                :: "volatile", "intel");

            // A far jump updates the code segment.
            // Put the full address on the stack to `jmp far m16:32`.
            let address48: (u32, u16) = (uninitialized(), code);
            asm!("movl $$.flush, ($0)
                  ljmp *($0)
                  .flush:"
                :: "r"(&address48)
                :: "volatile");
        }
    }
}

impl super::Load for GdtEntry {
    #[inline]
    unsafe fn load(reg: &super::DtReg<GdtEntry>) {
        asm!("lgdt [$0]" :: "r"(reg) :: "intel");
    }
}
