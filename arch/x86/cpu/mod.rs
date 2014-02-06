use core::mem::size_of;

use cpu::gdt::{Gdt, GdtEntry, SIZE_32, STORAGE, CODE_READ, DATA_WRITE};
use kernel;

mod gdt;
mod idt;
pub mod interrupt;
pub mod io;
mod exception;
pub mod mmu;

macro_rules! cpuid(
    ($n:expr, $s1:expr, $s2:expr, $s3:expr, $s4:expr) => (
        asm!("cpuid"
            : "=A"($s1),
              "={ebx}"($s2),
              "={edx}"($s3),
              "={ecx}"($s4)
            : "A"($n) :: "intel");
    );
    ($n:expr, *$s1:expr) => (
        cpuid!($n, (*$s1)[0], (*$s1)[1], (*$s1)[2], (*$s1)[3]);
    );
    ($n:expr, $s1:expr) => (
        asm!("cpuid"
            : "=A"($s1)
            : "A"($n) : "ebx", "edx", "ecx" : "intel");
    );
)

#[packed]
struct DtReg<T> {
    size: u16,
    addr: *T,
}

impl<T> DtReg<T> {
    pub fn new(descriptor_table: *T) -> DtReg<T> {
        DtReg {
            size: size_of::<T>() as u16,
            addr: descriptor_table,
        }
    }
}

pub fn init() {
    let t = Gdt::new();
    t.enable(1, GdtEntry::new(0, 0xFFFFF, SIZE_32 | STORAGE | CODE_READ, 0));
    t.enable(2, GdtEntry::new(0, 0xFFFFF, SIZE_32 | STORAGE | DATA_WRITE, 0));
    t.load();

    unsafe {
        kernel::int_table.map(|t| {
            use cpu::exception::{BREAKPOINT, exception_handler};
            use cpu::interrupt::{Isr, Fault};
            (*t.table)[BREAKPOINT as u8] = Isr::new(Fault(BREAKPOINT), false).idt_entry(exception_handler());
        });

        mmu::init();
    }
}

pub fn info() {
}
