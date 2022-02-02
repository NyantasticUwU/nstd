use crate::core::def::NSTDBool;

/// Generates degrees/radians functions.
macro_rules! gen_deg_rad_fn {
    ($name: ident, $type: ty, $method: ident) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(x: $type) -> $type {
            x.$method()
        }
    };
}
// f32.
gen_deg_rad_fn!(nstd_core_math_degrees_f32, f32, to_degrees);
gen_deg_rad_fn!(nstd_core_math_radians_f32, f32, to_radians);
// f64.
gen_deg_rad_fn!(nstd_core_math_degrees_f64, f64, to_degrees);
gen_deg_rad_fn!(nstd_core_math_radians_f64, f64, to_radians);

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
