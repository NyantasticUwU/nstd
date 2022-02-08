#ifndef NSTD_OS_LINUX_ALLOC_H_INCLUDED
#define NSTD_OS_LINUX_ALLOC_H_INCLUDED
#include "../../core/def.h"
#include "../../nstd.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Allocates a block of memory on the heap.
/// Parameters:
///     `const NSTDUSize size` - The number of bytes to allocate.
/// Returns: `NSTDAny ptr` - A pointer to the heap block of memory.
NSTDAPI NSTDAny nstd_os_linux_alloc_allocate(const NSTDUSize size);

/// Allocates a zero-initialized block of memory on the heap.
/// Parameters:
///     `const NSTDUSize num` - The number of objects to allocate.
///     `const NSTDUSize size` - The size of each object to allocate.
/// Returns: `NSTDAny ptr` - A pointer to the heap block of memory.
NSTDAPI NSTDAny nstd_os_linux_alloc_allocate_zeroed(const NSTDUSize num, const NSTDUSize size);

/// Reallocates a memory block with a new size.
/// Parameters:
///     `NSTDAny *const ptr` - Pointer to the pointer to the memory block to reallocate.
///     `const NSTDUSize new_size` - The number of bytes the new memory block will have.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_os_linux_alloc_reallocate(NSTDAny *const ptr, const NSTDUSize new_size);

/// Deallocates a block of memory.
/// Parameters:
///     `NSTDAny *const ptr` - Pointer to the block of memory.
NSTDAPI void nstd_os_linux_alloc_deallocate(NSTDAny *const ptr);

#ifdef __cplusplus
}
#endif
#endif
