#ifndef NSTD_SYS_H_INCLUDED
#define NSTD_SYS_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Causes abnormal program termination with a message.
void nstd_sys_abort();

/// Asserts that `assertion` is true (not 0) otherwise throws.
/// Parameters:
///     `const NSTDISize assertion` - The value to assert.
void nstd_sys_assert(const NSTDISize assertion);

#ifdef __cplusplus
}
#endif
#endif
