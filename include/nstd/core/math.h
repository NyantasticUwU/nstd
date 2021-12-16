#ifndef NSTD_CORE_MATH_H_INCLUDED
#define NSTD_CORE_MATH_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Computes the absolute value of `x`.
/// Parameters:
///     `const signed char x` - The value.
/// Returns: `signed char abs` - The absolute value.
NSTDAPI signed char nstd_core_math_abs_schar(const signed char x);
/// Computes the absolute value of `x`.
/// Parameters:
///     `const short x` - The value.
/// Returns: `short abs` - The absolute value.
NSTDAPI short nstd_core_math_abs_short(const short x);
/// Computes the absolute value of `x`.
/// Parameters:
///     `const int x` - The value.
/// Returns: `int abs` - The absolute value.
NSTDAPI int nstd_core_math_abs_int(const int x);
/// Computes the absolute value of `x`.
/// Parameters:
///     `const long x` - The value.
/// Returns: `long abs` - The absolute value.
NSTDAPI long nstd_core_math_abs_long(const long x);
/// Computes the absolute value of `x`.
/// Parameters:
///     `const long long x` - The value.
/// Returns: `long long abs` - The absolute value.
NSTDAPI long long nstd_core_math_abs_longlong(const long long x);



/// Computes the modulus value of `x` % `y`.
/// Parameters:
///     `const float x` - The first value.
///     `const float y` - The second value.
/// Returns: `float mod` - The modulus value.
NSTDAPI float nstd_core_math_mod_float(const float x, const float y);
/// Computes the modulus value of `x` % `y`.
/// Parameters:
///     `const double x` - The first value.
///     `const double y` - The second value.
/// Returns: `double mod` - The modulus value.
NSTDAPI double nstd_core_math_mod_double(const double x, const double y);
/// Computes the modulus value of `x` % `y`.
/// Parameters:
///     `const signed char x` - The first value.
///     `const signed char y` - The second value.
/// Returns: `signed char mod` - The modulus value.
NSTDAPI signed char nstd_core_math_mod_schar(const signed char x, const signed char y);
/// Computes the modulus value of `x` % `y`.
/// Parameters:
///     `const short x` - The first value.
///     `const short y` - The second value.
/// Returns: `short mod` - The modulus value.
NSTDAPI short nstd_core_math_mod_short(const short x, const short y);
/// Computes the modulus value of `x` % `y`.
/// Parameters:
///     `const int x` - The first value.
///     `const int y` - The second value.
/// Returns: `int mod` - The modulus value.
NSTDAPI int nstd_core_math_mod_int(const int x, const int y);
/// Computes the modulus value of `x` % `y`.
/// Parameters:
///     `const long x` - The first value.
///     `const long y` - The second value.
/// Returns: `long mod` - The modulus value.
NSTDAPI long nstd_core_math_mod_long(const long x, const long y);
/// Computes the modulus value of `x` % `y`.
/// Parameters:
///     `const long long x` - The first value.
///     `const long long y` - The second value.
/// Returns: `long long mod` - The modulus value.
NSTDAPI long long nstd_core_math_mod_longlong(const long long x, const long long y);



/// Determines the larger of two values.
/// Parameters:
///     `const float x` - The first value.
///     `const float y` - The second value.
/// Returns: `float max` - The larger value.
NSTDAPI float nstd_core_math_max_float(const float x, const float y);
/// Determines the larger of two values.
/// Parameters:
///     `const double x` - The first value.
///     `const double y` - The second value.
/// Returns: `double max` - The larger value.
NSTDAPI double nstd_core_math_max_double(const double x, const double y);
/// Determines the larger of two values.
/// Parameters:
///     `const signed char x` - The first value.
///     `const signed char y` - The second value.
/// Returns: `signed char max` - The larger value.
NSTDAPI signed char nstd_core_math_max_schar(const signed char x, const signed char y);
/// Determines the larger of two values.
/// Parameters:
///     `const short x` - The first value.
///     `const short y` - The second value.
/// Returns: `short max` - The larger value.
NSTDAPI short nstd_core_math_max_short(const short x, const short y);
/// Determines the larger of two values.
/// Parameters:
///     `const int x` - The first value.
///     `const int y` - The second value.
/// Returns: `int max` - The larger value.
NSTDAPI int nstd_core_math_max_int(const int x, const int y);
/// Determines the larger of two values.
/// Parameters:
///     `const long x` - The first value.
///     `const long y` - The second value.
/// Returns: `long max` - The larger value.
NSTDAPI long nstd_core_math_max_long(const long x, const long y);
/// Determines the larger of two values.
/// Parameters:
///     `const long long x` - The first value.
///     `const long long y` - The second value.
/// Returns: `long long max` - The larger value.
NSTDAPI long long nstd_core_math_max_longlong(const long long x, const long long y);



