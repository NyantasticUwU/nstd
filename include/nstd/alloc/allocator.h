#ifndef NSTD_ALLOC_ALLOCATOR_H_INCLUDED
#define NSTD_ALLOC_ALLOCATOR_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
#ifdef NSTDCPP
extern "C"
{
#endif

/// A heap memory allocator type.
typedef struct
{
    /// Allocates a new block of memory.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of the allocator.
    ///     `NSTDUSize size` - Number of bytes to allocate.
    /// Returns: `NSTDAny ptr` - The new block of memory.
    NSTDAny (*allocate)(NSTDAny, NSTDUSize);
    /// Allocates a new block of memory with all bytes set to 0.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of the allocator.
    ///     `NSTDUSize size` - Number of bytes to allocate.
    /// Returns: `NSTDAny ptr` - The new block of memory.
    NSTDAny (*allocate_zeroed)(NSTDAny, NSTDUSize);
    /// Reallocates a block of memory.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of the allocator.
    ///     `NSTDAny *ptr` - Pointer to the block of memory.
    ///     `NSTDUSize size` - The current size of the block of memory.
    ///     `NSTDUSize new_size` - The new size of the block of memory.
    /// Returns: `NSTDErrorCode errc` - Nonzero on error.
    NSTDErrorCode (*reallocate)(NSTDAny, NSTDAny *, NSTDUSize, NSTDUSize);
    /// Deallocates a block of memory.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of the allocator.
    ///     `NSTDAny *ptr` - Pointer to the block of memory.
    ///     `NSTDUSize size` - Number of bytes to deallocate.
    /// Returns: `NSTDErrorCode errc` - Nonzero on error.
    NSTDErrorCode (*deallocate)(NSTDAny, NSTDAny *, NSTDUSize);
} NSTDAllocator;

/// Returns the default memory allocator.
/// Returns: `NSTDAllocator allocator` - The default memory allocator.
NSTDAPI NSTDAllocator nstd_alloc_allocator_default();

#ifdef NSTDCPP
}
#endif
#endif
