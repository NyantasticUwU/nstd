//! A queue for audio output.
use crate::{
    audio::player::NSTDAudioPlayer,
    core::def::{NSTDBool, NSTDErrorCode},
    fs::file::NSTDFile,
};
use rodio::{Decoder, Sink};
use std::io::BufReader;

/// Represents an audio queue.
pub type NSTDAudioQueue = *mut Sink;

/// Creates a new audio queue.
///
/// # Parameters
///
/// - `const NSTDAudioPlayer *const stream` - The stream to create the queue on.
///
/// # Returns
///
/// `NSTDAudioQueue queue` - The new audio queue.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_queue_new(stream: &NSTDAudioPlayer) -> NSTDAudioQueue {
    if let Ok(queue) = Sink::try_new(&*stream.handle) {
        return Box::into_raw(Box::new(queue));
    }
    std::ptr::null_mut()
}

/// Appends audio to a queue from a file.
///
/// # Parameters
///
/// - `const NSTDAudioQueue queue` - The audio queue.
///
/// - `const NSTDFile *const file` - The audio file.
///
/// - `const NSTDBool should_loop` - Nonzero if the audio should be looped.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_queue_append_from_file(
    queue: NSTDAudioQueue,
    file: &NSTDFile,
    should_loop: NSTDBool,
) -> NSTDErrorCode {
    let buf = BufReader::new((&*file.handle).get_ref());
    match should_loop {
        NSTDBool::NSTD_BOOL_FALSE => {
            if let Ok(decoder) = Decoder::new(buf) {
                (*queue).append(decoder);
                return 0;
            }
        }
        _ => {
            if let Ok(decoder) = Decoder::new_looped(buf) {
                (*queue).append(decoder);
                return 0;
            }
        }
    }
    1
}

/// Plays an audio queue.
///
/// # Parameters
///
/// - `const NSTDAudioQueue queue` - The audio queue.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_queue_play(queue: NSTDAudioQueue) {
    (*queue).play();
}

/// Pauses an audio queue.
///
/// # Parameters
///
/// - `const NSTDAudioQueue queue` - The audio queue.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_queue_pause(queue: NSTDAudioQueue) {
    (*queue).pause();
}

/// Checks if an audio queue is paused.
///
/// # Parameters
///
/// - `const NSTDAudioQueue queue` - The audio queue.
///
/// # Returns
///
/// `NSTDBool is_paused` - Whether or not the audio queue is paused.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_queue_is_paused(queue: NSTDAudioQueue) -> NSTDBool {
    NSTDBool::from((*queue).is_paused())
}

/// Stops audio playback for a queue by clearing it's queue.
///
/// # Parameters
///
/// - `const NSTDAudioQueue queue` - The audio queue.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_queue_stop(queue: NSTDAudioQueue) {
    (*queue).stop();
}

/// Sleeps the current thread until all sounds in the queue are done playing.
///
/// # Parameters
///
/// - `const NSTDAudioQueue queue` - The audio queue.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_queue_sleep_until_end(queue: NSTDAudioQueue) {
    (*queue).sleep_until_end();
}

/// Returns the volume of the audio queue.
///
/// # Parameters
///
/// - `const NSTDAudioQueue queue` - The audio queue.
///
/// # Returns
///
/// `NSTDFloat32 volume` - The volume of the queue.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_queue_get_volume(queue: NSTDAudioQueue) -> f32 {
    (*queue).volume()
}

/// Sets the volume of the audio queue.
///
/// # Parameters
///
/// - `const NSTDAudioQueue queue` - The audio queue.
///
/// - `const NSTDFloat32 volume` - The volume of the queue.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_queue_set_volume(queue: NSTDAudioQueue, volume: f32) {
    (*queue).set_volume(volume);
}

/// Gets the number of audio sources currently in a queue.
///
/// # Parameters
///
/// - `const NSTDAudioQueue queue` - The audio queue.
///
/// # Returns
///
/// `NSTDUSize size` - The number of audio sources in an audio queue.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_queue_length(queue: NSTDAudioQueue) -> usize {
    (*queue).len()
}

/// Detaches a queue from it's thread while freeing its memory.
///
/// # Parameters
///
/// - `NSTDAudioQueue *const queue` - The audio queue.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_queue_detach(queue: &mut NSTDAudioQueue) {
    let boxed_queue = Box::from_raw(*queue);
    boxed_queue.detach();
    *queue = std::ptr::null_mut();
}

/// Frees an audio queue.
///
/// # Parameters
///
/// - `NSTDAudioQueue *const queue` - The audio queue.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_queue_free(queue: &mut NSTDAudioQueue) {
    Box::from_raw(*queue);
    *queue = std::ptr::null_mut();
}
