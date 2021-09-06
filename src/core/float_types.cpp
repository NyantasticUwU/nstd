#include <nstd/core/float_types.h>

extern "C"
{
    /// Returns the minimum value of a float.
    /// Returns: `float min`: Minimum value of a float.
    NSTDAPI float nstd_core_float_types_float_min()
    {
        return -3.40282347e+38f;
    }
    /// Returns the maximum value of a float.
    /// Returns: `float min`: Maximum value of a float.
    NSTDAPI float nstd_core_float_types_float_max()
    {
        return 3.40282347e+38f;
    }

    /// Returns the minimum value of a double.
    /// Returns: `double min`: Minimum value of a double.
    NSTDAPI double nstd_core_float_types_double_min()
    {
        return -1.7976931348623157e+308;
    }
    /// Returns the maximum value of a double.
    /// Returns: `double min`: Maximum value of a double.
    NSTDAPI double nstd_core_float_types_double_max()
    {
        return 1.7976931348623157e+308;
    }

    /// Returns NaN as a float.
    /// Returns: `float nan`: NaN represented as a float.
    NSTDAPI float nstd_core_float_types_float_nan()
    {
        return 0.0f / 0.0f;
    }
    /// Returns infinity as a float.
    /// Returns: `float infinity`: Infinity represented as a float.
    NSTDAPI float nstd_core_float_types_float_infinity()
    {
        return 1.0f / 0.0f;
    }
    /// Returns negative infinity as a float.
    /// Returns: `float negative infinity`: Negative infinity represented as a float.
    NSTDAPI float nstd_core_float_types_float_negative_infinity()
    {
        return -1.0f / 0.0f;
    }

    /// Returns NaN as a double.
    /// Returns: `double nan`: NaN represented as a double.
    NSTDAPI double nstd_core_float_types_double_nan()
    {
        return 0.0 / 0.0;
    }
    /// Returns infinity as a double.
    /// Returns: `double infinity`: Infinity represented as a double.
    NSTDAPI double nstd_core_float_types_double_infinity()
    {
        return 1.0 / 0.0;
    }
    /// Returns negative infinity as a double.
    /// Returns: `double negative infinity`: Negative infinity represented as a double.
    NSTDAPI double nstd_core_float_types_double_negative_infinity()
    {
        return -1.0 / 0.0;
    }
}
