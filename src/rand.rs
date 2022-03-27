//! Random value generation.
use crate::core::def::{NSTDBool, NSTDUnichar};

/// Generates random value gen functions for nstd typed values.
macro_rules! gen_random_nstd_fn {
    ($name: ident, $nstdtype: ty, $rusttype: ty) => {
        ///
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name() -> $nstdtype {
            rand::random::<$rusttype>().into()
        }
    };
}
gen_random_nstd_fn!(nstd_rand_bool, NSTDBool, bool);
gen_random_nstd_fn!(nstd_rand_unichar, NSTDUnichar, char);

/// Generates random value gen functions.
macro_rules! gen_random_fn {
    ($name: ident, $type: ty) => {
        ///
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name() -> $type {
            rand::random::<$type>()
        }
    };
}
gen_random_fn!(nstd_rand_f32, f32);
gen_random_fn!(nstd_rand_f64, f64);
gen_random_fn!(nstd_rand_u8, u8);
gen_random_fn!(nstd_rand_i8, i8);
gen_random_fn!(nstd_rand_u16, u16);
gen_random_fn!(nstd_rand_i16, i16);
gen_random_fn!(nstd_rand_u32, u32);
gen_random_fn!(nstd_rand_i32, i32);
gen_random_fn!(nstd_rand_u64, u64);
gen_random_fn!(nstd_rand_i64, i64);
gen_random_fn!(nstd_rand_usize, usize);
gen_random_fn!(nstd_rand_isize, isize);
