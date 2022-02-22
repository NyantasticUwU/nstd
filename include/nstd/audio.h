#ifndef NSTD_AUDIO_H_INCLUDED
#define NSTD_AUDIO_H_INCLUDED
#include "core/def.h"
#include "fs.h"
#include "nstd.h"
#include "string.h"
NSTDCPPSTART

/// Represents an audio host.
typedef NSTDAny NSTDAudioHost;

/// Represents an audio device.
typedef NSTDAny NSTDAudioDevice;

/// Represents an audio stream.
typedef NSTDAny NSTDAudioStream;

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
/// NOTE: `buffer_size` will be set to 0 for default.
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

/// Represents an audio play stream.
typedef struct
{
    /// The output stream.
    NSTDAny stream;
    /// A handle to the output stream.
    NSTDAny handle;
} NSTDAudioPlayStream;

/// Represents an audio sink.
typedef NSTDAny NSTDAudioSink;

/// Gets the default audio host.
/// Returns: `NSTDAudioHost host` - The default audio host.
NSTDAPI NSTDAudioHost nstd_audio_host_default();

/// Returns the default input device from a host.
/// Parameters:
///     `const NSTDAudioHost host` - The audio host.
/// Returns: `NSTDAudioDevice device` - The default input device.
NSTDAPI NSTDAudioDevice nstd_audio_host_default_input_device(const NSTDAudioHost host);

/// Returns the default output device from a host.
/// Parameters:
///     `const NSTDAudioHost host` - The audio host.
/// Returns: `NSTDAudioDevice device` - The default output device.
NSTDAPI NSTDAudioDevice nstd_audio_host_default_output_device(const NSTDAudioHost host);

/// Frees a host's memory.
/// Parameters:
///     `NSTDAudioHost *const host` - Pointer to an audio host.
NSTDAPI void nstd_audio_host_free(NSTDAudioHost *const host);

/// Gets the name of a device.
/// Parameters:
///     `const NSTDAudioDevice device` - The device.
/// Returns: `NSTDString name` - The device name.
NSTDAPI NSTDString nstd_audio_device_name(const NSTDAudioDevice device);

/// Gets the default input config from the device.
/// Parameters:
///     `const NSTDAudioDevice device` - The device.
///     `NSTDAudioStreamConfig *const config` - Returns as the default input config for the device.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_audio_device_default_input_config(
    const NSTDAudioDevice device,
    NSTDAudioStreamConfig *const config);

/// Gets the default output config from the device.
/// Parameters:
///     `const NSTDAudioDevice device` - The device.
///     `NSTDAudioStreamConfig *const config` - Returns as the default output config for the device.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_audio_device_default_output_config(
    const NSTDAudioDevice device,
    NSTDAudioStreamConfig *const config);

/// Builds an input stream on a device.
/// Parameters:
///     `const NSTDAudioDevice device` - The device.
///     `const NSTDAudioStreamConfig *const config` - The stream config.
///     `const NSTDAudioSampleFormat format` - The audio sample format.
///     `void (*callback)(NSTDAnyConst, NSTDUSize)` - The stream callback.
///     `void (*err_callback)()` - The stream error callback.
/// Returns: `NSTDAudioStream stream` - The audio stream.
NSTDAPI NSTDAudioStream nstd_audio_device_build_input_stream(
    const NSTDAudioDevice device,
    const NSTDAudioStreamConfig *const config,
    const NSTDAudioSampleFormat format,
    void (*callback)(NSTDAnyConst, NSTDUSize),
    void (*err_callback)());

/// Builds an output stream on a device.
/// Parameters:
///     `const NSTDAudioDevice device` - The device.
///     `const NSTDAudioStreamConfig *const config` - The stream config.
///     `const NSTDAudioSampleFormat format` - The audio sample format.
///     `void (*callback)(NSTDAny, NSTDUSize)` - The stream callback.
///     `void (*err_callback)()` - The stream error callback.
/// Returns: `NSTDAudioStream stream` - The audio stream.
NSTDAPI NSTDAudioStream nstd_audio_device_build_output_stream(
    const NSTDAudioDevice device,
    const NSTDAudioStreamConfig *const config,
    const NSTDAudioSampleFormat format,
    void (*callback)(NSTDAny, NSTDUSize),
    void (*err_callback)());

/// Frees a device.
/// Parameters:
///     `NSTDAudioDevice *const device` - Pointer to a device.
NSTDAPI void nstd_audio_device_free(NSTDAudioDevice *const device);

