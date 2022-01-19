use platforms::{
    target::{Arch, OS},
    TARGET_ARCH, TARGET_OS,
};

/// Represents an endianness of a CPU.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NSTDEndian {
    /// An unknown-endian.
    NSTD_ENDIAN_UNKNOWN,
    /// Little-endian.
    NSTD_ENDIAN_LITTLE,
    /// Big-endian.
    NSTD_ENDIAN_BIG,
}

/// Represents a CPU architecture.
#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Hash)]
#[allow(non_camel_case_types)]
pub enum NSTDCPUArch {
    /// An unknown architecture.
    NSTD_CPU_ARCH_UNKNOWN,
    /// ARM architecture.
    NSTD_CPU_ARCH_ARM,
    /// ARM64 architecture.
    NSTD_CPU_ARCH_ARM64,
    /// Asm.js architecture.
    NSTD_CPU_ARCH_ASMJS,
    /// MIPS architecture.
    NSTD_CPU_ARCH_MIPS,
    /// MIPS64 architecture.
    NSTD_CPU_ARCH_MIPS64,
    /// MSP430 architecture.
    NSTD_CPU_ARCH_MSP430,
    /// NVPTX architecture.
    NSTD_CPU_ARCH_NVPTX,
    /// POWERPC architecture.
    NSTD_CPU_ARCH_POWERPC,
    /// POWERPC64 architecture.
    NSTD_CPU_ARCH_POWERPC64,
    /// RISCV architecture.
    NSTD_CPU_ARCH_RISCV,
    /// S390X architecture.
    NSTD_CPU_ARCH_S390X,
    /// SPARC architecture.
    NSTD_CPU_ARCH_SPARC,
    /// SPARC64 architecture.
    NSTD_CPU_ARCH_SPARC64,
    /// THUMBV6 architecture.
    NSTD_CPU_ARCH_THUMBV6,
    /// THUMBV7 architecture.
    NSTD_CPU_ARCH_THUMBV7,
    /// WASM architecture.
    NSTD_CPU_ARCH_WASM,
    /// X64 architecture.
    NSTD_CPU_ARCH_X64,
    /// X86 architecture.
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
    /// An unknown platform.
    NSTD_OPERATING_SYSTEM_UNKNOWN,
    /// The Android platform.
    NSTD_OPERATING_SYSTEM_ANDROID,
    /// The CUDA platform.
    NSTD_OPERATING_SYSTEM_CUDA,
    /// The Dragonfly platform.
    NSTD_OPERATING_SYSTEM_DRAGONFLY,
    /// The EMSCRIPTEN platform.
    NSTD_OPERATING_SYSTEM_EMSCRIPTEN,
    /// The FreeBSD platform.
    NSTD_OPERATING_SYSTEM_FREE_BSD,
    /// The Fuchsia platform.
    NSTD_OPERATING_SYSTEM_FUCHSIA,
    /// The Haiku platform.
    NSTD_OPERATING_SYSTEM_HAIKU,
    /// The Hermit platform.
    NSTD_OPERATING_SYSTEM_HERMIT,
    /// The Illumos platform.
    NSTD_OPERATING_SYSTEM_ILLUMOS,
    /// The IOS platform.
    NSTD_OPERATING_SYSTEM_IOS,
    /// The Linux platform.
    NSTD_OPERATING_SYSTEM_LINUX,
    /// The macOS platform.
    NSTD_OPERATING_SYSTEM_MACOS,
    /// The NetBSD platform.
    NSTD_OPERATING_SYSTEM_NET_BSD,
    /// The OpenBSD platform.
    NSTD_OPERATING_SYSTEM_OPEN_BSD,
    /// The Redox platform.
    NSTD_OPERATING_SYSTEM_REDOX,
    /// The Solaris platform.
    NSTD_OPERATING_SYSTEM_SOLARIS,
    /// The tvOS platform.
    NSTD_OPERATING_SYSTEM_TVOS,
    /// The WASI platform.
    NSTD_OPERATING_SYSTEM_WASI,
    /// The Windows platform.
    NSTD_OPERATING_SYSTEM_WINDOWS,
    /// The VxWorks platform.
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

/// Represents a computing platform.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDPlatform {
    /// The CPU architecture.
    pub arch: NSTDCPUArch,
    /// The operating system.
    pub os: NSTDOperatingSystem,
}

/// Returns the target CPU's endianness.
/// Returns: `NSTDEndian endian` - The target CPU endianness.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_platform_endian() -> NSTDEndian {
    #[cfg(target_endian = "little")]
    return NSTDEndian::NSTD_ENDIAN_LITTLE;
    #[cfg(target_endian = "big")]
    return NSTDEndian::NSTD_ENDIAN_BIG;
    #[cfg(not(any(target_endian = "little", target_endian = "big")))]
    return NSTDEndian::NSTD_ENDIAN_UNKNOWN;
}

/// Returns an `NSTDCPUArch` value representing the target CPU architecture.
/// Returns: `NSTDCPUArch arch` - The target CPU architecture.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_platform_arch() -> NSTDCPUArch {
    NSTDCPUArch::from(TARGET_ARCH)
}

/// Returns an `NSTDOperatingSystem` value representing the target OS.
/// Returns: `NSTDOperatingSystem os` - The target OS.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_platform_os() -> NSTDOperatingSystem {
    NSTDOperatingSystem::from(TARGET_OS)
}

/// Returns an `NSTDPlatform` value representing the target platform.
/// Returns: `NSTDPlatform platform` - The target platform.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_platform_target() -> NSTDPlatform {
    NSTDPlatform {
        arch: nstd_core_platform_arch(),
        os: nstd_core_platform_os(),
    }
}
