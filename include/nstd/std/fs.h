#ifndef NSTD_STD_FS_H_INCLUDED
#define NSTD_STD_FS_H_INCLUDED
#include "../core/def.h"
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif
#define NSTD_STD_FS_CREATE 0b00000001
#define NSTD_STD_FS_READ 0b00000010
#define NSTD_STD_FS_WRITE 0b00000100
#define NSTD_STD_FS_APPEND 0b00001000
#define NSTD_STD_FS_TRUNCATE 0b00010000

/// Represents a file handle.
typedef void *NSTDFile;

/// Creates a directory with the name `name`.
/// Parameters:
///     `const char *const name` - The name of the directory.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_fs_create_dir(const char *const name);

/// Creates a directory and all of it's parents if they are missing.
/// Parameters:
///     `const char *const name` - The name of the directory.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_fs_create_dir_all(const char *const name);

/// Removes an empty directory.
/// Parameters:
///     `const char *const name` - The name of the directory.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_fs_remove_dir(const char *const name);

/// Removes a directory and all of it's contents.
/// Parameters:
///     `const char *const name` - The name of the directory.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_fs_remove_dir_all(const char *const name);

/// Opens a file and returns the file handle. Files must be closed.
/// Parameters:
///     `const char *const name` - The name of the file.
///     `const NSTDSize mask` - Bit mask defining how to open the file.
///         - Bit 1 - Create the file if it doesn't exist. Write bit must be set for this to work.
///         - Bit 2 - Read from the file.
///         - Bit 3 - Write to the file.
///         - Bit 4 - Append to the file.
///         - Bit 5 - Truncate the file.
/// Returns: `NSTDFile file` - A handle to the opened file.
NSTDAPI NSTDFile nstd_std_fs_open(const char *const name, const NSTDSize mask);

/// Writes a string buffer to the specified file.
/// Parameters:
///     `NSTDFile file` - The file to write to.
///     `const char *const buf` - The buffer to write.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_fs_write(NSTDFile file, const char *const buf);

/// Reads file into string.
/// Parameters:
///     `NSTDFile file` - The file to read from.
/// Returns: `char *contents` - The file contents, null on error.
NSTDAPI char *nstd_std_fs_read(NSTDFile file);

/// Frees data from `nstd_std_fs_read`.
/// Parameters:
///     `char **contents` - Pointer to the string.
NSTDAPI void nstd_std_fs_free_read(char **contents);

/// Closes a file.
/// Parameters:
///     `NSTDFile *handle` - The handle to the file.
NSTDAPI void nstd_std_fs_close(NSTDFile *handle);

#ifdef __cplusplus
}
#endif
#endif
