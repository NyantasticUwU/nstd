#ifndef NSTD_RAND_H_INCLUDED
#define NSTD_RAND_H_INCLUDED
#include "core/def.h"
#include "nstd.h"
NSTDCPPSTART

/// Generates a random `NSTDBool`.
///
/// # Returns
///
/// `NSTDBool boolean` - Either `NSTD_BOOL_TRUE` or `NSTD_BOOL_FALSE`.
NSTDAPI NSTDBool nstd_rand_bool();

/// Generates a random `NSTDUnichar`.
///
/// # Returns
///
/// `NSTDUnichar unichar` - A random unicode character.
NSTDAPI NSTDUnichar nstd_rand_unichar();

/// Generates a random `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 flt` - A random floating point value with the range of [0, 1).
NSTDAPI NSTDFloat32 nstd_rand_f32();
/// Generates a random `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 flt` - A random floating point value with the range of [0, 1).
NSTDAPI NSTDFloat64 nstd_rand_f64();

/// Generates a random `NSTDUInt8`.
///
/// # Returns
///
/// `NSTDUInt8 u8` - A random `NSTDUInt8`.
NSTDAPI NSTDUInt8 nstd_rand_u8();
/// Generates a random `NSTDInt8`.
///
/// # Returns
///
/// `NSTDInt8 i8` - A random `NSTDInt8`.
NSTDAPI NSTDInt8 nstd_rand_i8();
/// Generates a random `NSTDUInt16`.
///
/// # Returns
///
/// `NSTDUInt16 u16` - A random `NSTDUInt16`.
NSTDAPI NSTDUInt16 nstd_rand_u16();
/// Generates a random `NSTDInt16`.
///
/// # Returns
///
/// `NSTDInt16 i16` - A random `NSTDInt16`.
NSTDAPI NSTDInt16 nstd_rand_i16();
/// Generates a random `NSTDUInt32`.
///
/// # Returns
///
/// `NSTDUInt32 u32` - A random `NSTDUInt32`.
NSTDAPI NSTDUInt32 nstd_rand_u32();
/// Generates a random `NSTDInt32`.
///
/// # Returns
///
/// `NSTDInt32 i32` - A random `NSTDInt32`.
NSTDAPI NSTDInt32 nstd_rand_i32();
/// Generates a random `NSTDUInt64`.
///
/// # Returns
///
/// `NSTDUInt64 u64` - A random `NSTDUInt64`.
NSTDAPI NSTDUInt64 nstd_rand_u64();
/// Generates a random `NSTDInt64`.
///
/// # Returns
///
/// `NSTDInt64 i64` - A random `NSTDInt64`.
NSTDAPI NSTDInt64 nstd_rand_i64();
/// Generates a random `NSTDUSize`.
///
/// # Returns
///
/// `NSTDUSize usize` - A random `NSTDUSize`.
NSTDAPI NSTDUSize nstd_rand_usize();
/// Generates a random `NSTDISize`.
///
/// # Returns
///
/// `NSTDISize isize` - A random `NSTDISize`.
NSTDAPI NSTDISize nstd_rand_isize();

NSTDCPPEND
#endif
