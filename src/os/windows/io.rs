use crate::{
    core::{
        def::{NSTDChar, NSTDErrorCode},
        slice::NSTDSlice,
    },
    os::windows::def::NSTDOSWindowsHandle,
};
use windows::Win32::{
    Globalization::CP_UTF8,
    System::Console::{
        GetStdHandle, SetConsoleOutputCP, WriteConsoleA, STD_HANDLE, STD_OUTPUT_HANDLE,
    },
};

/// Represents a handle to a standard IO stream.
pub type NSTDOSWindowsIOHandle = u32;

/// Initializes the `nstd.os.windows.io` module. This function should be called before any others.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_io_init() -> NSTDErrorCode {
    (SetConsoleOutputCP(CP_UTF8).0 == 0) as NSTDErrorCode
}

/// Gets the `NSTDOSWindowsHandle` of a `NSTDOSWindowsIOHandle`.
/// Parameters:
///     `const NSTDOSWindowsIOHandle stream` - An IO handle.
/// Returns: `NSTDOSWindowsHandle handle` - The Window's handle.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_io_handle_as_handle(
    stream: NSTDOSWindowsIOHandle,
) -> NSTDOSWindowsHandle {
    GetStdHandle(STD_HANDLE(stream)).0
}

/// Writes a C string to stdout.
/// Parameters:
///     `const NSTDChar *const cstr` - The C string to write to stdout.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_io_print(cstr: *const NSTDChar) -> NSTDErrorCode {
    let stdout = nstd_os_windows_io_stdout();
    let bytes = crate::core::cstr::nstd_core_cstr_as_slice(cstr);
    nstd_os_windows_io_write(stdout, &bytes)
}

/// Writes a C string to stdout with a newline character.
/// Parameters:
///     `const NSTDChar *const cstr` - The C string to write to stdout.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_io_print_line(cstr: *const NSTDChar) -> NSTDErrorCode {
    let newline = b"\n\0";
    let a = nstd_os_windows_io_print(cstr);
    let b = nstd_os_windows_io_print(newline.as_ptr().cast());
    a | b
}

/// Retrieves a handle to stdout.
/// Returns: `NSTDOSWindowsIOHandle stdout` - The standard output stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_io_stdout() -> NSTDOSWindowsIOHandle {
    STD_OUTPUT_HANDLE.0
}

/// Writes a buffer of `bytes` to `stream`.
/// Parameters:
///     `const NSTDOSWindowsIOHandle stream` - Handle to a standard IO stream.
///     `const NSTDSlice *const bytes` - The bytes to write to the stream.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_io_write(
    stream: NSTDOSWindowsIOHandle,
    bytes: &NSTDSlice,
) -> NSTDErrorCode {
    (WriteConsoleA(
        GetStdHandle(STD_HANDLE(stream)),
        bytes.ptr.raw,
        bytes.size as u32,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    )
    .0 == 0) as NSTDErrorCode
}
