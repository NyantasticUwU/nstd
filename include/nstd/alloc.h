#ifndef NSTD_ALLOC_H_INCLUDED
#define NSTD_ALLOC_H_INCLUDED
#include "core/def.h"
#include "core/pointer.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a heap allocated object.
typedef struct
{
    /// Raw pointer to heap allocated object.
    NSTDPointer ptr;
} NSTDHeap;

/// Allocates a new memory block.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDByte *ptr` - The new memory block.
NSTDAPI NSTDByte *nstd_alloc_allocate(const NSTDUSize size);

/// Allocates a new memory block with all bytes set to 0.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDByte *ptr` - The new memory block.
NSTDAPI NSTDByte *nstd_alloc_allocate_zeroed(const NSTDUSize size);

/// Reallocates a memory block.
/// Parameters:
///     `NSTDByte **ptr` - Pointer to the memory block.
///     `const NSTDUSize size` - The current size of the memory block.
///     `const NSTDUSize new_size` - The new size of the memory block.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_alloc_reallocate(NSTDByte **ptr, const NSTDUSize size, const NSTDUSize new_size);

/// Deallocates a memory block.
/// Parameters:
///     `NSTDByte **ptr` - Pointer to the memory block.
///     `const NSTDUSize size` - Number of bytes to deallocate.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_alloc_deallocate(NSTDByte **ptr, const NSTDUSize size);

/// Creates a new heap allocated object.
/// Parameters:
///     `const NSTDPointer *const ptr` - Pointer to an object to be copied to the heap.
/// Returns: `NSTDHeap obj` - The new heap allocated object.
NSTDAPI NSTDHeap nstd_alloc_heap_new(const NSTDPointer *const ptr);

/// Frees a heap allocated object.
/// Parameters:
///     `NSTDHeap *const obj` - The heap allocated object.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_alloc_heap_free(NSTDHeap *const obj);

#ifdef __cplusplus
}
#endif
#endif
