#ifndef NSTD_STD_ALLOC_H_INCLUDED
#define NSTD_STD_ALLOC_H_INCLUDED
#include "../core/def.h"
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Allocates a new memory block.
/// Parameters:
///     `const NSTDSize size` - Number of bytes to allocate.
/// Returns: `NSTDByte *ptr` - The new memory block.
NSTDAPI NSTDByte *nstd_std_alloc_allocate(const NSTDSize size);

/// Allocates a new memory block with all bytes set to 0.
/// Parameters:
///     `const NSTDSize size` - Number of bytes to allocate.
/// Returns: `NSTDByte *ptr` - The new memory block.
NSTDAPI NSTDByte *nstd_std_alloc_allocate_zeroed(const NSTDSize size);

/// Reallocates a memory block.
/// Parameters:
///     `NSTDByte **ptr` - Pointer to the memory block.
///     `const NSTDSize size` - The current size of the memory block.
///     `const NSTDSize new_size` - The new size of the memory block.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_alloc_reallocate(
    NSTDByte **ptr,
    const NSTDSize size,
    const NSTDSize new_size);

/// Deallocates a memory block.
/// Parameters:
///     `NSTDByte **ptr` - Pointer to the memory block.
///     `const NSTDSize size` - Number of bytes to deallocate.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_alloc_deallocate(NSTDByte **ptr, const NSTDSize size);

#ifdef __cplusplus
}
#endif
#endif
