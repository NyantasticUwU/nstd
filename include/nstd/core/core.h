#ifndef NSTD_CORE_CORE_H_INCLUDED
#define NSTD_CORE_CORE_H_INCLUDED
#include "../nstd.h"
#include "def.h"
#ifdef NSTDCPP
extern "C"
{
#endif

/// A null pointer value.
#define NSTD_CORE_NULL 0

/// Terminates the program in an abnormal fashion.
NSTDAPI void nstd_core_abort();

/// Asserts that `b` is true, aborts if `b` is false.
/// Parameters:
///     `const NSTDBool b` - The boolean.
NSTDAPI void nstd_core_assert(const NSTDBool b);

#ifdef NSTDCPP
}
#endif
#endif
