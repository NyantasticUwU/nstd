#ifndef NSTD_IO_STDOUT_H_INCLUDED
#define NSTD_IO_STDOUT_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
#include "output_stream.h"
#ifdef NSTDCPP
extern "C"
{
#endif

/// A raw handle to stdout.
typedef NSTDAny NSTDStandardOutputHandle;

/// Represents a handle to the standard output stream.
typedef struct
{
    /// The output stream.
    NSTDOutputStream output_stream;
    /// The raw handle to stdout.
    NSTDStandardOutputHandle handle;
} NSTDStandardOutput;

/// Frees a handle to stdout.
/// Parameters:
///     `NSTDStandardOutput *const stdout` - The standard output stream.
NSTDAPI void nstd_io_stdout_free(NSTDStandardOutput *const stdout);

#ifdef NSTDCPP
}
#endif
#endif
