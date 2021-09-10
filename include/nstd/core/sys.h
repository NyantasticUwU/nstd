#ifndef NSTD_CORE_SYS_H_INCLUDED
#define NSTD_CORE_SYS_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Causes abnormal program termination with a message.
NSTDAPI void nstd_core_sys_abort();

/// Asserts that `assertion` is true (not 0) otherwise throws.
/// Parameters:
///     `const NSTDCOREISize assertion` - The value to assert.
NSTDAPI void nstd_core_sys_assert(const NSTDCOREISize assertion);

#ifdef __cplusplus
}
#endif
#endif
