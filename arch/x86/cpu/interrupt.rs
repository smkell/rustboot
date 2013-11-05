use x86::drivers::pic;
use x86::cpu::idt;
use x86::cpu::exception;
use kernel::memory;

struct table {
    reg: *mut idt::reg,
    table: *mut idt::table
}

impl table {
    pub unsafe fn new() -> table {
        let table = memory::malloc(0x800) as *mut idt::table;
        let reg = memory::malloc(6) as *mut idt::reg;
        *reg = idt::reg::new(table as *idt::table);

        table {
            reg: reg,
            table: table
        }
    }

    pub unsafe fn enable(&self, irq: u8, isr: u32) {
        (*self.table)[irq] = idt::entry::new(
            isr,
            1 << 3,
            idt::PM_32 | idt::PRESENT
        );

        pic::enable(irq);
    }

    pub unsafe fn load(&self) {
        (*self.table)[exception::PF] = idt::entry::new(exception::page_fault(), 1 << 3, idt::PM_32 | idt::PRESENT);

        idt::load(self.reg);
        pic::remap();
        asm!("sti" :::: "intel");
    }
}
