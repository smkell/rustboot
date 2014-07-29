#![macro_escape]

macro_rules! define_reg (
    (
        $Reg:ident, $flags:ident: $T:ty {
            $($flag:ident $(= $v:expr)*),*
        }
    ) => (
        bitflags!(flags $flags: $T { $( static $flag $(= $v)* ),* })

        pub struct $Reg;

        impl_ops!($Reg, $flags, $flags, $Reg::read(), $flags)
    )
)

macro_rules! impl_ops (
    ($T:ident, $RHS:ident) => (
        impl core::ops::BitOr<$RHS, $T> for $T {
            #[inline(always)]
            fn bitor(&self, other: &$RHS) -> $T {
                match (*self, other) {
                    ($T(p), &$RHS(f)) => $T(p | f)
                }
            }
        }

        impl core::ops::BitAnd<$RHS, $T> for $T {
            #[inline(always)]
            fn bitand(&self, other: &$RHS) -> $T {
                match (*self, other) {
                    ($T(p), &$RHS(f)) => $T(p & f)
                }
            }
        }

        impl core::ops::Sub<$RHS, $T> for $T {
            #[inline(always)]
            fn sub(&self, other: &$RHS) -> $T {
                match (*self, other) {
                    ($T(flags1), &$RHS(flags2)) => $T(flags1 & !flags2)
                }
            }
        }
    );

    ($LHS:ident, $RHS:ident, $T:ident, $e:expr, $X:ident) => (
        impl core::ops::BitOr<$RHS, $T> for $LHS {
            #[inline(always)]
            fn bitor(&self, other: &$RHS) -> $T {
                match ($e, other) {
                    ($X(p), &$RHS(f)) => $T(p | f)
                }
            }
        }

        impl core::ops::BitAnd<$RHS, $T> for $LHS {
            #[inline(always)]
            fn bitand(&self, other: &$RHS) -> $T {
                match ($e, other) {
                    ($X(p), &$RHS(f)) => $T(p & f)
                }
            }
        }

        impl core::ops::Sub<$RHS, $T> for $LHS {
            #[inline(always)]
            fn sub(&self, other: &$RHS) -> $T {
                match ($e, other) {
                    ($X(flags1), &$RHS(flags2)) => $T(flags1 & !flags2)
                }
            }
        }
    )
)

macro_rules! print(
    ($($arg:tt)*) => (format_args!(::platform::io::print_args, $($arg)*))
)

macro_rules! println(
    ($($arg:tt)*) => (format_args!(::platform::io::println_args, $($arg)*))
)
