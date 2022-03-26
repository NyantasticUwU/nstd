#ifndef NSTD_AUDIO_STREAM_H_INCLUDED
#define NSTD_AUDIO_STREAM_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
NSTDCPPSTART

/// Represents an audio stream.
typedef NSTDAny NSTDAudioStream;

/// Plays an audio stream.
///
/// # Parameters
///
/// - `const NSTDAudioStream stream` - The audio stream.
/// Returns `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_audio_stream_play(const NSTDAudioStream stream);

/// Pauses an audio stream.
///
/// # Parameters
///
/// - `const NSTDAudioStream stream` - The audio stream.
/// Returns `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_audio_stream_pause(const NSTDAudioStream stream);

/// Frees an audio stream
///
/// # Parameters
///
/// - `NSTDAudioStream *const stream` - Pointer to an audio stream.
NSTDAPI void nstd_audio_stream_free(NSTDAudioStream *const stream);

NSTDCPPEND
#endif
