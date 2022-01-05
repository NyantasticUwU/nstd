#ifndef NSTD_CORE_MATH_H_INCLUDED
#define NSTD_CORE_MATH_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Computes the absolute value of `x`.
/// Parameters:
///     `const NSTDInt8 x` - The value.
/// Returns: `NSTDInt8 abs` - The absolute value.
NSTDAPI NSTDInt8 nstd_core_math_abs_i8(const NSTDInt8 x);
/// Computes the absolute value of `x`.
/// Parameters:
///     `const NSTDInt16 x` - The value.
/// Returns: `NSTDInt16 abs` - The absolute value.
NSTDAPI NSTDInt16 nstd_core_math_abs_i16(const NSTDInt16 x);
/// Computes the absolute value of `x`.
/// Parameters:
///     `const NSTDInt32 x` - The value.
/// Returns: `NSTDInt32 abs` - The absolute value.
NSTDAPI NSTDInt32 nstd_core_math_abs_i32(const NSTDInt32 x);
/// Computes the absolute value of `x`.
/// Parameters:
///     `const NSTDInt64 x` - The value.
/// Returns: `NSTDInt64 abs` - The absolute value.
NSTDAPI NSTDInt64 nstd_core_math_abs_i64(const NSTDInt64 x);
/// Computes the absolute value of `x`.
/// Parameters:
///     `const NSTDISize x` - The value.
/// Returns: `NSTDISize abs` - The absolute value.
NSTDAPI NSTDISize nstd_core_math_abs_isize(const NSTDISize x);



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
///     `const NSTDUInt8 x` - The first value.
///     `const NSTDUInt8 y` - The second value.
/// Returns: `NSTDUInt8 mod` - The modulus value.
NSTDAPI NSTDUInt8 nstd_core_math_mod_u8(const NSTDUInt8 x, const NSTDUInt8 y);
/// Computes the modulus value of `x` % `y`.
/// Parameters:
///     `const NSTDInt8 x` - The first value.
///     `const NSTDInt8 y` - The second value.
/// Returns: `NSTDInt8 mod` - The modulus value.
NSTDAPI NSTDInt8 nstd_core_math_mod_i8(const NSTDInt8 x, const NSTDInt8 y);
/// Computes the modulus value of `x` % `y`.
/// Parameters:
///     `const NSTDUInt16 x` - The first value.
///     `const NSTDUInt16 y` - The second value.
/// Returns: `NSTDUInt16 mod` - The modulus value.
NSTDAPI NSTDUInt16 nstd_core_math_mod_u16(const NSTDUInt16 x, const NSTDUInt16 y);
/// Computes the modulus value of `x` % `y`.
/// Parameters:
///     `const NSTDInt16 x` - The first value.
///     `const NSTDInt16 y` - The second value.
/// Returns: `NSTDInt16 mod` - The modulus value.
NSTDAPI NSTDInt16 nstd_core_math_mod_i16(const NSTDInt16 x, const NSTDInt16 y);
/// Computes the modulus value of `x` % `y`.
/// Parameters:
///     `const NSTDUInt32 x` - The first value.
///     `const NSTDUInt32 y` - The second value.
/// Returns: `NSTDUInt32 mod` - The modulus value.
NSTDAPI NSTDUInt32 nstd_core_math_mod_u32(const NSTDUInt32 x, const NSTDUInt32 y);
/// Computes the modulus value of `x` % `y`.
/// Parameters:
///     `const NSTDInt32 x` - The first value.
///     `const NSTDInt32 y` - The second value.
/// Returns: `NSTDInt32 mod` - The modulus value.
NSTDAPI NSTDInt32 nstd_core_math_mod_i32(const NSTDInt32 x, const NSTDInt32 y);
/// Computes the modulus value of `x` % `y`.
/// Parameters:
///     `const NSTDUInt64 x` - The first value.
///     `const NSTDUInt64 y` - The second value.
/// Returns: `NSTDUInt64 mod` - The modulus value.
NSTDAPI NSTDUInt64 nstd_core_math_mod_u64(const NSTDUInt64 x, const NSTDUInt64 y);
/// Computes the modulus value of `x` % `y`.
/// Parameters:
///     `const NSTDInt64 x` - The first value.
///     `const NSTDInt64 y` - The second value.
/// Returns: `NSTDInt64 mod` - The modulus value.
NSTDAPI NSTDInt64 nstd_core_math_mod_i64(const NSTDInt64 x, const NSTDInt64 y);
/// Computes the modulus value of `x` % `y`.
/// Parameters:
///     `const NSTDUSize x` - The first value.
///     `const NSTDUSize y` - The second value.
/// Returns: `NSTDUSize mod` - The modulus value.
NSTDAPI NSTDUSize nstd_core_math_mod_usize(const NSTDUSize x, const NSTDUSize y);
/// Computes the modulus value of `x` % `y`.
/// Parameters:
///     `const NSTDISize x` - The first value.
///     `const NSTDISize y` - The second value.
/// Returns: `NSTDISize mod` - The modulus value.
NSTDAPI NSTDISize nstd_core_math_mod_isize(const NSTDISize x, const NSTDISize y);



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
///     `const NSTDUInt8 x` - The first value.
///     `const NSTDUInt8 y` - The second value.
/// Returns: `NSTDUInt8 max` - The larger value.
NSTDAPI NSTDUInt8 nstd_core_math_max_u8(const NSTDUInt8 x, const NSTDUInt8 y);
/// Determines the larger of two values.
/// Parameters:
///     `const NSTDInt8 x` - The first value.
///     `const NSTDInt8 y` - The second value.
/// Returns: `NSTDInt8 max` - The larger value.
NSTDAPI NSTDInt8 nstd_core_math_max_i8(const NSTDInt8 x, const NSTDInt8 y);
/// Determines the larger of two values.
/// Parameters:
///     `const NSTDUInt16 x` - The first value.
///     `const NSTDUInt16 y` - The second value.
/// Returns: `NSTDUInt16 max` - The larger value.
NSTDAPI NSTDUInt16 nstd_core_math_max_u16(const NSTDUInt16 x, const NSTDUInt16 y);
/// Determines the larger of two values.
/// Parameters:
///     `const NSTDInt16 x` - The first value.
///     `const NSTDInt16 y` - The second value.
/// Returns: `NSTDInt16 max` - The larger value.
NSTDAPI NSTDInt16 nstd_core_math_max_i16(const NSTDInt16 x, const NSTDInt16 y);
/// Determines the larger of two values.
/// Parameters:
///     `const NSTDUInt32 x` - The first value.
///     `const NSTDUInt32 y` - The second value.
/// Returns: `NSTDUInt32 max` - The larger value.
NSTDAPI NSTDUInt32 nstd_core_math_max_u32(const NSTDUInt32 x, const NSTDUInt32 y);
/// Determines the larger of two values.
/// Parameters:
///     `const NSTDInt32 x` - The first value.
///     `const NSTDInt32 y` - The second value.
/// Returns: `NSTDInt32 max` - The larger value.
NSTDAPI NSTDInt32 nstd_core_math_max_i32(const NSTDInt32 x, const NSTDInt32 y);
/// Determines the larger of two values.
/// Parameters:
///     `const NSTDUInt64 x` - The first value.
///     `const NSTDUInt64 y` - The second value.
/// Returns: `NSTDUInt64 max` - The larger value.
NSTDAPI NSTDUInt64 nstd_core_math_max_u64(const NSTDUInt64 x, const NSTDUInt64 y);
/// Determines the larger of two values.
/// Parameters:
///     `const NSTDInt64 x` - The first value.
///     `const NSTDInt64 y` - The second value.
/// Returns: `NSTDInt64 max` - The larger value.
NSTDAPI NSTDInt64 nstd_core_math_max_i64(const NSTDInt64 x, const NSTDInt64 y);
/// Determines the larger of two values.
/// Parameters:
///     `const NSTDUSize x` - The first value.
///     `const NSTDUSize y` - The second value.
/// Returns: `NSTDUSize max` - The larger value.
NSTDAPI NSTDUSize nstd_core_math_max_usize(const NSTDUSize x, const NSTDUSize y);
/// Determines the larger of two values.
/// Parameters:
///     `const NSTDISize x` - The first value.
///     `const NSTDISize y` - The second value.
/// Returns: `NSTDISize max` - The larger value.
NSTDAPI NSTDISize nstd_core_math_max_isize(const NSTDISize x, const NSTDISize y);



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
///     `const NSTDUInt8 x` - The first value.
///     `const NSTDUInt8 y` - The second value.
/// Returns: `NSTDUInt8 min` - The smaller value.
NSTDAPI NSTDUInt8 nstd_core_math_min_u8(const NSTDUInt8 x, const NSTDUInt8 y);
/// Determines the smaller of two values.
/// Parameters:
///     `const NSTDInt8 x` - The first value.
///     `const NSTDInt8 y` - The second value.
/// Returns: `NSTDInt8 min` - The smaller value.
NSTDAPI NSTDInt8 nstd_core_math_min_i8(const NSTDInt8 x, const NSTDInt8 y);
/// Determines the smaller of two values.
/// Parameters:
///     `const NSTDUInt16 x` - The first value.
///     `const NSTDUInt16 y` - The second value.
/// Returns: `NSTDUInt16 min` - The smaller value.
NSTDAPI NSTDUInt16 nstd_core_math_min_u16(const NSTDUInt16 x, const NSTDUInt16 y);
/// Determines the smaller of two values.
/// Parameters:
///     `const NSTDInt16 x` - The first value.
///     `const NSTDInt16 y` - The second value.
/// Returns: `NSTDInt16 min` - The smaller value.
NSTDAPI NSTDInt16 nstd_core_math_min_i16(const NSTDInt16 x, const NSTDInt16 y);
/// Determines the smaller of two values.
/// Parameters:
///     `const NSTDUInt32 x` - The first value.
///     `const NSTDUInt32 y` - The second value.
/// Returns: `NSTDUInt32 min` - The smaller value.
NSTDAPI NSTDUInt32 nstd_core_math_min_u32(const NSTDUInt32 x, const NSTDUInt32 y);
/// Determines the smaller of two values.
/// Parameters:
///     `const NSTDInt32 x` - The first value.
///     `const NSTDInt32 y` - The second value.
/// Returns: `NSTDInt32 min` - The smaller value.
NSTDAPI NSTDInt32 nstd_core_math_min_i32(const NSTDInt32 x, const NSTDInt32 y);
/// Determines the smaller of two values.
/// Parameters:
///     `const NSTDUInt64 x` - The first value.
///     `const NSTDUInt64 y` - The second value.
/// Returns: `NSTDUInt64 min` - The smaller value.
NSTDAPI NSTDUInt64 nstd_core_math_min_u64(const NSTDUInt64 x, const NSTDUInt64 y);
/// Determines the smaller of two values.
/// Parameters:
///     `const NSTDInt64 x` - The first value.
///     `const NSTDInt64 y` - The second value.
/// Returns: `NSTDInt64 min` - The smaller value.
NSTDAPI NSTDInt64 nstd_core_math_min_i64(const NSTDInt64 x, const NSTDInt64 y);
/// Determines the smaller of two values.
/// Parameters:
///     `const NSTDUSize x` - The first value.
///     `const NSTDUSize y` - The second value.
/// Returns: `NSTDUSize min` - The smaller value.
NSTDAPI NSTDUSize nstd_core_math_min_usize(const NSTDUSize x, const NSTDUSize y);
/// Determines the smaller of two values.
/// Parameters:
///     `const NSTDISize x` - The first value.
///     `const NSTDISize y` - The second value.
/// Returns: `NSTDISize min` - The smaller value.
NSTDAPI NSTDISize nstd_core_math_min_isize(const NSTDISize x, const NSTDISize y);



