#ifndef NSTD_COLLECTIONS_RC_H_INCLUDED
#define NSTD_COLLECTIONS_RC_H_INCLUDED
#include "../alloc/heap.h"
#include "../core/pointer.h"
#include "../nstd.h"
#ifdef NSTDCPP
extern "C"
{
#endif

/// The state of a reference counter.
typedef struct
{
    /// The number of reference counters that own `data`.
    NSTDUSize count;
    /// The shared data.
    NSTDHeap data;
} NSTDRCState;

/// A reference counter. A heap allocated type that can be shared across multiple objects, once the
/// last object to own the data is freed the data is freed as well.
typedef struct
{
    /// The reference counter state. Only free once `state.count` == 0.
    NSTDHeap state;
} NSTDRC;

/// Creates a new reference counter.
/// Parameters:
///     `const NSTDPointer *const ptr` - A pointer to the object to be placed on the heap.
/// Returns: `NSTDRC rc` - The new reference counter.
NSTDAPI NSTDRC nstd_collections_rc_new(const NSTDPointer *const ptr);

/// Shares the reference counter, creating a new instance of `NSTDRC` and increasing the reference
/// count.
/// Parameters:
///     `const NSTDRC *const rc` - A pointer to the reference counter.
/// Returns: `NSTDRC new_rc` - The new reference counter that points to the new data.
NSTDAPI NSTDRC nstd_collections_rc_share(const NSTDRC *const rc);

/// Returns a pointer to the underlying data.
/// Parameters:
///     `const NSTDRC *const rc` - A pointer to the reference counter.
/// Returns: `NSTDAny ptr` - A pointer to the underlying data.
NSTDAPI NSTDAny nstd_collections_rc_get(const NSTDRC *const rc);

/// Frees a reference counter, only frees the underlying data once all other reference counters have
/// been freed as well.
/// Parameters:
///     `NSTDRC *const rc` - The reference counter to free.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_collections_rc_free(NSTDRC *const rc);

#ifdef NSTDCPP
}
#endif
#endif
