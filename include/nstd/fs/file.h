#ifndef NSTD_FS_FILE_H_INCLUDED
#define NSTD_FS_FILE_H_INCLUDED
#include "../core/def.h"
#include "../io/io_stream.h"
#include "../nstd.h"
#ifdef __cplusplus
extern "C"
{
#endif
#define NSTD_FS_FILE_CREATE 0b00000001
#define NSTD_FS_FILE_READ 0b00000010
#define NSTD_FS_FILE_WRITE 0b00000100
#define NSTD_FS_FILE_APPEND 0b00001000
#define NSTD_FS_FILE_TRUNCATE 0b00010000

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

/// Opens a file and returns the file stream. Files must be closed.
/// Parameters:
///     `const char *const name` - The name of the file.
///     `const NSTDUSize mask` - Bit mask defining how to open the file.
///         - Bit 1 - Create the file if it doesn't exist. Write bit must be set for this to work.
///         - Bit 2 - Read from the file.
///         - Bit 3 - Write to the file.
///         - Bit 4 - Append to the file.
///         - Bit 5 - Truncate the file.
/// Returns: `NSTDFile file` - The file stream.
NSTDAPI NSTDFile nstd_fs_file_open(const char *const name, const NSTDUSize mask);

/// Frees a file stream and closes the file.
/// Parameters:
///     `NSTDFile *const file` - The file stream to free.
NSTDAPI void nstd_fs_file_close(NSTDFile *const file);

#ifdef __cplusplus
}
#endif
#endif
