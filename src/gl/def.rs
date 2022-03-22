//! Commonly used graphics related types.
use wgpu::{Backend, Backends, Color, DeviceType, PowerPreference, PresentMode};

/// Represents a color.
// Must match https://docs.rs/wgpu/0.12.0/wgpu/struct.Color.html.
pub type NSTDGLColor = Color;

/// Represents a graphics backend.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Hash)]
pub enum NSTDGLBackend {
    /// An unknown graphics backend.
    NSTD_GL_BACKEND_UNKNOWN,
    /// Vulkan.
    NSTD_GL_BACKEND_VULKAN,
    /// Metal.
    NSTD_GL_BACKEND_METAL,
    /// Direct3D 12.
    NSTD_GL_BACKEND_DX12,
    /// Direct3D 11.
    NSTD_GL_BACKEND_DX11,
    /// OpenGL.
    NSTD_GL_BACKEND_GL,
    /// Web based GPU.
    NSTD_GL_BACKEND_WEBGPU,
}
impl Default for NSTDGLBackend {
    #[inline]
    fn default() -> Self {
        Self::NSTD_GL_BACKEND_UNKNOWN
    }
}
impl Into<Backends> for NSTDGLBackend {
    #[inline]
    fn into(self) -> Backends {
        match self {
            Self::NSTD_GL_BACKEND_UNKNOWN => Backends::all(),
            Self::NSTD_GL_BACKEND_VULKAN => Backends::VULKAN,
            Self::NSTD_GL_BACKEND_METAL => Backends::METAL,
            Self::NSTD_GL_BACKEND_DX11 => Backends::DX11,
            Self::NSTD_GL_BACKEND_DX12 => Backends::DX12,
            Self::NSTD_GL_BACKEND_GL => Backends::GL,
            Self::NSTD_GL_BACKEND_WEBGPU => Backends::BROWSER_WEBGPU,
        }
    }
}
impl From<Backend> for NSTDGLBackend {
    #[inline]
    fn from(backend: Backend) -> Self {
        match backend {
            Backend::Empty => Self::NSTD_GL_BACKEND_UNKNOWN,
            Backend::Vulkan => Self::NSTD_GL_BACKEND_VULKAN,
            Backend::Metal => Self::NSTD_GL_BACKEND_METAL,
            Backend::Dx12 => Self::NSTD_GL_BACKEND_DX12,
            Backend::Dx11 => Self::NSTD_GL_BACKEND_DX11,
            Backend::Gl => Self::NSTD_GL_BACKEND_GL,
            Backend::BrowserWebGpu => Self::NSTD_GL_BACKEND_WEBGPU,
        }
    }
}

/// Represents a state's presentation mode.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Hash)]
pub enum NSTDGLPresentationMode {
    /// `wgpu`'s presentation engine will request drawing immediately.
    NSTD_GL_PRESENTATION_MODE_IMMEDIATE,
    /// Waits for the vertical blanking period, but frames are submitted immediately.
    NSTD_GL_PRESENTATION_MODE_MAILBOX,
    /// Waits for the vertical blanking period, and frames are
    /// submitted with the monitor's referesh rate.
    NSTD_GL_PRESENTATION_MODE_FIFO,
}
impl Into<PresentMode> for NSTDGLPresentationMode {
    #[inline]
    fn into(self) -> PresentMode {
        match self {
            Self::NSTD_GL_PRESENTATION_MODE_IMMEDIATE => PresentMode::Immediate,
            Self::NSTD_GL_PRESENTATION_MODE_MAILBOX => PresentMode::Mailbox,
            Self::NSTD_GL_PRESENTATION_MODE_FIFO => PresentMode::Fifo,
        }
    }
}

/// Represents a power preference.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Hash)]
pub enum NSTDGLPowerPreference {
    /// Use the default power preference.
    NSTD_GL_POWER_PREFERENCE_DEFAULT,
    /// Use low GPU power.
    NSTD_GL_POWER_PREFERENCE_LOW,
    /// Use high GPU power.
    NSTD_GL_POWER_PREFERENCE_HIGH,
}
impl Into<PowerPreference> for NSTDGLPowerPreference {
    #[inline]
    fn into(self) -> PowerPreference {
        match self {
            Self::NSTD_GL_POWER_PREFERENCE_DEFAULT => PowerPreference::default(),
            Self::NSTD_GL_POWER_PREFERENCE_LOW => PowerPreference::LowPower,
            Self::NSTD_GL_POWER_PREFERENCE_HIGH => PowerPreference::HighPerformance,
        }
    }
}

/// Represents a device type.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Hash)]
pub enum NSTDGLDeviceType {
    /// An unknown device type.
    NSTD_GL_DEVICE_TYPE_UNKNOWN,
    /// `wgpu`'s integrated GPU.
    NSTD_GL_DEVICE_TYPE_INTEGRATED_GPU,
    /// A physical GPU.
    NSTD_GL_DEVICE_TYPE_DISCRETE_GPU,
    /// A virtual/hosted GPU.
    NSTD_GL_DEVICE_TYPE_VIRTUAL_GPU,
    /// CPU/Software rendering.
    NSTD_GL_DEVICE_TYPE_CPU,
}
impl Default for NSTDGLDeviceType {
    #[inline]
    fn default() -> Self {
        Self::NSTD_GL_DEVICE_TYPE_UNKNOWN
    }
}
impl From<DeviceType> for NSTDGLDeviceType {
    #[inline]
    fn from(device_type: DeviceType) -> Self {
        match device_type {
            DeviceType::Other => Self::NSTD_GL_DEVICE_TYPE_UNKNOWN,
            DeviceType::IntegratedGpu => Self::NSTD_GL_DEVICE_TYPE_INTEGRATED_GPU,
            DeviceType::DiscreteGpu => Self::NSTD_GL_DEVICE_TYPE_DISCRETE_GPU,
            DeviceType::VirtualGpu => Self::NSTD_GL_DEVICE_TYPE_VIRTUAL_GPU,
            DeviceType::Cpu => Self::NSTD_GL_DEVICE_TYPE_CPU,
        }
    }
}
