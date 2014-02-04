use core::mem::{size_of, transmute};
use core::ptr::offset;

use cpu::exception::Fault;
use cpu::idt::{IdtEntry, IdtReg, Idt, INTR_GATE, PRESENT};
use drivers::pic;
use kernel;

pub enum Int {
    Fault(Fault)
}

pub struct Table {
    reg: &'static IdtReg,
    table: *mut Idt,
    mask: u16,
}

impl Table {
    pub fn new() -> Table {
        unsafe {
            let table = kernel::zero_alloc(size_of::<Idt>());
            let reg = kernel::malloc_raw(size_of::<IdtReg>());
            *(reg as *mut IdtReg) = IdtReg::new(table as *Idt);
            Table {
                reg: transmute(reg),
                table: table as *mut Idt,
                mask: 0xffff
            }
        }
    }

    pub unsafe fn enable_maskable(&mut self, irq: u8, isr: extern "C" unsafe fn()) {
        (*self.table)[irq] = IdtEntry::new(
            isr,                // interrupt service routine
            1 << 3,             // segment selector
            INTR_GATE | PRESENT // flags
        );

        self.mask &= !(1u16 << (irq & 0b1111));
        pic::mask(self.mask);
    }

    pub fn load(&self) {
        self.reg.load();
        pic::remap();
        pic::mask(self.mask);
        enable();
    }
}

fn enable() {
    unsafe {
        asm!("sti" :::: "volatile", "intel");
    }
}

// exception info and processor state saved on stack
pub struct IsrStack {
    // Registers saved by the ISR (in reverse order)
    edi: u32, esi: u32, ebp: u32, esp: u32, ebx: u32, edx: u32, ecx: u32, eax: u32,
    ds: u32, es: u32, fs: u32, gs: u32,
    int_no: u32,   // added by ISRs
    err_code: u32, // added by some exceptions
     // the cpu adds these when calling the ISR
    eip: u32, cs: u32, eflags: u32, useresp: u32, ss: u32
}

#[packed]
pub struct Isr {
    push_dummy: u8, // push eax  // (only for exceptions without error codes)
    push: u8,       // push byte <imm>  // save int. number
    value: Int,
    jmp: u8,        // jmp rel  // jump to the common handler
    rel: i32
}

impl Isr {
    pub fn new(val: Int, code: bool) -> &mut Isr {
        let this: &mut Isr = unsafe { transmute(kernel::malloc_raw(size_of::<Isr>())) };
        *this = Isr {
            push_dummy: if code { 0x90 } else { 0x50 },   // [9]
            push: 0x6a, value: val,
            jmp: 0xe9, rel: -5
        };
        this
    }

    pub unsafe fn idt_entry(&mut self, handler: extern "C" unsafe fn()) -> IdtEntry {
        self.rel = handler as i32 - offset(transmute::<&Isr, *Isr>(self), 1) as i32;
        IdtEntry::new(transmute(self), 1 << 3, INTR_GATE | PRESENT)
    }
}
