#ifndef NSTD_SYS_H_INCLUDED
#define NSTD_SYS_H_INCLUDED
#include "nstd.h"
#include "string.h"
#ifdef NSTDCPP
extern "C"
{
#endif

/// Returns a string describing the specific operating system in use.
/// Returns: `NSTDString os_name` - The os's name as a string.
NSTDAPI NSTDString nstd_sys_os_name();

/// Returns a string describing the specific cpu architecture in use.
/// Returns: `NSTDString arch_name` - The cpu architecture's name as a string.
NSTDAPI NSTDString nstd_sys_arch_name();

#ifdef NSTDCPP
}
#endif
#endif
