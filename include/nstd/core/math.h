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
///     `const NSTDFloat32 x` - The first value.
///     `const NSTDFloat32 y` - The second value.
/// Returns: `NSTDFloat32 mod` - The modulus value.
NSTDAPI NSTDFloat32 nstd_core_math_mod_f32(const NSTDFloat32 x, const NSTDFloat32 y);
/// Computes the modulus value of `x` % `y`.
/// Parameters:
///     `const NSTDFloat64 x` - The first value.
///     `const NSTDFloat64 y` - The second value.
/// Returns: `NSTDFloat64 mod` - The modulus value.
NSTDAPI NSTDFloat64 nstd_core_math_mod_f64(const NSTDFloat64 x, const NSTDFloat64 y);
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
///     `const NSTDFloat32 x` - The first value.
///     `const NSTDFloat32 y` - The second value.
/// Returns: `NSTDFloat32 max` - The larger value.
NSTDAPI NSTDFloat32 nstd_core_math_max_f32(const NSTDFloat32 x, const NSTDFloat32 y);
/// Determines the larger of two values.
/// Parameters:
///     `const NSTDFloat64 x` - The first value.
///     `const NSTDFloat64 y` - The second value.
/// Returns: `NSTDFloat64 max` - The larger value.
NSTDAPI NSTDFloat64 nstd_core_math_max_f64(const NSTDFloat64 x, const NSTDFloat64 y);
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
///     `const NSTDFloat32 x` - The first value.
///     `const NSTDFloat32 y` - The second value.
/// Returns: `NSTDFloat32 min` - The smaller value.
NSTDAPI NSTDFloat32 nstd_core_math_min_f32(const NSTDFloat32 x, const NSTDFloat32 y);
/// Determines the smaller of two values.
/// Parameters:
///     `const NSTDFloat64 x` - The first value.
///     `const NSTDFloat64 y` - The second value.
/// Returns: `NSTDFloat64 min` - The smaller value.
NSTDAPI NSTDFloat64 nstd_core_math_min_f64(const NSTDFloat64 x, const NSTDFloat64 y);
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



/// Restricts a NSTDFloat32 to a certain range.
/// Parameters:
///     `const NSTDFloat32 x` - The value.
///     `const NSTDFloat32 min` - The minimum value.
///     `const NSTDFloat32 max` - The maximum value.
/// Returns: `NSTDFloat32 clamp` - The clamped value.
NSTDAPI NSTDFloat32 nstd_core_math_clamp_f32(
    const NSTDFloat32 x,
    const NSTDFloat32 min,
    const NSTDFloat32 max);
/// Restricts a NSTDFloat64 to a certain range.
/// Parameters:
///     `const NSTDFloat64 x` - The value.
///     `const NSTDFloat64 min` - The minimum value.
///     `const NSTDFloat64 max` - The maximum value.
/// Returns: `NSTDFloat64 clamp` - The clamped value.
NSTDAPI NSTDFloat64 nstd_core_math_clamp_f64(
    const NSTDFloat64 x,
    const NSTDFloat64 min,
    const NSTDFloat64 max);
/// Restricts a NSTDUInt8 to a certain range.
/// Parameters:
///     `const NSTDUInt8 x` - The value.
///     `const NSTDUInt8 min` - The minimum value.
///     `const NSTDUInt8 max` - The maximum value.
/// Returns: `NSTDUInt8 clamp` - The clamped value.
NSTDAPI NSTDUInt8 nstd_core_math_clamp_u8(
    const NSTDUInt8 x,
    const NSTDUInt8 min,
    const NSTDUInt8 max);
/// Restricts a NSTDInt8 to a certain range.
/// Parameters:
///     `const NSTDInt8 x` - The value.
///     `const NSTDInt8 min` - The minimum value.
///     `const NSTDInt8 max` - The maximum value.
/// Returns: `NSTDInt8 clamp` - The clamped value.
NSTDAPI NSTDInt8 nstd_core_math_clamp_i8(
    const NSTDInt8 x,
    const NSTDInt8 min,
    const NSTDInt8 max);
