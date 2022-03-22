#ifndef NSTD_GL_COMMAND_BUFFER_H_INCLUDED
#define NSTD_GL_COMMAND_BUFFER_H_INCLUDED
#include "../../core/def.h"
#include "../../nstd.h"
NSTDCPPSTART

/// A command buffer for a graphics device.
typedef NSTDAny NSTDGLCommandBuffer;

/// Frees a command buffer.
///
/// # Parameters
///
/// - `NSTDGLCommandBuffer *const command_buffer` - The gpu command buffer.
NSTDAPI void nstd_gl_command_buffer_free(NSTDGLCommandBuffer *const command_buffer);

NSTDCPPEND
#endif
