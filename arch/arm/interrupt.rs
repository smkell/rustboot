use rust::zero;

#[lang="fail_"]
pub fn fail(expr: *u8, file: *u8, line: uint) -> ! {
    unsafe { zero::abort(); }
}

#[lang="fail_bounds_check"]
pub fn fail_bounds_check(file: *u8, line: uint, index: uint, len: uint) {
    unsafe { zero::abort(); }
}
