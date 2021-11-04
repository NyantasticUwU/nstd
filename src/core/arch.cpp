#include <nstd/core/arch.h>

extern "C"
{
    /// Returns the size (in bytes) of a pointer.
    /// Returns: `NSTDCOREUSize size` - Size of a pointer.
    NSTDAPI inline NSTDCOREUSize nstd_core_arch_ptr_size()
    {
        return sizeof(void *);
    }
}
