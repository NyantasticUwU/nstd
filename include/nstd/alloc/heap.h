#ifndef NSTD_ALLOC_HEAP_H_INCLUDED
#define NSTD_ALLOC_HEAP_H_INCLUDED
#include "../core/def.h"
#include "../core/pointer.h"
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
