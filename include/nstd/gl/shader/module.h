#ifndef NSTD_GL_SHADER_MODULE_H_INCLUDED
#define NSTD_GL_SHADER_MODULE_H_INCLUDED
#include "../../core/def.h"
#include "../../core/slice.h"
#include "../../nstd.h"
#include "../device/device.h"
NSTDCPPSTART

/// Represents a shader module.
typedef NSTDAny NSTDGLShaderModule;

/// Creates a new shader module.
///
/// # Parameters
///
/// - `const NSTDSlice *const data` - Raw spirv data.
///
/// - `const NSTDGLDevice device` - The device to create the shader module on.
///
/// # Returns
///
/// `NSTDGLShaderModule shader` - The new shader module.
NSTDAPI NSTDGLShaderModule nstd_gl_shader_module_new(
    const NSTDSlice *const data,
    const NSTDGLDevice device);

/// Frees a shader module.
///
/// # Parameters
///
/// - `NSTDGLShaderModule *const shader` - Pointer to a shader module.
NSTDAPI void nstd_gl_shader_module_free(NSTDGLShaderModule *const shader);

NSTDCPPEND
#endif
