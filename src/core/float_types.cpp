#include <nstd/core/float_types.h>
static float static_float_zero{0.0f};
static double static_double_zero{0.0};

extern "C"
{
    /// Returns the minimum value of a float.
    /// Returns: `float min`: Minimum value of a float.
    NSTDAPI inline float nstd_core_float_types_float_min()
    {
        return -3.40282347e+38f;
    }
    /// Returns the maximum value of a float.
    /// Returns: `float min`: Maximum value of a float.
    NSTDAPI inline float nstd_core_float_types_float_max()
    {
        return 3.40282347e+38f;
    }

    /// Returns the minimum value of a double.
    /// Returns: `double min`: Minimum value of a double.
    NSTDAPI inline double nstd_core_float_types_double_min()
    {
        return -1.7976931348623157e+308;
    }
    /// Returns the maximum value of a double.
    /// Returns: `double min`: Maximum value of a double.
    NSTDAPI inline double nstd_core_float_types_double_max()
    {
        return 1.7976931348623157e+308;
    }

    /// Returns NaN as a float.
    /// Returns: `float nan`: NaN represented as a float.
    NSTDAPI inline float nstd_core_float_types_float_nan()
    {
        return static_float_zero / static_float_zero;
    }
    /// Returns infinity as a float.
    /// Returns: `float infinity`: Infinity represented as a float.
    NSTDAPI inline float nstd_core_float_types_float_infinity()
    {
        return 1.0f / static_float_zero;
    }
    /// Returns negative infinity as a float.
    /// Returns: `float negative infinity`: Negative infinity represented as a float.
    NSTDAPI inline float nstd_core_float_types_float_negative_infinity()
    {
        return -1.0f / static_float_zero;
    }

    /// Returns NaN as a double.
    /// Returns: `double nan`: NaN represented as a double.
    NSTDAPI inline double nstd_core_float_types_double_nan()
    {
        return static_double_zero / static_double_zero;
    }
    /// Returns infinity as a double.
    /// Returns: `double infinity`: Infinity represented as a double.
    NSTDAPI inline double nstd_core_float_types_double_infinity()
    {
        return 1.0 / static_double_zero;
    }
    /// Returns negative infinity as a double.
    /// Returns: `double negative infinity`: Negative infinity represented as a double.
    NSTDAPI inline double nstd_core_float_types_double_negative_infinity()
    {
        return -1.0 / static_double_zero;
    }
}
