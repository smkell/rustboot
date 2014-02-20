use core::fail::assert;

pub fn range(lo: uint, hi: uint, it: |uint|) {
    let mut iter = lo;
    while iter < hi {
        it(iter);
        iter += 1;
    }
}

macro_rules! int_module (($T:ty, $bits:expr) => (

#[inline]
pub fn to_str_bytes(num: $T, radix: uint, f: |u8|) {
    assert(2 <= radix && radix <= 36);
    let neg = num < 0;

    // Radix can be as low as 2, so we need 64 characters for a number
    // near 2^64, plus another one for a possible '-' character.
    let mut buf = [0u8, ..65];
    let mut cur = 0;

    // TODO: test overflow
    let mut deccum = num / radix as $T;
    let mut digit = num % radix as $T;

    // Calculate the absolute value after dividing the whole number once,
    // because a U2 representable negative number doesn't necessarily have
    // a unique inverse of the same type.
    // Example: -128i8 == 128i8. For radix=2, -128i8/2 = -64i8 != 64i8.
    if neg {
        deccum = -deccum;
        digit = -digit;
    }

    loop {
        buf[cur] = match digit as u8 {
            i @ 0..9 => '0' as u8 + i,
            i        => 'a' as u8 + (i-10),
        };
        cur += 1;
        deccum /= radix as $T;
        if deccum == 0 { break; }
        digit = deccum % radix as $T;
    }

    if neg {
        f('-' as u8);
    }

    while cur > 0 {
        cur -= 1;
        f(buf[cur]);
    }
}

))

#[cfg(target_word_size = "32")] int_module!(int, 32)
#[cfg(target_word_size = "64")] int_module!(int, 64)
