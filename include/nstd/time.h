#ifndef NSTD_TIME_H_INCLUDED
#define NSTD_TIME_H_INCLUDED
#include "core/def.h"
#include "nstd.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a datetime object.
typedef struct
{
    int year;
    unsigned int month;
    unsigned int day;
    unsigned int hour;
    unsigned int minute;
    unsigned int second;
    unsigned int nanosecond;
} NSTDDateTime;

/// Gets the time in seconds since the unix epoch.
/// Returns: `double time` - Number of seconds since unix epoch.
NSTDAPI double nstd_time_time();

/// Gets an `NSTDDateTime` object representing the local time it was created.
/// Returns: `NSTDDateTime now` - Now represented as a `NSTDDateTime` object.
NSTDAPI NSTDDateTime nstd_time_now();

/// Gets an `NSTDDateTime` object representing the UTC time it was created.
/// Returns: `NSTDDateTime now` - UTC now represented as a `NSTDDateTime` object.
NSTDAPI NSTDDateTime nstd_time_utc_now();

#ifdef __cplusplus
}
#endif
#endif
