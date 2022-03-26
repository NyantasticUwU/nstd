//! Audio I/O.
pub mod def;
pub mod device;
pub mod host;
use crate::{
    core::def::{NSTDBool, NSTDErrorCode},
    fs::file::NSTDFile,
};
use cpal::{traits::*, Stream};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io::BufReader;

/// Represents an audio stream.
pub type NSTDAudioStream = *mut Stream;

/// Represents an audio play stream.
#[repr(C)]
pub struct NSTDAudioPlayStream {
    /// The output stream.
    pub stream: *mut OutputStream,
    /// A handle to the output stream.
    pub handle: *mut OutputStreamHandle,
}
impl Default for NSTDAudioPlayStream {
    fn default() -> Self {
        Self {
            stream: std::ptr::null_mut(),
            handle: std::ptr::null_mut(),
        }
    }
}

/// Represents an audio sink.
pub type NSTDAudioSink = *mut Sink;

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

/// Creates a play stream.
///
/// # Returns
///
/// `NSTDAudioPlayStream stream` - The new play stream.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_play_stream_new() -> NSTDAudioPlayStream {
    match OutputStream::try_default() {
        Ok((stream, handle)) => NSTDAudioPlayStream {
            stream: Box::into_raw(Box::new(stream)),
            handle: Box::into_raw(Box::new(handle)),
        },
        _ => NSTDAudioPlayStream::default(),
    }
}

/// Frees a play stream.
///
/// # Parameters
///
/// - `NSTDAudioPlayStream *const stream` - The play stream.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_play_stream_free(stream: &mut NSTDAudioPlayStream) {
    Box::from_raw(stream.stream);
    Box::from_raw(stream.handle);
    stream.stream = std::ptr::null_mut();
    stream.handle = std::ptr::null_mut();
}

/// Creates a new audio sink.
///
/// # Parameters
///
/// - `const NSTDAudioPlayStream *const stream` - The stream to create the sink on.
///
/// # Returns
///
/// `NSTDAudioSink sink` - The new audio sink.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_sink_new(stream: &NSTDAudioPlayStream) -> NSTDAudioSink {
    if let Ok(sink) = Sink::try_new(&*stream.handle) {
        return Box::into_raw(Box::new(sink));
    }
    std::ptr::null_mut()
}

/// Appends audio to a sink from a file.
///
/// # Parameters
///
/// - `const NSTDAudioSink sink` - The audio sink.
///
/// - `const NSTDFile *const file` - The audio file.
///
/// - `const NSTDBool should_loop` - Nonzero if the audio should be looped.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_sink_append_from_file(
    sink: NSTDAudioSink,
    file: &NSTDFile,
    should_loop: NSTDBool,
) -> NSTDErrorCode {
    let buf = BufReader::new((&*file.handle).get_ref());
    match should_loop {
        NSTDBool::NSTD_BOOL_FALSE => {
            if let Ok(decoder) = Decoder::new(buf) {
                (*sink).append(decoder);
                return 0;
            }
        }
        _ => {
            if let Ok(decoder) = Decoder::new_looped(buf) {
                (*sink).append(decoder);
                return 0;
            }
        }
    }
    1
}

/// Plays an audio sink.
///
/// # Parameters
///
/// - `const NSTDAudioSink sink` - The audio sink.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_sink_play(sink: NSTDAudioSink) {
    (*sink).play();
}

/// Pauses an audio sink.
///
/// # Parameters
///
/// - `const NSTDAudioSink sink` - The audio sink.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_sink_pause(sink: NSTDAudioSink) {
    (*sink).pause();
}

/// Checks if an audio sink is paused.
///
/// # Parameters
///
/// - `const NSTDAudioSink sink` - The audio sink.
///
/// # Returns
///
/// `NSTDBool is_paused` - Whether or not the audio sink is paused.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_sink_is_paused(sink: NSTDAudioSink) -> NSTDBool {
    NSTDBool::from((*sink).is_paused())
}

/// Stops audio playback for a sink by clearing it's queue.
///
/// # Parameters
///
/// - `const NSTDAudioSink sink` - The audio sink.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_sink_stop(sink: NSTDAudioSink) {
    (*sink).stop();
}

/// Sleeps the current thread until all sounds in the sink are done playing.
///
/// # Parameters
///
/// - `const NSTDAudioSink sink` - The audio sink.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_sink_sleep_until_end(sink: NSTDAudioSink) {
    (*sink).sleep_until_end();
}

/// Returns the volume of the audio sink.
///
/// # Parameters
///
/// - `const NSTDAudioSink sink` - The audio sink.
///
/// # Returns
///
/// `NSTDFloat32 volume` - The volume of the sink.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_sink_get_volume(sink: NSTDAudioSink) -> f32 {
    (*sink).volume()
}

/// Sets the volume of the audio sink.
///
/// # Parameters
///
/// - `const NSTDAudioSink sink` - The audio sink.
///
/// - `const NSTDFloat32 volume` - The volume of the sink.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_sink_set_volume(sink: NSTDAudioSink, volume: f32) {
    (*sink).set_volume(volume);
}

/// Gets the number of audio sources currently in a sink.
///
/// # Parameters
///
/// - `const NSTDAudioSink sink` - The audio sink.
///
/// # Returns
///
/// `NSTDUSize size` - The number of audio sources in an audio sink.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_sink_length(sink: NSTDAudioSink) -> usize {
    (*sink).len()
}

/// Detaches a sink from it's thread while freeing its memory.
///
/// # Parameters
///
/// - `NSTDAudioSink *const sink` - The audio sink.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_sink_detach(sink: &mut NSTDAudioSink) {
    let boxed_sink = Box::from_raw(*sink);
    boxed_sink.detach();
    *sink = std::ptr::null_mut();
}

/// Frees an audio sink.
///
/// # Parameters
///
/// - `NSTDAudioSink *const sink` - The audio sink.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_sink_free(sink: &mut NSTDAudioSink) {
    Box::from_raw(*sink);
    *sink = std::ptr::null_mut();
}
