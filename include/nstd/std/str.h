#ifndef NSTD_STD_STR_H_INCLUDED
#define NSTD_STD_STR_H_INCLUDED
#include "../core/def.h"
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Calculates a string's length.
/// Parameters:
///     `const char *const str` - The string.
/// Returns: `NSTDCSize len` - The length of the string.
NSTDAPI NSTDSize nstd_std_str_len(const char *const str);

/// Compares two strings.
/// Parameters:
///     `const char *const str1` - The first string to compare.
///     `const char *const str2` - The second string to compare.
/// Returns: `int e` - Nonzero if the two strings are lexicographically equal.
NSTDAPI int nstd_std_str_compare(const char *const str1, const char *const str2);

/// Checks if `str` contains `pattern`.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char *const pattern` - The pattern to check for.
/// Returns: `int b` - Nonzero if `str` contains `pattern`.
NSTDAPI int nstd_std_str_contains(const char *const str, const char *const pattern);
/// Checks if `str` starts with `pattern`.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char *const pattern` - The pattern to check for.
/// Returns: `int b` - Nonzero if `str` starts with `pattern`.
NSTDAPI int nstd_std_str_starts_with(const char *const str, const char *const pattern);
/// Checks if `str` ends with `pattern`.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char *const pattern` - The pattern to check for.
/// Returns: `int b` - Nonzero if `str` ends with `pattern`.
NSTDAPI int nstd_std_str_ends_with(const char *const str, const char *const pattern);
/// Attempts to find `pattern` in `str`. Returns -1 if it is not found.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char *const pattern` - The pattern to find.
/// Returns: `NSTDSize pos` - The position in `str` of the pattern.
NSTDAPI NSTDSize nstd_std_str_find(const char *const str, const char *const pattern);
/// Attempts to find the last `pattern` in `str`. Returns -1 if it is not found.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char *const pattern` - The pattern to find.
/// Returns: `NSTDSize pos` - The position in `str` of the pattern.
NSTDAPI NSTDSize nstd_std_str_find_last(const char *const str, const char *const pattern);

/// Checks if `str` contains `pattern`.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char pattern` - The pattern to check for.
/// Returns: `int b` - Nonzero if `str` contains `pattern`.
NSTDAPI int nstd_std_str_contains_char(const char *const str, const char pattern);
/// Checks if `str` starts with `pattern`.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char pattern` - The pattern to check for.
/// Returns: `int b` - Nonzero if `str` starts with `pattern`.
NSTDAPI int nstd_std_str_starts_with_char(const char *const str, const char pattern);
/// Checks if `str` ends with `pattern`.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char pattern` - The pattern to check for.
/// Returns: `int b` - Nonzero if `str` ends with `pattern`.
NSTDAPI int nstd_std_str_ends_with_char(const char *const str, const char pattern);
/// Attempts to find `pattern` in `str`. Returns -1 if it is not found.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char pattern` - The pattern to find.
/// Returns: `NSTDSize pos` - The position in `str` of the pattern.
NSTDAPI NSTDSize nstd_std_str_find_char(const char *const str, const char pattern);
/// Attempts to find the last `pattern` in `str`. Returns -1 if it is not found.
/// Parameters:
///     `const char *const str` - The string to check.
///     `const char pattern` - The pattern to find.
/// Returns: `NSTDSize pos` - The position in `str` of the pattern.
NSTDAPI NSTDSize nstd_std_str_find_last_char(const char *const str, const char pattern);



/// Converts a c-string into a float.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `float f` - The float parsed from the string.
NSTDAPI float nstd_std_str_to_float(const char *const str, int *errc);
/// Converts a c-string into a double.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `double d` - The double parsed from the string.
NSTDAPI double nstd_std_str_to_double(const char *const str, int *errc);

/// Converts a c-string into a signed char.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `signed char sc` - The signed char parsed from the string.
NSTDAPI signed char nstd_std_str_to_schar(const char *const str, int *errc);
/// Converts a c-string into an unsigned char.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `unsigned char uc` - The unsigned char parsed from the string.
NSTDAPI unsigned char nstd_std_str_to_uchar(const char *const str, int *errc);

/// Converts a c-string into a short.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `short s` - The short parsed from the string.
NSTDAPI short nstd_std_str_to_short(const char *const str, int *errc);
/// Converts a c-string into an unsigned short.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `unsigned short us` - The unsigned short parsed from the string.
NSTDAPI unsigned short nstd_std_str_to_ushort(const char *const str, int *errc);

/// Converts a c-string into a int.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `int i` - The int parsed from the string.
NSTDAPI int nstd_std_str_to_int(const char *const str, int *errc);
/// Converts a c-string into an unsigned int.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `unsigned int ui` - The unsigned int parsed from the string.
NSTDAPI unsigned int nstd_std_str_to_uint(const char *const str, int *errc);

