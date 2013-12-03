mod idt;
pub mod interrupt;
mod exception;
mod paging;

pub static mut max: u32 = 0;
pub static mut features: *mut [u32, ..40] = 0x100810 as *mut [u32, ..40];

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

pub unsafe fn init() {
    asm!("mov eax, cr4
          or eax, 512
          mov cr4, eax"
        ::: "eax" : "intel");
}

pub unsafe fn info() -> [u8, ..12] {
    let mut vendor = [0u8, ..12];
    let ptr = &vendor as *[u8, ..12] as *mut [u32, ..3];

    cpuid!(0, max, *ptr, (*ptr)[1], (*ptr)[2]);

    vendor
}
