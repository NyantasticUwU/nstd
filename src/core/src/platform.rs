use platforms::{
    target::{Arch, OS},
    TARGET_ARCH, TARGET_OS,
};

/// Represents a CPU architecture.
#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Hash)]
#[allow(non_camel_case_types)]
pub enum NSTDCPUArch {
    NSTD_CPU_ARCH_UNKNOWN,
    NSTD_CPU_ARCH_ARM,
    NSTD_CPU_ARCH_ARM64,
    NSTD_CPU_ARCH_ASMJS,
    NSTD_CPU_ARCH_MIPS,
    NSTD_CPU_ARCH_MIPS64,
    NSTD_CPU_ARCH_MSP430,
    NSTD_CPU_ARCH_NVPTX,
    NSTD_CPU_ARCH_POWERPC,
    NSTD_CPU_ARCH_POWERPC64,
    NSTD_CPU_ARCH_RISCV,
    NSTD_CPU_ARCH_S390X,
    NSTD_CPU_ARCH_SPARC,
    NSTD_CPU_ARCH_SPARC64,
    NSTD_CPU_ARCH_THUMBV6,
    NSTD_CPU_ARCH_THUMBV7,
    NSTD_CPU_ARCH_WASM,
    NSTD_CPU_ARCH_X64,
    NSTD_CPU_ARCH_X86,
}
impl Default for NSTDCPUArch {
    #[inline]
    fn default() -> Self {
        Self::NSTD_CPU_ARCH_UNKNOWN
    }
}
impl From<Arch> for NSTDCPUArch {
    #[inline]
    fn from(arch: Arch) -> Self {
        match arch {
            Arch::Arm => Self::NSTD_CPU_ARCH_ARM,
            Arch::AArch64 => Self::NSTD_CPU_ARCH_ARM64,
            Arch::AsmJs => Self::NSTD_CPU_ARCH_ASMJS,
            Arch::Mips => Self::NSTD_CPU_ARCH_MIPS,
            Arch::Mips64 => Self::NSTD_CPU_ARCH_MIPS64,
            Arch::Msp430 => Self::NSTD_CPU_ARCH_MSP430,
            Arch::Nvptx64 => Self::NSTD_CPU_ARCH_NVPTX,
            Arch::PowerPc => Self::NSTD_CPU_ARCH_POWERPC,
            Arch::PowerPc64 => Self::NSTD_CPU_ARCH_POWERPC64,
            Arch::RiscV => Self::NSTD_CPU_ARCH_RISCV,
            Arch::S390X => Self::NSTD_CPU_ARCH_S390X,
            Arch::Sparc => Self::NSTD_CPU_ARCH_SPARC,
            Arch::Sparc64 => Self::NSTD_CPU_ARCH_SPARC64,
            Arch::ThumbV6 => Self::NSTD_CPU_ARCH_THUMBV6,
            Arch::ThumbV7 => Self::NSTD_CPU_ARCH_THUMBV7,
            Arch::Wasm32 => Self::NSTD_CPU_ARCH_WASM,
            Arch::X86_64 => Self::NSTD_CPU_ARCH_X64,
            Arch::X86 => Self::NSTD_CPU_ARCH_X86,
            _ => Self::NSTD_CPU_ARCH_UNKNOWN,
        }
    }
}

/// Represents an operating system.
#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Hash)]
#[allow(non_camel_case_types)]
pub enum NSTDOperatingSystem {
    NSTD_OPERATING_SYSTEM_UNKNOWN,
    NSTD_OPERATING_SYSTEM_ANDROID,
    NSTD_OPERATING_SYSTEM_CUDA,
    NSTD_OPERATING_SYSTEM_DRAGONFLY,
    NSTD_OPERATING_SYSTEM_EMSCRIPTEN,
    NSTD_OPERATING_SYSTEM_FREE_BSD,
    NSTD_OPERATING_SYSTEM_FUCHSIA,
    NSTD_OPERATING_SYSTEM_HAIKU,
    NSTD_OPERATING_SYSTEM_HERMIT,
    NSTD_OPERATING_SYSTEM_ILLUMOS,
    NSTD_OPERATING_SYSTEM_IOS,
    NSTD_OPERATING_SYSTEM_LINUX,
    NSTD_OPERATING_SYSTEM_MACOS,
    NSTD_OPERATING_SYSTEM_NET_BSD,
    NSTD_OPERATING_SYSTEM_OPEN_BSD,
    NSTD_OPERATING_SYSTEM_REDOX,
    NSTD_OPERATING_SYSTEM_SOLARIS,
    NSTD_OPERATING_SYSTEM_TVOS,
    NSTD_OPERATING_SYSTEM_WASI,
    NSTD_OPERATING_SYSTEM_WINDOWS,
    NSTD_OPERATING_SYSTEM_VXWORKS,
}
impl Default for NSTDOperatingSystem {
    #[inline]
    fn default() -> Self {
        Self::NSTD_OPERATING_SYSTEM_UNKNOWN
    }
}
impl From<OS> for NSTDOperatingSystem {
    #[inline]
    fn from(os: OS) -> Self {
        match os {
            OS::Android => Self::NSTD_OPERATING_SYSTEM_ANDROID,
            OS::Cuda => Self::NSTD_OPERATING_SYSTEM_CUDA,
            OS::Dragonfly => Self::NSTD_OPERATING_SYSTEM_DRAGONFLY,
            OS::Emscripten => Self::NSTD_OPERATING_SYSTEM_EMSCRIPTEN,
            OS::FreeBSD => Self::NSTD_OPERATING_SYSTEM_FREE_BSD,
            OS::Fuchsia => Self::NSTD_OPERATING_SYSTEM_FUCHSIA,
            OS::Haiku => Self::NSTD_OPERATING_SYSTEM_HAIKU,
            OS::Hermit => Self::NSTD_OPERATING_SYSTEM_HERMIT,
            OS::Illumos => Self::NSTD_OPERATING_SYSTEM_ILLUMOS,
            OS::iOS => Self::NSTD_OPERATING_SYSTEM_IOS,
            OS::Linux => Self::NSTD_OPERATING_SYSTEM_LINUX,
            OS::MacOS => Self::NSTD_OPERATING_SYSTEM_MACOS,
            OS::NetBSD => Self::NSTD_OPERATING_SYSTEM_NET_BSD,
            OS::OpenBSD => Self::NSTD_OPERATING_SYSTEM_OPEN_BSD,
            OS::Redox => Self::NSTD_OPERATING_SYSTEM_REDOX,
            OS::Solaris => Self::NSTD_OPERATING_SYSTEM_SOLARIS,
            OS::TvOS => Self::NSTD_OPERATING_SYSTEM_TVOS,
            OS::Wasi => Self::NSTD_OPERATING_SYSTEM_WASI,
            OS::Windows => Self::NSTD_OPERATING_SYSTEM_WINDOWS,
            OS::VxWorks => Self::NSTD_OPERATING_SYSTEM_VXWORKS,
            _ => Self::NSTD_OPERATING_SYSTEM_UNKNOWN,
        }
    }
}

/// Returns an `NSTDCPUArch` value representing the target CPU architecture.
/// Returns: `NSTDCPUArch arch` - The target CPU architecture.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_platform_arch() -> NSTDCPUArch {
    NSTDCPUArch::from(TARGET_ARCH)
}

/// Returns an `NSTDOperatingSystem` value representing the target OS.
/// Returns: `NSTDOperatingSystem os` - The target OS.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_platform_os() -> NSTDOperatingSystem {
    NSTDOperatingSystem::from(TARGET_OS)
}
