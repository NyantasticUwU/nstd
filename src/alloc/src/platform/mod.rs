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
use nstd_core::def::NSTDAny;

/// Trait for each platform implementation.
pub trait PlatformImpl {
    unsafe fn allocate(size: usize) -> NSTDAny;
    unsafe fn allocate_zeroed(size: usize) -> NSTDAny;
    unsafe fn reallocate(ptr: *mut NSTDAny, size: usize, new_size: usize) -> i32;
    unsafe fn deallocate(ptr: *mut NSTDAny, size: usize) -> i32;
}
