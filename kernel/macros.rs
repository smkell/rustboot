macro_rules! define_flags (
    (
        $name:ident {
            $($flag:ident = $value:expr),+
        }
    ) => {
        define_flags!($name: u32 { $($flag = $value),+ })
    };

    (
        $name:ident: $t:ty {
            $($flag:ident = $value:expr),*
        }
    ) => {
        #[packed]
        pub struct $name($t);

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

        $(
            pub static $flag: $name = $name($value);
        )+
    };
)
