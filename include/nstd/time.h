#ifndef NSTD_TIME_H_INCLUDED
#define NSTD_TIME_H_INCLUDED
#include "core/def.h"
#include "nstd.h"
NSTDCPPSTART

/// Represents a datetime object.
typedef struct
{
    /// The year.
    NSTDInt32 year;
    /// The month.
    NSTDUInt32 month;
    /// The day.
    NSTDUInt32 day;
    /// The hour.
    NSTDUInt32 hour;
    /// The minute.
    NSTDUInt32 minute;
    /// The second.
    NSTDUInt32 second;
    /// The nanosecond.
    NSTDUInt32 nanosecond;
} NSTDDateTime;

/// Gets the time in seconds since the unix epoch.
///
/// # Returns
///
/// `NSTDFloat64 time` - Number of seconds since unix epoch.
NSTDAPI NSTDFloat64 nstd_time_time();

/// Gets an `NSTDDateTime` object representing the local time it was created.
///
/// # Returns
///
/// `NSTDDateTime now` - Now represented as a `NSTDDateTime` object.
NSTDAPI NSTDDateTime nstd_time_now();

/// Gets an `NSTDDateTime` object representing the UTC time it was created.
///
/// # Returns
///
/// `NSTDDateTime now` - UTC now represented as a `NSTDDateTime` object.
NSTDAPI NSTDDateTime nstd_time_utc_now();

NSTDCPPEND
#endif
