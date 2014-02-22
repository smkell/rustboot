use core::mem::size_of;
use core::option::{Option, None, Some};
use core;

use cpu::gdt::{Gdt, GdtEntry, SIZE_32, STORAGE, CODE_READ, DATA_WRITE};
use util::rt;
use util::ptr::mut_offset;
use kernel;

mod gdt;
mod idt;
mod tss;
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

define_flags!(Eflags: u32 {
    CF,
    IF = 1 << 9
})

impl Eflags {
    fn read() -> Eflags {
        unsafe {
            let mut flags: u32;
            asm!("pushf; pop $0;" : "=r"(flags) ::: "volatile")
            Eflags(flags)
        }
    }
}

define_flags!(CR0Flags: u32 {
    CR0_PG = 1 << 31
})

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

// TODO: make push_dummy push ds?
// exception info and processor state saved on stack
struct Context {
    // Registers saved by the ISR (in reverse order)
    edi: u32, esi: u32, ebp: u32, esp: u32, ebx: u32, edx: u32, ecx: u32, eax: u32,
    ds: u32, es: u32, fs: u32, gs: u32,
    int_no: u32,   // added by ISRs
    err_code: u32, // added by some exceptions
    call_stack: IsrCallStack
}

// the cpu adds these when calling the ISR
struct IsrCallStack {
    eip: u32, cs: u32, eflags: u32, esp: u32, ss: u32
}

impl Context {
    unsafe fn save() -> &mut Context {
        let this: &mut Context;
        asm!("push gs
              push fs
              .byte 0x06 // push es
              .byte 0x1e // push ds
              pusha"
            : "={esp}"(this) ::: "volatile", "intel");
        this
    }

    unsafe fn restore() {
        asm!("popa
              .byte 0x1f // pop ds
              .byte 0x07 // pop es
              pop fs
              pop gs
              add esp, 8
              iretd"
            :::: "volatile", "intel");
    }
}

struct LocalSegment {
    ts: tss::TssEntry,
}

impl LocalSegment {
    // FIXME: Rust needs address spaces
    fn get() -> &mut LocalSegment {
        unsafe {
            let this: &mut LocalSegment;
            asm!("mov $0, dword[gs:0]" : "=r"(this) ::: "volatile", "intel")
            this
        }
    }
}

pub static mut desc_table: Option<gdt::Gdt> = None;

pub fn init() {
    let local_data = unsafe { kernel::zero_alloc(size_of::<LocalSegment>()) as *mut LocalSegment };
    let local_seg = unsafe {
        let seg = kernel::zero_alloc(0x80) as *mut [u32, ..32];
        *(seg as *mut *mut LocalSegment) = local_data;
        *(mut_offset(seg, 0x30) as *mut u32) = 0;
        seg
    };

    let t = Gdt::new();
    t.enable(1, GdtEntry::flat(SIZE_32 | STORAGE | CODE_READ, 0));
    t.enable(2, GdtEntry::flat(SIZE_32 | STORAGE | DATA_WRITE, 0));
    t.enable(3, GdtEntry::flat(SIZE_32 | STORAGE | CODE_READ, 3));
    t.enable(4, GdtEntry::flat(SIZE_32 | STORAGE | DATA_WRITE, 3));
    t.enable(5, GdtEntry::seg(local_seg, SIZE_32 | STORAGE, 3));
    t.load(1 << 3, 2 << 3, 5 << 3);

    unsafe {
        desc_table = Some(t);

        kernel::int_table.map(|mut t| {
            use cpu::exception::{Breakpoint, exception_handler};
            t.set_isr(Breakpoint, false, exception_handler());
        });

        mmu::init();
    }
}

pub fn info() {
}
