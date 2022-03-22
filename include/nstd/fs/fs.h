#ifndef NSTD_FS_FS_H_INCLUDED
#define NSTD_FS_FS_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
#include "../vec.h"
NSTDCPPSTART

/// Checks if the given path exists.
///
/// # Parameters
///
/// - `const NSTDChar *const path` - The path to check.
///
/// # Returns
///
/// `NSTDBool exists` - `NSTD_BOOL_TRUE` if the path exists.
NSTDAPI NSTDBool nstd_fs_exists(const NSTDChar *const path);

/// Checks if the given path is a file.
///
/// # Parameters
///
/// - `const NSTDChar *const path` - The path to check.
///
/// # Returns
///
/// `NSTDBool is_file` - `NSTD_BOOL_TRUE` if the path is a file.
NSTDAPI NSTDBool nstd_fs_is_file(const NSTDChar *const path);

/// Checks if the given path is a directory.
///
/// # Parameters
///
/// - `const NSTDChar *const path` - The path to check.
///
/// # Returns
///
/// `NSTDBool is_dir` - `NSTD_BOOL_TRUE` if the path is a directory.
NSTDAPI NSTDBool nstd_fs_is_dir(const NSTDChar *const path);

/// Returns a vector of all a directory's contents.
///
/// # Note
///
/// Memory allocated by this function should be freed with `nstd_fs_dir_contents_free`.
///
/// # Parameters
///
/// - `const NSTDChar *const dir` - The directory.
///
/// # Returns
///
/// `NSTDVec contents` - An `NSTDVec` of `NSTDString`.
NSTDAPI NSTDVec nstd_fs_dir_contents(const NSTDChar *const dir);

/// Frees memory allocated by `nstd_fs_dir_contents`.
///
/// # Parameters
///
/// - `NSTDVec *const contents` - A directory's contents.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_fs_dir_contents_free(NSTDVec *const contents);

/// Creates a directory with the name `name`.
///
/// # Parameters
///
/// - `const NSTDChar *const name` - The name of the directory.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_fs_create_dir(const NSTDChar *const name);

/// Creates a directory and all of it's parents if they are missing.
///
/// # Parameters
///
/// - `const NSTDChar *const name` - The name of the directory.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_fs_create_dir_all(const NSTDChar *const name);

/// Removes an empty directory.
///
/// # Parameters
///
/// - `const NSTDChar *const name` - The name of the directory.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_fs_remove_dir(const NSTDChar *const name);

/// Removes a directory and all of it's contents.
///
/// # Parameters
///
/// - `const NSTDChar *const name` - The name of the directory.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_fs_remove_dir_all(const NSTDChar *const name);

NSTDCPPEND
#endif