/// Computes `x` raised to the power of `y`.
/// Parameters:
///     `const NSTDUInt8 x` - The first value.
///     `const NSTDUInt32 y` - The second value.
/// Returns: `NSTDUInt8 pow` - The larger value.
NSTDAPI NSTDUInt8 nstd_core_math_pow_u8(const NSTDUInt8 x, const NSTDUInt32 y);
/// Computes `x` raised to the power of `y`.
/// Parameters:
///     `const NSTDInt8 x` - The first value.
///     `const NSTDUInt32 y` - The second value.
/// Returns: `NSTDInt8 pow` - The larger value.
NSTDAPI NSTDInt8 nstd_core_math_pow_i8(const NSTDInt8 x, const NSTDUInt32 y);
/// Computes `x` raised to the power of `y`.
/// Parameters:
///     `const NSTDUInt16 x` - The first value.
///     `const NSTDUInt32 y` - The second value.
/// Returns: `NSTDUInt16 pow` - The larger value.
NSTDAPI NSTDUInt16 nstd_core_math_pow_u16(const NSTDUInt16 x, const NSTDUInt32 y);
/// Computes `x` raised to the power of `y`.
/// Parameters:
///     `const NSTDInt16 x` - The first value.
///     `const NSTDUInt32 y` - The second value.
/// Returns: `NSTDInt16 pow` - The larger value.
NSTDAPI NSTDInt16 nstd_core_math_pow_i16(const NSTDInt16 x, const NSTDUInt32 y);
/// Computes `x` raised to the power of `y`.
/// Parameters:
///     `const NSTDUInt32 x` - The first value.
///     `const NSTDUInt32 y` - The second value.
/// Returns: `NSTDUInt32 pow` - The larger value.
NSTDAPI NSTDUInt32 nstd_core_math_pow_u32(const NSTDUInt32 x, const NSTDUInt32 y);
/// Computes `x` raised to the power of `y`.
/// Parameters:
///     `const NSTDInt32 x` - The first value.
///     `const NSTDUInt32 y` - The second value.
/// Returns: `NSTDInt32 pow` - The larger value.
NSTDAPI NSTDInt32 nstd_core_math_pow_i32(const NSTDInt32 x, const NSTDUInt32 y);
/// Computes `x` raised to the power of `y`.
/// Parameters:
///     `const NSTDUInt64 x` - The first value.
///     `const NSTDUInt32 y` - The second value.
/// Returns: `NSTDUInt64 pow` - The larger value.
NSTDAPI NSTDUInt64 nstd_core_math_pow_u64(const NSTDUInt64 x, const NSTDUInt32 y);
/// Computes `x` raised to the power of `y`.
/// Parameters:
///     `const NSTDInt64 x` - The first value.
///     `const NSTDUInt32 y` - The second value.
/// Returns: `NSTDInt64 pow` - The larger value.
NSTDAPI NSTDInt64 nstd_core_math_pow_i64(const NSTDInt64 x, const NSTDUInt32 y);
/// Computes `x` raised to the power of `y`.
/// Parameters:
///     `const NSTDUSize x` - The first value.
///     `const NSTDUInt32 y` - The second value.
/// Returns: `NSTDUSize pow` - The larger value.
NSTDAPI NSTDUSize nstd_core_math_pow_usize(const NSTDUSize x, const NSTDUInt32 y);
/// Computes `x` raised to the power of `y`.
/// Parameters:
///     `const NSTDISize x` - The first value.
///     `const NSTDUInt32 y` - The second value.
/// Returns: `NSTDISize pow` - The larger value.
NSTDAPI NSTDISize nstd_core_math_pow_isize(const NSTDISize x, const NSTDUInt32 y);

#ifdef __cplusplus
}
#endif
#endif
