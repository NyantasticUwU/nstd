//! Contains low level math functions.

/// Generates the rad and deg functions.
macro_rules! nstd_create_rad_deg_fn {
    ($name: ident, $method: ident, $type: ty) => {
        ///
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.$method()
        }
    };
}
nstd_create_rad_deg_fn!(nstd_core_math_deg_f32, to_degrees, f32);
nstd_create_rad_deg_fn!(nstd_core_math_rad_f32, to_radians, f32);
nstd_create_rad_deg_fn!(nstd_core_math_deg_f64, to_degrees, f64);
nstd_create_rad_deg_fn!(nstd_core_math_rad_f64, to_radians, f64);

/// Generates the abs function.
macro_rules! nstd_create_abs_fn {
    ($name: ident, $type: ty) => {
        ///
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.abs()
        }
    };
}
nstd_create_abs_fn!(nstd_core_math_abs_i8, i8);
nstd_create_abs_fn!(nstd_core_math_abs_i16, i16);
nstd_create_abs_fn!(nstd_core_math_abs_i32, i32);
nstd_create_abs_fn!(nstd_core_math_abs_i64, i64);
nstd_create_abs_fn!(nstd_core_math_abs_isize, isize);

/// Generates the div_ceil functions.
macro_rules! nstd_create_div_ceil_fn {
    ($name: ident, $type: ty) => {
        ///
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type, y: $type) -> $type {
            x / y + ((x % y != 0) as $type)
        }
    };
}
nstd_create_div_ceil_fn!(nstd_core_math_div_ceil_u8, u8);
nstd_create_div_ceil_fn!(nstd_core_math_div_ceil_i8, i8);
nstd_create_div_ceil_fn!(nstd_core_math_div_ceil_u16, u16);
nstd_create_div_ceil_fn!(nstd_core_math_div_ceil_i16, i16);
nstd_create_div_ceil_fn!(nstd_core_math_div_ceil_u32, u32);
nstd_create_div_ceil_fn!(nstd_core_math_div_ceil_i32, i32);
nstd_create_div_ceil_fn!(nstd_core_math_div_ceil_u64, u64);
nstd_create_div_ceil_fn!(nstd_core_math_div_ceil_i64, i64);
nstd_create_div_ceil_fn!(nstd_core_math_div_ceil_usize, usize);
nstd_create_div_ceil_fn!(nstd_core_math_div_ceil_isize, isize);

/// Generates the div_floor functions.
macro_rules! nstd_create_div_floor_fn {
    ($name: ident, $type: ty) => {
        ///
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type, y: $type) -> $type {
            x / y
        }
    };
}
nstd_create_div_floor_fn!(nstd_core_math_div_floor_u8, u8);
nstd_create_div_floor_fn!(nstd_core_math_div_floor_i8, i8);
nstd_create_div_floor_fn!(nstd_core_math_div_floor_u16, u16);
nstd_create_div_floor_fn!(nstd_core_math_div_floor_i16, i16);
nstd_create_div_floor_fn!(nstd_core_math_div_floor_u32, u32);
nstd_create_div_floor_fn!(nstd_core_math_div_floor_i32, i32);
nstd_create_div_floor_fn!(nstd_core_math_div_floor_u64, u64);
nstd_create_div_floor_fn!(nstd_core_math_div_floor_i64, i64);
nstd_create_div_floor_fn!(nstd_core_math_div_floor_usize, usize);
nstd_create_div_floor_fn!(nstd_core_math_div_floor_isize, isize);

/// Generates the mod function.
macro_rules! nstd_create_mod_fn {
    ($name: ident, $type: ty) => {
        ///
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type, y: $type) -> $type {
            x % y
        }
    };
}
nstd_create_mod_fn!(nstd_core_math_mod_f32, f32);
nstd_create_mod_fn!(nstd_core_math_mod_f64, f64);
nstd_create_mod_fn!(nstd_core_math_mod_u8, u8);
nstd_create_mod_fn!(nstd_core_math_mod_i8, i8);
nstd_create_mod_fn!(nstd_core_math_mod_u16, u16);
nstd_create_mod_fn!(nstd_core_math_mod_i16, i16);
nstd_create_mod_fn!(nstd_core_math_mod_u32, u32);
nstd_create_mod_fn!(nstd_core_math_mod_i32, i32);
nstd_create_mod_fn!(nstd_core_math_mod_u64, u64);
nstd_create_mod_fn!(nstd_core_math_mod_i64, i64);
nstd_create_mod_fn!(nstd_core_math_mod_usize, usize);
nstd_create_mod_fn!(nstd_core_math_mod_isize, isize);

