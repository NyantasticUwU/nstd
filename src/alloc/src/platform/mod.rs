mod cp;
mod linux;
mod macos;
mod windows;
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub use crate::platform::cp::*;
#[cfg(target_os = "linux")]
pub use crate::platform::linux::*;
#[cfg(target_os = "macos")]
pub use crate::platform::macos::*;
#[cfg(target_os = "windows")]
pub use crate::platform::windows::*;
use std::os::raw::c_int;

/// Trait for each platform implementation.
pub trait PlatformImpl {
    unsafe fn allocate(size: usize) -> *mut u8;
    unsafe fn allocate_zeroed(size: usize) -> *mut u8;
    unsafe fn reallocate(ptr: *mut *mut u8, size: usize, new_size: usize) -> c_int;
    unsafe fn deallocate(ptr: *mut *mut u8, size: usize) -> c_int;
}
