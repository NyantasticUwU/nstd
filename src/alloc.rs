pub mod allocator;
pub mod heap;
use crate::core::def::{NSTDAny, NSTDErrorCode};

/// Allocates a new memory block.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDAny ptr` - The new memory block.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_allocate(size: usize) -> NSTDAny {
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        use std::alloc::Layout;
        let layout = Layout::from_size_align_unchecked(size, 1);
        std::alloc::alloc(layout).cast()
    }
    #[cfg(target_os = "linux")]
    {
        crate::os::linux::alloc::nstd_os_linux_alloc_allocate(size)
    }
    #[cfg(target_os = "windows")]
    {
        crate::os::windows::alloc::nstd_os_windows_alloc_allocate(size)
    }
}

/// Allocates a new memory block with all bytes set to 0.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDAny ptr` - The new memory block.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_allocate_zeroed(size: usize) -> NSTDAny {
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        use std::alloc::Layout;
        let layout = Layout::from_size_align_unchecked(size, 1);
        std::alloc::alloc_zeroed(layout).cast()
    }
    #[cfg(target_os = "linux")]
    {
        crate::os::linux::alloc::nstd_os_linux_alloc_allocate_zeroed(size, 1)
    }
    #[cfg(target_os = "windows")]
    {
        crate::os::windows::alloc::nstd_os_windows_alloc_allocate_zeroed(size)
    }
}

/// Reallocates a memory block.
/// Parameters:
///     `NSTDAny *const ptr` - Pointer to the memory block.
///     `const NSTDUSize size` - The current size of the memory block.
///     `const NSTDUSize new_size` - The new size of the memory block.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[cfg_attr(
    any(target_os = "linux", target_os = "windows"),
    allow(unused_variables)
)]
pub unsafe extern "C" fn nstd_alloc_reallocate(
    ptr: &mut NSTDAny,
    size: usize,
    new_size: usize,
) -> NSTDErrorCode {
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        use std::alloc::Layout;
        let layout = Layout::from_size_align_unchecked(size, 1);
        let new_mem = std::alloc::realloc((*ptr).cast(), layout, new_size);
        if !new_mem.is_null() {
            *ptr = new_mem.cast();
            return 0;
        }
        1
    }
    #[cfg(target_os = "linux")]
    {
        crate::os::linux::alloc::nstd_os_linux_alloc_reallocate(ptr, new_size)
    }
    #[cfg(target_os = "windows")]
    {
        crate::os::windows::alloc::nstd_os_windows_alloc_reallocate(ptr, new_size)
    }
}

/// Deallocates a memory block.
/// Parameters:
///     `NSTDAny *const ptr` - Pointer to the memory block.
///     `const NSTDUSize size` - Number of bytes to deallocate.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[cfg_attr(
    any(target_os = "linux", target_os = "windows"),
    allow(unused_variables)
)]
pub unsafe extern "C" fn nstd_alloc_deallocate(ptr: &mut NSTDAny, size: usize) -> NSTDErrorCode {
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        use crate::core::NSTD_CORE_NULL;
        use std::alloc::Layout;
        let layout = Layout::from_size_align_unchecked(size, 1);
        std::alloc::dealloc((*ptr).cast(), layout);
        *ptr = NSTD_CORE_NULL;
        0
    }
    #[cfg(target_os = "linux")]
    {
        crate::os::linux::alloc::nstd_os_linux_alloc_deallocate(ptr);
        0
    }
    #[cfg(target_os = "windows")]
    {
        crate::os::windows::alloc::nstd_os_windows_alloc_deallocate(ptr)
    }
}
