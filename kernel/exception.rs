use rust::zero;
use rust::int;
use drivers::cga;

#[lang="fail_"]
pub fn fail(expr: *i8, file: *i8, line: uint) -> ! {
    unsafe {
        let mut i = 0;
        let expr_str = expr as *[u8, ..1000];
        while (*expr_str)[i] != 0 {
            (*cga::SCREEN)[i].char = (*expr_str)[i];
            (*cga::SCREEN)[i].attr = 16;
            i += 1;
        }

        i = 0;
        let file_str = file as *[u8, ..1000];
        while (*file_str)[i] != 0 {
            (*cga::SCREEN)[80+i].char = (*file_str)[i];
            (*cga::SCREEN)[80+i].attr = 16;
            i += 1;
        }

        i = 80*2;
        int::to_str_bytes(line as int, 10, |n| {
            (*cga::SCREEN)[i].char = n;
            (*cga::SCREEN)[i].attr = 16;
            i += 1;
        });

        zero::abort();
    }
}

#[lang="fail_bounds_check"]
pub fn fail_bounds_check(file: *i8, line: uint, index: uint, len: uint) {
    unsafe {
        let mut i = 0;
        let file_str = file as *[u8, ..1000];
        while (*file_str)[i] != 0 {
            (*cga::SCREEN)[i].char = (*file_str)[i];
            (*cga::SCREEN)[i].attr = 16;
            i += 1;
        }

        i = 80;
        int::to_str_bytes(line as int, 10, |n| {
            (*cga::SCREEN)[i].char = n;
            (*cga::SCREEN)[i].attr = 16;
            i += 1;
        });
        i = 80*2;
        int::to_str_bytes(index as int, 10, |n| {
            (*cga::SCREEN)[i].char = n;
            (*cga::SCREEN)[i].attr = 16;
            i += 1;
        });
        i = 80*3;
        int::to_str_bytes(len as int, 10, |n| {
            (*cga::SCREEN)[i].char = n;
            (*cga::SCREEN)[i].attr = 16;
            i += 1;
        });

        zero::abort();
    }
}
