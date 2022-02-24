use crate::core::def::{NSTDAny, NSTDErrorCode};

/// A heap memory allocator type.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NSTDAllocator {
    /// Set to nonzero if an error occurs.
    pub errc: NSTDErrorCode,
    /// Allocates a new block of memory.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of the allocator.
    ///     `NSTDUSize size` - Number of bytes to allocate.
    /// Returns: `NSTDAny ptr` - The new block of memory.
    pub allocate: Option<unsafe extern "C" fn(NSTDAny, usize) -> NSTDAny>,
    /// Allocates a new block of memory with all bytes set to 0.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of the allocator.
    ///     `NSTDUSize size` - Number of bytes to allocate.
    /// Returns: `NSTDAny ptr` - The new block of memory.
    pub allocate_zeroed: Option<unsafe extern "C" fn(NSTDAny, usize) -> NSTDAny>,
    /// Reallocates a block of memory.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of the allocator.
    ///     `NSTDAny *ptr` - Pointer to the block of memory.
    ///     `NSTDUSize size` - The current size of the block of memory.
    ///     `NSTDUSize new_size` - The new size of the block of memory.
    pub reallocate: Option<unsafe extern "C" fn(NSTDAny, &mut NSTDAny, usize, usize)>,
    /// Deallocates a block of memory.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of the allocator.
    ///     `NSTDAny *ptr` - Pointer to the block of memory.
    ///     `NSTDUSize size` - Number of bytes to deallocate.
    pub deallocate: Option<unsafe extern "C" fn(NSTDAny, &mut NSTDAny, usize)>,
}

/// Returns the default memory allocator.
/// Returns: `NSTDAllocator allocator` - The default memory allocator.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_allocator_default() -> NSTDAllocator {
    NSTDAllocator {
        errc: 0,
        allocate: Some(allocate),
        allocate_zeroed: Some(allocate_zeroed),
        reallocate: Some(reallocate),
        deallocate: Some(deallocate),
    }
}

/// Default allocate function.
#[inline]
unsafe extern "C" fn allocate(_: NSTDAny, size: usize) -> NSTDAny {
    crate::alloc::nstd_alloc_allocate(size)
}

/// Default allocate_zeroed function.
#[inline]
unsafe extern "C" fn allocate_zeroed(_: NSTDAny, size: usize) -> NSTDAny {
    crate::alloc::nstd_alloc_allocate_zeroed(size)
}

/// Default reallocate function.
#[inline]
unsafe extern "C" fn reallocate(this: NSTDAny, ptr: &mut NSTDAny, size: usize, new_size: usize) {
    let this = this as *mut NSTDAllocator;
    (*this).errc |= crate::alloc::nstd_alloc_reallocate(ptr, size, new_size);
}

/// Default deallocate function.
#[inline]
unsafe extern "C" fn deallocate(this: NSTDAny, ptr: &mut NSTDAny, size: usize) {
    let this = this as *mut NSTDAllocator;
    (*this).errc |= crate::alloc::nstd_alloc_deallocate(ptr, size);
}
