#ifndef NSTD_IO_IO_STREAM_H_INCLUDED
#define NSTD_IO_IO_STREAM_H_INCLUDED
#include "../nstd.h"
#include "input_stream.h"
#include "output_stream.h"

/// Represents both an input and an output stream.
typedef struct
{
    /// The input stream.
    NSTDInputStream input_stream;
    /// The output stream.
    NSTDOutputStream output_stream;
} NSTDIOStream;

#endif
