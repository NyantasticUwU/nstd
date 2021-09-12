use std::{
    os::raw::c_double,
    time::{SystemTime, UNIX_EPOCH},
};

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
