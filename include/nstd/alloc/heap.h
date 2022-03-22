#ifndef NSTD_ALLOC_HEAP_H_INCLUDED
#define NSTD_ALLOC_HEAP_H_INCLUDED
#include "../core/def.h"
#include "../core/pointer.h"
#include "../nstd.h"
NSTDCPPSTART

/// Represents a heap allocated object.
typedef struct
{
    /// Raw pointer to heap allocated object.
    NSTDPointer ptr;
} NSTDHeap;

/// Creates a new heap allocated object.
///
/// # Parameters
///
/// - `const NSTDPointer *const ptr` - Pointer to an object to be copied to the heap.
///
/// # Returns
///
/// `NSTDHeap obj` - The new heap allocated object.
NSTDAPI NSTDHeap nstd_alloc_heap_new(const NSTDPointer *const ptr);

/// Creates a new heap allocated object from a raw pointer.
///
/// # Parameters
///
/// - `const NSTDAny ptr` - A raw pointer to the object to copy to the heap.
///
/// - `const NSTDUSize size` - The size of the object.
///
/// # Returns
///
/// `NSTDHeap obj` - The new heap allocated object.
NSTDAPI NSTDHeap nstd_alloc_heap_from_raw(const NSTDAny ptr, const NSTDUSize size);

/// Creates a new heap object from a raw pointer without making any allocations.
///
/// # Parameters
///
/// - `const NSTDAny ptr` - A raw pointer to the heap object.
///
/// - `const NSTDUSize size` - The size of the heap object.
///
/// # Returns
///
/// `NSTDHeap obj` - The new heap object.
NSTDAPI NSTDHeap nstd_alloc_heap_from_existing(const NSTDAny ptr, const NSTDUSize size);

/// Frees a heap allocated object.
///
/// # Parameters
///
/// - `NSTDHeap *const obj` - The heap allocated object.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_alloc_heap_free(NSTDHeap *const obj);

NSTDCPPEND
#endif
