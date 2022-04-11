//! Thread support for Windows.
use windows_sys::Win32::System::Threading::Sleep;

/// Puts the current thread to sleep for `ms` milliseconds.
///
/// # Parameters:
///
/// - `const NSTDUInt32 ms` - The number of milliseconds to sleep for.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_thread_sleep(ms: u32) {
    Sleep(ms);
}
