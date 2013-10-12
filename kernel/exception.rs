use rust::zero;
use kernel::io;
use drivers::vga;

pub static PF: u8 = 8;
pub static DF: u8 = 14;

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

#[no_mangle]
#[inline(never)]
pub unsafe fn ex14() {
    io::puti(0, 14);
}

#[inline(never)]
pub unsafe fn page_fault() -> u32 {
    let mut ptr: u32 = 0;

    asm!("call n2
        n2: pop eax
          jmp skip2

          .word 0xa80f
          .word 0xa00f
          .byte 0x06
          .byte 0x1e
          pusha

          call ex14
          jmp .

          popa
          .byte 0x1f
          .byte 0x07
          .word 0xa10f
          .word 0xa90f
          iretd
      skip2:
          add eax, 6"
        : "=A"(ptr) ::: "intel");

    ptr
}
