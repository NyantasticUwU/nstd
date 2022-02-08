#ifndef NSTD_FS_H_INCLUDED
#define NSTD_FS_H_INCLUDED
#include "core/def.h"
#include "collections/vec.h"
#include "nstd.h"
#ifdef __cplusplus
extern "C"
{
#endif
#define NSTD_FS_CREATE 0b00000001
#define NSTD_FS_READ 0b00000010
#define NSTD_FS_WRITE 0b00000100
#define NSTD_FS_APPEND 0b00001000
#define NSTD_FS_TRUNCATE 0b00010000

/// Represents a file handle.
typedef void *NSTDFile;

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

/// Opens a file and returns the file handle. Files must be closed.
/// Parameters:
///     `const char *const name` - The name of the file.
///     `const NSTDUSize mask` - Bit mask defining how to open the file.
///         - Bit 1 - Create the file if it doesn't exist. Write bit must be set for this to work.
///         - Bit 2 - Read from the file.
///         - Bit 3 - Write to the file.
///         - Bit 4 - Append to the file.
///         - Bit 5 - Truncate the file.
/// Returns: `NSTDFile file` - A handle to the opened file.
NSTDAPI NSTDFile nstd_fs_open(const char *const name, const NSTDUSize mask);

/// Writes a string buffer to the specified file.
/// Parameters:
///     `NSTDFile file` - The file to write to.
///     `const char *const buf` - The buffer to write.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_fs_write(NSTDFile file, const char *const buf);

/// Reads file into string.
/// Parameters:
///     `NSTDFile file` - The file to read from.
/// Returns: `char *contents` - The file contents, null on error.
NSTDAPI char *nstd_fs_read(NSTDFile file);

/// Frees data from `nstd_fs_read`.
/// Parameters:
///     `char **contents` - Pointer to the string.
NSTDAPI void nstd_fs_free_read(char **contents);

/// Reads raw data from a file.
/// Parameters:
///     `NSTDFile file` - The file to read from.
///     `NSTDUSize *const size` - Returns as number of bytes read.
/// Returns: `NSTDByte *data` - The raw file data.
NSTDAPI NSTDByte *nstd_fs_read_raw(NSTDFile file, NSTDUSize *const size);

/// Frees raw data that has been read from a file.
/// Parameters:
///     `NSTDByte **const data` - The data to be freed.
///     `const NSTDUSize size` - Number of bytes to free.
NSTDAPI void nstd_fs_free_raw(NSTDByte **const data, const NSTDUSize size);

/// Sets the position of the stream pointer from the current pos of the stream pointer.
/// Parameters:
///     `NSTDFile file` - The file handle.
///     `long long pos` - The position to set the stream pointer to.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_fs_seek(NSTDFile file, long long pos);

/// Sets the position of the stream pointer from the start of a file.
/// Parameters:
///     `NSTDFile file` - The file handle.
///     `long long pos` - The position to set the stream pointer to.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_fs_seek_from_start(NSTDFile file, long long pos);

/// Sets the position of the stream pointer from the end of a file.
/// Parameters:
///     `NSTDFile file` - The file handle.
///     `long long pos` - The position to set the stream pointer to.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_fs_seek_from_end(NSTDFile file, long long pos);

/// Rewinds the stream pointer to the start of the file.
/// Parameters:
///     `NSTDFile file` - The file handle.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_fs_rewind(NSTDFile file);

/// Closes a file.
/// Parameters:
///     `NSTDFile *handle` - The handle to the file.
NSTDAPI void nstd_fs_close(NSTDFile *handle);

#ifdef __cplusplus
}
#endif
#endif
