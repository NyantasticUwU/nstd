use crate::core::def::{NSTDAny, NSTDErrorCode};

/// A heap memory allocator type.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NSTDAllocator {
    /// Allocates a new block of memory.
    /// Parameters:
    ///     `NSTDUSize size` - Number of bytes to allocate.
    /// Returns: `NSTDAny ptr` - The new block of memory.
    pub allocate: Option<unsafe extern "C" fn(usize) -> NSTDAny>,
    /// Allocates a new block of memory with all bytes set to 0.
    /// Parameters:
    ///     `NSTDUSize size` - Number of bytes to allocate.
    /// Returns: `NSTDAny ptr` - The new block of memory.
    pub allocate_zeroed: Option<unsafe extern "C" fn(usize) -> NSTDAny>,
    /// Reallocates a block of memory.
    /// Parameters:
    ///     `NSTDAny *ptr` - Pointer to the block of memory.
    ///     `NSTDUSize size` - The current size of the block of memory.
    ///     `NSTDUSize new_size` - The new size of the block of memory.
    /// Returns: `NSTDErrorCode errc` - Nonzero on error.
    pub reallocate: Option<unsafe extern "C" fn(&mut NSTDAny, usize, usize) -> NSTDErrorCode>,
    /// Deallocates a block of memory.
    /// Parameters:
    ///     `NSTDAny *ptr` - Pointer to the block of memory.
    ///     `NSTDUSize size` - Number of bytes to deallocate.
    /// Returns: `NSTDErrorCode errc` - Nonzero on error.
    pub deallocate: Option<unsafe extern "C" fn(&mut NSTDAny, usize) -> NSTDErrorCode>,
}

/// Returns the default memory allocator.
/// Returns: `NSTDAllocator allocator` - The default memory allocator.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_allocator_default() -> NSTDAllocator {
    NSTDAllocator {
        allocate: Some(crate::alloc::nstd_alloc_allocate),
        allocate_zeroed: Some(crate::alloc::nstd_alloc_allocate_zeroed),
        reallocate: Some(crate::alloc::nstd_alloc_reallocate),
        deallocate: Some(crate::alloc::nstd_alloc_deallocate),
    }
}
