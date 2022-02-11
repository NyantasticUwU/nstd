#ifndef NSTD_IO_INPUT_STREAM_H_INCLUDED
#define NSTD_IO_INPUT_STREAM_H_INCLUDED
#include "../collections/vec.h"
#include "../core/def.h"
#include "../nstd.h"
#include "../string.h"
#include "stream.h"

/// Represents a raw handle to an input stream.
typedef NSTDAny NSTDRawInputStream;

/// Represents an input stream.
typedef struct
{
    /// The base stream.
    NSTDStream stream;
    /// A raw handle to the input stream.
    NSTDRawInputStream istream;
    /// Reads data from this input stream into a vector.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of this stream.
    /// Returns: `NSTDVec buff` - Bytes read from the input stream.
    NSTDVec (*read)(NSTDAny);
    /// Reads a line from this input stream into a string.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of this stream.
    /// Returns: `NSTDString string` - Line read from the input stream.
    NSTDString (*read_line)(NSTDAny);
} NSTDInputStream;

#endif
