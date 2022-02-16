#ifndef NSTD_AUDIO_H_INCLUDED
#define NSTD_AUDIO_H_INCLUDED
#include "core/def.h"
#include "fs.h"
#include "nstd.h"
#include "string.h"
#ifdef __cplusplus
extern "C"
{
#endif

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
///     `NSTDAudioHost host` - The audio host.
/// Returns: `NSTDAudioDevice device` - The default input device.
NSTDAPI NSTDAudioDevice nstd_audio_host_default_input_device(NSTDAudioHost host);

/// Returns the default output device from a host.
/// Parameters:
///     `NSTDAudioHost host` - The audio host.
/// Returns: `NSTDAudioDevice device` - The default output device.
NSTDAPI NSTDAudioDevice nstd_audio_host_default_output_device(NSTDAudioHost host);

/// Frees a host's memory.
/// Parameters:
///     `NSTDAudioHost *host` - Pointer to an audio host.
NSTDAPI void nstd_audio_host_free(NSTDAudioHost *host);

/// Gets the name of a device.
/// Parameters:
///     `NSTDAudioDevice device` - The device.
/// Returns: `NSTDString name` - The device name.
NSTDAPI NSTDString nstd_audio_device_name(NSTDAudioDevice device);

/// Gets the default input config from the device.
/// Parameters:
///     `NSTDAudioDevice device` - The device.
///     `NSTDAudioStreamConfig *const config` - Returns as the default input config for the device.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_audio_device_default_input_config(
    NSTDAudioDevice device,
    NSTDAudioStreamConfig *const config);

/// Gets the default output config from the device.
/// Parameters:
///     `NSTDAudioDevice device` - The device.
///     `NSTDAudioStreamConfig *const config` - Returns as the default output config for the device.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_audio_device_default_output_config(
    NSTDAudioDevice device,
    NSTDAudioStreamConfig *const config);

/// Builds an input stream on a device.
/// Parameters:
///     `NSTDAudioDevice device` - The device.
///     `const NSTDAudioStreamConfig *const config` - The stream config.
///     `const NSTDAudioSampleFormat format` - The audio sample format.
///     `void(*callback)(NSTDAnyConst, NSTDUSize)` - The stream callback.
///     `void(*err_callback)()` - The stream error callback.
/// Returns: `NSTDAudioStream stream` - The audio stream.
NSTDAPI NSTDAudioStream nstd_audio_device_build_input_stream(
    NSTDAudioDevice device,
    const NSTDAudioStreamConfig *const config,
    const NSTDAudioSampleFormat format,
    void(*callback)(NSTDAnyConst, NSTDUSize),
    void(*err_callback)());

/// Builds an output stream on a device.
/// Parameters:
///     `NSTDAudioDevice device` - The device.
///     `const NSTDAudioStreamConfig *const config` - The stream config.
///     `const NSTDAudioSampleFormat format` - The audio sample format.
///     `void(*callback)(NSTDAny, NSTDUSize)` - The stream callback.
///     `void(*err_callback)()` - The stream error callback.
/// Returns: `NSTDAudioStream stream` - The audio stream.
NSTDAPI NSTDAudioStream nstd_audio_device_build_output_stream(
    NSTDAudioDevice device,
    const NSTDAudioStreamConfig *const config,
    const NSTDAudioSampleFormat format,
    void(*callback)(NSTDAny, NSTDUSize),
    void(*err_callback)());

/// Frees a device.
/// Parameters:
///     `NSTDAudioDevice *device` - Pointer to a device.
NSTDAPI void nstd_audio_device_free(NSTDAudioDevice *device);

/// Plays an audio stream.
/// Parameters:
///     `NSTDAudioStream stream` - The audio stream.
/// Returns `int errc` - Nonzero on error.
NSTDAPI int nstd_audio_stream_play(NSTDAudioStream stream);

/// Pauses an audio stream.
/// Parameters:
///     `NSTDAudioStream stream` - The audio stream.
/// Returns `int errc` - Nonzero on error.
NSTDAPI int nstd_audio_stream_pause(NSTDAudioStream stream);

/// Frees an audio stream
/// Parameters:
///     `NSTDAudioStream *stream` - Pointer to an audio stream.
NSTDAPI void nstd_audio_stream_free(NSTDAudioStream *stream);

/// Creates a play stream.
/// Returns: `NSTDAudioPlayStream stream` - The new play stream.
NSTDAPI NSTDAudioPlayStream nstd_audio_play_stream_new();

/// Frees a play stream.
/// Parameters:
///     `NSTDAudioPlayStream *stream` - The play stream.
NSTDAPI void nstd_audio_play_stream_free(NSTDAudioPlayStream *stream);

/// Creates a new audio sink.
/// Parameters:
///     `const NSTDAudioPlayStream *const stream` - The stream to create the sink on.
/// Returns: `NSTDAudioSink sink` - The new audio sink.
NSTDAPI NSTDAudioSink nstd_audio_sink_new(const NSTDAudioPlayStream *const stream);

/// Appends audio to a sink from a file.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
///     `const NSTDFile *const file` - The audio file.
///     `const NSTDBool should_loop` - Nonzero if the audio should be looped.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_audio_sink_append_from_file(
    NSTDAudioSink sink,
    const NSTDFile *const file,
    const NSTDBool should_loop);

/// Plays an audio sink.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
NSTDAPI void nstd_audio_sink_play(NSTDAudioSink sink);

/// Pauses an audio sink.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
NSTDAPI void nstd_audio_sink_pause(NSTDAudioSink sink);

/// Checks if an audio sink is paused.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
/// Returns: `NSTDBool is_paused` - Whether or not the audio sink is paused.
NSTDAPI NSTDBool nstd_audio_sink_is_paused(NSTDAudioSink sink);

/// Stops audio playback for a sink by clearing it's queue.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
NSTDAPI void nstd_audio_sink_stop(NSTDAudioSink sink);

/// Sleeps the current thread until all sounds in the sink are done playing.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
NSTDAPI void nstd_audio_sink_sleep_until_end(NSTDAudioSink sink);

/// Returns the volume of the audio sink.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
/// Returns: `NSTDFloat32 volume` - The volume of the sink.
NSTDAPI NSTDFloat32 nstd_audio_sink_get_volume(NSTDAudioSink sink);

/// Sets the volume of the audio sink.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
///     `const NSTDFloat32 volume` - The volume of the sink.
NSTDAPI void nstd_audio_sink_set_volume(NSTDAudioSink sink, const NSTDFloat32 volume);

/// Gets the number of audio sources currently in a sink.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
/// Returns: `NSTDUSize size` - The number of audio sources in an audio sink.
NSTDAPI NSTDUSize nstd_audio_sink_length(NSTDAudioSink sink);

/// Detaches a sink from it's thread while freeing its memory.
/// Parameters:
///     `NSTDAudioSink *sink` - The audio sink.
NSTDAPI void nstd_audio_sink_detach(NSTDAudioSink *sink);

/// Frees an audio sink.
/// Parameters:
///     `NSTDAudioSink *sink` - The audio sink.
NSTDAPI void nstd_audio_sink_free(NSTDAudioSink *sink);

#ifdef __cplusplus
}
#endif
#endif
