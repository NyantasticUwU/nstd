#ifndef NSTD_AUDIO_QUEUE_H_INCLUDED
#define NSTD_AUDIO_QUEUE_H_INCLUDED
#include "../core/def.h"
#include "../fs/file.h"
#include "../nstd.h"
#include "player.h"
NSTDCPPSTART

/// Represents an audio queue.
typedef NSTDAny NSTDAudioQueue;

/// Creates a new audio queue.
///
/// # Parameters
///
/// - `const NSTDAudioPlayer *const stream` - The stream to create the queue on.
///
/// # Returns
///
/// `NSTDAudioQueue queue` - The new audio queue.
NSTDAPI NSTDAudioQueue nstd_audio_queue_new(const NSTDAudioPlayer *const stream);

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
NSTDAPI NSTDErrorCode nstd_audio_queue_append_from_file(
    const NSTDAudioQueue queue,
    const NSTDFile *const file,
    const NSTDBool should_loop);

/// Plays an audio queue.
///
/// # Parameters
///
/// - `const NSTDAudioQueue queue` - The audio queue.
NSTDAPI void nstd_audio_queue_play(const NSTDAudioQueue queue);

/// Pauses an audio queue.
///
/// # Parameters
///
/// - `const NSTDAudioQueue queue` - The audio queue.
NSTDAPI void nstd_audio_queue_pause(const NSTDAudioQueue queue);

/// Checks if an audio queue is paused.
///
/// # Parameters
///
/// - `const NSTDAudioQueue queue` - The audio queue.
///
/// # Returns
///
/// `NSTDBool is_paused` - Whether or not the audio queue is paused.
NSTDAPI NSTDBool nstd_audio_queue_is_paused(const NSTDAudioQueue queue);

/// Stops audio playback for a queue by clearing it's queue.
///
/// # Parameters
///
/// - `const NSTDAudioQueue queue` - The audio queue.
NSTDAPI void nstd_audio_queue_stop(const NSTDAudioQueue queue);

/// Sleeps the current thread until all sounds in the queue are done playing.
///
/// # Parameters
///
/// - `const NSTDAudioQueue queue` - The audio queue.
NSTDAPI void nstd_audio_queue_sleep_until_end(const NSTDAudioQueue queue);

/// Returns the volume of the audio queue.
///
/// # Parameters
///
/// - `const NSTDAudioQueue queue` - The audio queue.
///
/// # Returns
///
/// `NSTDFloat32 volume` - The volume of the queue.
NSTDAPI NSTDFloat32 nstd_audio_queue_get_volume(const NSTDAudioQueue queue);

/// Sets the volume of the audio queue.
///
/// # Parameters
///
/// - `const NSTDAudioQueue queue` - The audio queue.
///
/// - `const NSTDFloat32 volume` - The volume of the queue.
NSTDAPI void nstd_audio_queue_set_volume(const NSTDAudioQueue queue, const NSTDFloat32 volume);

/// Gets the number of audio sources currently in a queue.
///
/// # Parameters
///
/// - `const NSTDAudioQueue queue` - The audio queue.
///
/// # Returns
///
/// `NSTDUSize size` - The number of audio sources in an audio queue.
NSTDAPI NSTDUSize nstd_audio_queue_length(const NSTDAudioQueue queue);

/// Detaches a queue from it's thread while freeing its memory.
///
/// # Parameters
///
/// - `NSTDAudioQueue *const queue` - The audio queue.
NSTDAPI void nstd_audio_queue_detach(NSTDAudioQueue *const queue);

/// Frees an audio queue.
///
/// # Parameters
///
/// - `NSTDAudioQueue *const queue` - The audio queue.
NSTDAPI void nstd_audio_queue_free(NSTDAudioQueue *const queue);

NSTDCPPEND
#endif
