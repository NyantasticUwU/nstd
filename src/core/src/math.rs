use cty::*;

/// Generates the abs function.
macro_rules! nstd_create_abs_fn {
    ($name: ident, $type: ty) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.abs()
        }
    };
}
nstd_create_abs_fn!(nstd_core_math_abs_schar, c_schar);
nstd_create_abs_fn!(nstd_core_math_abs_short, c_short);
nstd_create_abs_fn!(nstd_core_math_abs_int, c_int);
nstd_create_abs_fn!(nstd_core_math_abs_long, c_long);
nstd_create_abs_fn!(nstd_core_math_abs_longlong, c_longlong);

/// Generates the mod function.
macro_rules! nstd_create_mod_fn {
    ($name: ident, $type: ty) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type, y: $type) -> $type {
            x % y
        }
    };
}
nstd_create_mod_fn!(nstd_core_math_mod_float, c_float);
nstd_create_mod_fn!(nstd_core_math_mod_double, c_double);
nstd_create_mod_fn!(nstd_core_math_mod_schar, c_schar);
nstd_create_mod_fn!(nstd_core_math_mod_short, c_short);
nstd_create_mod_fn!(nstd_core_math_mod_int, c_int);
nstd_create_mod_fn!(nstd_core_math_mod_long, c_long);
nstd_create_mod_fn!(nstd_core_math_mod_longlong, c_longlong);

/// Generates the max function.
macro_rules! nstd_create_max_fn {
    ($name: ident, $type: ty) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type, y: $type) -> $type {
            x.max(y)
        }
    };
}
nstd_create_max_fn!(nstd_core_math_max_float, c_float);
nstd_create_max_fn!(nstd_core_math_max_double, c_double);
nstd_create_max_fn!(nstd_core_math_max_schar, c_schar);
nstd_create_max_fn!(nstd_core_math_max_short, c_short);
nstd_create_max_fn!(nstd_core_math_max_int, c_int);
nstd_create_max_fn!(nstd_core_math_max_long, c_long);
nstd_create_max_fn!(nstd_core_math_max_longlong, c_longlong);

/// Generates the min function.
macro_rules! nstd_create_min_fn {
    ($name: ident, $type: ty) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type, y: $type) -> $type {
            x.min(y)
        }
    };
}
nstd_create_min_fn!(nstd_core_math_min_float, c_float);
nstd_create_min_fn!(nstd_core_math_min_double, c_double);
nstd_create_min_fn!(nstd_core_math_min_schar, c_schar);
nstd_create_min_fn!(nstd_core_math_min_short, c_short);
nstd_create_min_fn!(nstd_core_math_min_int, c_int);
nstd_create_min_fn!(nstd_core_math_min_long, c_long);
nstd_create_min_fn!(nstd_core_math_min_longlong, c_longlong);

/// Generates the pow function.
macro_rules! nstd_create_pow_fn {
    ($name: ident, $type: ty) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type, y: c_uint) -> $type {
            x.wrapping_pow(y as u32)
        }
    };
}
nstd_create_pow_fn!(nstd_core_math_pow_schar, c_schar);
nstd_create_pow_fn!(nstd_core_math_pow_short, c_short);
nstd_create_pow_fn!(nstd_core_math_pow_int, c_int);
nstd_create_pow_fn!(nstd_core_math_pow_long, c_long);
nstd_create_pow_fn!(nstd_core_math_pow_longlong, c_longlong);
