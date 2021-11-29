/// Gets an f* attribute.
macro_rules! get_float_attr {
    ($name: ident, $type: ty, $attr: ident) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name() -> $type {
            <$type>::$attr
        }
    };
}
// f32.
get_float_attr!(nstd_core_float_types_float_min, f32, MIN);
get_float_attr!(nstd_core_float_types_float_max, f32, MAX);
get_float_attr!(nstd_core_float_types_float_nan, f32, NAN);
get_float_attr!(nstd_core_float_types_float_infinity, f32, INFINITY);
get_float_attr!(
    nstd_core_float_types_float_negative_infinity,
    f32,
    NEG_INFINITY
);
// f64.
get_float_attr!(nstd_core_float_types_double_min, f64, MIN);
get_float_attr!(nstd_core_float_types_double_max, f64, MAX);
get_float_attr!(nstd_core_float_types_double_nan, f64, NAN);
get_float_attr!(nstd_core_float_types_double_infinity, f64, INFINITY);
get_float_attr!(
    nstd_core_float_types_double_negative_infinity,
    f64,
    NEG_INFINITY
);

/// Checks an f* boolean method.
macro_rules! check_float {
    ($name: ident, $fname: ident, $type: ty) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(f: $type) -> i32 {
            f.$fname() as i32
        }
    };
}
// f32.
check_float!(nstd_core_float_types_float_is_nan, is_nan, f32);
check_float!(nstd_core_float_types_float_is_infinite, is_infinite, f32);
// f64.
check_float!(nstd_core_float_types_double_is_nan, is_nan, f64);
check_float!(nstd_core_float_types_double_is_infinite, is_infinite, f64);

/// Gets an f* constant.
macro_rules! get_float_const {
    ($name: ident, $ns: ident, $type: ty, $const: ident) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name() -> $type {
            core::$ns::consts::$const
        }
    };
}
get_float_const!(nstd_core_float_types_float_pi, f32, f32, PI);
get_float_const!(nstd_core_float_types_double_pi, f64, f64, PI);
get_float_const!(nstd_core_float_types_float_e, f32, f32, E);
get_float_const!(nstd_core_float_types_double_e, f64, f64, E);
