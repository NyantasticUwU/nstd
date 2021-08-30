#include <nstd/def.h>
#include <nstd/sys.h>

extern "C"
{
    /// Causes abnormal program termination with a message.
    void nstd_sys_abort()
    {
        throw;
    }

    /// Asserts that `assertion` is true (not 0) otherwise throws.
    /// Parameters:
    ///     `const NSTDISize assertion` - The value to assert.
    void nstd_sys_assert(const NSTDISize assertion)
    {
        if (!assertion)
            nstd_sys_abort();
    }
}
