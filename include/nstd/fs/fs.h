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

#ifdef __cplusplus
}
#endif
#endif
