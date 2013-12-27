mod gdt;
mod idt;
pub mod interrupt;
pub mod io;
mod exception;
mod paging;

pub static mut max: u32 = 0;

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

pub fn init() {
    unsafe {
        asm!("mov eax, cr4
              or eax, 512
              mov cr4, eax"
            ::: "eax" : "intel");
    }

    let t = gdt::table::new();
    t.enable(1, gdt::entry::new(0, 0xFFFFF, gdt::SIZE_32 | gdt::STORAGE | gdt::CODE_READ, 0));
    t.enable(2, gdt::entry::new(0, 0xFFFFF, gdt::SIZE_32 | gdt::STORAGE | gdt::DATA_WRITE, 0));
    t.load();
}

pub unsafe fn info() -> [u8, ..12] {
    let vendor = [0u8, ..12];
    let ptr = &vendor as *[u8, ..12] as *mut [u32, ..3];

    cpuid!(0, max, *ptr, (*ptr)[1], (*ptr)[2]);

    vendor
}
