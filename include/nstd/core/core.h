#ifndef NSTD_CORE_CORE_H_INCLUDED
#define NSTD_CORE_CORE_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Terminates the program in an abnormal fashion.
/// NOTE: This will only abort if the `panics` feature is enabled when compiling. Also note that it
/// is enabled by default.
NSTDAPI void nstd_core_abort();

/// Asserts that `b` is true, aborts if `b` is false.
/// Parameters:
///     `const NSTDBool b` - The boolean.
NSTDAPI void nstd_core_assert(const NSTDBool b);

#ifdef __cplusplus
}
#endif
#endif
