mod cp;
mod windows;
#[cfg(not(target_os = "windows"))]
pub use crate::platform::cp::*;
#[cfg(target_os = "windows")]
pub use crate::platform::windows::*;
