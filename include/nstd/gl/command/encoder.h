#ifndef NSTD_GL_COMMAND_ENCODER_H_INCLUDED
#define NSTD_GL_COMMAND_ENCODER_H_INCLUDED
#include "../../core/def.h"
#include "../../nstd.h"
#include "../device/device.h"
#include "buffer.h"
NSTDCPPSTART

/// A command encoder for a graphics device.
typedef NSTDAny NSTDGLCommandEncoder;

/// Creates a gpu command encoder.
///
/// # Parameters
///
/// - `const NSTDGLDevice *const device` - The device to create commands for.
///
/// # Returns
///
/// `NSTDGLCommandEncoder command_encoder` - The new gpu command encoder.
NSTDAPI NSTDGLCommandEncoder nstd_gl_command_encoder_new(const NSTDGLDevice *const device);

/// Frees a command encoder and returns a command buffer.
///
/// # Parameters
///
/// - `NSTDGLCommandEncoder *const command_encoder` - The gpu command encoder.
///
/// # Returns
///
/// `NSTDGLCommandBuffer command_buffer` - The gpu command buffer.
NSTDAPI NSTDGLCommandBuffer nstd_gl_command_encoder_finish(
    NSTDGLCommandEncoder *const command_encoder);

/// Frees a command encoder and submits the command buffer to the device immediately.
///
/// # Parameters
///
/// - `NSTDGLCommandEncoder *const command_encoder` - The gpu command encoder.
///
/// - `const NSTDGLDevice *const device` - The device to submit the command buffer to.
NSTDAPI void nstd_gl_command_encoder_finish_submit(
    NSTDGLCommandEncoder *const command_encoder,
    const NSTDGLDevice *const device);

NSTDCPPEND
#endif
