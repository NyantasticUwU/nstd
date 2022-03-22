//! Provides functions for operating on integer types.

/// Generates `nstd_core_int_types_*_[min|max]` functions.
macro_rules! get_int_size {
    ($name: ident, $type: ty, $attrib: ident) => {
        ///
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name() -> $type {
            <$type>::$attrib
        }
    };
}
// 8-bit ints.
get_int_size!(nstd_core_int_types_u8_min, u8, MIN);
get_int_size!(nstd_core_int_types_u8_max, u8, MAX);
get_int_size!(nstd_core_int_types_i8_min, i8, MIN);
get_int_size!(nstd_core_int_types_i8_max, i8, MAX);
// 16-bit ints.
get_int_size!(nstd_core_int_types_u16_min, u16, MIN);
get_int_size!(nstd_core_int_types_u16_max, u16, MAX);
get_int_size!(nstd_core_int_types_i16_min, i16, MIN);
get_int_size!(nstd_core_int_types_i16_max, i16, MAX);
// 32-bit ints.
get_int_size!(nstd_core_int_types_u32_min, u32, MIN);
get_int_size!(nstd_core_int_types_u32_max, u32, MAX);
get_int_size!(nstd_core_int_types_i32_min, i32, MIN);
get_int_size!(nstd_core_int_types_i32_max, i32, MAX);
// 64-bit ints.
get_int_size!(nstd_core_int_types_u64_min, u64, MIN);
get_int_size!(nstd_core_int_types_u64_max, u64, MAX);
get_int_size!(nstd_core_int_types_i64_min, i64, MIN);
get_int_size!(nstd_core_int_types_i64_max, i64, MAX);
// size-bit ints.
get_int_size!(nstd_core_int_types_usize_min, usize, MIN);
get_int_size!(nstd_core_int_types_usize_max, usize, MAX);
get_int_size!(nstd_core_int_types_isize_min, isize, MIN);
get_int_size!(nstd_core_int_types_isize_max, isize, MAX);
