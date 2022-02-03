#ifndef NSTD_CORE_STR_CSTR_H_INCLUDED
#define NSTD_CORE_STR_CSTR_H_INCLUDED
#include "../def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Returns the length (in bytes) of a null terminated C string.
/// Parameters:
///     `const NSTDChar *const cstr` - The C string.
/// Returns: `NSTDUSize len` - The length of the C string.
NSTDAPI NSTDUSize nstd_core_str_cstr_len(const NSTDChar *const cstr);

/// Compares two C strings and returns `NSTD_BOOL_TRUE` if they contain the same data.
/// Parameters:
///     `const NSTDChar *const cstr1` - The first C string.
///     `const NSTDChar *const cstr2` - The second C string.
/// Returns: `NSTDBool is_eq` - `NSTD_BOOL_TRUE` if the two strings are lexicographically equal.
NSTDAPI NSTDBool nstd_core_str_cstr_compare(
    const NSTDChar *const cstr1,
    const NSTDChar *const cstr2);

#ifdef __cplusplus
}
#endif
#endif