/// Determines the smaller of two values.
/// Parameters:
///     `const float x` - The first value.
///     `const float y` - The second value.
/// Returns: `float min` - The smaller value.
NSTDAPI float nstd_core_math_min_float(const float x, const float y);
/// Determines the smaller of two values.
/// Parameters:
///     `const double x` - The first value.
///     `const double y` - The second value.
/// Returns: `double min` - The smaller value.
NSTDAPI double nstd_core_math_min_double(const double x, const double y);
/// Determines the smaller of two values.
/// Parameters:
///     `const signed char x` - The first value.
///     `const signed char y` - The second value.
/// Returns: `signed char min` - The smaller value.
NSTDAPI signed char nstd_core_math_min_schar(const signed char x, const signed char y);
/// Determines the smaller of two values.
/// Parameters:
///     `const short x` - The first value.
///     `const short y` - The second value.
/// Returns: `short min` - The smaller value.
NSTDAPI short nstd_core_math_min_short(const short x, const short y);
/// Determines the smaller of two values.
/// Parameters:
///     `const int x` - The first value.
///     `const int y` - The second value.
/// Returns: `int min` - The smaller value.
NSTDAPI int nstd_core_math_min_int(const int x, const int y);
/// Determines the smaller of two values.
/// Parameters:
///     `const long x` - The first value.
///     `const long y` - The second value.
/// Returns: `long min` - The smaller value.
NSTDAPI long nstd_core_math_min_long(const long x, const long y);
/// Determines the smaller of two values.
/// Parameters:
///     `const long long x` - The first value.
///     `const long long y` - The second value.
/// Returns: `long long min` - The smaller value.
NSTDAPI long long nstd_core_math_min_longlong(const long long x, const long long y);



/// Computes `x` raised to the power of `y`.
/// Parameters:
///     `const signed char x` - The first value.
///     `const unsigned int y` - The second value.
/// Returns: `signed char pow` - The larger value.
NSTDAPI signed char nstd_core_math_pow_schar(const signed char x, const unsigned int y);
/// Computes `x` raised to the power of `y`.
/// Parameters:
///     `const short x` - The first value.
///     `const unsigned int y` - The second value.
/// Returns: `short pow` - The larger value.
NSTDAPI short nstd_core_math_pow_short(const short x, const unsigned int y);
/// Computes `x` raised to the power of `y`.
/// Parameters:
///     `const int x` - The first value.
///     `const unsigned int y` - The second value.
/// Returns: `int pow` - The larger value.
NSTDAPI int nstd_core_math_pow_int(const int x, const unsigned int y);
/// Computes `x` raised to the power of `y`.
/// Parameters:
///     `const long x` - The first value.
///     `const unsigned int y` - The second value.
/// Returns: `long pow` - The larger value.
NSTDAPI long nstd_core_math_pow_long(const long x, const unsigned int y);
/// Computes `x` raised to the power of `y`.
/// Parameters:
///     `const long long x` - The first value.
///     `const unsigned int y` - The second value.
/// Returns: `long long pow` - The larger value.
NSTDAPI long long nstd_core_math_pow_longlong(const long long x, const unsigned int y);

#ifdef __cplusplus
}
#endif
#endif
