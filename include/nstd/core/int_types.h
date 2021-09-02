#ifndef NSTD_CORE_INT_TYPES_H_INCLUDED
#define NSTD_CORE_INT_TYPES_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Returns the min value of a signed char.
/// Returns: `signed char min` - Minimum value of a signed char.
NSTDAPI signed char nstd_core_int_types_char_min();
/// Returns the max value of a signed char.
/// Returns: `signed char max` - Maximum value of a signed char.
NSTDAPI signed char nstd_core_int_types_char_max();
/// Returns the max value of an unsigned char.
/// Returns: `unsigned char max` - Maximum value of an unsigned char.
NSTDAPI unsigned char nstd_core_int_types_uchar_max();

/// Returns the min value of a signed short.
/// Returns: `short min` - Minimum value of a signed short.
NSTDAPI short nstd_core_int_types_short_min();
/// Returns the max value of a signed short.
/// Returns: `short max` - Maximum value of a signed short.
NSTDAPI short nstd_core_int_types_short_max();
/// Returns the max value of an unsigned short.
/// Returns: `unsigned short max` - Maximum value of an unsigned short.
NSTDAPI unsigned short nstd_core_int_types_ushort_max();

/// Returns the min value of a signed int.
/// Returns: `int min` - Minimum value of a signed int.
NSTDAPI int nstd_core_int_types_int_min();
/// Returns the max value of a signed int.
/// Returns: `int max` - Maximum value of a signed int.
NSTDAPI int nstd_core_int_types_int_max();
/// Returns the max value of an unsigned int.
/// Returns: `unsigned int max` - Maximum value of an unsigned int.
NSTDAPI unsigned int nstd_core_int_types_uint_max();

/// Returns the min value of a signed long.
/// Returns: `long min` - Minimum value of a signed long.
NSTDAPI long nstd_core_int_types_long_min();
/// Returns the max value of a signed long.
/// Returns: `long max` - Maximum value of a signed long.
NSTDAPI long nstd_core_int_types_long_max();
/// Returns the max value of an unsigned long.
/// Returns: `unsigned long max` - Maximum value of an unsigned long.
NSTDAPI unsigned long nstd_core_int_types_ulong_max();

/// Returns the min value of a signed long long.
/// Returns: `long long min` - Minimum value of a signed long long.
NSTDAPI long long nstd_core_int_types_longlong_min();
/// Returns the max value of a signed long long.
/// Returns: `long long max` - Maximum value of a signed long long.
NSTDAPI long long nstd_core_int_types_longlong_max();
/// Returns the max value of an unsigned long long.
/// Returns: `unsigned long long max` - Maximum value of an unsigned long long.
NSTDAPI unsigned long long nstd_core_int_types_ulonglong_max();

#ifdef __cplusplus
}
#endif
#endif
