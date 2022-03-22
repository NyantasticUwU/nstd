#ifndef NSTD_OS_WINDOWS_IO_IO_H_INCLUDED
#define NSTD_OS_WINDOWS_IO_IO_H_INCLUDED
#include "../../../core/def.h"
#include "../../../core/slice.h"
#include "../../../nstd.h"
#include "../def.h"
#include "handle.h"
NSTDCPPSTART

/// Initializes the `nstd.os.windows.io` module. This function should be called before any others in
/// this module.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_os_windows_io_init();

/// Writes a C string to stdout.
/// Parameters:
///     `const NSTDOSWindowsIOHandle stream` - An IO stream.
///     `const NSTDChar *const cstr` - The C string to write to stdout.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_os_windows_io_print(
    const NSTDOSWindowsIOHandle stream,
    const NSTDChar *const cstr);

/// Writes a C string to stdout with a newline character.
/// Parameters:
///     `const NSTDChar *const cstr` - The C string to write to stdout.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_os_windows_io_print_line(const NSTDChar *const cstr);

/// Retrieves a handle to stdin.
/// Returns: `NSTDOSWindowsIOHandle stdin` - The standard input stream.
NSTDAPI NSTDOSWindowsIOHandle nstd_os_windows_io_stdin();

/// Retrieves a handle to stdout.
/// Returns: `NSTDOSWindowsIOHandle stdout` - The standard output stream.
NSTDAPI NSTDOSWindowsIOHandle nstd_os_windows_io_stdout();

/// Retrieves a handle to stderr.
/// Returns: `NSTDOSWindowsIOHandle stderr` - The standard error stream.
NSTDAPI NSTDOSWindowsIOHandle nstd_os_windows_io_stderr();

/// Reads a buffer from `stream` into `buffer`.
/// Parameters:
///     `const NSTDOSWindowsHandle stream` - Handle to an IO stream.
///     `NSTDSlice *const buffer` - The buffer to read into.
///     `NSTDUInt32 *const read` - Returns as the number of bytes actually read.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_os_windows_io_read(
    const NSTDOSWindowsHandle stream,
    NSTDSlice *const buffer,
    NSTDUInt32 *const read);

/// Writes a buffer of `bytes` to `stream`.
/// Parameters:
///     `const NSTDOSWindowsHandle stream` - Handle to an IO stream.
///     `const NSTDSlice *const bytes` - The bytes to write to the stream.
///     `NSTDUInt32 *const written` - Returns as the number of bytes actually written.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_os_windows_io_write(
    const NSTDOSWindowsHandle stream,
    const NSTDSlice *const bytes,
    NSTDUInt32 *const written);

NSTDCPPEND
#endif
