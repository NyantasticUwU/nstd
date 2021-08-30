#ifndef NSTD_CORE_SYS_H_INCLUDED
#define NSTD_CORE_SYS_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Causes abnormal program termination with a message.
void nstd_core_sys_abort();

/// Asserts that `assertion` is true (not 0) otherwise throws.
/// Parameters:
///     `const NSTDCISize assertion` - The value to assert.
void nstd_core_sys_assert(const NSTDCISize assertion);

#ifdef __cplusplus
}
#endif
#endif
