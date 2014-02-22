// based on define_flags by lexs
#[macro_escape];

macro_rules! define_flags (
    (
        $name:ident {
            $($flag:ident = $value:expr),+
        }
    ) => {
        define_flags!($name: u32 { $($flag = $value),+ })
    };

    (
        $name:ident: $T:ty {
            $($flag:ident $(= $v:expr)*),*
        }
    ) => {
        #[packed]
        pub struct $name($T);

        impl $name {
            /// Return the value.
            pub fn get(self) -> $T {
                match self { $name(x) => x }
            }

            /// Maps by applying a function to a contained value.
            pub fn map(self, f: |$T| -> $T) -> $name {
                match self { $name(x) => $name(f(x)) }
            }
        }

        impl core::ops::BitOr<$name, $name> for $name {
            #[inline(always)]
            fn bitor(&self, other: &$name) -> $name {
                match (self, other) {
                    (&$name(flags1), &$name(flags2)) => $name(flags1 | flags2)
                }
            }
        }

        impl core::ops::BitAnd<$name, bool> for $name {
            #[inline(always)]
            fn bitand(&self, other: &$name) -> bool {
                match (self, other) {
                    (&$name(flags1), &$name(flags2)) => flags1 & flags2 != 0
                }
            }
        }

        impl core::ops::Not<$name> for $name {
            #[inline(always)]
            fn not(&self) -> $name {
                match self {
                    &$name(flags) => $name(!flags)
                }
            }
        }

        define_flags_rec!($name, 1, $( $flag $(= $v)* ),+)
    };
)
macro_rules! define_flags_rec (
    // full list (original behavior)
    (
        $name:ident,
        $default:expr,
        $($flag:ident = $value:expr),*
    ) => (
        $( pub static $flag: $name = $name($value); )+
    );
    // ----------
    // only one default value
    (
        $name:ident,
        $default:expr,
        $flag:ident
    ) => (
        pub static $flag: $name = $name($default);
    );
    // only one value
    (
        $name:ident,
        $default:expr,
        $flag:ident = $value:expr
    ) => (
        pub static $flag: $name = $name($value);
    );
    // ----------
    // without value (default)
    (
        $name:ident,
        $default:expr,
        $flag:ident,
        $($f:ident $(= $v:expr)*),*
    ) => (
        pub static $flag: $name = $name($default);
        define_flags_rec!($name, $default << 1, $($f $(= $v)*),+)
    );
    // with value
    (
        $name:ident,
        $default:expr,
        $flag:ident = $value:expr,
        $($f:ident $(= $v:expr)*),*
    ) => (
        pub static $flag: $name = $name($value);
        define_flags_rec!($name, $value << 1, $( $f $(= $v)* ),+)
    );
)
