#ifndef NSTD_CORE_PLATFORM_H_INCLUDED
#define NSTD_CORE_PLATFORM_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents an endianness of a CPU.
typedef enum
{
    /// An unknown-endian.
    NSTD_ENDIAN_UNKNOWN,
    /// Little-endian.
    NSTD_ENDIAN_LITTLE,
    /// Big-endian.
    NSTD_ENDIAN_BIG,
} NSTDEndian;

/// Represents a CPU architecture.
typedef enum
{
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
    NSTD_CPU_ARCH_X86
} NSTDCPUArch;

/// Represents an operating system.
typedef enum
{
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
    NSTD_OPERATING_SYSTEM_VXWORKS
} NSTDOperatingSystem;

/// Represents a computing platform.
typedef struct
{
    /// The CPU architecture.
    NSTDCPUArch arch;
    /// The operating system.
    NSTDOperatingSystem os;
} NSTDPlatform;

/// Returns the target CPU's endianness.
/// Returns: `NSTDEndian endian` - The target CPU endianness.
NSTDAPI NSTDEndian nstd_core_platform_endian();

/// Returns an `NSTDCPUArch` value representing the target CPU architecture.
/// Returns: `NSTDCPUArch arch` - The target CPU architecture.
NSTDAPI NSTDCPUArch nstd_core_platform_arch();

/// Returns an `NSTDOperatingSystem` value representing the target OS.
/// Returns: `NSTDOperatingSystem os` - The target OS.
NSTDAPI NSTDOperatingSystem nstd_core_platform_os();

/// Returns an `NSTDPlatform` value representing the target platform.
/// Returns: `NSTDPlatform platform` - The target platform.
NSTDAPI NSTDPlatform nstd_core_platform_target();

#ifdef __cplusplus
}
#endif
#endif
