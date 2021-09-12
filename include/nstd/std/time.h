#ifndef NSTD_STD_TIME_H_INCLUDED
#define NSTD_STD_TIME_H_INCLUDED
#include "../core/def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Gets the time in seconds since the unix epoch.
/// Returns: `double time` - Number of seconds since unix epoch.
NSTDAPI double nstd_std_time_time();

#ifdef __cplusplus
}
#endif
#endif
