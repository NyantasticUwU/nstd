#include <nstd/sys.h>

extern "C"
{
    /// Causes abnormal program termination with a message.
    void nstd_sys_abort()
    {
        throw;
    }
}
