#ifndef NSTD_CORE_ARCH_H_INCLUDED
#define NSTD_CORE_ARCH_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Returns the size (in bytes) of a pointer.
/// Returns: `NSTDCOREUSize size` - Size of a pointer.
NSTDAPI NSTDCOREUSize nstd_core_arch_ptr_size();

#ifdef __cplusplus
}
#endif
#endif
