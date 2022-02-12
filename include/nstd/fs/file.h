#ifndef NSTD_FS_FILE_H_INCLUDED
#define NSTD_FS_FILE_H_INCLUDED
#include "../core/def.h"
#include "../io/io_stream.h"
#include "../nstd.h"

/// Represents a raw handle to a file.
typedef NSTDAny NSTDFileHandle;

/// Represents a file stream.
typedef struct
{
    /// The input/output stream.
    NSTDIOStream io_stream;
    /// The handle to the file.
    NSTDFileHandle handle;
} NSTDFile;

#endif
