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
