use cpu::idt::{IdtEntry, IdtReg, Idt, INTR_GATE, PRESENT};
use cpu::idt;
use drivers::pic;
use kernel::allocator;
use kernel::memory::Allocator;

pub struct Table {
    reg: *IdtReg,
    table: *mut Idt
}

impl Table {
    pub fn new() -> Table {
        unsafe {
            let (table, _) = allocator.alloc(0x800);
            let (reg, _) = allocator.alloc(6);
            *(reg as *mut IdtReg) = IdtReg::new(table as *Idt);
            Table { reg: reg as *IdtReg, table: table as *mut Idt }
        }
    }

    pub unsafe fn enable(&self, irq: u8, isr: extern "C" unsafe fn()) {
        (*self.table)[irq] = IdtEntry::new(
            isr,                // interrupt service routine
            1 << 3,             // segment selector
            INTR_GATE | PRESENT // flags
        );

        pic::enable(irq);
    }

    pub fn load(&self) {
        unsafe {
            idt::load(self.reg);
            pic::remap();
            asm!("sti" :::: "intel");
        }
    }
}
