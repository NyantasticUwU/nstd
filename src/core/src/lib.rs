#![cfg_attr(not(any(test, feature = "std")), no_std)]
pub mod arch;
pub mod char_types;
pub mod def;
pub mod float_types;
pub mod int_types;
pub mod math;
pub mod platform;
pub mod pointer;
pub mod slice;
pub mod str;
#[cfg(feature = "deps")]
pub mod deps {
    pub use cty;
    pub use platforms;
}
use crate::def::NSTDBool;

/// Terminates the program in an abnormal fashion.
/// NOTE: This will only abort if the `panics` feature is enabled when compiling. Also note that it
/// is enabled by default.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_abort() -> ! {
    #[cfg(feature = "panics")]
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
