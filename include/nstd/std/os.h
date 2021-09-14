#ifndef NSTD_STD_OS_H_INCLUDED
#define NSTD_STD_OS_H_INCLUDED
#include "../core/def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Returns a string describing the specific operating system in use.
/// `nstd_std_os_free_name` must be called to free memory allocated by this function.
/// Returns: `char *os_name` - The os's name as a string.
NSTDAPI char *nstd_std_os_name();

/// Frees memory allocated by `nstd_std_os_name`.
/// Parameters:
///     `const char **os_name` - Pointer to the os name cstr.
NSTDAPI void nstd_std_os_free_name(const char **os_name);

/// Returns a string describing the specific cpu architecture in use.
/// `nstd_std_os_free_arch_name` must be called to free memory allocated by this function.
/// Returns: `char *arch_name` - The cpu architecture's name as a string.
NSTDAPI char *nstd_std_os_arch_name();

/// Frees memory allocated by `nstd_std_os_arch_name`.
/// Parameters:
///     `const char **arch_name` - Pointer to the arch name cstr.
NSTDAPI void nstd_std_os_free_arch_name(const char **arch_name);

#ifdef __cplusplus
}
#endif
#endif
