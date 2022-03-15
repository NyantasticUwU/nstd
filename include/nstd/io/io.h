#ifndef NSTD_IO_IO_H_INCLUDED
#define NSTD_IO_IO_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
#include "../string.h"
NSTDCPPSTART

/// Writes a C string to stdout.
/// Parameters:
///     `const NSTDChar *const msg` - The message to write to stdout.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_io_print(const NSTDChar *const msg);

/// Writes a C string to stdout with a preceding new line.
/// Parameters:
///     `const NSTDChar *const msg` - The message to write to stdout.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_io_print_line(const NSTDChar *const msg);

/// Reads a line from stdin as an `NSTDString` but doesn't include the new line.
/// Returns: `NSTDString input` - Input read from stdin.
NSTDAPI NSTDString nstd_io_read();

/// Reads a line from stdin as an `NSTDString`.
/// Returns: `NSTDString input` - Input read from stdin.
NSTDAPI NSTDString nstd_io_read_line();

NSTDCPPEND
#endif
