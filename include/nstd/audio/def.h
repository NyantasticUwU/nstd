#ifndef NSTD_AUDIO_DEF_H_INCLUDED
#define NSTD_AUDIO_DEF_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"

/// Represents an audio sample format.
typedef enum
{
    /// Signed 16-bit integer.
    NSTD_AUDIO_SAMPLE_FORMAT_INT16,
    /// Unsigned 16-bit integer.
    NSTD_AUDIO_SAMPLE_FORMAT_UINT16,
    /// 32-bit float.
    NSTD_AUDIO_SAMPLE_FORMAT_FLOAT32
} NSTDAudioSampleFormat;

/// Represents a stream config.
///
/// # Note
///
/// `buffer_size` will be set to 0 for default.
typedef struct
{
    /// The number of channels this audio stream owns.
    NSTDUInt16 channels;
    /// The sample rate of the audio stream.
    NSTDUInt32 sample_rate;
    /// The size of the sample buffer.
    NSTDUInt32 buffer_size;
    /// The data format of each sample.
    NSTDAudioSampleFormat format;
} NSTDAudioStreamConfig;

#endif
