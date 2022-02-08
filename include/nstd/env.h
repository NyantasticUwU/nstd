#ifndef NSTD_ENV_H_INCLUDED
#define NSTD_ENV_H_INCLUDED
#include "core/def.h"
#include "collections/vec.h"
#include "nstd.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Returns the path to which the executable is in.
/// Use `nstd_env_free_path` to free memory allocated by this function.
/// Parameters:
///     `int *errc` - Returns as nonzero on error.
/// Returns: `char *path` - The path to the executable.
NSTDAPI char *nstd_env_path_to_exe(int *errc);

/// Returns the path of the current working directory
/// Use `nstd_env_free_path` to free memory allocated by this function.
/// Parameters:
///     `int *errc` - Returns as nonzero on error.
/// Returns: `char *path` - The path of the working directory.
NSTDAPI char *nstd_env_current_dir(int *errc);

/// Returns the path of a temporary directory.
/// Use `nstd_env_free_path` to free memory allocated by this function.
/// Returns: `char *path` - The path of the temp dir.
NSTDAPI char *nstd_env_temp_dir();

/// Frees memory allocated by `nstd_env_path_to_exe`,  `nstd_env_current_dir` or
/// `nstd_env_temp_dir`.
/// Parameters:
///     `char **path` - String from `nstd_env_path_to_exe` or `nstd_env_current_dir`.
NSTDAPI void nstd_env_free_path(char **path);

/// Sets the current working directory.
/// Parameters:
///     `const char *const path` - The new working directory.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_env_set_current_dir(const char *const path);

/// Returns a vector of strings that contain the cmd args that the program was started with.
/// Returns: `NSTDVec args` - The command line arguments.
NSTDAPI NSTDVec nstd_env_args();

/// Frees memory allocated by `nstd_env_args`.
/// Parameters:
///     `NSTDVec *const args` - Returned from `nstd_env_args`.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_env_free_args(NSTDVec *const args);

/// Sets an environment variable.
/// Parameters:
///     `const char *const k` - The var key.
///     `const char *const v` - The var value.
NSTDAPI void nstd_env_set_var(const char *const k, const char *const v);

/// Gets an environment variable.
/// Parameters:
///     `const char *const k` - The var key.
/// Returns: `char *v` - The value of the variable.
NSTDAPI char *nstd_env_get_var(const char *const k);

/// Removes an environment variable.
/// This will not free memory allocated by `nstd_env_get_var`.
/// Parameters:
///     `const char *const k` - The var key.
NSTDAPI void nstd_env_remove_var(const char *const k);

/// Frees memory allocated by `nstd_env_get_var`.
/// Parameters:
///     `char **v` - The value returned from `nstd_env_get_var`.
NSTDAPI void nstd_env_free_var(char **v);

/// Returns an array of strings that contain the environment variables.
/// Parameters:
///     `NSTDUSize *size` - Number of variables.
/// Returns: `char *vars` - The environment variables keys.
NSTDAPI char *nstd_env_vars(NSTDUSize *size);

/// Frees memory allocated by `nstd_env_vars`.
/// Parameters:
///     `char **vars` - Returned from `nstd_env_vars`.
NSTDAPI void nstd_env_free_vars(char **vars);

#ifdef __cplusplus
}
#endif
#endif
