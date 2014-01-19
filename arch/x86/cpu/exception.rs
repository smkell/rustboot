use core::mem::{size_of, transmute};
use core::ptr::offset;
use platform::io;
use cpu::idt;
use kernel::allocator;
use kernel::memory::Allocator;

#[repr(u8)]
pub enum Fault {
    PF = 8,
    DF = 14
}

/*
#[lang="fail_"]
#[fixed_stack_segment]
pub fn fail(expr: *u8, file: *u8, line: uint) -> ! {
    unsafe {
        io::puts(0, expr);
        io::puts(80, file);
        io::puti(80*2, line as int);

        zero::abort();
    }
}

#[lang="fail_bounds_check"]
#[fixed_stack_segment]
pub fn fail_bounds_check(file: *u8, line: uint, index: uint, len: uint) {
    unsafe {
        io::puts(0, file);
        io::puti(80, line as int);
        io::puti(80*2, index as int);
        io::puti(80*3, len as int);

        zero::abort();
    }
}
*/

// exception info and processor state saved on stack
pub struct IsrStack {
    edi: u32, esi: u32, ebp: u32, esp: u32, ebx: u32, edx: u32, ecx: u32, eax: u32, // pushad last
    ds: u32, es: u32, fs: u32, gs: u32, // segment registers
    int_no: u32, err_code: u32, // added by ISRs
    eip: u32, cs: u32, eflags: u32, useresp: u32, ss: u32 // the cpu adds these on the interrupt
}

#[no_split_stack]
#[inline(never)]
unsafe fn blue_screen(stack: *IsrStack) {
    io::puts("Exception ");
    io::puti((*stack).int_no as int);
    asm!("hlt");
}

#[packed]
pub struct IsrWithCode {
    push: u8,
    value: Fault,
    jmp: u8,
    rel: u32
}

#[packed]
pub struct Isr {
    dec_esp: u8,
    push: u8,
    value: Fault,
    jmp: u8,
    rel: u32
}

impl IsrWithCode {
    pub unsafe fn new(val: Fault) -> idt::IdtEntry {
        let (isr_ptr, _) = allocator.alloc(size_of::<IsrWithCode>());
        let isr = isr_ptr as *mut IsrWithCode;
        *isr = IsrWithCode {
            push: 0x6a, value: val,
            jmp: 0xe9, rel: exception_handler() as u32 - offset(isr_ptr as *IsrWithCode, 1) as u32
        };
        idt::IdtEntry::new(transmute(isr_ptr), 1 << 3, idt::INTR_GATE | idt::PRESENT)
    }
}

impl Isr {
    pub unsafe fn new(val: Fault) -> idt::IdtEntry {
        let (isr_ptr, _) = allocator.alloc(size_of::<Isr>());
        let isr = isr_ptr as *mut Isr;
        *isr = Isr {
            dec_esp: 0x4c,
            push: 0x6a, value: val,
            jmp: 0xe9, rel: exception_handler() as u32 - offset(isr_ptr as *Isr, 1) as u32
        };
        idt::IdtEntry::new(transmute(isr_ptr), 1 << 3, idt::INTR_GATE | idt::PRESENT)
    }
}

#[no_split_stack]
#[inline(never)]
pub unsafe fn exception_handler() -> extern "C" unsafe fn() {
    // Points to the data on stack
    // WARN: local var should use registers
    let mut stack_ptr: *IsrStack;
    asm!("jmp skip_exception_handler
      exception_handler_asm:
          .word 0xa80f // push gs
          .word 0xa00f // push fs
          .byte 0x06 // push es
          .byte 0x1e // push ds
          pusha"
        : "={esp}"(stack_ptr) ::: "volatile", "intel");

            blue_screen(stack_ptr);

    asm!("popa
          .byte 0x1f
          .byte 0x07
          .word 0xa10f
          .word 0xa90f
          iretd
      skip_exception_handler:"
        :::: "volatile", "intel");

    exception_handler_asm
}

extern "C" { pub fn exception_handler_asm(); }
