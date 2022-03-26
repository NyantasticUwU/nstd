//! An audio I/O stream.
use crate::core::def::NSTDErrorCode;
use cpal::{traits::*, Stream};

/// Represents an audio stream.
pub type NSTDAudioStream = *mut Stream;

/// Generates `nstd_audio_stream_*` functions.
macro_rules! generate_stream_play_pause {
    ($name: ident, $method: ident) => {
        ///
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(stream: NSTDAudioStream) -> NSTDErrorCode {
            if (*stream).$method().is_ok() {
                return 0;
            }
            1
        }
    };
}
generate_stream_play_pause!(nstd_audio_stream_play, play);
generate_stream_play_pause!(nstd_audio_stream_pause, pause);

/// Frees an audio stream
///
/// # Parameters
///
/// - `NSTDAudioStream *const stream` - Pointer to an audio stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_stream_free(stream: *mut NSTDAudioStream) {
    Box::from_raw(*stream);
    *stream = std::ptr::null_mut();
}
