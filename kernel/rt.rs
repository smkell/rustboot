/* rt.rs
 * Implementation of functions translated from compiler-rt
 * mulodi4
 * udivdi3
 * divmoddi4
 * udivmoddi4
 */

use core::i32::{ctlz32, cttz32};
use core::mem::transmute;

#[no_mangle]
pub unsafe fn __mulodi4(a: i64, b: i64, overflow: *mut int) -> i64 {
    let N: int = 64;
    let MIN: i64 = (1 as i64) << (N-1);
    let MAX: i64 = !MIN;
    // const di_int MIN = (di_int)1 << (N-1);
    // const di_int MAX = ~MIN;
    *overflow = 0;
    let result = a * b;
    if a == MIN {
        if b != 0 && b != 1 { *overflow = 1; }
        return result;
    }
    if b == MIN {
        if a != 0 && a != 1 { *overflow = 1; }
        return result;
    }
    let sa: i64 = a >> (N - 1);
    let abs_a: i64 = (a ^ sa) - sa;
    let sb: i64 = b >> (N - 1);
    let abs_b: i64 = (b ^ sb) - sb;
    if abs_a < 2 || abs_b < 2 {
        return result;
    }
    if (sa == sb && abs_a > MAX / abs_b) || abs_a > MIN / -abs_b {
        *overflow = 1;
    }
    return result;
}

/* Returns: a / b */

#[no_mangle]
pub fn __divdi3(mut a: i64, mut b: i64) -> i64 {
    let bits_in_dword_m1: int = 8 * 8 - 1;
    let mut s_a: i64 = a >> bits_in_dword_m1;           /* s_a = a < 0 ? -1 : 0 */
    let s_b: i64 = b >> bits_in_dword_m1;           /* s_b = b < 0 ? -1 : 0 */
    a = (a ^ s_a) - s_a;                         /* negate if s_a == -1 */
    b = (b ^ s_b) - s_b;                         /* negate if s_b == -1 */
    s_a ^= s_b;                                  /*sign of quotient */
    return (__udivdi3(a as u64, b as u64) as i64 ^ s_a) - s_a;  /* negate if s_a == -1 */
}

// typedef      int si_int;
// typedef unsigned su_int;
// typedef          long long di_int;
// typedef unsigned long long du_int;

#[cfg(target_endian = "little")]
#[packed]
struct udwords {
    low: i32,
    high: i32
}

#[cfg(target_endian = "big")]
#[packed]
struct udwords {
    high: i32,
    low: i32
}

//__udivmoddi4(a, b, rem=0)
#[no_mangle]
pub fn __udivmoddi4(a: u64, b: u64, rem: *mut u64) -> u64 {
    // TODO: rem
    return __udivdi3(a, b);
}

