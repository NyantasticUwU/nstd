#ifndef NSTD_FS_FS_H_INCLUDED
#define NSTD_FS_FS_H_INCLUDED
#include "../core/def.h"
#include "../collections/vec.h"
#include "../nstd.h"
#include "file.h"
#ifdef __cplusplus
extern "C"
{
#endif
#define NSTD_FS_CREATE 0b00000001
#define NSTD_FS_READ 0b00000010
#define NSTD_FS_WRITE 0b00000100
#define NSTD_FS_APPEND 0b00001000
#define NSTD_FS_TRUNCATE 0b00010000

/// Checks if the given path exists.
/// Parameters:
///     `const char *const path` - The path to check.
/// Returns: `int exists` - Nonzero if the path exists.
NSTDAPI int nstd_fs_exists(const char *const path);

/// Checks if the given path is a file.
/// Parameters:
///     `const char *const path` - The path to check.
/// Returns: `int is_file` - Nonzero if the path is a file.
NSTDAPI int nstd_fs_is_file(const char *const path);

/// Checks if the given path is a directory.
/// Parameters:
///     `const char *const path` - The path to check.
/// Returns: `int is_dir` - Nonzero if the path is a directory.
NSTDAPI int nstd_fs_is_dir(const char *const path);

/// Returns a vector of all a directory's contents.
/// NOTE: Memory allocated by this function should be freed with `nstd_fs_dir_contents_free`.
/// Parameters:
///     `const char *const dir` - The directory.
/// Returns: `NSTDVec contents` - The directory's contents.
NSTDAPI NSTDVec nstd_fs_dir_contents(const char *const dir);

/// Frees memory allocated by `nstd_fs_dir_contents`.
/// Parameters:
///     `NSTDVec *const contents` - A directory's contents.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_fs_dir_contents_free(NSTDVec *const contents);

/// Creates a directory with the name `name`.
/// Parameters:
///     `const char *const name` - The name of the directory.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_fs_create_dir(const char *const name);

/// Creates a directory and all of it's parents if they are missing.
/// Parameters:
///     `const char *const name` - The name of the directory.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_fs_create_dir_all(const char *const name);

/// Removes an empty directory.
/// Parameters:
///     `const char *const name` - The name of the directory.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_fs_remove_dir(const char *const name);

/// Removes a directory and all of it's contents.
/// Parameters:
///     `const char *const name` - The name of the directory.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_fs_remove_dir_all(const char *const name);

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
NSTDAPI NSTDFile nstd_fs_open(const char *const name, const NSTDUSize mask);

/// Frees a file stream and closes the file.
/// Parameters:
///     `NSTDFile *const file` - The file stream to free.
NSTDAPI void nstd_fs_close(NSTDFile *const file);

#ifdef __cplusplus
}
#endif
#endif
