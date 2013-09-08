static UART0: *mut u32 = 0x101f1000 as *mut u32;

pub unsafe fn write_char(c: char) {
    *UART0 = c as u32;
}
