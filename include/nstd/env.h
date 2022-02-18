#ifndef NSTD_ENV_H_INCLUDED
#define NSTD_ENV_H_INCLUDED
#include "core/def.h"
#include "core/str.h"
#include "collections/vec.h"
#include "nstd.h"
#include "string.h"
#ifdef NSTDCPP
extern "C"
{
#endif

/// Returns the path to which the executable is in.
/// Parameters:
///     `NSTDErrorCode *const errc` - Returns as nonzero on error.
/// Returns: `NSTDString path` - The path to the executable.
NSTDAPI NSTDString nstd_env_path_to_exe(NSTDErrorCode *const errc);

/// Returns the path of the current working directory
/// Parameters:
///     `NSTDErrorCode *const errc` - Returns as nonzero on error.
/// Returns: `NSTDString path` - The path of the working directory.
NSTDAPI NSTDString nstd_env_current_dir(NSTDErrorCode *const errc);

/// Returns the path of a temporary directory.
/// Returns: `NSTDString path` - The path of the temp dir.
NSTDAPI NSTDString nstd_env_temp_dir();

/// Sets the current working directory.
/// Parameters:
///     `const NSTDStr *const path` - The new working directory.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_env_set_current_dir(const NSTDStr *const path);

/// Returns a vector of strings that contain the cmd args that the program was started with.
/// Returns: `NSTDVec args` - A vector of `NSTDString`.
NSTDAPI NSTDVec nstd_env_args();

/// Frees memory allocated by `nstd_env_args`.
/// Parameters:
///     `NSTDVec *const args` - Returned from `nstd_env_args`.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_env_free_args(NSTDVec *const args);

/// Sets an environment variable.
/// Parameters:
///     `const NSTDChar *const k` - The var key.
///     `const NSTDChar *const v` - The var value.
NSTDAPI void nstd_env_set_var(const NSTDChar *const k, const NSTDChar *const v);

/// Gets an environment variable.
/// Parameters:
///     `const NSTDChar *const k` - The var key.
/// Returns: `NSTDString v` - The value of the variable.
NSTDAPI NSTDString nstd_env_get_var(const NSTDChar *const k);

/// Removes an environment variable.
/// This will not free memory allocated by `nstd_env_get_var`.
/// Parameters:
///     `const NSTDChar *const k` - The var key.
NSTDAPI void nstd_env_remove_var(const NSTDChar *const k);

/// Returns an array of strings that contain the environment variables.
/// Returns: `NSTDVec vars` - Vector of `NSTDString`.
NSTDAPI NSTDVec nstd_env_vars();

/// Frees memory allocated by `nstd_env_vars`.
/// Parameters:
///     `NSTDVec *const vars` - Returned from `nstd_env_vars`.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_env_free_vars(NSTDVec *const vars);

#ifdef NSTDCPP
}
#endif
#endif
