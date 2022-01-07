#ifndef NSTD_CORE_SYS_H_INCLUDED
#define NSTD_CORE_SYS_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Terminates the program in an abnormal fashion.
NSTDAPI void nstd_core_sys_abort();

/// Asserts that `b` is true, aborts if `b` is false.
/// Parameters:
///     `const NSTDBool b` - The boolean.
NSTDAPI void nstd_core_sys_assert(const NSTDBool b);

#ifdef __cplusplus
}
#endif
#endif
