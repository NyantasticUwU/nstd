use crate::string::NSTDString;
use std::env::consts::{ARCH, OS};

/// Returns a string describing the specific operating system in use.
/// Returns: `NSTDString os_name` - The os's name as a string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_sys_os_name() -> NSTDString {
    NSTDString::from(OS.as_bytes())
}

/// Returns a string describing the specific cpu architecture in use.
/// Returns: `NSTDString arch_name` - The cpu architecture's name as a string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_sys_arch_name() -> NSTDString {
    NSTDString::from(ARCH.as_bytes())
}