/// Generates the max function.
macro_rules! nstd_create_max_fn {
    ($name: ident, $type: ty) => {
        ///
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type, y: $type) -> $type {
            x.max(y)
        }
    };
}
nstd_create_max_fn!(nstd_core_math_max_f32, f32);
nstd_create_max_fn!(nstd_core_math_max_f64, f64);
nstd_create_max_fn!(nstd_core_math_max_u8, u8);
nstd_create_max_fn!(nstd_core_math_max_i8, i8);
nstd_create_max_fn!(nstd_core_math_max_u16, u16);
nstd_create_max_fn!(nstd_core_math_max_i16, i16);
nstd_create_max_fn!(nstd_core_math_max_u32, u32);
nstd_create_max_fn!(nstd_core_math_max_i32, i32);
nstd_create_max_fn!(nstd_core_math_max_u64, u64);
nstd_create_max_fn!(nstd_core_math_max_i64, i64);
nstd_create_max_fn!(nstd_core_math_max_usize, usize);
nstd_create_max_fn!(nstd_core_math_max_isize, isize);

/// Generates the min function.
macro_rules! nstd_create_min_fn {
    ($name: ident, $type: ty) => {
        ///
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type, y: $type) -> $type {
            x.min(y)
        }
    };
}
nstd_create_min_fn!(nstd_core_math_min_f32, f32);
nstd_create_min_fn!(nstd_core_math_min_f64, f64);
nstd_create_min_fn!(nstd_core_math_min_u8, u8);
nstd_create_min_fn!(nstd_core_math_min_i8, i8);
nstd_create_min_fn!(nstd_core_math_min_u16, u16);
nstd_create_min_fn!(nstd_core_math_min_i16, i16);
nstd_create_min_fn!(nstd_core_math_min_u32, u32);
nstd_create_min_fn!(nstd_core_math_min_i32, i32);
nstd_create_min_fn!(nstd_core_math_min_u64, u64);
nstd_create_min_fn!(nstd_core_math_min_i64, i64);
nstd_create_min_fn!(nstd_core_math_min_usize, usize);
nstd_create_min_fn!(nstd_core_math_min_isize, isize);

/// Generates the pow function.
macro_rules! nstd_create_pow_fn {
    ($name: ident, $type: ty) => {
        ///
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type, y: u32) -> $type {
            x.wrapping_pow(y)
        }
    };
}
nstd_create_pow_fn!(nstd_core_math_pow_u8, u8);
nstd_create_pow_fn!(nstd_core_math_pow_i8, i8);
nstd_create_pow_fn!(nstd_core_math_pow_u16, u16);
nstd_create_pow_fn!(nstd_core_math_pow_i16, i16);
nstd_create_pow_fn!(nstd_core_math_pow_u32, u32);
nstd_create_pow_fn!(nstd_core_math_pow_i32, i32);
nstd_create_pow_fn!(nstd_core_math_pow_u64, u64);
nstd_create_pow_fn!(nstd_core_math_pow_i64, i64);
nstd_create_pow_fn!(nstd_core_math_pow_usize, usize);
nstd_create_pow_fn!(nstd_core_math_pow_isize, isize);

/// Generates the clamp function.
macro_rules! nstd_create_clamp_fn {
    ($name: ident, $type: ty) => {
        ///
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(value: $type, min: $type, max: $type) -> $type {
            value.clamp(min, max)
        }
    };
}
nstd_create_clamp_fn!(nstd_core_math_clamp_f32, f32);
nstd_create_clamp_fn!(nstd_core_math_clamp_f64, f64);
nstd_create_clamp_fn!(nstd_core_math_clamp_u8, u8);
nstd_create_clamp_fn!(nstd_core_math_clamp_i8, i8);
nstd_create_clamp_fn!(nstd_core_math_clamp_u16, u16);
nstd_create_clamp_fn!(nstd_core_math_clamp_i16, i16);
nstd_create_clamp_fn!(nstd_core_math_clamp_u32, u32);
nstd_create_clamp_fn!(nstd_core_math_clamp_i32, i32);
nstd_create_clamp_fn!(nstd_core_math_clamp_u64, u64);
nstd_create_clamp_fn!(nstd_core_math_clamp_i64, i64);
nstd_create_clamp_fn!(nstd_core_math_clamp_usize, usize);
nstd_create_clamp_fn!(nstd_core_math_clamp_isize, isize);
