#include <nstd/core/arch.h>

extern "C"
{
    /// Returns the size (in bytes) of a pointer.
    /// Returns: `NSTDCORESize size` - Size of a pointer.
    NSTDAPI NSTDCORESize nstd_core_arch_ptr_size()
    {
        return sizeof(void *);
    }
}
