#ifndef NSTD_CORE_FLOAT_TYPES_H_INCLUDED
#define NSTD_CORE_FLOAT_TYPES_H_INCLUDED
#include "../nstd.h"
#include "def.h"
NSTDCPPSTART

/// Returns the smallest finite NSTDFloat32 value.
///
/// # Returns
///
/// `NSTDFloat32 min` - Smallest finite NSTDFloat32 value.
NSTDAPI NSTDFloat32 nstd_core_float_types_f32_min();
/// Returns the largest finite NSTDFloat32 value.
///
/// # Returns
///
/// `NSTDFloat32 max` - Largest finite NSTDFloat32 value.
NSTDAPI NSTDFloat32 nstd_core_float_types_f32_max();
/// Returns NaN represented as a NSTDFloat32.
///
/// # Returns
///
/// `NSTDFloat32 nan` - NaN.
NSTDAPI NSTDFloat32 nstd_core_float_types_f32_nan();
/// Returns infinity represented as a NSTDFloat32.
///
/// # Returns
///
/// `NSTDFloat32 infinity` - Infinity.
NSTDAPI NSTDFloat32 nstd_core_float_types_f32_infinity();
/// Returns negative infinity represented as a NSTDFloat32.
///
/// # Returns
///
/// `NSTDFloat32 negative_infinity` - Negative infinity.
NSTDAPI NSTDFloat32 nstd_core_float_types_f32_negative_infinity();
/// Returns nonzero if the NSTDFloat32 is nan.
///
/// # Parameters
///
/// - `const NSTDFloat32 f` - The NSTDFloat32.
///
/// # Returns
///
/// `NSTDBool is_nan` - True if the NSTDFloat32 is nan.
NSTDAPI NSTDBool nstd_core_float_types_f32_is_nan(const NSTDFloat32 f);
/// Returns nonzero if the NSTDFloat32 is infinite (positive or negative infinity).
///
/// # Parameters
///
/// - `const NSTDFloat32 f` - The NSTDFloat32.
///
/// # Returns
///
/// `NSTDBool is_infinite` - True if the NSTDFloat32 is infinite.
NSTDAPI NSTDBool nstd_core_float_types_f32_is_infinite(const NSTDFloat32 f);
/// Returns PI represented as a NSTDFloat32.
///
/// # Returns
///
/// `NSTDFloat32 pi` - PI.
NSTDAPI NSTDFloat32 nstd_core_float_types_f32_pi();
/// Returns Euler's number represented as a NSTDFloat32.
///
/// # Returns
///
/// `NSTDFloat32 e` - Euler's number.
NSTDAPI NSTDFloat32 nstd_core_float_types_f32_e();

/// Returns the smallest finite NSTDFloat64 value.
///
/// # Returns
///
/// `NSTDFloat64 min` - Smallest finite NSTDFloat64 value.
NSTDAPI NSTDFloat64 nstd_core_float_types_f64_min();
/// Returns the largest finite NSTDFloat64 value.
///
/// # Returns
///
/// `NSTDFloat64 max` - Largest finite NSTDFloat64 value.
NSTDAPI NSTDFloat64 nstd_core_float_types_f64_max();
/// Returns NaN represented as a NSTDFloat64.
///
/// # Returns
///
/// `NSTDFloat64 nan` - NaN.
NSTDAPI NSTDFloat64 nstd_core_float_types_f64_nan();
/// Returns infinity represented as a NSTDFloat64.
///
/// # Returns
///
/// `NSTDFloat64 infinity` - Infinity.
NSTDAPI NSTDFloat64 nstd_core_float_types_f64_infinity();
/// Returns negative infinity represented as a NSTDFloat64.
///
/// # Returns
///
/// `NSTDFloat64 negative_infinity` - Negative infinity.
NSTDAPI NSTDFloat64 nstd_core_float_types_f64_negative_infinity();
/// Returns nonzero if the NSTDFloat64 is nan.
///
/// # Parameters
///
/// - `const NSTDFloat64 d` - The NSTDFloat64.
///
/// # Returns
///
/// `NSTDBool is_nan` - True if the NSTDFloat64 is nan.
NSTDAPI NSTDBool nstd_core_float_types_f64_is_nan(const NSTDFloat64 d);
/// Returns nonzero if the NSTDFloat64 is infinite (positive or negative infinity).
///
/// # Parameters
///
/// - `const NSTDFloat64 d` - The NSTDFloat64.
///
/// # Returns
///
/// `NSTDBool is_infinite` - True if the NSTDFloat64 is infinite.
NSTDAPI NSTDBool nstd_core_float_types_f64_is_infinite(const NSTDFloat64 d);
/// Returns PI represented as a NSTDFloat64.
///
/// # Returns
///
/// `NSTDFloat64 pi` - PI.
NSTDAPI NSTDFloat64 nstd_core_float_types_f64_pi();
/// Returns Euler's number represented as a NSTDFloat64.
///
/// # Returns
///
/// `NSTDFloat64 e` - Euler's number.
NSTDAPI NSTDFloat64 nstd_core_float_types_f64_e();

NSTDCPPEND
#endif
