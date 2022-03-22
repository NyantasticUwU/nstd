#ifndef NSTD_IO_STDIN_H_INCLUDED
#define NSTD_IO_STDIN_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
#include "input_stream.h"
NSTDCPPSTART

/// A raw handle to stdin.
typedef NSTDAny NSTDStandardInputHandle;

/// Represents a handle to the standard input stream.
typedef struct
{
    /// The input stream.
    NSTDInputStream input_stream;
    /// The raw handle to stdin.
    NSTDStandardInputHandle handle;
} NSTDStandardInput;

/// Returns a handle to stdin.
///
/// # Returns
///
/// `NSTDStandardInput stdin` - The standard input stream.
NSTDAPI NSTDStandardInput nstd_io_stdin();

/// Frees a handle to stdin.
///
/// # Parameters
///
/// - `NSTDStandardInput *const stdin` - The standard input stream.
NSTDAPI void nstd_io_stdin_free(NSTDStandardInput *const stdin);

NSTDCPPEND
#endif
