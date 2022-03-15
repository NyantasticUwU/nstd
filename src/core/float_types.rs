use crate::core::def::NSTDBool;

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
get_float_attr!(nstd_core_float_types_f32_min, f32, MIN);
get_float_attr!(nstd_core_float_types_f32_max, f32, MAX);
get_float_attr!(nstd_core_float_types_f32_nan, f32, NAN);
get_float_attr!(nstd_core_float_types_f32_infinity, f32, INFINITY);
get_float_attr!(
    nstd_core_float_types_f32_negative_infinity,
    f32,
    NEG_INFINITY
);
// f64.
get_float_attr!(nstd_core_float_types_f64_min, f64, MIN);
get_float_attr!(nstd_core_float_types_f64_max, f64, MAX);
get_float_attr!(nstd_core_float_types_f64_nan, f64, NAN);
get_float_attr!(nstd_core_float_types_f64_infinity, f64, INFINITY);
get_float_attr!(
    nstd_core_float_types_f64_negative_infinity,
    f64,
    NEG_INFINITY
);

/// Checks an f* boolean method.
macro_rules! check_float {
    ($name: ident, $fname: ident, $type: ty) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(f: $type) -> NSTDBool {
            NSTDBool::from(f.$fname())
        }
    };
}
// f32.
check_float!(nstd_core_float_types_f32_is_nan, is_nan, f32);
check_float!(nstd_core_float_types_f32_is_infinite, is_infinite, f32);
// f64.
check_float!(nstd_core_float_types_f64_is_nan, is_nan, f64);
check_float!(nstd_core_float_types_f64_is_infinite, is_infinite, f64);

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
get_float_const!(nstd_core_float_types_f32_pi, f32, f32, PI);
get_float_const!(nstd_core_float_types_f64_pi, f64, f64, PI);
get_float_const!(nstd_core_float_types_f32_e, f32, f32, E);
get_float_const!(nstd_core_float_types_f64_e, f64, f64, E);
