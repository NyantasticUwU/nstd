#ifndef NSTD_AUDIO_HOST_H_INCLUDED
#define NSTD_AUDIO_HOST_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
#include "audio.h"
NSTDCPPSTART

/// Represents an audio host.
typedef NSTDAny NSTDAudioHost;

/// Gets the default audio host.
///
/// # Returns
///
/// `NSTDAudioHost host` - The default audio host.
NSTDAPI NSTDAudioHost nstd_audio_host_default();

/// Returns the default input device from a host.
///
/// # Parameters
///
/// - `const NSTDAudioHost host` - The audio host.
///
/// # Returns
///
/// `NSTDAudioDevice device` - The default input device.
NSTDAPI NSTDAudioDevice nstd_audio_host_default_input_device(const NSTDAudioHost host);

/// Returns the default output device from a host.
///
/// # Parameters
///
/// - `const NSTDAudioHost host` - The audio host.
///
/// # Returns
///
/// `NSTDAudioDevice device` - The default output device.
NSTDAPI NSTDAudioDevice nstd_audio_host_default_output_device(const NSTDAudioHost host);

/// Frees a host's memory.
///
/// # Parameters
///
/// - `NSTDAudioHost *const host` - Pointer to an audio host.
NSTDAPI void nstd_audio_host_free(NSTDAudioHost *const host);

NSTDCPPEND
#endif
