#ifndef NSTD_ALLOC_ALLOC_H_INCLUDED
#define NSTD_ALLOC_ALLOC_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Allocates a new memory block.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDAny ptr` - The new memory block.
NSTDAPI NSTDAny nstd_alloc_allocate(const NSTDUSize size);

/// Allocates a new memory block with all bytes set to 0.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDAny ptr` - The new memory block.
NSTDAPI NSTDAny nstd_alloc_allocate_zeroed(const NSTDUSize size);

/// Reallocates a memory block.
/// Parameters:
///     `NSTDAny *const ptr` - Pointer to the memory block.
///     `const NSTDUSize size` - The current size of the memory block.
///     `const NSTDUSize new_size` - The new size of the memory block.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_alloc_reallocate(
    NSTDAny *const ptr,
    const NSTDUSize size,
    const NSTDUSize new_size);

/// Deallocates a memory block.
/// Parameters:
///     `NSTDAny *const ptr` - Pointer to the memory block.
///     `const NSTDUSize size` - Number of bytes to deallocate.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_alloc_deallocate(NSTDAny *const ptr, const NSTDUSize size);

#ifdef __cplusplus
}
#endif
#endif
