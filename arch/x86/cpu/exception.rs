use core::mem::{size_of, transmute};
use core::ptr::offset;
use platform::io;
use cpu::idt;
use kernel::allocator;
use kernel::memory::Allocator;

#[repr(u8)]
pub enum Fault {
    DIVIDE_ERROR = 0,
    NMI = 2,
    BREAKPOINT = 3,
    OVERFLOW = 4,
    BOUND_EXCEEDED = 5,
    INVALID_OPCODE = 6,
    NO_MATH_COPROCESSOR = 7,
    DOUBLE_FAULT = 8,
    COPROCESSOR_SEGMENT_OVERUN = 9,
    INVALID_TSS = 10,
    SEGMENT_NOT_PRESENT = 11,
    STACK_SEGMENT_FAULT = 12,
    GENERAL_PROTECTION = 13,
    PAGE_FAULT = 14,
    FLOATING_POINT_ERROR = 16,
    ALIGNMENT_CHECK = 17,
    MACHINE_CHECK = 18,
    SIMD_FP_EXCEPTION = 19,
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

#[no_split_stack]
#[inline(never)]
unsafe fn blue_screen(stack: *IsrStack) {
    io::puts("Exception ");
    io::puti((*stack).int_no as int);
    asm!("hlt");
}

#[packed]
pub struct Isr {
    push_dummy: u8, // push eax  // (only for exceptions without error codes)
    push: u8,       // push byte <imm>  // save int. number
    value: Fault,
    jmp: u8,        // jmp rel  // jump to the common handler
    rel: u32
}

impl Isr {
    pub unsafe fn new(val: Fault, code: bool) -> idt::IdtEntry {
        let (isr_ptr, _) = allocator.alloc(size_of::<Isr>());
        let isr = isr_ptr as *mut Isr;
        *isr = Isr {
            push_dummy: if code { 0x90 } else { 0x50 },   // [9]
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
