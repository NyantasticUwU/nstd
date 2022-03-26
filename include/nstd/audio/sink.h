#ifndef NSTD_AUDIO_SINK_H_INCLUDED
#define NSTD_AUDIO_SINK_H_INCLUDED
#include "../core/def.h"
#include "../fs/file.h"
#include "../nstd.h"
#include "audio.h"
NSTDCPPSTART

/// Represents an audio sink.
typedef NSTDAny NSTDAudioSink;

/// Creates a new audio sink.
///
/// # Parameters
///
/// - `const NSTDAudioPlayer *const stream` - The stream to create the sink on.
///
/// # Returns
///
/// `NSTDAudioSink sink` - The new audio sink.
NSTDAPI NSTDAudioSink nstd_audio_sink_new(const NSTDAudioPlayer *const stream);

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
NSTDAPI NSTDErrorCode nstd_audio_sink_append_from_file(
    const NSTDAudioSink sink,
    const NSTDFile *const file,
    const NSTDBool should_loop);

/// Plays an audio sink.
///
/// # Parameters
///
/// - `const NSTDAudioSink sink` - The audio sink.
NSTDAPI void nstd_audio_sink_play(const NSTDAudioSink sink);

/// Pauses an audio sink.
///
/// # Parameters
///
/// - `const NSTDAudioSink sink` - The audio sink.
NSTDAPI void nstd_audio_sink_pause(const NSTDAudioSink sink);

/// Checks if an audio sink is paused.
///
/// # Parameters
///
/// - `const NSTDAudioSink sink` - The audio sink.
///
/// # Returns
///
/// `NSTDBool is_paused` - Whether or not the audio sink is paused.
NSTDAPI NSTDBool nstd_audio_sink_is_paused(const NSTDAudioSink sink);

/// Stops audio playback for a sink by clearing it's queue.
///
/// # Parameters
///
/// - `const NSTDAudioSink sink` - The audio sink.
NSTDAPI void nstd_audio_sink_stop(const NSTDAudioSink sink);

/// Sleeps the current thread until all sounds in the sink are done playing.
///
/// # Parameters
///
/// - `const NSTDAudioSink sink` - The audio sink.
NSTDAPI void nstd_audio_sink_sleep_until_end(const NSTDAudioSink sink);

/// Returns the volume of the audio sink.
///
/// # Parameters
///
/// - `const NSTDAudioSink sink` - The audio sink.
///
/// # Returns
///
/// `NSTDFloat32 volume` - The volume of the sink.
NSTDAPI NSTDFloat32 nstd_audio_sink_get_volume(const NSTDAudioSink sink);

/// Sets the volume of the audio sink.
///
/// # Parameters
///
/// - `const NSTDAudioSink sink` - The audio sink.
///
/// - `const NSTDFloat32 volume` - The volume of the sink.
NSTDAPI void nstd_audio_sink_set_volume(const NSTDAudioSink sink, const NSTDFloat32 volume);

/// Gets the number of audio sources currently in a sink.
///
/// # Parameters
///
/// - `const NSTDAudioSink sink` - The audio sink.
///
/// # Returns
///
/// `NSTDUSize size` - The number of audio sources in an audio sink.
NSTDAPI NSTDUSize nstd_audio_sink_length(const NSTDAudioSink sink);

/// Detaches a sink from it's thread while freeing its memory.
///
/// # Parameters
///
/// - `NSTDAudioSink *const sink` - The audio sink.
NSTDAPI void nstd_audio_sink_detach(NSTDAudioSink *const sink);

/// Frees an audio sink.
///
/// # Parameters
///
/// - `NSTDAudioSink *const sink` - The audio sink.
NSTDAPI void nstd_audio_sink_free(NSTDAudioSink *const sink);

NSTDCPPEND
#endif