/// Restricts a NSTDUInt16 to a certain range.
/// Parameters:
///     `const NSTDUInt16 x` - The value.
///     `const NSTDUInt16 min` - The minimum value.
///     `const NSTDUInt16 max` - The maximum value.
/// Returns: `NSTDUInt16 clamp` - The clamped value.
NSTDAPI NSTDUInt16 nstd_core_math_clamp_u16(
    const NSTDUInt16 x,
    const NSTDUInt16 min,
    const NSTDUInt16 max);
/// Restricts a NSTDInt16 to a certain range.
/// Parameters:
///     `const NSTDInt16 x` - The value.
///     `const NSTDInt16 min` - The minimum value.
///     `const NSTDInt16 max` - The maximum value.
/// Returns: `NSTDInt16 clamp` - The clamped value.
NSTDAPI NSTDInt16 nstd_core_math_clamp_i16(
    const NSTDInt16 x,
    const NSTDInt16 min,
    const NSTDInt16 max);
/// Restricts a NSTDUInt32 to a certain range.
/// Parameters:
///     `const NSTDUInt32 x` - The value.
///     `const NSTDUInt32 min` - The minimum value.
///     `const NSTDUInt32 max` - The maximum value.
/// Returns: `NSTDUInt32 clamp` - The clamped value.
NSTDAPI NSTDUInt32 nstd_core_math_clamp_u32(
    const NSTDUInt32 x,
    const NSTDUInt32 min,
    const NSTDUInt32 max);
/// Restricts a NSTDInt32 to a certain range.
/// Parameters:
///     `const NSTDInt32 x` - The value.
///     `const NSTDInt32 min` - The minimum value.
///     `const NSTDInt32 max` - The maximum value.
/// Returns: `NSTDInt32 clamp` - The clamped value.
NSTDAPI NSTDInt32 nstd_core_math_clamp_i32(
    const NSTDInt32 x,
    const NSTDInt32 min,
    const NSTDInt32 max);
/// Restricts a NSTDUInt64 to a certain range.
/// Parameters:
///     `const NSTDUInt64 x` - The value.
///     `const NSTDUInt64 min` - The minimum value.
///     `const NSTDUInt64 max` - The maximum value.
/// Returns: `NSTDUInt64 clamp` - The clamped value.
NSTDAPI NSTDUInt64 nstd_core_math_clamp_u64(
    const NSTDUInt64 x,
    const NSTDUInt64 min,
    const NSTDUInt64 max);
/// Restricts a NSTDInt64 to a certain range.
/// Parameters:
///     `const NSTDInt64 x` - The value.
///     `const NSTDInt64 min` - The minimum value.
///     `const NSTDInt64 max` - The maximum value.
/// Returns: `NSTDInt64 clamp` - The clamped value.
NSTDAPI NSTDInt64 nstd_core_math_clamp_i64(
    const NSTDInt64 x,
    const NSTDInt64 min,
    const NSTDInt64 max);
/// Restricts a NSTDISize to a certain range.
/// Parameters:
///     `const NSTDISize x` - The value.
///     `const NSTDISize min` - The minimum value.
///     `const NSTDISize max` - The maximum value.
/// Returns: `NSTDISize clamp` - The clamped value.
NSTDAPI NSTDISize nstd_core_math_clamp_usize(
    const NSTDISize x,
    const NSTDISize min,
    const NSTDISize max);
/// Restricts a NSTDUSize to a certain range.
/// Parameters:
///     `const NSTDUSize x` - The value.
///     `const NSTDUSize min` - The minimum value.
///     `const NSTDUSize max` - The maximum value.
/// Returns: `NSTDUSize clamp` - The clamped value.
NSTDAPI NSTDUSize nstd_core_math_clamp_isize(
    const NSTDUSize x,
    const NSTDUSize min,
    const NSTDUSize max);

#ifdef __cplusplus
}
#endif
#endif
