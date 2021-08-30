#pragma warning(disable: 4297)
#include <nstd/core/def.h>
#include <nstd/core/sys.h>

extern "C"
{
    /// Causes abnormal program termination with a message.
    void nstd_core_sys_abort()
    {
        throw;
    }

    /// Asserts that `assertion` is true (not 0) otherwise throws.
    /// Parameters:
    ///     `const NSTDCISize assertion` - The value to assert.
    void nstd_core_sys_assert(const NSTDCISize assertion)
    {
        if (!assertion)
            nstd_core_sys_abort();
    }
}
