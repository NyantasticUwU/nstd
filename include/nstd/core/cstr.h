#ifndef NSTD_CORE_CSTR_H_INCLUDED
#define NSTD_CORE_CSTR_H_INCLUDED
#include "../nstd.h"
#include "def.h"
#include "slice.h"
NSTDCPPSTART

/// Creates a slice over a C string, discarding the null byte.
///
/// # Parameters
///
/// - `const NSTDChar *const cstr` - The C string.
///
/// # Returns
///
/// `NSTDSlice slice` - A slice representing the C string's data.
NSTDAPI NSTDSlice nstd_core_cstr_as_slice(const NSTDChar *const cstr);

/// Creates a slice over a C string, including the null byte.
///
/// # Parameters
///
/// - `const NSTDChar *const cstr` - The C string.
///
/// # Returns
///
/// `NSTDSlice slice` - A slice representing the C string's data.
NSTDAPI NSTDSlice nstd_core_cstr_as_slice_with_null(const NSTDChar *const cstr);

/// Returns the length (in bytes) of a null terminated C string.
///
/// # Parameters
///
/// - `const NSTDChar *const cstr` - The C string.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the C string.
NSTDAPI NSTDUSize nstd_core_cstr_len(const NSTDChar *const cstr);

/// Returns the length (in bytes) of a null terminated C string including the null terminator.
///
/// # Parameters
///
/// - `const NSTDChar *const cstr` - The C string.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the C string with the null terminator.
NSTDAPI NSTDUSize nstd_core_cstr_len_with_null(const NSTDChar *const cstr);

/// Compares two C strings and returns `NSTD_BOOL_TRUE` if they contain the same data.
///
/// # Parameters
///
/// - `const NSTDChar *cstr1` - The first C string.
///
/// - `const NSTDChar *cstr2` - The second C string.
///
/// # Returns
///
/// `NSTDBool is_eq` - `NSTD_BOOL_TRUE` if the two strings are lexicographically equal.
NSTDAPI NSTDBool nstd_core_cstr_compare(const NSTDChar *cstr1, const NSTDChar *cstr2);

NSTDCPPEND
#endif
