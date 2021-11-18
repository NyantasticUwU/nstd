#ifndef NSTD_CORE_FLOAT_TYPES_H_INCLUDED
#define NSTD_CORE_FLOAT_TYPES_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Returns the smallest finite float value.
/// Returns: `float min` - Smallest finite float value.
NSTDAPI float nstd_core_float_types_float_min();
/// Returns the largest finite float value.
/// Returns: `float max` - Largest finite float value.
NSTDAPI float nstd_core_float_types_float_max();
/// Returns NaN represented as a float.
/// Returns: `float nan` - NaN.
NSTDAPI float nstd_core_float_types_float_nan();
/// Returns infinity represented as a float.
/// Returns: `float infinity` - Infinity.
NSTDAPI float nstd_core_float_types_float_infinity();
/// Returns negative infinity represented as a float.
/// Returns: `float negative_infinity` - Negative infinity.
NSTDAPI float nstd_core_float_types_float_negative_infinity();
/// Returns PI represented as a float.
/// Returns: `float pi` - PI.
NSTDAPI float nstd_core_float_types_float_pi();
/// Returns Euler's number represented as a float.
/// Returns: `float e` - Euler's number.
NSTDAPI float nstd_core_float_types_float_e();

/// Returns the smallest finite double value.
/// Returns: `double min` - Smallest finite double value.
NSTDAPI double nstd_core_float_types_double_min();
/// Returns the largest finite double value.
/// Returns: `double max` - Largest finite double value.
NSTDAPI double nstd_core_float_types_double_max();
/// Returns NaN represented as a double.
/// Returns: `double nan` - NaN.
NSTDAPI double nstd_core_float_types_double_nan();
/// Returns infinity represented as a double.
/// Returns: `double infinity` - Infinity.
NSTDAPI double nstd_core_float_types_double_infinity();
/// Returns negative infinity represented as a double.
/// Returns: `double negative_infinity` - Negative infinity.
NSTDAPI double nstd_core_float_types_double_negative_infinity();
/// Returns PI represented as a double.
/// Returns: `double pi` - PI.
NSTDAPI double nstd_core_float_types_double_pi();
/// Returns Euler's number represented as a double.
/// Returns: `double e` - Euler's number.
NSTDAPI double nstd_core_float_types_double_e();

#ifdef __cplusplus
}
#endif
#endif
