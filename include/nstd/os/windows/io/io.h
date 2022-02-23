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
///     `const NSTDChar *const cstr` - The C string to write to stdout.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_os_windows_io_print(const NSTDChar *const cstr);

/// Writes a C string to stdout with a newline character.
/// Parameters:
///     `const NSTDChar *const cstr` - The C string to write to stdout.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_os_windows_io_print_line(const NSTDChar *const cstr);

/// Retrieves a handle to stdout.
/// Returns: `NSTDOSWindowsIOHandle stdout` - The standard output stream.
NSTDAPI NSTDOSWindowsIOHandle nstd_os_windows_io_stdout();

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
