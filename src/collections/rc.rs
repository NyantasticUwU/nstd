//! A reference counter. A heap allocated type that can be shared across multiple objects, once the
//! last object to own the data is freed the data is freed as well.
use crate::{
    alloc::heap::NSTDHeap,
    core::{
        def::{NSTDAny, NSTDErrorCode},
        pointer::NSTDPointer,
    },
};
use std::ptr::addr_of_mut;

/// The state of a reference counter.
#[repr(C)]
#[derive(Debug)]
pub struct NSTDRCState {
    /// The number of reference counters that own `data`.
    pub count: usize,
    /// The shared data.
    pub data: NSTDHeap,
}

/// A reference counter. A heap allocated type that can be shared across multiple objects, once the
/// last object to own the data is freed the data is freed as well.
#[repr(C)]
#[derive(Debug)]
pub struct NSTDRC {
    /// The reference counter state. Only free once `state.count` == 0.
    pub state: NSTDHeap,
}

/// Creates a new reference counter.
///
/// # Parameters
///
/// - `const NSTDPointer *const ptr` - A pointer to the object to be placed on the heap.
///
/// # Returns
///
/// `NSTDRC rc` - The new reference counter.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_rc_new(ptr: &NSTDPointer) -> NSTDRC {
    // Create a new RC state.
    let mut state = NSTDRCState {
        count: 1,
        data: crate::alloc::heap::nstd_alloc_heap_new(ptr),
    };
    // Construct a reference counter with a new state.
    NSTDRC {
        state: crate::alloc::heap::nstd_alloc_heap_from_raw(
            addr_of_mut!(state).cast(),
            std::mem::size_of::<NSTDRCState>(),
        ),
    }
}

/// Shares the reference counter, creating a new instance of `NSTDRC` and increasing the reference
/// count.
///
/// # Parameters
///
/// - `const NSTDRC *const rc` - A pointer to the reference counter.
///
/// # Returns
///
/// `NSTDRC new_rc` - The new reference counter that points to the new data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_rc_share(rc: &NSTDRC) -> NSTDRC {
    // Increase the state's reference count by one.
    let state = rc.state.ptr.raw as *mut NSTDRCState;
    (*state).count += 1;
    // Create a new reference counter that points to the state.
    NSTDRC {
        state: crate::alloc::heap::nstd_alloc_heap_from_existing(state.cast(), rc.state.ptr.size),
    }
}

/// Returns a pointer to the underlying data.
///
/// # Parameters
///
/// - `const NSTDRC *const rc` - A pointer to the reference counter.
///
/// # Returns
///
/// `NSTDAny ptr` - A pointer to the underlying data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_rc_get(rc: &NSTDRC) -> NSTDAny {
    // Return the reference counter state's raw data.
    let state = rc.state.ptr.raw as *mut NSTDRCState;
    (*state).data.ptr.raw
}

/// Frees a reference counter, only frees the underlying data once all other reference counters have
/// been freed as well.
///
/// # Parameters
///
/// - `NSTDRC *const rc` - The reference counter to free.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_rc_free(rc: &mut NSTDRC) -> NSTDErrorCode {
    let mut errc = 0;
    let state = &mut *(rc.state.ptr.raw as *mut NSTDRCState);
    // Decrease the reference count.
    state.count -= 1;
    // If the count is 0, free the referenced memory.
    if state.count == 0 {
        errc |= crate::alloc::heap::nstd_alloc_heap_free(&mut state.data);
        errc |= crate::alloc::heap::nstd_alloc_heap_free(&mut rc.state);
    }
    errc
}