#[no_mangle]
pub fn __udivdi3(a: u64, b: u64) -> u64 {
    // n_uword_bits = sizeof(su_int) * CHAR_BIT; type su_int = unsigned
    // n_udword_bits = sizeof(du_int) * CHAR_BIT; type du_unt = unsigned long long
    let n_uword_bits = 32;
    let n_udword_bits = 64;
    let n: udwords;
    let d: udwords;
    let mut q: udwords;
    let mut r: udwords;
    let mut sr: i32; //uint/u32
    unsafe {
        n = transmute(a);
        d = transmute(b);
    }
    /* special cases, X is unknown, K != 0 */
    if n.high == 0 {
        if d.high == 0 {
            /* 0 X
             * ---
             * 0 X
             */
            return (n.low / d.low) as u64;
        }
        /* 0 X
         * ---
         * K X
         */
        return 0;
    }
    /* n.s.high != 0 */
    if d.low == 0 {
        if d.high == 0 {
            /* K X
             * ---
             * 0 0
             */
            return (n.high / d.low) as u64;
        }
        /* d.s.high != 0 */
        if n.low == 0 {
            /* K 0
             * ---
             * K 0
             */
            return (n.high / d.high) as u64;
        }
        /* K K
         * ---
         * K 0
         */
        if (d.high & (d.high - 1)) == 0 {   /* if d is a power of 2 */
            return unsafe { (n.high >> cttz32(d.high)) as u64 };
        }
        /* K K
         * ---
         * K 0
         */
        sr = unsafe { ctlz32(d.high) - ctlz32(n.high) };
        /* 0 <= sr <= n_uword_bits - 2 or sr large */
        if sr > n_uword_bits - 2 {
            return 0;
        }
        sr += 1;
        /* 1 <= sr <= n_uword_bits - 1 */
        /* q.all = n.all << (n_udword_bits - sr); */
        q = udwords {
            low: 0,
            high: n.low << (n_uword_bits - sr)
        };
        /* r.all = n.all >> sr; */
        r = udwords {
            high: n.high >> sr,
            low: (n.high << (n_uword_bits - sr)) | (n.low >> sr)
        };
    }
    else { /* d.s.low != 0 */
        if d.high == 0 {
            /* K X
             * ---
             * 0 K
             */
            if (d.low & (d.low - 1)) == 0 {   /* if d is a power of 2 */
                if (d.low == 1) {
                    return a;
                }
                sr = unsafe { cttz32(d.low) };
                q = udwords {
                    high: n.high >> sr,
                    low: (n.high << (n_uword_bits - sr)) | (n.low >> sr)
                };
                return unsafe { transmute(q) };
            }
            /* K X
             * ---
             *0 K
             */
            sr = 1 + n_uword_bits + unsafe { ctlz32(d.low) - ctlz32(n.high) };
            /* 2 <= sr <= n_udword_bits - 1
             * q.all = n.all << (n_udword_bits - sr);
             * r.all = n.all >> sr;
             * if (sr == n_uword_bits)
             * {
             *     q.low = 0;
             *     q.high = n.low;
             *     r.high = 0;
             *     r.low = n.high;
             * }
             * else if (sr < n_uword_bits)  // 2 <= sr <= n_uword_bits - 1
             * {
             *     q.low = 0;
             *     q.high = n.low << (n_uword_bits - sr);
             *     r.high = n.high >> sr;
             *     r.low = (n.high << (n_uword_bits - sr)) | (n.low >> sr);
             * }
             * else              // n_uword_bits + 1 <= sr <= n_udword_bits - 1
             * {
             *     q.low = n.low << (n_udword_bits - sr);
             *     q.high = (n.high << (n_udword_bits - sr)) |
             *              (n.low >> (sr - n_uword_bits));
             *     r.high = 0;
             *     r.low = n.high >> (sr - n_uword_bits);
             * }
             */

            q = udwords {
                high: ((n.low << ( n_uword_bits - sr))                       &
                      (((sr - n_uword_bits - 1) as i32) >> (n_uword_bits-1)))|
                      (((n.high << (n_udword_bits - sr))                     |
                      (n.low >> (sr - n_uword_bits)))                        &
                      (((n_uword_bits - sr) as i32) >> (n_uword_bits-1))),
                low: (n.low << (n_udword_bits - sr)) &
                     (((n_uword_bits - sr) as i32) >> (n_uword_bits-1))
            };
            r = udwords {
                high: (n.high >> sr) &
                      (((sr - n_uword_bits) as i32) >> (n_uword_bits-1)),
                low: ((n.high >> (sr - n_uword_bits))                       &
                     (((n_uword_bits - sr - 1) as i32) >> (n_uword_bits-1)))|
                     (((n.high << (n_uword_bits - sr))                      |
                     (n.low >> sr))                                         &
                     (((sr - n_uword_bits) as i32) >> (n_uword_bits-1)))
            }
        }
        else {
            /* K X
             * ---
             * K K
             */
            sr = unsafe { ctlz32(d.high) - ctlz32(n.high) };
            /* 0 <= sr <= n_uword_bits - 1 or sr large */
            if sr > n_uword_bits - 1 {
                return 0;
            }
            sr += 1;
            /* 1 <= sr <= n_uword_bits */
            /*  q.all = n.all << (n_udword_bits - sr); */
            q = udwords {
                high: n.low << (n_uword_bits - sr),
                low: 0
            };
            /* r.all = n.all >> sr;
             * if (sr < n_uword_bits)
             * {
             *     r.s.high = n.s.high >> sr;
             *     r.s.low = (n.s.high << (n_uword_bits - sr)) | (n.s.low >> sr);
             * }
             * else
             * {
             *     r.s.high = 0;
             *     r.s.low = n.s.high;
             * }
             */
            r = udwords {
                high: (n.high >> sr) &
                     (((sr - n_uword_bits) as i32) >> (n_uword_bits-1)),
                low: (n.high << (n_uword_bits - sr)) |
                    ((n.low >> sr)                  &
                    (((sr - n_uword_bits) as i32) >> (n_uword_bits-1)))
            };
        }
    }
    /* Not a special case
     * q and r are initialized with:
     * q.all = n.all << (n_udword_bits - sr);
     * r.all = n.all >> sr;
     * 1 <= sr <= n_udword_bits - 1
     */
    let mut carry: u64 = 0;
    while sr > 0 { //for (; sr > 0; --sr)
        /* r:q = ((r:q)  << 1) | carry */

        r = udwords {
            high: (r.high << 1) | (r.low  >> (n_uword_bits - 1)),
            low:  (r.low  << 1) | (q.high >> (n_uword_bits - 1))
        };
        q = udwords {
            high: (q.high << 1) | (q.low  >> (n_uword_bits - 1)),
            low:  (q.low  << 1) | carry as i32
        };
        /* carry = 0;
         * if (r.all >= d.all)
         * {
         *      r.all -= d.all;
         *      carry = 1;
         * }
         */
        unsafe {
            let s: u64 = (b - transmute::<udwords, u64>(r) - 1) >> (n_udword_bits - 1);
            // const di_int s = (di_int)(d.all - r.all - 1) >> (n_udword_bits - 1);
            carry = s & 1;
            r = transmute(transmute::<udwords, u64>(r) - transmute(d) & s);
        }
    }
    return unsafe { transmute((transmute::<udwords, u64>(q) << 1) | carry) };
}

#[no_mangle]
pub unsafe fn __divmoddi4(a: i64, b: i64, rem: *mut i64) -> i64 {
    let d: i64 = __divdi3(a, b);
    *rem = a - (d*b);
    return d;
}
