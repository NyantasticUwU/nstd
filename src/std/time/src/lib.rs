use chrono::prelude::*;
use std::{
    os::raw::{c_double, c_int, c_uint},
    time::{SystemTime, UNIX_EPOCH},
};

/// Represents a datetime object.
#[repr(C)]
pub struct NSTDDateTime {
    pub year: c_int,
    pub month: c_uint,
    pub day: c_uint,
    pub hour: c_uint,
    pub minute: c_uint,
    pub second: c_uint,
    pub nanosecond: c_uint,
}
impl NSTDDateTime {
    /// Creates a new `NSTDDateTime` object from a chrono `Datelike` + `Timelike` object.
    #[inline]
    fn new<DT: Datelike + Timelike>(dt: DT) -> NSTDDateTime {
        NSTDDateTime {
            year: dt.year() as c_int,
            month: dt.month() as c_uint,
            day: dt.day() as c_uint,
            hour: dt.hour() as c_uint,
            minute: dt.minute() as c_uint,
            second: dt.second() as c_uint,
            nanosecond: dt.nanosecond() as c_uint,
        }
    }
}

/// Gets the time in seconds since the unix epoch.
/// Returns: `double time` - Number of seconds since unix epoch.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_time_time() -> c_double {
    let sysnow = SystemTime::now();
    match sysnow.duration_since(UNIX_EPOCH) {
        Ok(dur) => dur.as_secs_f64(),
        Err(_) => 0.0,
    }
}

/// Gets an `NSTDDateTime` object representing the local time it was created.
/// Returns: `NSTDDateTime now` - Now represented as a `NSTDDateTime` object.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_time_now() -> NSTDDateTime {
    let local = Local::now();
    NSTDDateTime::new(local)
}

/// Gets an `NSTDDateTime` object representing the UTC time it was created.
/// Returns: `NSTDDateTime now` - Now represented as a `NSTDDateTime` object.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_time_utc_now() -> NSTDDateTime {
    let utc = Utc::now();
    NSTDDateTime::new(utc)
}
