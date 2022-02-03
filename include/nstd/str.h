#ifndef NSTD_STR_H_INCLUDED
#define NSTD_STR_H_INCLUDED
#include "core/def.h"
#include "core/slice.h"
#include "core/str.h"
#include "collections/vec.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a dynamic-sized array of UTF-8 chars.
typedef struct
{
    NSTDVec bytes;
} NSTDString;



/// Creates a new `NSTDString` instance.
/// Returns: `NSTDString string` - The new string.
NSTDAPI NSTDString nstd_str_string_new();

/// Creates a new `NSTDString` from a raw C string.
/// Parameters:
///     `const char *const cstr` - The C string.
/// Returns: `NSTDString string` - The new NSTD string.
NSTDAPI NSTDString nstd_str_string_from_cstring(const char *const cstr);

/// Creates a string view from an `NSTDString`.
/// Parameters:
///     `const NSTDString *const string` - The string.
/// Returns: `NSTDStr str` - The new string view.
NSTDAPI NSTDStr nstd_str_string_as_str(const NSTDString *const string);

/// Creates an `NSTDSlice` from an `NSTDString`.
/// Parameters:
///     `const NSTDString *const string` - The string.
/// Returns: `NSTDSlice slice` - The new slice.
NSTDAPI NSTDSlice nstd_str_string_as_slice(const NSTDString *const string);

/// Gets the length of a string.
/// Parameters:
///     `const NSTDString *const string` - The string.
/// Returns: `NSTDUSize len` - The length of the UTF-8 encoded string, -1 on error.
NSTDAPI NSTDUSize nstd_str_string_len(const NSTDString *const string);

/// Returns the number of bytes used by this string.
/// Parameters:
///     `const NSTDString *const string` - The string.
/// Returns: `NSTDUSize len` - The number of bytes in the string.
NSTDAPI NSTDUSize nstd_str_string_byte_len(const NSTDString *const string);

/// Pushes an `NSTDUnichar` to an `NSTDString`.
/// Parameters:
///     `NSTDString *const string` - The string.
///     `const NSTDUnichar chr` - The unicode character to push to the string.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_str_string_push(NSTDString *const string, const NSTDUnichar chr);

/// Removes an `NSTDUnichar` from the end of an `NSTDString`.
/// Parameters:
///     `NSTDString *const string` - The string.
/// Returns: `NSTDUnichar chr` - The unichar that was popped off the string, fill char on error.
NSTDAPI NSTDUnichar nstd_str_string_pop(NSTDString *const string);

/// Extends an `NSTDString` by an `NSTDSlice` of `NSTDUnichar`s.
/// Parameters:
///     `NSTDString *const string` - The string.
///     `const NSTDSlice *const chars` - `NSTDSlice` of `NSTDUnichar`s.
NSTDAPI void nstd_str_string_extend(NSTDString *const string, const NSTDSlice *const chars);

/// Converts a float to a c-string.
/// To keep this from leaking memory please call `nstd_str_string_free`.
/// Parameters:
///     `const float num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_str_string_from_float(const float num);
/// Converts a double to a c-string.
/// To keep this from leaking memory please call `nstd_str_string_free`.
/// Parameters:
///     `const double num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_str_string_from_double(const double num);

/// Converts a signed char to a c-string.
/// To keep this from leaking memory please call `nstd_str_string_free`.
/// Parameters:
///     `const signed char num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_str_string_from_schar(const signed char num);
/// Converts an unsigned char to a c-string.
/// To keep this from leaking memory please call `nstd_str_string_free`.
/// Parameters:
///     `const unsigned char num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_str_string_from_uchar(const unsigned char num);

/// Converts a short to a c-string.
/// To keep this from leaking memory please call `nstd_str_string_free`.
/// Parameters:
///     `const short num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_str_string_from_short(const short num);
/// Converts an unsigned short to a c-string.
/// To keep this from leaking memory please call `nstd_str_string_free`.
/// Parameters:
///     `const unsigned short num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_str_string_from_ushort(const unsigned short num);

/// Converts a int to a c-string.
/// To keep this from leaking memory please call `nstd_str_string_free`.
/// Parameters:
///     `const int num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_str_string_from_int(const int num);
/// Converts an unsigned int to a c-string.
/// To keep this from leaking memory please call `nstd_str_string_free`.
/// Parameters:
///     `const unsigned int num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_str_string_from_uint(const unsigned int num);

/// Converts a long to a c-string.
/// To keep this from leaking memory please call `nstd_str_string_free`.
/// Parameters:
///     `const long num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_str_string_from_long(const long num);
/// Converts an unsigned long to a c-string.
/// To keep this from leaking memory please call `nstd_str_string_free`.
/// Parameters:
///     `const unsigned long num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_str_string_from_ulong(const unsigned long num);

/// Converts a long long to a c-string.
/// To keep this from leaking memory please call `nstd_str_string_free`.
/// Parameters:
///     `const long long num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_str_string_from_longlong(const long long num);
/// Converts an unsigned long long to a c-string.
/// To keep this from leaking memory please call `nstd_str_string_free`.
/// Parameters:
///     `const unsigned long long num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_str_string_from_ulonglong(const unsigned long long num);

/// Converts an `NSTDISize` to a c-string.
/// To keep this from leaking memory please call `nstd_str_string_free`.
/// Parameters:
///     `const NSTDISize num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_str_string_from_isize(const NSTDISize num);
/// Converts an `NSTDUSize` to a c-string.
/// To keep this from leaking memory please call `nstd_str_string_free`.
/// Parameters:
///     `const NSTDUSize num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_str_string_from_usize(const NSTDUSize num);

/// Frees an `NSTDString` instance.
/// Parameters:
///     `NSTDString *const string` - Pointer to a string.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_str_string_free(NSTDString *const string);

#ifdef __cplusplus
}
#endif
#endif
