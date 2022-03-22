#ifndef NSTD_IO_STDERR_H_INCLUDED
#define NSTD_IO_STDERR_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
#include "output_stream.h"
NSTDCPPSTART

/// A raw handle to stdout.
typedef NSTDAny NSTDStandardErrorHandle;

/// Represents a handle to the standard error stream.
typedef struct
{
    /// The output stream.
    NSTDOutputStream output_stream;
    /// The raw handle to stderr.
    NSTDStandardErrorHandle handle;
} NSTDStandardError;

/// Returns a handle to stderr.
///
/// # Returns
///
/// `NSTDStandardError stderr` - The standard error stream.
NSTDAPI NSTDStandardError nstd_io_stderr();

/// Frees a handle to stderr.
///
/// # Parameters
///
/// - `NSTDStandardError *const stderr` - The standard error stream.
NSTDAPI void nstd_io_stderr_free(NSTDStandardError *const stderr);

NSTDCPPEND
#endif
