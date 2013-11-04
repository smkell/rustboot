use kernel::io;

extern {
    static vectors: [u32, ..10];
}

pub unsafe fn enable() {
    let mut i = 0;
    while i < 10 {
        *((i*4) as *mut u32) = vectors[i];
        i += 1;
    }

    *io::VIC_INTENABLE = 1 << 12;
    *io::UART0_IMSC = 1 << 4;
}

/*
#[lang="fail_"]
#[fixed_stack_segment]
pub fn fail(expr: *u8, file: *u8, line: uint) -> ! {
    unsafe { zero::abort(); }
}

#[lang="fail_bounds_check"]
#[fixed_stack_segment]
pub fn fail_bounds_check(file: *u8, line: uint, index: uint, len: uint) {
    unsafe { zero::abort(); }
}
*/
