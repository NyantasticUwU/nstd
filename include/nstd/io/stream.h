#ifndef NSTD_IO_STREAM_H_INCLUDED
#define NSTD_IO_STREAM_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"

/// An interface that represents a data stream.
typedef struct
{
    /// Set to nonzero if an error has occurred on the stream.
    NSTDErrorCode errc;
} NSTDStream;

#endif
