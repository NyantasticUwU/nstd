#ifndef NSTD_GL_BUFFER_H_INCLUDED
#define NSTD_GL_BUFFER_H_INCLUDED
#include "../core/def.h"
#include "../core/slice.h"
#include "../nstd.h"
#include "device.h"
NSTDCPPSTART

/// Represents a GPU buffer.
typedef NSTDAny NSTDGLBuffer;

/// Represents a vertex format when sending data to the vertex shader.
typedef enum
{
    /// Two `UINT8`s.
    NSTD_GL_VERTEX_FORMAT_UINT8X2,
    /// Four `UINT8`s.
    NSTD_GL_VERTEX_FORMAT_UINT8X4,
    /// Two `INT8`s.
    NSTD_GL_VERTEX_FORMAT_INT8X2,
    /// Four `INT8`s.
    NSTD_GL_VERTEX_FORMAT_INT8X4,
    /// Two `UNORM8`s.
    NSTD_GL_VERTEX_FORMAT_UNORM8X2,
    /// Four `UNORM8`s.
    NSTD_GL_VERTEX_FORMAT_UNORM8X4,
    /// Two `NORM8`s.
    NSTD_GL_VERTEX_FORMAT_NORM8X2,
    /// Four `NORM8`s.
    NSTD_GL_VERTEX_FORMAT_NORM8X4,
    /// Two `UINT16`s.
    NSTD_GL_VERTEX_FORMAT_UINT16X2,
    /// Four `UINT16`s.
    NSTD_GL_VERTEX_FORMAT_UINT16X4,
    /// Two `INT16`s.
    NSTD_GL_VERTEX_FORMAT_INT16X2,
    /// Four `INT16`s.
    NSTD_GL_VERTEX_FORMAT_INT16X4,
    /// Two `UNORM16`s.
    NSTD_GL_VERTEX_FORMAT_UNORM16X2,
    /// Four `UNORM16`s.
    NSTD_GL_VERTEX_FORMAT_UNORM16X4,
    /// Two `NORM16`s.
    NSTD_GL_VERTEX_FORMAT_NORM16X2,
    /// Four `NORM16`s.
    NSTD_GL_VERTEX_FORMAT_NORM16X4,
    /// Two `FLOAT16`s.
    NSTD_GL_VERTEX_FORMAT_FLOAT16X2,
    /// Four `FLOAT16`s.
    NSTD_GL_VERTEX_FORMAT_FLOAT16X4,
    /// One `FLOAT32`.
    NSTD_GL_VERTEX_FORMAT_FLOAT32,
    /// Two `FLOAT32`s.
    NSTD_GL_VERTEX_FORMAT_FLOAT32X2,
    /// Three `FLOAT32`s.
    NSTD_GL_VERTEX_FORMAT_FLOAT32X3,
    /// Four `FLOAT32`s.
    NSTD_GL_VERTEX_FORMAT_FLOAT32X4,
    /// One `UINT32`.
    NSTD_GL_VERTEX_FORMAT_UINT32,
    /// Two `UINT32`s.
    NSTD_GL_VERTEX_FORMAT_UINT32X2,
    /// Three `UINT32`s.
    NSTD_GL_VERTEX_FORMAT_UINT32X3,
    /// Four `UINT32`s.
    NSTD_GL_VERTEX_FORMAT_UINT32X4,
    /// One `INT32`.
    NSTD_GL_VERTEX_FORMAT_INT32,
    /// Two `INT32`s.
    NSTD_GL_VERTEX_FORMAT_INT32X2,
    /// Three `INT32`s.
    NSTD_GL_VERTEX_FORMAT_INT32X3,
    /// Four `INT32`s.
    NSTD_GL_VERTEX_FORMAT_INT32X4,
    /// One `FLOAT64`.
    NSTD_GL_VERTEX_FORMAT_FLOAT64,
    /// Two `FLOAT64`s.
    NSTD_GL_VERTEX_FORMAT_FLOAT64X2,
    /// Three `FLOAT64`s.
    NSTD_GL_VERTEX_FORMAT_FLOAT64X3,
    /// Four `FLOAT64`s.
    NSTD_GL_VERTEX_FORMAT_FLOAT64X4
} NSTDGLVertexFormat;

/// Represents an index format when drawing indexed verticies.
typedef enum
{
    /// `NSTDUInt16`.
    NSTD_GL_INDEX_FORMAT_UINT16,
    /// `NSTDUInt32`.
    NSTD_GL_INDEX_FORMAT_UINT32,
} NSTDGLIndexFormat;

/// Represents a vertex attribute.
/// NOTE: This struct must directly mirror `VertexAttribute` defined by wgpu in terms of size,
/// alignment, and order of fields.
typedef struct
{
    /// The vertex format to be used.
    NSTDGLVertexFormat format;
    /// The offset in bytes from the start of the input.
    NSTDUInt64 offset;
    /// The location for this input.
    NSTDUInt32 location;
} NSTDGLVertexAttribute;

/// Represents a vertex stepping mode.
typedef enum
{
    /// Vertex data is advanced each vertex.
    NSTD_GL_VERTEX_STEP_MODE_VERTEX,
    /// Vertex data is advanced each instance.
    NSTD_GL_VERTEX_STEP_MODE_INSTANCE,
} NSTDGLVertexStepMode;

/// Represents a vertex buffer layout.
/// `attributes` - `&mut [NSTDGLVertexAttribute]`.
typedef struct
{
    /// The number of bytes between each element.
    NSTDUInt64 stride;
    /// Determines how often the vertex data is advanced.
    NSTDGLVertexStepMode step_mode;
    /// A slice of `NSTDGLVertexAttribute`s.
    NSTDSlice attributes;
} NSTDGLVertexBufferLayout;

/// Creates a new GPU buffer.
/// Parameters:
///     `const NSTDSlice *const bytes` - The bytes to send to the GPU.
///     `const NSTDGLDevice device` - The device to create the buffer on.
/// Returns: `NSTDGLBuffer buffer` - The new GPU buffer.
NSTDAPI NSTDGLBuffer nstd_gl_buffer_new(const NSTDSlice *const bytes, const NSTDGLDevice device);

/// Frees a GPU buffer.
/// Parameters:
///     `NSTDGLBuffer *const buffer` - The GPU buffer.
NSTDAPI void nstd_gl_buffer_free(NSTDGLBuffer *const buffer);

NSTDCPPEND
#endif