/// Plays an audio stream.
/// Parameters:
///     `const NSTDAudioStream stream` - The audio stream.
/// Returns `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_audio_stream_play(const NSTDAudioStream stream);

/// Pauses an audio stream.
/// Parameters:
///     `const NSTDAudioStream stream` - The audio stream.
/// Returns `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_audio_stream_pause(const NSTDAudioStream stream);

/// Frees an audio stream
/// Parameters:
///     `NSTDAudioStream *const stream` - Pointer to an audio stream.
NSTDAPI void nstd_audio_stream_free(NSTDAudioStream *const stream);

/// Creates a play stream.
/// Returns: `NSTDAudioPlayStream stream` - The new play stream.
NSTDAPI NSTDAudioPlayStream nstd_audio_play_stream_new();

/// Frees a play stream.
/// Parameters:
///     `NSTDAudioPlayStream *const stream` - The play stream.
NSTDAPI void nstd_audio_play_stream_free(NSTDAudioPlayStream *const stream);

/// Creates a new audio sink.
/// Parameters:
///     `const NSTDAudioPlayStream *const stream` - The stream to create the sink on.
/// Returns: `NSTDAudioSink sink` - The new audio sink.
NSTDAPI NSTDAudioSink nstd_audio_sink_new(const NSTDAudioPlayStream *const stream);

/// Appends audio to a sink from a file.
/// Parameters:
///     `const NSTDAudioSink sink` - The audio sink.
///     `const NSTDFile *const file` - The audio file.
///     `const NSTDBool should_loop` - Nonzero if the audio should be looped.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_audio_sink_append_from_file(
    const NSTDAudioSink sink,
    const NSTDFile *const file,
    const NSTDBool should_loop);

/// Plays an audio sink.
/// Parameters:
///     `const NSTDAudioSink sink` - The audio sink.
NSTDAPI void nstd_audio_sink_play(const NSTDAudioSink sink);

/// Pauses an audio sink.
/// Parameters:
///     `const NSTDAudioSink sink` - The audio sink.
NSTDAPI void nstd_audio_sink_pause(const NSTDAudioSink sink);

/// Checks if an audio sink is paused.
/// Parameters:
///     `const NSTDAudioSink sink` - The audio sink.
/// Returns: `NSTDBool is_paused` - Whether or not the audio sink is paused.
NSTDAPI NSTDBool nstd_audio_sink_is_paused(const NSTDAudioSink sink);

/// Stops audio playback for a sink by clearing it's queue.
/// Parameters:
///     `const NSTDAudioSink sink` - The audio sink.
NSTDAPI void nstd_audio_sink_stop(const NSTDAudioSink sink);

/// Sleeps the current thread until all sounds in the sink are done playing.
/// Parameters:
///     `const NSTDAudioSink sink` - The audio sink.
NSTDAPI void nstd_audio_sink_sleep_until_end(const NSTDAudioSink sink);

/// Returns the volume of the audio sink.
/// Parameters:
///     `const NSTDAudioSink sink` - The audio sink.
/// Returns: `NSTDFloat32 volume` - The volume of the sink.
NSTDAPI NSTDFloat32 nstd_audio_sink_get_volume(const NSTDAudioSink sink);

/// Sets the volume of the audio sink.
/// Parameters:
///     `const NSTDAudioSink sink` - The audio sink.
///     `const NSTDFloat32 volume` - The volume of the sink.
NSTDAPI void nstd_audio_sink_set_volume(const NSTDAudioSink sink, const NSTDFloat32 volume);

/// Gets the number of audio sources currently in a sink.
/// Parameters:
///     `const NSTDAudioSink sink` - The audio sink.
/// Returns: `NSTDUSize size` - The number of audio sources in an audio sink.
NSTDAPI NSTDUSize nstd_audio_sink_length(const NSTDAudioSink sink);

/// Detaches a sink from it's thread while freeing its memory.
/// Parameters:
///     `NSTDAudioSink *const sink` - The audio sink.
NSTDAPI void nstd_audio_sink_detach(NSTDAudioSink *const sink);

/// Frees an audio sink.
/// Parameters:
///     `NSTDAudioSink *const sink` - The audio sink.
NSTDAPI void nstd_audio_sink_free(NSTDAudioSink *const sink);

NSTDCPPEND
#endif
