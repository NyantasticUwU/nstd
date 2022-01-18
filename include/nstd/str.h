#ifndef NSTD_STR_H_INCLUDED
#define NSTD_STR_H_INCLUDED
#include "core/char_types.h"
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

/// Frees an `NSTDString` instance.
/// Parameters:
///     `NSTDString *const string` - Pointer to a string.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_str_string_free(NSTDString *const string);



/// Calculates a string's length.
/// Parameters:
///     `const char *const str` - The string.
/// Returns: `NSTDUSize len` - The length of the string.
NSTDAPI NSTDUSize nstd_str_len(const char *const str);

/// Concatenates two strings.
/// Parameters:
///     `const char *const str1` - The first string.
///     `const char *const str2` - The second string.
/// Returns: `char *str` - The new string, null on error.
NSTDAPI char *nstd_str_concat(const char *const str1, const char *const str2);

/// Frees memory allocated by `nstd_str_concat`.
/// Parameters:
///     `const char **str` - The string.
NSTDAPI void nstd_str_free_concat(const char **str);

/// Compares two strings.
/// Parameters:
///     `const char *const str1` - The first string to compare.
///     `const char *const str2` - The second string to compare.
/// Returns: `int e` - Nonzero if the two strings are lexicographically equal.
NSTDAPI int nstd_str_compare(const char *const str1, const char *const str2);

/// Checks if `str` contains `pattern`.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char *const pattern` - The pattern to check for.
/// Returns: `int b` - Nonzero if `str` contains `pattern`.
NSTDAPI int nstd_str_contains(const char *const str, const char *const pattern);
/// Checks if `str` starts with `pattern`.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char *const pattern` - The pattern to check for.
/// Returns: `int b` - Nonzero if `str` starts with `pattern`.
NSTDAPI int nstd_str_starts_with(const char *const str, const char *const pattern);
/// Checks if `str` ends with `pattern`.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char *const pattern` - The pattern to check for.
/// Returns: `int b` - Nonzero if `str` ends with `pattern`.
NSTDAPI int nstd_str_ends_with(const char *const str, const char *const pattern);
/// Attempts to find `pattern` in `str`. Returns -1 if it is not found.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char *const pattern` - The pattern to find.
/// Returns: `NSTDUSize pos` - The position in `str` of the pattern.
NSTDAPI NSTDUSize nstd_str_find(const char *const str, const char *const pattern);
/// Attempts to find the last `pattern` in `str`. Returns -1 if it is not found.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char *const pattern` - The pattern to find.
/// Returns: `NSTDUSize pos` - The position in `str` of the pattern.
NSTDAPI NSTDUSize nstd_str_find_last(const char *const str, const char *const pattern);

/// Checks if `str` contains `pattern`.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char pattern` - The pattern to check for.
/// Returns: `int b` - Nonzero if `str` contains `pattern`.
NSTDAPI int nstd_str_contains_char(const char *const str, const char pattern);
/// Checks if `str` starts with `pattern`.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char pattern` - The pattern to check for.
/// Returns: `int b` - Nonzero if `str` starts with `pattern`.
NSTDAPI int nstd_str_starts_with_char(const char *const str, const char pattern);
/// Checks if `str` ends with `pattern`.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char pattern` - The pattern to check for.
/// Returns: `int b` - Nonzero if `str` ends with `pattern`.
NSTDAPI int nstd_str_ends_with_char(const char *const str, const char pattern);
/// Attempts to find `pattern` in `str`. Returns -1 if it is not found.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char pattern` - The pattern to find.
/// Returns: `NSTDUSize pos` - The position in `str` of the pattern.
NSTDAPI NSTDUSize nstd_str_find_char(const char *const str, const char pattern);
/// Attempts to find the last `pattern` in `str`. Returns -1 if it is not found.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char pattern` - The pattern to find.
/// Returns: `NSTDUSize pos` - The position in `str` of the pattern.
NSTDAPI NSTDUSize nstd_str_find_last_char(const char *const str, const char pattern);



/// Converts a c-string into a float.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `float f` - The float parsed from the string.
NSTDAPI float nstd_str_to_float(const char *const str, int *errc);
/// Converts a c-string into a double.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `double d` - The double parsed from the string.
NSTDAPI double nstd_str_to_double(const char *const str, int *errc);

/// Converts a c-string into a signed char.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `signed char sc` - The signed char parsed from the string.
NSTDAPI signed char nstd_str_to_schar(const char *const str, int *errc);
/// Converts a c-string into an unsigned char.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `unsigned char uc` - The unsigned char parsed from the string.
NSTDAPI unsigned char nstd_str_to_uchar(const char *const str, int *errc);

/// Converts a c-string into a short.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `short s` - The short parsed from the string.
NSTDAPI short nstd_str_to_short(const char *const str, int *errc);
/// Converts a c-string into an unsigned short.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `unsigned short us` - The unsigned short parsed from the string.
NSTDAPI unsigned short nstd_str_to_ushort(const char *const str, int *errc);

/// Converts a c-string into a int.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `int i` - The int parsed from the string.
NSTDAPI int nstd_str_to_int(const char *const str, int *errc);
/// Converts a c-string into an unsigned int.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `unsigned int ui` - The unsigned int parsed from the string.
NSTDAPI unsigned int nstd_str_to_uint(const char *const str, int *errc);

/// Converts a c-string into a long.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `long l` - The long parsed from the string.
NSTDAPI long nstd_str_to_long(const char *const str, int *errc);
/// Converts a c-string into an unsigned long.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `unsigned long ul` - The unsigned long parsed from the string.
NSTDAPI unsigned long nstd_str_to_ulong(const char *const str, int *errc);

/// Converts a c-string into a long long.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `long long ll` - The long long parsed from the string.
NSTDAPI long long nstd_str_to_longlong(const char *const str, int *errc);
/// Converts a c-string into an unsigned long long.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `unsigned long long ull` - The unsigned long long parsed from the string.
NSTDAPI unsigned long long nstd_str_to_ulonglong(const char *const str, int *errc);



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

#ifdef __cplusplus
}
#endif
#endif