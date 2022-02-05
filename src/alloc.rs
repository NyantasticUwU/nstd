pub mod heap;
use crate::core::def::NSTDAny;

/// Allocates a new memory block.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDAny ptr` - The new memory block.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_allocate(size: usize) -> NSTDAny {
    #[cfg(not(target_os = "windows"))]
    {
        use std::alloc::Layout;
        match Layout::array::<u8>(size) {
            Ok(layout) => std::alloc::alloc(layout).cast(),
            _ => std::ptr::null_mut(),
        }
    }
    #[cfg(target_os = "windows")]
    crate::os::windows::alloc::nstd_os_windows_alloc_allocate(size)
}

/// Allocates a new memory block with all bytes set to 0.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDAny ptr` - The new memory block.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_allocate_zeroed(size: usize) -> NSTDAny {
    #[cfg(not(target_os = "windows"))]
    {
        use std::alloc::Layout;
        match Layout::array::<u8>(size) {
            Ok(layout) => std::alloc::alloc_zeroed(layout).cast(),
            _ => std::ptr::null_mut(),
        }
    }
    #[cfg(target_os = "windows")]
    crate::os::windows::alloc::nstd_os_windows_alloc_allocate_zeroed(size)
}

/// Reallocates a memory block.
/// Parameters:
///     `NSTDAny *const ptr` - Pointer to the memory block.
///     `const NSTDUSize size` - The current size of the memory block.
///     `const NSTDUSize new_size` - The new size of the memory block.
/// Returns: `NSTDInt32 errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[cfg_attr(target_os = "windows", allow(unused_variables))]
pub unsafe extern "C" fn nstd_alloc_reallocate(
    ptr: &mut NSTDAny,
    size: usize,
    new_size: usize,
) -> i32 {
    #[cfg(not(target_os = "windows"))]
    {
        use std::alloc::Layout;
        let new_mem = match Layout::array::<u8>(size) {
            Ok(layout) => std::alloc::realloc((*ptr).cast(), layout, new_size),
            _ => return 1,
        };
        match new_mem.is_null() {
            false => {
                *ptr = new_mem.cast();
                0
            }
            true => 1,
        }
    }
    #[cfg(target_os = "windows")]
    crate::os::windows::alloc::nstd_os_windows_alloc_reallocate(ptr, new_size)
}

/// Deallocates a memory block.
/// Parameters:
///     `NSTDAny *const ptr` - Pointer to the memory block.
///     `const NSTDUSize size` - Number of bytes to deallocate.
/// Returns: `NSTDInt32 errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[cfg_attr(target_os = "windows", allow(unused_variables))]
pub unsafe extern "C" fn nstd_alloc_deallocate(ptr: &mut NSTDAny, size: usize) -> i32 {
    #[cfg(not(target_os = "windows"))]
    {
        use std::alloc::Layout;
        match Layout::array::<u8>(size) {
            Ok(layout) => {
                std::alloc::dealloc((*ptr).cast(), layout);
                *ptr = std::ptr::null_mut();
                0
            }
            _ => 1,
        }
    }
    #[cfg(target_os = "windows")]
    crate::os::windows::alloc::nstd_os_windows_alloc_deallocate(ptr)
}
