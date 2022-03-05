use crate::{core::slice::NSTDSlice, gl::device::NSTDGLDevice};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferAddress, BufferUsages, IndexFormat, VertexAttribute, VertexBufferLayout,
    VertexFormat, VertexStepMode,
};

/// Represents a GPU buffer.
pub type NSTDGLBuffer = *mut Buffer;

/// Represents a vertex format when sending data to the vertex shader.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Hash)]
pub enum NSTDGLVertexFormat {
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
    NSTD_GL_VERTEX_FORMAT_FLOAT64X4,
}
impl Into<VertexFormat> for NSTDGLVertexFormat {
    #[inline]
    fn into(self) -> VertexFormat {
        match self {
            Self::NSTD_GL_VERTEX_FORMAT_UINT8X2 => VertexFormat::Uint8x2,
            Self::NSTD_GL_VERTEX_FORMAT_UINT8X4 => VertexFormat::Uint8x4,
            Self::NSTD_GL_VERTEX_FORMAT_INT8X2 => VertexFormat::Sint8x2,
            Self::NSTD_GL_VERTEX_FORMAT_INT8X4 => VertexFormat::Sint8x4,
            Self::NSTD_GL_VERTEX_FORMAT_UNORM8X2 => VertexFormat::Unorm8x2,
            Self::NSTD_GL_VERTEX_FORMAT_UNORM8X4 => VertexFormat::Unorm8x4,
            Self::NSTD_GL_VERTEX_FORMAT_NORM8X2 => VertexFormat::Snorm8x2,
            Self::NSTD_GL_VERTEX_FORMAT_NORM8X4 => VertexFormat::Snorm8x4,
            Self::NSTD_GL_VERTEX_FORMAT_UINT16X2 => VertexFormat::Uint16x2,
            Self::NSTD_GL_VERTEX_FORMAT_UINT16X4 => VertexFormat::Uint16x4,
            Self::NSTD_GL_VERTEX_FORMAT_INT16X2 => VertexFormat::Sint16x2,
            Self::NSTD_GL_VERTEX_FORMAT_INT16X4 => VertexFormat::Sint16x4,
            Self::NSTD_GL_VERTEX_FORMAT_UNORM16X2 => VertexFormat::Unorm16x2,
            Self::NSTD_GL_VERTEX_FORMAT_UNORM16X4 => VertexFormat::Unorm16x4,
            Self::NSTD_GL_VERTEX_FORMAT_NORM16X2 => VertexFormat::Snorm16x2,
            Self::NSTD_GL_VERTEX_FORMAT_NORM16X4 => VertexFormat::Snorm16x4,
            Self::NSTD_GL_VERTEX_FORMAT_FLOAT16X2 => VertexFormat::Float16x2,
            Self::NSTD_GL_VERTEX_FORMAT_FLOAT16X4 => VertexFormat::Float16x4,
            Self::NSTD_GL_VERTEX_FORMAT_FLOAT32 => VertexFormat::Float32,
            Self::NSTD_GL_VERTEX_FORMAT_FLOAT32X2 => VertexFormat::Float32x2,
            Self::NSTD_GL_VERTEX_FORMAT_FLOAT32X3 => VertexFormat::Float32x3,
            Self::NSTD_GL_VERTEX_FORMAT_FLOAT32X4 => VertexFormat::Float32x4,
            Self::NSTD_GL_VERTEX_FORMAT_UINT32 => VertexFormat::Uint32,
            Self::NSTD_GL_VERTEX_FORMAT_UINT32X2 => VertexFormat::Uint32x2,
            Self::NSTD_GL_VERTEX_FORMAT_UINT32X3 => VertexFormat::Uint32x3,
            Self::NSTD_GL_VERTEX_FORMAT_UINT32X4 => VertexFormat::Uint32x4,
            Self::NSTD_GL_VERTEX_FORMAT_INT32 => VertexFormat::Sint32,
            Self::NSTD_GL_VERTEX_FORMAT_INT32X2 => VertexFormat::Sint32x2,
            Self::NSTD_GL_VERTEX_FORMAT_INT32X3 => VertexFormat::Sint32x3,
            Self::NSTD_GL_VERTEX_FORMAT_INT32X4 => VertexFormat::Sint32x4,
            Self::NSTD_GL_VERTEX_FORMAT_FLOAT64 => VertexFormat::Float64,
            Self::NSTD_GL_VERTEX_FORMAT_FLOAT64X2 => VertexFormat::Float64x2,
            Self::NSTD_GL_VERTEX_FORMAT_FLOAT64X3 => VertexFormat::Float64x3,
            Self::NSTD_GL_VERTEX_FORMAT_FLOAT64X4 => VertexFormat::Float64x4,
        }
    }
}

