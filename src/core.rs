pub mod char_types;
pub mod def;
pub mod float_types;
pub mod int_types;
pub mod math;
pub mod platform;
pub mod pointer;
pub mod range;
pub mod slice;
pub mod str;
use self::def::NSTDBool;

/// Terminates the program in an abnormal fashion.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_abort() -> ! {
    panic!();
}

/// Asserts that `b` is true, aborts if `b` is false.
/// Parameters:
///     `const NSTDBool b` - The boolean.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_assert(b: NSTDBool) {
    if b == NSTDBool::NSTD_BOOL_FALSE {
        nstd_core_abort();
    }
}
