//! An audio player.
use rodio::{OutputStream, OutputStreamHandle};

/// Represents an audio play stream.
#[repr(C)]
pub struct NSTDAudioPlayer {
    /// The output stream.
    pub stream: *mut OutputStream,
    /// A handle to the output stream.
    pub handle: *mut OutputStreamHandle,
}
impl Default for NSTDAudioPlayer {
    fn default() -> Self {
        Self {
            stream: std::ptr::null_mut(),
            handle: std::ptr::null_mut(),
        }
    }
}

/// Creates a play stream.
///
/// # Returns
///
/// `NSTDAudioPlayer stream` - The new play stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_player_new() -> NSTDAudioPlayer {
    match OutputStream::try_default() {
        Ok((stream, handle)) => NSTDAudioPlayer {
            stream: Box::into_raw(Box::new(stream)),
            handle: Box::into_raw(Box::new(handle)),
        },
        _ => NSTDAudioPlayer::default(),
    }
}

/// Frees a play stream.
///
/// # Parameters
///
/// - `NSTDAudioPlayer *const stream` - The play stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_player_free(stream: &mut NSTDAudioPlayer) {
    Box::from_raw(stream.stream);
    Box::from_raw(stream.handle);
    stream.stream = std::ptr::null_mut();
    stream.handle = std::ptr::null_mut();
}
