//! The vector table.

use core::intrinsics::{offset, transmute, volatile_store};

use core::failure;
use core::fmt;

use platform::io;

static VIC_INT_ENABLE: *mut u32 = (0x10140000 + 0x010) as *mut u32;
static UART0_IRQ: u8 = 12;
static VT: *mut u32 = 0 as *mut u32; // WARNING verify should be mutable.

#[repr(u8)]
pub enum Int {
    Reset = 0,

    /// In ARM mode, an undefined opcode is used as a breakpoint to break
    /// execution[[7]].
    Undef,

    /// Software interrupt.
    SWI,
    PrefetchAbort,
    DataAbort,
    IRQ = 6,
    FIQ
}

fn set_word(vector: u8, instruction: u32) {
    unsafe {
        volatile_store(offset(VT, vector as int) as *mut u32, instruction);
    }
}

fn branch(rel: u32) -> u32 {
    // b isr ; branch instruction [1]
    0xea000000 | (((rel - 8) >> 2) & 0xffffff)
}

/// Exception handlers can be dynamically installed[[1]] into the vector table[[2]].
/// Interrupts must be unmasked with the `VIC_INT_ENABLE`[[3]] interrupt controller register[[4]].
///
/// Enabling interrupts[[5]].
///
/// In ARM mode, an undefined opcode is used as a breakpoint to break execution[[7]].
///
/// When the exception handler has completed execution, the processor restores the state so that the program can resume. The following instructions are used to leave an exception handler[[8]]:
///
/// | Exception | Return instruction |
/// |-----------|--------------------|
/// | UNDEF     | `movs pc, lr`      |
/// | IRQ, FIQ  | `subs pc, lr, #4`  |
///
/// [1]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0056d/Caccfahd.html
/// [2]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0203j/Cihdidh2.html
/// [3]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0273a/Cihiicbh.html
/// [4]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/I1042232.html
/// [5]: http://balau82.wordpress.com/2012/04/15/arm926-interrupts-in-qemu/ "ARM926 interrupts in QEMU"
/// [7]: http://stackoverflow.com/questions/11345371/how-do-i-set-a-software-breakpoint-on-an-arm-processor "How do I set a software breakpoint on an ARM processor? - Stack Overflow"
/// [8]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0222b/I3108.html "2.9.1. Exception entry and exit summary"
/// [6]: https://github.com/torvalds/linux/blob/
pub struct Table;

impl Table {
    pub fn new() -> Table {
        Table
    }

    #[allow(visible_private_types)]
    pub fn enable(&self, which: Int, isr: unsafe fn()) {
        // Installing exception handlers into the vectors directly [1]
        let vector: u8 = unsafe { transmute(which) };
        set_word(vector, branch(isr as u32 - (vector as u32 * 4)));
    }

    pub fn load(&self) {
        let mut i = 0;
        while i < 10 {
            // make every handler loop indefinitely
            set_word(i, branch(0));
            i += 1;
        }

        self.enable(Reset, unsafe { transmute(start) });
        // breakpoints use an UND opcode to trigger UNDEF. [7]
        self.enable(Undef, debug);

        unsafe {
            // Enable IRQs [5]
            asm!("mov r2, sp
              mrs r0, cpsr      // get Program Status Register
              bic r1, r0, #0x1F // go in IRQ mode
              orr r1, r1, #0x12
              msr cpsr, r1
              mov sp, 0x19000   // set IRQ stack
              bic r0, r0, #0x80 // Enable IRQs
              msr cpsr, r0      // go back in Supervisor mode
              mov sp, r2"
            ::: "r0", "r1", "r2", "cpsr");

            // enable UART0 IRQ [4]
            *VIC_INT_ENABLE = 1 << UART0_IRQ;
            // enable RXIM interrupt
            *io::UART0_IMSC = 1 << 4;
        }
    }
}

extern {
    fn start();
}

#[no_mangle]
pub unsafe fn debug() {
    asm!("movs pc, lr")
}

// TODO respect destructors
#[lang="begin_unwind"]
unsafe extern "C" fn begin_unwind(fmt: &fmt::Arguments, file: &str, line: uint) -> ! {
    loop { };
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
