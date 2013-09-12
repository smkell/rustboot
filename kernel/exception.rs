use rust::zero;
use rust::int;
use drivers::vga;

pub static PF: u8 = 8;
pub static DF: u8 = 14;

unsafe fn puts(j: int, buf: *u8) {
    let mut i = j;
    let mut curr = buf;
    while *curr != 0 {
        (*vga::SCREEN)[i].char = *curr;
        (*vga::SCREEN)[i].attr = 16;
        i += 1;
        curr = (curr as uint + 1) as *u8;
    }
}

unsafe fn puti(j: uint, num: int) {
    let mut i = j;
    int::to_str_bytes(num, 10, |n| {
        (*vga::SCREEN)[i].char = n;
        (*vga::SCREEN)[i].attr = 16;
        i += 1;
    });
}

#[lang="fail_"]
pub fn fail(expr: *u8, file: *u8, line: uint) -> ! {
    unsafe {
        puts(0, expr);
        puts(80, file);
        puti(80*2, line as int);

        zero::abort();
    }
}

#[lang="fail_bounds_check"]
pub fn fail_bounds_check(file: *u8, line: uint, index: uint, len: uint) {
    unsafe {
        puts(0, file);
        puti(80*2, line as int);
        puti(80*2, index as int);
        puti(80*3, len as int);

        zero::abort();
    }
}

#[no_mangle]
#[inline(never)]
pub unsafe fn ex14() {
    puti(0, 14);
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
