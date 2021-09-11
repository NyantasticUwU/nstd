use std::os::raw::*;

/// Generates the abs function.
macro_rules! nstd_create_abs_fn {
    ($name: ident, $type: ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.abs()
        }
    };
}
nstd_create_abs_fn!(nstd_std_math_abs_float, c_float);
nstd_create_abs_fn!(nstd_std_math_abs_double, c_double);
nstd_create_abs_fn!(nstd_std_math_abs_schar, c_schar);
nstd_create_abs_fn!(nstd_std_math_abs_short, c_short);
nstd_create_abs_fn!(nstd_std_math_abs_int, c_int);
nstd_create_abs_fn!(nstd_std_math_abs_long, c_long);
nstd_create_abs_fn!(nstd_std_math_abs_longlong, c_longlong);

/// Generates the mod function.
macro_rules! nstd_create_mod_fn {
    ($name: ident, $type: ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(x: $type, y: $type) -> $type {
            x % y
        }
    };
}
nstd_create_mod_fn!(nstd_std_math_mod_float, c_float);
nstd_create_mod_fn!(nstd_std_math_mod_double, c_double);
nstd_create_mod_fn!(nstd_std_math_mod_schar, c_schar);
nstd_create_mod_fn!(nstd_std_math_mod_short, c_short);
nstd_create_mod_fn!(nstd_std_math_mod_int, c_int);
nstd_create_mod_fn!(nstd_std_math_mod_long, c_long);
nstd_create_mod_fn!(nstd_std_math_mod_longlong, c_longlong);

/// Generates the max function.
macro_rules! nstd_create_max_fn {
    ($name: ident, $type: ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(x: $type, y: $type) -> $type {
            x.max(y)
        }
    };
}
nstd_create_max_fn!(nstd_std_math_max_float, c_float);
nstd_create_max_fn!(nstd_std_math_max_double, c_double);
nstd_create_max_fn!(nstd_std_math_max_schar, c_schar);
nstd_create_max_fn!(nstd_std_math_max_short, c_short);
nstd_create_max_fn!(nstd_std_math_max_int, c_int);
nstd_create_max_fn!(nstd_std_math_max_long, c_long);
nstd_create_max_fn!(nstd_std_math_max_longlong, c_longlong);

/// Generates the min function.
macro_rules! nstd_create_min_fn {
    ($name: ident, $type: ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(x: $type, y: $type) -> $type {
            x.min(y)
        }
    };
}
nstd_create_min_fn!(nstd_std_math_min_float, c_float);
nstd_create_min_fn!(nstd_std_math_min_double, c_double);
nstd_create_min_fn!(nstd_std_math_min_schar, c_schar);
nstd_create_min_fn!(nstd_std_math_min_short, c_short);
nstd_create_min_fn!(nstd_std_math_min_int, c_int);
nstd_create_min_fn!(nstd_std_math_min_long, c_long);
nstd_create_min_fn!(nstd_std_math_min_longlong, c_longlong);

/// Generates the powf function.
macro_rules! nstd_create_powf_fn {
    ($name: ident, $type: ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(x: $type, y: $type) -> $type {
            x.powf(y)
        }
    };
}
nstd_create_powf_fn!(nstd_std_math_pow_float, c_float);
nstd_create_powf_fn!(nstd_std_math_pow_double, c_double);
/// Generates the pow function.
macro_rules! nstd_create_pow_fn {
    ($name: ident, $type: ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(x: $type, y: c_uint) -> $type {
            x.wrapping_pow(y as u32)
        }
    };
}
nstd_create_pow_fn!(nstd_std_math_pow_schar, c_schar);
nstd_create_pow_fn!(nstd_std_math_pow_short, c_short);
nstd_create_pow_fn!(nstd_std_math_pow_int, c_int);
nstd_create_pow_fn!(nstd_std_math_pow_long, c_long);
nstd_create_pow_fn!(nstd_std_math_pow_longlong, c_longlong);

/// Generates the sqrt function.
macro_rules! nstd_create_sqrt_fn {
    ($name: ident, $type: ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.sqrt()
        }
    };
}
nstd_create_sqrt_fn!(nstd_std_math_sqrt_float, c_float);
nstd_create_sqrt_fn!(nstd_std_math_sqrt_double, c_double);

/// Generates the cbrt function.
macro_rules! nstd_create_cbrt_fn {
    ($name: ident, $type: ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.cbrt()
        }
    };
}
nstd_create_cbrt_fn!(nstd_std_math_cbrt_float, c_float);
nstd_create_cbrt_fn!(nstd_std_math_cbrt_double, c_double);

/// Generates the sin function.
macro_rules! nstd_create_sin_fn {
    ($name: ident, $type: ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.sin()
        }
    };
}
nstd_create_sin_fn!(nstd_std_math_sin_float, c_float);
nstd_create_sin_fn!(nstd_std_math_sin_double, c_double);

/// Generates the cos function.
macro_rules! nstd_create_cos_fn {
    ($name: ident, $type: ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.cos()
        }
    };
}
nstd_create_cos_fn!(nstd_std_math_cos_float, c_float);
nstd_create_cos_fn!(nstd_std_math_cos_double, c_double);

/// Generates the tan function.
macro_rules! nstd_create_tan_fn {
    ($name: ident, $type: ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.tan()
        }
    };
}
nstd_create_tan_fn!(nstd_std_math_tan_float, c_float);
nstd_create_tan_fn!(nstd_std_math_tan_double, c_double);

/// Generates the ceil function.
macro_rules! nstd_create_ceil_fn {
    ($name: ident, $type: ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.ceil()
        }
    };
}
nstd_create_ceil_fn!(nstd_std_math_ceil_float, c_float);
nstd_create_ceil_fn!(nstd_std_math_ceil_double, c_double);

/// Generates the floor function.
macro_rules! nstd_create_floor_fn {
    ($name: ident, $type: ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.floor()
        }
    };
}
nstd_create_floor_fn!(nstd_std_math_floor_float, c_float);
nstd_create_floor_fn!(nstd_std_math_floor_double, c_double);

/// Generates the round function.
macro_rules! nstd_create_round_fn {
    ($name: ident, $type: ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.round()
        }
    };
}
nstd_create_round_fn!(nstd_std_math_round_float, c_float);
nstd_create_round_fn!(nstd_std_math_round_double, c_double);