/// Converts a c-string into a long.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `long l` - The long parsed from the string.
NSTDAPI long nstd_std_str_to_long(const char *const str, int *errc);
/// Converts a c-string into an unsigned long.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `unsigned long ul` - The unsigned long parsed from the string.
NSTDAPI unsigned long nstd_std_str_to_ulong(const char *const str, int *errc);

/// Converts a c-string into a long long.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `long long ll` - The long long parsed from the string.
NSTDAPI long long nstd_std_str_to_longlong(const char *const str, int *errc);
/// Converts a c-string into an unsigned long long.
/// Parameters:
///     `const char *const str` - The string to be converted.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `unsigned long long ull` - The unsigned long long parsed from the string.
NSTDAPI unsigned long long nstd_std_str_to_ulonglong(const char *const str, int *errc);



/// Converts a float to a c-string.
/// To keep this from leaking memory please call `nstd_std_str_free_from`.
/// Parameters:
///     `const float num` - The number to be converted.
/// Returns: `const char *str` - The number represented as a string.
NSTDAPI const char *nstd_std_str_from_float(const float num);
/// Converts a double to a c-string.
/// To keep this from leaking memory please call `nstd_std_str_free_from`.
/// Parameters:
///     `const double num` - The number to be converted.
/// Returns: `const char *str` - The number represented as a string.
NSTDAPI const char *nstd_std_str_from_double(const double num);

/// Converts a signed char to a c-string.
/// To keep this from leaking memory please call `nstd_std_str_free_from`.
/// Parameters:
///     `const signed char num` - The number to be converted.
/// Returns: `const char *str` - The number represented as a string.
NSTDAPI const char *nstd_std_str_from_schar(const signed char num);
/// Converts an unsigned char to a c-string.
/// To keep this from leaking memory please call `nstd_std_str_free_from`.
/// Parameters:
///     `const unsigned char num` - The number to be converted.
/// Returns: `const char *str` - The number represented as a string.
NSTDAPI const char *nstd_std_str_from_uchar(const unsigned char num);

/// Converts a short to a c-string.
/// To keep this from leaking memory please call `nstd_std_str_free_from`.
/// Parameters:
///     `const short num` - The number to be converted.
/// Returns: `const char *str` - The number represented as a string.
NSTDAPI const char *nstd_std_str_from_short(const short num);
/// Converts an unsigned short to a c-string.
/// To keep this from leaking memory please call `nstd_std_str_free_from`.
/// Parameters:
///     `const unsigned short num` - The number to be converted.
/// Returns: `const char *str` - The number represented as a string.
NSTDAPI const char *nstd_std_str_from_ushort(const unsigned short num);

/// Converts a int to a c-string.
/// To keep this from leaking memory please call `nstd_std_str_free_from`.
/// Parameters:
///     `const int num` - The number to be converted.
/// Returns: `const char *str` - The number represented as a string.
NSTDAPI const char *nstd_std_str_from_int(const int num);
/// Converts an unsigned int to a c-string.
/// To keep this from leaking memory please call `nstd_std_str_free_from`.
/// Parameters:
///     `const unsigned int num` - The number to be converted.
/// Returns: `const char *str` - The number represented as a string.
NSTDAPI const char *nstd_std_str_from_uint(const unsigned int num);

/// Converts a long to a c-string.
/// To keep this from leaking memory please call `nstd_std_str_free_from`.
/// Parameters:
///     `const long num` - The number to be converted.
/// Returns: `const char *str` - The number represented as a string.
NSTDAPI const char *nstd_std_str_from_long(const long num);
/// Converts an unsigned long to a c-string.
/// To keep this from leaking memory please call `nstd_std_str_free_from`.
/// Parameters:
///     `const unsigned long num` - The number to be converted.
/// Returns: `const char *str` - The number represented as a string.
NSTDAPI const char *nstd_std_str_from_ulong(const unsigned long num);

/// Converts a long long to a c-string.
/// To keep this from leaking memory please call `nstd_std_str_free_from`.
/// Parameters:
///     `const long long num` - The number to be converted.
/// Returns: `const char *str` - The number represented as a string.
NSTDAPI const char *nstd_std_str_from_longlong(const long long num);
/// Converts an unsigned long long to a c-string.
/// To keep this from leaking memory please call `nstd_std_str_free_from`.
/// Parameters:
///     `const unsigned long long num` - The number to be converted.
/// Returns: `const char *str` - The number represented as a string.
NSTDAPI const char *nstd_std_str_from_ulonglong(const unsigned long long num);

/// Frees a string allocated by `nstd_std_str_from_*`.
/// Parameters:
///     `const char **str` - Pointer to the character string.
NSTDAPI void nstd_std_str_free_from(const char **str);

#ifdef __cplusplus
}
#endif
#endif
