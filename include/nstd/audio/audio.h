#ifndef NSTD_AUDIO_AUDIO_H_INCLUDED
#define NSTD_AUDIO_AUDIO_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
NSTDCPPSTART

/// Represents an audio play stream.
typedef struct
{
    /// The output stream.
    NSTDAny stream;
    /// A handle to the output stream.
    NSTDAny handle;
} NSTDAudioPlayer;

/// Creates a play stream.
///
/// # Returns
///
/// `NSTDAudioPlayer stream` - The new play stream.
NSTDAPI NSTDAudioPlayer nstd_audio_player_new();

/// Frees a play stream.
///
/// # Parameters
///
/// - `NSTDAudioPlayer *const stream` - The play stream.
NSTDAPI void nstd_audio_player_free(NSTDAudioPlayer *const stream);

NSTDCPPEND
#endif
