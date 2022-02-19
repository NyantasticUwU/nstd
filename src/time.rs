use chrono::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a datetime object.
#[repr(C)]
pub struct NSTDDateTime {
    /// The year.
    pub year: i32,
    /// The month.
    pub month: u32,
    /// The day.
    pub day: u32,
    /// The hour.
    pub hour: u32,
    /// The minute.
    pub minute: u32,
    /// The second.
    pub second: u32,
    /// The nanosecond.
    pub nanosecond: u32,
}
impl NSTDDateTime {
    /// Creates a new `NSTDDateTime` object from a chrono `Datelike` + `Timelike` object.
    #[inline]
    fn new<DT: Datelike + Timelike>(dt: DT) -> NSTDDateTime {
        NSTDDateTime {
            year: dt.year(),
            month: dt.month(),
            day: dt.day(),
            hour: dt.hour(),
            minute: dt.minute(),
            second: dt.second(),
            nanosecond: dt.nanosecond(),
        }
    }
}

/// Gets the time in seconds since the unix epoch.
/// Returns: `NSTDFloat64 time` - Number of seconds since unix epoch.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_time_time() -> f64 {
    let sysnow = SystemTime::now();
    if let Ok(dur) = sysnow.duration_since(UNIX_EPOCH) {
        return dur.as_secs_f64();
    }
    0.0
}

/// Gets an `NSTDDateTime` object representing the local time it was created.
/// Returns: `NSTDDateTime now` - Now represented as a `NSTDDateTime` object.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_time_now() -> NSTDDateTime {
    let local = Local::now();
    NSTDDateTime::new(local)
}

/// Gets an `NSTDDateTime` object representing the UTC time it was created.
/// Returns: `NSTDDateTime now` - Now represented as a `NSTDDateTime` object.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_time_utc_now() -> NSTDDateTime {
    let utc = Utc::now();
    NSTDDateTime::new(utc)
}
