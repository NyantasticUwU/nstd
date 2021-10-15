#ifndef NSTD_STD_AUDIO_H_INCLUDED
#define NSTD_STD_AUDIO_H_INCLUDED
#include "../core/def.h"
#include "def.h"
#include "fs.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents an audio host.
typedef void *NSTDAudioHost;

/// Represents an audio device.
typedef void *NSTDAudioDevice;

/// Represents an audio stream.
typedef void *NSTDAudioStream;

/// Represents an audio sample format.
typedef enum
{
    NSTD_AUDIO_SAMPLE_FORMAT_INT16,
    NSTD_AUDIO_SAMPLE_FORMAT_UINT16,
    NSTD_AUDIO_SAMPLE_FORMAT_FLOAT32
} NSTDAudioSampleFormat;

/// Represents a stream config.
/// NOTE: `buffer_size` will be set to 0 for default.
typedef struct
{
    NSTDUInt16 channels;
    NSTDUInt32 sample_rate;
    NSTDUInt32 buffer_size;
    NSTDAudioSampleFormat format;
} NSTDAudioStreamConfig;

/// Represents an audio play stream.
typedef struct
{
    void *stream;
    void *handle;
} NSTDAudioPlayStream;

/// Represents an audio sink.
typedef void *NSTDAudioSink;

/// Gets the default audio host.
/// Returns: `NSTDAudioHost host` - The default audio host.
NSTDAPI NSTDAudioHost nstd_std_audio_host_default();

/// Returns the default input device from a host.
/// Parameters:
///     `NSTDAudioHost host` - The audio host.
/// Returns: `NSTDAudioDevice device` - The default input device.
NSTDAPI NSTDAudioDevice nstd_std_audio_host_default_input_device(NSTDAudioHost host);

/// Returns the default output device from a host.
/// Parameters:
///     `NSTDAudioHost host` - The audio host.
/// Returns: `NSTDAudioDevice device` - The default output device.
NSTDAPI NSTDAudioDevice nstd_std_audio_host_default_output_device(NSTDAudioHost host);

/// Frees a host's memory.
/// Parameters:
///     `NSTDAudioHost *host` - Pointer to an audio host.
NSTDAPI void nstd_std_audio_host_free(NSTDAudioHost *host);

/// Gets the name of a device.
/// Parameters:
///     `NSTDAudioDevice device` - The device.
/// Returns: `char *name` - The device name.
NSTDAPI char *nstd_std_audio_device_name(NSTDAudioDevice device);

/// Frees a device name.
/// Parameters:
///     `const char **name` - Pointer to a device name.
NSTDAPI void nstd_std_audio_device_free_name(const char **name);

/// Gets the default input config from the device.
/// Parameters:
///     `NSTDAudioDevice device` - The device.
///     `NSTDAudioStreamConfig *config` - Returns as the default input config for the device.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_audio_device_default_input_config(
    NSTDAudioDevice device,
    NSTDAudioStreamConfig *config);

/// Gets the default output config from the device.
/// Parameters:
///     `NSTDAudioDevice device` - The device.
///     `NSTDAudioStreamConfig *config` - Returns as the default output config for the device.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_audio_device_default_output_config(
    NSTDAudioDevice device,
    NSTDAudioStreamConfig *config);

/// Builds an input stream on a device.
/// Parameters:
///     `NSTDAudioDevice device` - The device.
///     `const NSTDAudioStreamConfig *const config` - The stream config.
///     `const NSTDAudioSampleFormat format` - The audio sample format.
///     `void(*callback)(const void *, NSTDSize)` - The stream callback.
///     `void(*err_callback)()` - The stream error callback.
/// Returns: `NSTDAudioStream stream` - The audio stream.
NSTDAPI NSTDAudioStream nstd_std_audio_device_build_input_stream(
    NSTDAudioDevice device,
    const NSTDAudioStreamConfig *const config,
    const NSTDAudioSampleFormat format,
    void(*callback)(const void *, NSTDSize),
    void(*err_callback)());

/// Builds an output stream on a device.
/// Parameters:
///     `NSTDAudioDevice device` - The device.
///     `const NSTDAudioStreamConfig *const config` - The stream config.
///     `const NSTDAudioSampleFormat format` - The audio sample format.
///     `void(*callback)(void *, NSTDSize)` - The stream callback.
///     `void(*err_callback)()` - The stream error callback.
/// Returns: `NSTDAudioStream stream` - The audio stream.
NSTDAPI NSTDAudioStream nstd_std_audio_device_build_output_stream(
    NSTDAudioDevice device,
    const NSTDAudioStreamConfig *const config,
    const NSTDAudioSampleFormat format,
    void(*callback)(void *, NSTDSize),
    void(*err_callback)());

/// Frees a device.
/// Parameters:
///     `NSTDAudioDevice *device` - Pointer to a device.
NSTDAPI void nstd_std_audio_device_free(NSTDAudioDevice *device);

/// Plays an audio stream.
/// Parameters:
///     `NSTDAudioStream stream` - The audio stream.
/// Returns `int errc` - Nonzero on error.
NSTDAPI int nstd_std_audio_stream_play(NSTDAudioStream stream);

/// Pauses an audio stream.
/// Parameters:
///     `NSTDAudioStream stream` - The audio stream.
/// Returns `int errc` - Nonzero on error.
NSTDAPI int nstd_std_audio_stream_pause(NSTDAudioStream stream);

/// Frees an audio stream
/// Parameters:
///     `NSTDAudioStream *stream` - Pointer to an audio stream.
NSTDAPI void nstd_std_audio_stream_free(NSTDAudioStream *stream);

/// Creates a play stream.
/// Returns: `NSTDAudioPlayStream stream` - The new play stream.
NSTDAPI NSTDAudioPlayStream nstd_std_audio_play_stream_new();

/// Frees a play stream.
/// Parameters:
///     `NSTDAudioPlayStream *stream` - The play stream.
NSTDAPI void nstd_std_audio_play_stream_free(NSTDAudioPlayStream *stream);

/// Creates a new audio sink.
/// Parameters:
///     `const NSTDAudioPlayStream *const stream` - The stream to create the sink on.
/// Returns: `NSTDAudioSink sink` - The new audio sink.
NSTDAPI NSTDAudioSink nstd_std_audio_sink_new(const NSTDAudioPlayStream *const stream);

/// Appends audio to a sink from a file.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
///     `NSTDFile file` - The audio file.
///     `const int should_loop` - Nonzero if the audio should be looped.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_audio_sink_append_from_file(
    NSTDAudioSink sink,
    NSTDFile file,
    const int should_loop);

/// Plays an audio sink.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
NSTDAPI void nstd_std_audio_sink_play(NSTDAudioSink sink);

/// Pauses an audio sink.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
NSTDAPI void nstd_std_audio_sink_pause(NSTDAudioSink sink);

/// Frees an audio sink.
/// Parameters:
///     `NSTDAudioSink *sink` - The audio sink.
NSTDAPI void nstd_std_audio_sink_free(NSTDAudioSink *sink);

#ifdef __cplusplus
}
#endif
#endif
