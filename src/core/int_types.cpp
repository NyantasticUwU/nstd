#include <nstd/core/int_types.h>

extern "C"
{
    /// Returns the min value of a signed char.
    /// Returns: `signed char min` - Minimum value of a signed char.
    NSTDAPI signed char nstd_core_int_types_char_min()
    {
        constexpr const unsigned char X{static_cast<unsigned char>(-1)};
        return static_cast<signed char>(X / 2 + X % 2);
    }
    /// Returns the max value of a signed char.
    /// Returns: `signed char max` - Maximum value of a signed char.
    NSTDAPI signed char nstd_core_int_types_char_max()
    {
        return -nstd_core_int_types_char_min() - 1;
    }
    /// Returns the max value of an unsigned char.
    /// Returns: `unsigned char max` - Maximum value of an unsigned char.
    NSTDAPI unsigned char nstd_core_int_types_uchar_max()
    {
        return static_cast<unsigned char>(-1);
    }

    /// Returns the min value of a signed short.
    /// Returns: `short min` - Minimum value of a signed short.
    NSTDAPI short nstd_core_int_types_short_min()
    {
        constexpr const unsigned short X{static_cast<unsigned short>(-1)};
        return static_cast<short>(X / 2 + X % 2);
    }
    /// Returns the max value of a signed short.
    /// Returns: `short max` - Maximum value of a signed short.
    NSTDAPI short nstd_core_int_types_short_max()
    {
        return -nstd_core_int_types_short_min() - 1;
    }
    /// Returns the max value of an unsigned short.
    /// Returns: `unsigned short max` - Maximum value of an unsigned short.
    NSTDAPI unsigned short nstd_core_int_types_ushort_max()
    {
        return static_cast<unsigned short>(-1);
    }

    /// Returns the min value of a signed int.
    /// Returns: `int min` - Minimum value of a signed int.
    NSTDAPI int nstd_core_int_types_int_min()
    {
        constexpr const unsigned int X{static_cast<unsigned int>(-1)};
        return static_cast<int>(X / 2 + X % 2);
    }
    /// Returns the max value of a signed int.
    /// Returns: `int max` - Maximum value of a signed int.
    NSTDAPI int nstd_core_int_types_int_max()
    {
        return -nstd_core_int_types_int_min() - 1;
    }
    /// Returns the max value of an unsigned int.
    /// Returns: `unsigned int max` - Maximum value of an unsigned int.
    NSTDAPI unsigned int nstd_core_int_types_uint_max()
    {
        return static_cast<unsigned int>(-1);
    }

    /// Returns the min value of a signed long.
    /// Returns: `long min` - Minimum value of a signed long.
    NSTDAPI long nstd_core_int_types_long_min()
    {
        constexpr const unsigned long X{static_cast<unsigned long>(-1)};
        return static_cast<long>(X / 2 + X % 2);
    }
    /// Returns the max value of a signed long.
    /// Returns: `long max` - Maximum value of a signed long.
    NSTDAPI long nstd_core_int_types_long_max()
    {
        return -nstd_core_int_types_long_min() - 1;
    }
    /// Returns the max value of an unsigned long.
    /// Returns: `unsigned long max` - Maximum value of an unsigned long.
    NSTDAPI unsigned long nstd_core_int_types_ulong_max()
    {
        return static_cast<unsigned long>(-1);
    }

    /// Returns the min value of a signed long long.
    /// Returns: `long long min` - Minimum value of a signed long long.
    NSTDAPI long long nstd_core_int_types_longlong_min()
    {
        constexpr const unsigned long long X{static_cast<unsigned long long>(-1)};
        return static_cast<long long>(X / 2 + X % 2);
    }
    /// Returns the max value of a signed long long.
    /// Returns: `long long max` - Maximum value of a signed long long.
    NSTDAPI long long nstd_core_int_types_longlong_max()
    {
        return -nstd_core_int_types_longlong_min() - 1;
    }
    /// Returns the max value of an unsigned long long.
    /// Returns: `unsigned long long max` - Maximum value of an unsigned long long.
    NSTDAPI unsigned long long nstd_core_int_types_ulonglong_max()
    {
        return static_cast<unsigned long long>(-1);
    }
}
