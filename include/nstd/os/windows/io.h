#ifndef NSTD_OS_WINDOWS_IO_H_INCLUDED
#define NSTD_OS_WINDOWS_IO_H_INCLUDED
#include "../../core/def.h"
#ifdef __cplusplus
extern "C"
{
#endif

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

#ifdef __cplusplus
}
#endif
#endif
