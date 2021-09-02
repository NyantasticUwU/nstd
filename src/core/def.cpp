#include <nstd/core/def.h>

extern "C"
{
    /// Returns a null pointer.
    /// Returns: `void *null` - A null pointer.
    NSTDAPI void *nstd_core_def_null()
    {
        return NSTDC_NULL;
    }
}
