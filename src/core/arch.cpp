#include <nstd/core/arch.h>

extern "C"
{
    /// Returns the size (in bytes) of a pointer.
    /// Returns: `NSTDCSize size` - Size of a pointer.
    NSTDCSize nstd_core_arch_ptr_size()
    {
        return sizeof(void *);
    }
}
