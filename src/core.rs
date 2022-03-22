//! Provides core functionality for `nstd`.
//!
//! This module is responsible for providing common to use types & functions to be used throughout
//! the crate.
//!
//! This module also supports embedded devices. It makes no use of `std` and does not require the
//! `std` feature flag.
pub mod char_types;
pub mod cstr;
pub mod def;
pub mod float_types;
pub mod int_types;
pub mod math;
pub mod platform;
pub mod pointer;
pub mod range;
pub mod slice;
pub mod str;
use self::def::{NSTDAny, NSTDBool};

/// A null pointer value.
pub const NSTD_CORE_NULL: NSTDAny = core::ptr::null_mut();

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
