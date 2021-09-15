#ifndef NSTD_STD_ENV_H_INCLUDED
#define NSTD_STD_ENV_H_INCLUDED
#include "../core/def.h"
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Returns the path to which the executable is in.
/// Use `nstd_std_env_free_path` to free memory allocated by this function.
/// Parameters:
///     `int *errc` - Returns as nonzero on error.
/// Returns: `char *path` - The path to the executable.
NSTDAPI char *nstd_std_env_path_to_exe(int *errc);

/// Returns the path of the current working directory
/// Use `nstd_std_env_free_path` to free memory allocated by this function.
/// Parameters:
///     `int *errc` - Returns as nonzero on error.
/// Returns: `char *path` - The path of the working directory.
NSTDAPI char *nstd_std_env_current_dir(int *errc);

/// Returns the path of a temporary directory.
/// Use `nstd_std_env_free_path` to free memory allocated by this function.
/// Returns: `char *path` - The path of the temp dir.
NSTDAPI char *nstd_std_env_temp_dir();

/// Frees memory allocated by `nstd_std_env_path_to_exe`,  `nstd_std_env_current_dir` or
/// `nstd_std_env_temp_dir`.
/// Parameters:
///     `char **path` - String from `nstd_std_env_path_to_exe` or `nstd_std_env_current_dir`.
NSTDAPI void nstd_std_env_free_path(char **path);

/// Sets the current working directory.
/// Parameters:
///     `const char *const path` - The new working directory.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_env_set_current_dir(const char *const path);

/// Returns an array of strings that contain the cmd args that the program was started with.
/// Parameters:
///     `NSTDSize *size` - Number of args.
/// Returns: `char *args` - The command line arguments.
NSTDAPI char *nstd_std_env_args(NSTDSize *size);

/// Frees memory allocated by `nstd_std_env_args`.
/// Parameters:
///     `char **args` - Returned from `nstd_std_env_args`.
NSTDAPI void nstd_std_env_free_args(char **args);

#ifdef __cplusplus
}
#endif
#endif
