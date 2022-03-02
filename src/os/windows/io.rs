pub mod handle;
use self::handle::NSTDOSWindowsIOHandle;
use crate::{
    core::{
        def::{NSTDChar, NSTDErrorCode},
        slice::NSTDSlice,
    },
    os::windows::def::NSTDOSWindowsHandle,
};
use windows_sys::Win32::{
    Globalization::CP_UTF8,
    System::Console::{
        GetStdHandle, ReadConsoleA, SetConsoleOutputCP, WriteConsoleA, CONSOLE_READCONSOLE_CONTROL,
        STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
    },
};

/// Initializes the `nstd.os.windows.io` module. This function should be called before any others in
/// this module.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_io_init() -> NSTDErrorCode {
    (SetConsoleOutputCP(CP_UTF8) == 0) as NSTDErrorCode
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
    nstd_os_windows_io_write(GetStdHandle(stdout), &bytes, std::ptr::null_mut())
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

/// Retrieves a handle to stdin.
/// Returns: `NSTDOSWindowsIOHandle stdin` - The standard input stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_io_stdin() -> NSTDOSWindowsIOHandle {
    STD_INPUT_HANDLE
}

/// Retrieves a handle to stdout.
/// Returns: `NSTDOSWindowsIOHandle stdout` - The standard output stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_io_stdout() -> NSTDOSWindowsIOHandle {
    STD_OUTPUT_HANDLE
}

/// Reads a buffer from `stream` into `buffer`.
/// Parameters:
///     `const NSTDOSWindowsHandle stream` - Handle to an IO stream.
///     `NSTDSlice *const buffer` - The buffer to read into.
///     `NSTDUInt32 *const read` - Returns as the number of bytes actually read.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_io_read(
    stream: NSTDOSWindowsHandle,
    buffer: &mut NSTDSlice,
    read: *mut u32,
) -> NSTDErrorCode {
    const EOL: CONSOLE_READCONSOLE_CONTROL = CONSOLE_READCONSOLE_CONTROL {
        nLength: std::mem::size_of::<CONSOLE_READCONSOLE_CONTROL>() as u32,
        nInitialChars: 0,
        dwCtrlWakeupMask: 1 << b'\n',
        dwControlKeyState: 0,
    };
    (ReadConsoleA(stream, buffer.ptr.raw, buffer.size as u32, read, &EOL) == 0) as NSTDErrorCode
}

/// Writes a buffer of `bytes` to `stream`.
/// Parameters:
///     `const NSTDOSWindowsHandle stream` - Handle to an IO stream.
///     `const NSTDSlice *const bytes` - The bytes to write to the stream.
///     `NSTDUInt32 *const written` - Returns as the number of bytes actually written.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_io_write(
    stream: NSTDOSWindowsHandle,
    bytes: &NSTDSlice,
    written: *mut u32,
) -> NSTDErrorCode {
    (WriteConsoleA(
        stream,
        bytes.ptr.raw,
        bytes.size as u32,
        written,
        std::ptr::null_mut(),
    ) == 0) as NSTDErrorCode
}
