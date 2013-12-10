use platform::drivers::pic;
use super::{idt, exception};
use kernel::memory;
use core::mem::Allocator;

struct table {
    reg: *mut idt::reg,
    table: *mut idt::table
}

impl table {
    pub unsafe fn new() -> table {
        let (table, _) = memory::allocator.alloc(0x800);
        let (reg, _) = memory::allocator.alloc(6);
        *(reg as *mut idt::reg) = idt::reg::new(table as *idt::table);

        table {
            reg: reg as *mut idt::reg,
            table: table as *mut idt::table
        }
    }

    pub unsafe fn enable(&self, irq: u8, isr: extern "C" unsafe fn()) {
        (*self.table)[irq] = idt::entry::new(
            isr,
            1 << 3,
            idt::INTR_GATE | idt::PRESENT
        );

        pic::enable(irq);
    }

    pub unsafe fn load(&self) {
        (*self.table)[exception::PF] = idt::entry::new(exception::page_fault(), 1 << 3, idt::INTR_GATE | idt::PRESENT);

        idt::load(self.reg);
        pic::remap();
        asm!("sti" :::: "intel");
    }
}
