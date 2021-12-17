mod cp;
mod linux;
mod macos;
mod windows;
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub use crate::platform::cp::*;
#[cfg(target_os = "linux")]
pub use crate::platform::linux::*;
#[cfg(target_os = "macos")]
pub use crate::platform::macos::*;
#[cfg(target_os = "windows")]
pub use crate::platform::windows::*;
