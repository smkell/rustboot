use arm::io;

struct table;

impl table {
    pub unsafe fn new() -> table {
        table
    }

    pub unsafe fn enable(&self) {
        *io::VIC_INTENABLE = 1 << 12;
        *io::UART0_IMSC = 1 << 4;
    }

    pub unsafe fn load(&self) {
        asm!("mov r2, sp
          mrs r0, cpsr
          bic r1, r0, #0x1F
          orr r1, r1, #0x12
          msr cpsr, r1
          mov sp, 0x19000
          bic r0, r0, #0x80
          msr cpsr, r0
          mov sp, r2"
        ::: "r0", "r1", "r2", "cpsr");

        let mut i = 0;
        while i < 10 {
            *((i*4) as *mut u32) = vectors[i];
            i += 1;
        }
    }
}

extern {
    static vectors: [u32, ..10];
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
