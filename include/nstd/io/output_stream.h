#ifndef NSTD_IO_OUTPUT_STREAM_H_INCLUDED
#define NSTD_IO_OUTPUT_STREAM_H_INCLUDED
#include "../core/def.h"
#include "../core/slice.h"
#include "../nstd.h"
#include "stream.h"

/// Represents a raw handle to an output stream.
typedef NSTDAny NSTDRawOutputStream;

/// Represents an output stream.
typedef struct
{
    /// The base stream.
    NSTDStream stream;
    /// A raw handle to the output stream.
    NSTDRawOutputStream ostream;
    /// Flushes this stream.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of the stream.
    void (*flush)(NSTDAny);
    /// Writes a slice to this stream.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of the stream.
    ///     `const NSTDSlice *buff` - The buffer to write to this stream.
    void (*write)(NSTDAny, const NSTDSlice *);
} NSTDOutputStream;

#endif
