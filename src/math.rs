/// Generates the sqrt function.
macro_rules! nstd_create_sqrt_fn {
    ($name: ident, $type: ty) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.sqrt()
        }
    };
}
nstd_create_sqrt_fn!(nstd_math_sqrt_f32, f32);
nstd_create_sqrt_fn!(nstd_math_sqrt_f64, f64);

/// Generates the cbrt function.
macro_rules! nstd_create_cbrt_fn {
    ($name: ident, $type: ty) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.cbrt()
        }
    };
}
nstd_create_cbrt_fn!(nstd_math_cbrt_f32, f32);
nstd_create_cbrt_fn!(nstd_math_cbrt_f64, f64);

/// Generates the sin function.
macro_rules! nstd_create_sin_fn {
    ($name: ident, $type: ty) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.sin()
        }
    };
}
nstd_create_sin_fn!(nstd_math_sin_f32, f32);
nstd_create_sin_fn!(nstd_math_sin_f64, f64);

/// Generates the cos function.
macro_rules! nstd_create_cos_fn {
    ($name: ident, $type: ty) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.cos()
        }
    };
}
nstd_create_cos_fn!(nstd_math_cos_f32, f32);
nstd_create_cos_fn!(nstd_math_cos_f64, f64);

/// Generates the tan function.
macro_rules! nstd_create_tan_fn {
    ($name: ident, $type: ty) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.tan()
        }
    };
}
nstd_create_tan_fn!(nstd_math_tan_f32, f32);
nstd_create_tan_fn!(nstd_math_tan_f64, f64);

/// Generates the ceil function.
macro_rules! nstd_create_ceil_fn {
    ($name: ident, $type: ty) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.ceil()
        }
    };
}
nstd_create_ceil_fn!(nstd_math_ceil_f32, f32);
nstd_create_ceil_fn!(nstd_math_ceil_f64, f64);

/// Generates the floor function.
macro_rules! nstd_create_floor_fn {
    ($name: ident, $type: ty) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.floor()
        }
    };
}
nstd_create_floor_fn!(nstd_math_floor_f32, f32);
nstd_create_floor_fn!(nstd_math_floor_f64, f64);

/// Generates the round function.
macro_rules! nstd_create_round_fn {
    ($name: ident, $type: ty) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.round()
        }
    };
}
nstd_create_round_fn!(nstd_math_round_f32, f32);
nstd_create_round_fn!(nstd_math_round_f64, f64);
