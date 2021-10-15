#include <nstd/core/def.h>

extern "C"
{
    /// Returns a null pointer.
    /// Returns: `void *null` - A null pointer.
    NSTDAPI inline void *nstd_core_def_null()
    {
        return NSTD_CORE_DEF_NULL;
    }
}
