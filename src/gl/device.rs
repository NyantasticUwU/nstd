use crate::{gl::def::NSTDGLBackend, string::NSTDString};
use wgpu::{Adapter, Device, DeviceType, Queue};

/// Represents a handle to a physical graphics device.
pub type NSTDGLDeviceHandle = *mut Adapter;

/// Represents a graphics device.
pub type NSTDGLDevice = *mut Device;

/// Represents a graphics device command queue.
pub type NSTDGLQueue = *mut Queue;

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

/// Contains information on a device.
#[repr(C)]
#[derive(Clone, Debug, Hash)]
pub struct NSTDGLDeviceInfo {
    /// The name of the drawing device.
    pub name: NSTDString,
    /// The device's vendor.
    pub vendor: usize,
    /// The ID of the device adapter.
    pub device: usize,
    /// The type of drawing device.
    pub device_type: NSTDGLDeviceType,
    /// The drawing backend in use.
    pub backend: NSTDGLBackend,
}
