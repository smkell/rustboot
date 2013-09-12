use drivers::keyboard;
use drivers::pic;
use kernel::exception;
use kernel::idt;

pub static TABLE: *mut idt::table = 0x100000 as *mut idt::table;

pub unsafe fn enable() {
    (*TABLE)[exception::PF] = idt::entry(exception::page_fault(), 1 << 3, idt::PM_32 | idt::PRESENT);
    //(*TABLE)[exception::DF] = idt::entry(exception::double_fault(), 1 << 3, idt::PM_32 | idt::PRESENT);

    let idt_reg = 0x100800 as *mut idt::reg;
    *idt_reg = idt::reg::new(TABLE);
    idt::load(idt_reg);

    pic::remap();
    pic::enable(keyboard::IRQ);

    asm!("sti" :::: "intel");
}
