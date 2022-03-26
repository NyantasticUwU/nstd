//! A queue for audio output.
use super::NSTDAudioPlayer;
use crate::{
    core::def::{NSTDBool, NSTDErrorCode},
    fs::file::NSTDFile,
};
use rodio::{Decoder, Sink};
use std::io::BufReader;

/// Represents an audio sink.
pub type NSTDAudioSink = *mut Sink;

/// Creates a new audio sink.
///
/// # Parameters
///
/// - `const NSTDAudioPlayer *const stream` - The stream to create the sink on.
///
/// # Returns
///
/// `NSTDAudioSink sink` - The new audio sink.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_sink_new(stream: &NSTDAudioPlayer) -> NSTDAudioSink {
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
