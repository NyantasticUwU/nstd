#ifndef NSTD_CORE_STR_H_INCLUDED
#define NSTD_CORE_STR_H_INCLUDED
#include "char_types.h"
#include "def.h"
#include "slice.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a view into an array of UTF-8 chars.
typedef struct
{
    NSTDSlice bytes;
} NSTDStr;

/// Creates a new `NSTDStr` from a cstring.
/// Parameters:
///     `const char *const cstr` - The cstring.
/// Returns: `NSTDStr str` - The new string slice.
NSTDAPI NSTDStr nstd_core_str_from_cstring(const char *const cstr);

/// Creates a new `NSTDStr` from an `NSTDSlice`. `slice->element_size` must be the size of one byte.
/// Parameters:
///     `const NSTDSlice *const slice` - The UTF-8 encoded byte slice.
/// Returns: `NSTDStr str` - The new string slice.
NSTDAPI NSTDStr nstd_core_str_from_bytes(const NSTDSlice *const slice);

/// Gets the length of a string slice.
/// Parameters:
///     `const NSTDStr *const str` - The string slice.
/// Returns: `NSTDUSize len` - The length of the UTF-8 encoded string slice, -1 on error.
NSTDAPI NSTDUSize nstd_core_str_len(const NSTDStr *const str);

/// Returns the number of bytes used by this string slice.
/// Parameters:
///     `const NSTDStr *const str` - The string slice.
/// Returns: `NSTDUSize len` - The number of bytes in the string slice.
NSTDAPI NSTDUSize nstd_core_str_byte_len(const NSTDStr *const str);

/// Compares two string slices.
/// Parameters:
///     `const NSTDStr *const str1` - The first string slice.
///     `const NSTDStr *const str2` - The second string slice.
/// Returns: `NSTDInt32 is_eq` - 1 if the two slices are equal, 0 otherwise.
NSTDAPI NSTDInt32 nstd_core_str_compare(const NSTDStr *const str1, const NSTDStr *const str2);

/// Checks if `str` contains `pattern`.
/// Parameters:
///     `const NSTDStr *const str` - The string slice to check.
///     `const NSTDStr *const pattern` - The pattern to check for.
/// Returns: `NSTDInt32 b` - Nonzero if `str` contains `pattern`.
NSTDAPI NSTDInt32 nstd_core_str_contains(const NSTDStr *const str, const NSTDStr *const pattern);

/// Checks if `str` starts with `pattern`.
/// Parameters:
///     `const NSTDStr *const str` - The string slice to check.
///     `const NSTDStr *const pattern` - The pattern to check for.
/// Returns: `NSTDInt32 b` - Nonzero if `str` starts with `pattern`.
NSTDAPI NSTDInt32 nstd_core_str_starts_with(const NSTDStr *const str, const NSTDStr *const pattern);

/// Checks if `str` ends with `pattern`.
/// Parameters:
///     `const NSTDStr *const str` - The string slice to check.
///     `const NSTDStr *const pattern` - The pattern to check for.
/// Returns: `NSTDInt32 b` - Nonzero if `str` ends with `pattern`.
NSTDAPI NSTDInt32 nstd_core_str_ends_with(const NSTDStr *const str, const NSTDStr *const pattern);

/// Attempts to find `pattern` in `str`. Returns -1 if it is not found.
/// Parameters:
///     `const NSTDStr *const str` - The string slice to check.
///     `const NSTDStr *const pattern` - The pattern to find.
/// Returns: `NSTDUSize pos` - The position of `pattern` in str.
NSTDAPI NSTDUSize nstd_core_str_find(const NSTDStr *const str, const NSTDStr *const pattern);

/// Attempts to find the last `pattern` in `str`. Returns -1 if it is not found.
/// Parameters:
///     `const NSTDStr *const str` - The string slice to check.
///     `const NSTDStr *const pattern` - The pattern to find.
/// Returns: `NSTDUSize pos` - The position of `pattern` in str.
NSTDAPI NSTDUSize nstd_core_str_find_last(const NSTDStr *const str, const NSTDStr *const pattern);

/// Converts a c-string into a NSTDFloat32.
/// Parameters:
///     `const NSTDStr *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `NSTDFloat32 f` - The NSTDFloat32 parsed from the string.
NSTDAPI NSTDFloat32 nstd_core_str_to_f32(const NSTDStr *const str, int *errc);
/// Converts a c-string into a NSTDFloat64.
/// Parameters:
///     `const NSTDStr *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `NSTDFloat64 d` - The NSTDFloat64 parsed from the string.
NSTDAPI NSTDFloat64 nstd_core_str_to_f64(const NSTDStr *const str, int *errc);
/// Converts a c-string into an `NSTDInt8`.
/// Parameters:
///     `const NSTDStr *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `NSTDInt8 sc` - The `NSTDInt8` parsed from the string.
NSTDAPI NSTDInt8 nstd_core_str_to_i8(const NSTDStr *const str, int *errc);
/// Converts a c-string into an `NSTDUInt8`.
/// Parameters:
///     `const NSTDStr *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `NSTDUInt8 uc` - The `NSTDUInt8` parsed from the string.
NSTDAPI NSTDUInt8 nstd_core_str_to_u8(const NSTDStr *const str, int *errc);
/// Converts a c-string into a `NSTDInt16`.
/// Parameters:
///     `const NSTDStr *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `NSTDInt16 s` - The `NSTDInt16` parsed from the string.
NSTDAPI NSTDInt16 nstd_core_str_to_i16(const NSTDStr *const str, int *errc);
/// Converts a c-string into an `NSTDUInt16`.
/// Parameters:
///     `const NSTDStr *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `NSTDUInt16 us` - The `NSTDUInt16` parsed from the string.
NSTDAPI NSTDUInt16 nstd_core_str_to_u16(const NSTDStr *const str, int *errc);
/// Converts a c-string into a `NSTDInt32`.
/// Parameters:
///     `const NSTDStr *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `NSTDInt32 i` - The `NSTDInt32` parsed from the string.
NSTDAPI NSTDInt32 nstd_core_str_to_i32(const NSTDStr *const str, int *errc);
/// Converts a c-string into an `NSTDUInt32`.
/// Parameters:
///     `const NSTDStr *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `NSTDUInt32 ui` - The `NSTDUInt32` parsed from the string.
NSTDAPI NSTDUInt32 nstd_core_str_to_u32(const NSTDStr *const str, int *errc);
/// Converts a c-string into a `NSTDInt64`.
/// Parameters:
///     `const NSTDStr *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `NSTDInt64 l` - The `NSTDInt64` parsed from the string.
NSTDAPI NSTDInt64 nstd_core_str_to_i64(const NSTDStr *const str, int *errc);
/// Converts a c-string into an `NSTDUInt64`.
/// Parameters:
///     `const NSTDStr *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `NSTDUInt64 ul` - The `NSTDUInt64` parsed from the string.
NSTDAPI NSTDUInt64 nstd_core_str_to_u64(const NSTDStr *const str, int *errc);
/// Converts a c-string into a `NSTDISize`.
/// Parameters:
///     `const NSTDStr *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `NSTDISize l` - The `NSTDISize` parsed from the string.
NSTDAPI NSTDISize nstd_core_str_to_isize(const NSTDStr *const str, int *errc);
/// Converts a c-string into an `NSTDUSize`.
/// Parameters:
///     `const NSTDStr *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `NSTDUSize ul` - The `NSTDUSize` parsed from the string.
NSTDAPI NSTDUSize nstd_core_str_to_usize(const NSTDStr *const str, int *errc);

#ifdef __cplusplus
}
#endif
#endif
