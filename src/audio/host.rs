//! A platform's audio host.
use super::device::NSTDAudioDevice;
use cpal::{traits::*, Host};

/// Represents an audio host.
pub type NSTDAudioHost = *mut Host;

/// Gets the default audio host.
///
/// # Returns
///
/// `NSTDAudioHost host` - The default audio host.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_host_default() -> NSTDAudioHost {
    Box::into_raw(Box::new(cpal::default_host()))
}

/// Generates `nstd_audio_host_default_*_device` functions.
macro_rules! generate_default_device {
    ($name: ident, $method: ident) => {
        ///
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(host: NSTDAudioHost) -> NSTDAudioDevice {
            if let Some(device) = (*host).$method() {
                return Box::into_raw(Box::new(device));
            }
            std::ptr::null_mut()
        }
    };
}
generate_default_device!(nstd_audio_host_default_input_device, default_input_device);
generate_default_device!(nstd_audio_host_default_output_device, default_output_device);

/// Frees a host's memory.
///
/// # Parameters
///
/// - `NSTDAudioHost *const host` - Pointer to an audio host.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_host_free(host: *mut NSTDAudioHost) {
    Box::from_raw(*host);
    *host = std::ptr::null_mut();
}
