use crate::def::NSTDBool;

/// Terminates the program in an abnormal fashion.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_sys_abort() -> ! {
    panic!();
}

/// Asserts that `b` is true, aborts if `b` is false.
/// Parameters:
///     `const NSTDBool b` - The boolean.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_sys_assert(b: NSTDBool) {
    if b == NSTDBool::NSTD_BOOL_FALSE {
        nstd_core_sys_abort();
    }
}
