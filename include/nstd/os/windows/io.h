#ifndef NSTD_OS_WINDOWS_IO_H_INCLUDED
#define NSTD_OS_WINDOWS_IO_H_INCLUDED
#include "../../core/def.h"
#include "../../core/slice.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a handle to a standard IO stream.
typedef NSTDUInt32 NSTDOSWindowsIOHandle;

/// Initializes the `nstd.os.windows.io` module. This function should be called before any others.
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
///     `const NSTDOSWindowsIOHandle stream` - Handle to a standard IO stream.
///     `const NSTDSlice *const bytes` - The bytes to write to the stream.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_os_windows_io_write(
    const NSTDOSWindowsIOHandle stream,
    const NSTDSlice *const bytes);

#ifdef __cplusplus
}
#endif
#endif
