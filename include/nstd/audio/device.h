#ifndef NSTD_AUDIO_DEVICE_H_INCLUDED
#define NSTD_AUDIO_DEVICE_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
#include "../string.h"
#include "audio.h"
NSTDCPPSTART

/// Represents an audio device.
typedef NSTDAny NSTDAudioDevice;

/// Gets the name of a device.
///
/// # Parameters
///
/// - `const NSTDAudioDevice device` - The device.
///
/// # Returns
///
/// `NSTDString name` - The device name.
NSTDAPI NSTDString nstd_audio_device_name(const NSTDAudioDevice device);

/// Gets the default input config from the device.
///
/// # Parameters
///
/// - `const NSTDAudioDevice device` - The device.
///
/// - `NSTDAudioStreamConfig *const config` - Returns as the default input config for the device.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_audio_device_default_input_config(
    const NSTDAudioDevice device,
    NSTDAudioStreamConfig *const config);

/// Gets the default output config from the device.
///
/// # Parameters
///
/// - `const NSTDAudioDevice device` - The device.
///
/// - `NSTDAudioStreamConfig *const config` - Returns as the default output config for the device.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_audio_device_default_output_config(
    const NSTDAudioDevice device,
    NSTDAudioStreamConfig *const config);

/// Builds an input stream on a device.
///
/// # Parameters
///
/// - `const NSTDAudioDevice device` - The device.
///
/// - `const NSTDAudioStreamConfig *const config` - The stream config.
///
/// - `const NSTDAudioSampleFormat format` - The audio sample format.
///
/// - `void (*callback)(NSTDAnyConst, NSTDUSize)` - The stream callback.
///
/// - `void (*err_callback)()` - The stream error callback.
///
/// # Returns
///
/// `NSTDAudioStream stream` - The audio stream.
NSTDAPI NSTDAudioStream nstd_audio_device_build_input_stream(
    const NSTDAudioDevice device,
    const NSTDAudioStreamConfig *const config,
    const NSTDAudioSampleFormat format,
    void (*callback)(NSTDAnyConst, NSTDUSize),
    void (*err_callback)());

/// Builds an output stream on a device.
///
/// # Parameters
///
/// - `const NSTDAudioDevice device` - The device.
///
/// - `const NSTDAudioStreamConfig *const config` - The stream config.
///
/// - `const NSTDAudioSampleFormat format` - The audio sample format.
///
/// - `void (*callback)(NSTDAny, NSTDUSize)` - The stream callback.
///
/// - `void (*err_callback)()` - The stream error callback.
///
/// # Returns
///
/// `NSTDAudioStream stream` - The audio stream.
NSTDAPI NSTDAudioStream nstd_audio_device_build_output_stream(
    const NSTDAudioDevice device,
    const NSTDAudioStreamConfig *const config,
    const NSTDAudioSampleFormat format,
    void (*callback)(NSTDAny, NSTDUSize),
    void (*err_callback)());

/// Frees a device.
///
/// # Parameters
///
/// - `NSTDAudioDevice *const device` - Pointer to a device.
NSTDAPI void nstd_audio_device_free(NSTDAudioDevice *const device);

NSTDCPPEND
#endif
