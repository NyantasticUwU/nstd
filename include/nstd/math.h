#ifndef NSTD_MATH_H_INCLUDED
#define NSTD_MATH_H_INCLUDED
#include "core/def.h"
#include "nstd.h"
NSTDCPPSTART

/// Computes the square root of `x`.
/// Parameters:
///     `const NSTDFloat32 x` - The value.
/// Returns: `NSTDFloat32 sqrt` - The square root.
NSTDAPI NSTDFloat32 nstd_math_sqrt_f32(const NSTDFloat32 x);
/// Computes the square root of `x`.
/// Parameters:
///     `const NSTDFloat64 x` - The value.
/// Returns: `NSTDFloat64 sqrt` - The square root.
NSTDAPI NSTDFloat64 nstd_math_sqrt_f64(const NSTDFloat64 x);

/// Computes the cube root of `x`.
/// Parameters:
///     `const NSTDFloat32 x` - The value.
/// Returns: `NSTDFloat32 cbrt` - The cube root.
NSTDAPI NSTDFloat32 nstd_math_cbrt_f32(const NSTDFloat32 x);
/// Computes the cube root of `x`.
/// Parameters:
///     `const NSTDFloat64 x` - The value.
/// Returns: `NSTDFloat64 cbrt` - The cube root.
NSTDAPI NSTDFloat64 nstd_math_cbrt_f64(const NSTDFloat64 x);



/// Computes the sine value of `x`.
/// Parameters:
///     `const NSTDFloat32 x` - The value.
/// Returns: `NSTDFloat32 sin` - The sine value.
NSTDAPI NSTDFloat32 nstd_math_sin_f32(const NSTDFloat32 x);
/// Computes the sine value of `x`.
/// Parameters:
///     `const NSTDFloat64 x` - The value.
/// Returns: `NSTDFloat64 sin` - The sine value.
NSTDAPI NSTDFloat64 nstd_math_sin_f64(const NSTDFloat64 x);

/// Computes the cosine value of `x`.
/// Parameters:
///     `const NSTDFloat32 x` - The value.
/// Returns: `NSTDFloat32 cos` - The cosine value.
NSTDAPI NSTDFloat32 nstd_math_cos_f32(const NSTDFloat32 x);
/// Computes the cosine value of `x`.
/// Parameters:
///     `const NSTDFloat64 x` - The value.
/// Returns: `NSTDFloat64 cos` - The cosine value.
NSTDAPI NSTDFloat64 nstd_math_cos_f64(const NSTDFloat64 x);

/// Computes the tangent value of `x`.
/// Parameters:
///     `const NSTDFloat32 x` - The value.
/// Returns: `NSTDFloat32 tan` - The tangent value.
NSTDAPI NSTDFloat32 nstd_math_tan_f32(const NSTDFloat32 x);
/// Computes the tangent value of `x`.
/// Parameters:
///     `const NSTDFloat64 x` - The value.
/// Returns: `NSTDFloat64 tan` - The tangent value.
NSTDAPI NSTDFloat64 nstd_math_tan_f64(const NSTDFloat64 x);



/// Rounds the value of `x` up.
/// Parameters:
///     `const NSTDFloat32 x` - The value.
/// Returns: `NSTDFloat32 r` - The rounded value.
NSTDAPI NSTDFloat32 nstd_math_ceil_f32(const NSTDFloat32 x);
/// Rounds the value of `x` up.
/// Parameters:
///     `const NSTDFloat64 x` - The value.
/// Returns: `NSTDFloat64 r` - The rounded value.
NSTDAPI NSTDFloat64 nstd_math_ceil_f64(const NSTDFloat64 x);

/// Rounds the value of `x` down.
/// Parameters:
///     `const NSTDFloat32 x` - The value.
/// Returns: `NSTDFloat32 r` - The rounded value.
NSTDAPI NSTDFloat32 nstd_math_floor_f32(const NSTDFloat32 x);
/// Rounds the value of `x` down.
/// Parameters:
///     `const NSTDFloat64 x` - The value.
/// Returns: `NSTDFloat64 r` - The rounded value.
NSTDAPI NSTDFloat64 nstd_math_floor_f64(const NSTDFloat64 x);

/// Rounds the value of `x`.
/// Parameters:
///     `const NSTDFloat32 x` - The value.
/// Returns: `NSTDFloat32 r` - The rounded value.
NSTDAPI NSTDFloat32 nstd_math_round_f32(const NSTDFloat32 x);
/// Rounds the value of `x`.
/// Parameters:
///     `const NSTDFloat64 x` - The value.
/// Returns: `NSTDFloat64 r` - The rounded value.
NSTDAPI NSTDFloat64 nstd_math_round_f64(const NSTDFloat64 x);

NSTDCPPEND
#endif