/// Represents an index format when drawing indexed verticies.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Hash)]
pub enum NSTDGLIndexFormat {
    /// `NSTDUInt16`.
    NSTD_GL_INDEX_FORMAT_UINT16,
    /// `NSTDUInt32`.
    NSTD_GL_INDEX_FORMAT_UINT32,
}
impl Into<IndexFormat> for NSTDGLIndexFormat {
    #[inline]
    fn into(self) -> IndexFormat {
        match self {
            NSTDGLIndexFormat::NSTD_GL_INDEX_FORMAT_UINT16 => IndexFormat::Uint16,
            NSTDGLIndexFormat::NSTD_GL_INDEX_FORMAT_UINT32 => IndexFormat::Uint32,
        }
    }
}

/// Represents a vertex attribute.
/// NOTE: This struct must directly mirror `VertexAttribute` defined by wgpu in terms of size,
/// alignment, and order of fields.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDGLVertexAttribute {
    /// The vertex format to be used.
    pub format: NSTDGLVertexFormat,
    /// The offset in bytes from the start of the input.
    pub offset: u64,
    /// The location for this input.
    pub location: u32,
}
impl Into<VertexAttribute> for NSTDGLVertexAttribute {
    #[inline]
    fn into(self) -> VertexAttribute {
        VertexAttribute {
            offset: self.offset,
            shader_location: self.location,
            format: self.format.into(),
        }
    }
}

/// Represents a vertex stepping mode.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Hash)]
pub enum NSTDGLVertexStepMode {
    /// Vertex data is advanced each vertex.
    NSTD_GL_VERTEX_STEP_MODE_VERTEX,
    /// Vertex data is advanced each instance.
    NSTD_GL_VERTEX_STEP_MODE_INSTANCE,
}
impl Into<VertexStepMode> for NSTDGLVertexStepMode {
    #[inline]
    fn into(self) -> VertexStepMode {
        match self {
            Self::NSTD_GL_VERTEX_STEP_MODE_VERTEX => VertexStepMode::Vertex,
            Self::NSTD_GL_VERTEX_STEP_MODE_INSTANCE => VertexStepMode::Instance,
        }
    }
}

/// Represents a vertex buffer layout.
/// `attributes` - `&mut [NSTDGLVertexAttribute]`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDGLVertexBufferLayout {
    /// The number of bytes between each element.
    pub stride: u64,
    /// Determines how often the vertex data is advanced.
    pub step_mode: NSTDGLVertexStepMode,
    /// A slice of `NSTDGLVertexAttribute`s.
    pub attributes: NSTDSlice,
}
impl<'a> Into<VertexBufferLayout<'a>> for NSTDGLVertexBufferLayout {
    #[inline]
    fn into(self) -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: self.stride as BufferAddress,
            step_mode: self.step_mode.into(),
            attributes: unsafe {
                std::slice::from_raw_parts(self.attributes.ptr.raw.cast(), self.attributes.size)
            },
        }
    }
}

/// Creates a new GPU buffer.
/// Parameters:
///     `const NSTDSlice *const bytes` - The bytes to send to the GPU.
///     `const NSTDGLDevice device` - The device to create the buffer on.
/// Returns: `NSTDGLBuffer buffer` - The new GPU buffer.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_buffer_new(
    bytes: &NSTDSlice,
    device: NSTDGLDevice,
) -> NSTDGLBuffer {
    let buffer_desc = BufferInitDescriptor {
        label: None,
        contents: std::slice::from_raw_parts(bytes.ptr.raw.cast(), bytes.byte_count()),
        usage: BufferUsages::all(),
    };
    Box::into_raw(Box::new((*device.raw).create_buffer_init(&buffer_desc)))
}

/// Frees a GPU buffer.
/// Parameters:
///     `NSTDGLBuffer *const buffer` - The GPU buffer.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_buffer_free(buffer: &mut NSTDGLBuffer) {
    Box::from_raw(*buffer);
    *buffer = std::ptr::null_mut();
}
