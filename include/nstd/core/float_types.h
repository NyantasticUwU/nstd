#ifndef NSTD_CORE_FLOAT_TYPES_H_INCLUDED
#define NSTD_CORE_FLOAT_TYPES_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Returns the minimum value of a float.
/// Returns: `float min`: Minimum value of a float.
NSTDAPI float nstd_core_float_types_float_min();
/// Returns the maximum value of a float.
/// Returns: `float min`: Maximum value of a float.
NSTDAPI float nstd_core_float_types_float_max();

/// Returns the minimum value of a double.
/// Returns: `double min`: Minimum value of a double.
NSTDAPI double nstd_core_float_types_double_min();
/// Returns the maximum value of a double.
/// Returns: `double min`: Maximum value of a double.
NSTDAPI double nstd_core_float_types_double_max();

/// Returns NaN as a float.
/// Returns: `float nan`: NaN represented as a float.
NSTDAPI float nstd_core_float_types_float_nan();
/// Returns infinity as a float.
/// Returns: `float infinity`: Infinity represented as a float.
NSTDAPI float nstd_core_float_types_float_infinity();
/// Returns negative infinity as a float.
/// Returns: `float negative infinity`: Negative infinity represented as a float.
NSTDAPI float nstd_core_float_types_float_negative_infinity();

/// Returns NaN as a double.
/// Returns: `double nan`: NaN represented as a double.
NSTDAPI double nstd_core_float_types_double_nan();
/// Returns infinity as a double.
/// Returns: `double infinity`: Infinity represented as a double.
NSTDAPI double nstd_core_float_types_double_infinity();
/// Returns negative infinity as a double.
/// Returns: `double negative infinity`: Negative infinity represented as a double.
NSTDAPI double nstd_core_float_types_double_negative_infinity();

#ifdef __cplusplus
}
#endif
#endif
