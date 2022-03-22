//! Handle type for Windows standard IO streams.
use crate::os::windows::def::NSTDOSWindowsHandle;
use windows_sys::Win32::System::Console::GetStdHandle;

/// Represents a handle to a standard IO stream.
pub type NSTDOSWindowsIOHandle = u32;

/// Gets the `NSTDOSWindowsHandle` of a `NSTDOSWindowsIOHandle`.
/// Parameters:
///     `const NSTDOSWindowsIOHandle stream` - An IO handle.
/// Returns: `NSTDOSWindowsHandle handle` - The Window's handle.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_io_handle_as_handle(
    stream: NSTDOSWindowsIOHandle,
) -> NSTDOSWindowsHandle {
    GetStdHandle(stream)
}
