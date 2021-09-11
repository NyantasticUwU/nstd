#pragma warning(disable: 4297)
#include <nstd/core/def.h>
#include <nstd/core/sys.h>

extern "C"
{
    /// Causes abnormal program termination.
    NSTDAPI void nstd_core_sys_abort()
    {
        throw;
    }

    /// Asserts that `assertion` is true (not 0) otherwise throws.
    /// Parameters:
    ///     `const NSTDCOREISize assertion` - The value to assert.
    NSTDAPI void nstd_core_sys_assert(const NSTDCOREISize assertion)
    {
        if (!assertion)
            nstd_core_sys_abort();
    }
}
