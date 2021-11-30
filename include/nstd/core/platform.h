#ifndef NSTD_CORE_PLATFORM_H_INCLUDED
#define NSTD_CORE_PLATFORM_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a CPU architecture.
typedef enum
{
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
    NSTD_CPU_ARCH_X86
} NSTDCPUArch;

/// Represents an operating system.
typedef enum
{
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
    NSTD_OPERATING_SYSTEM_VXWORKS
} NSTDOperatingSystem;

/// Returns an `NSTDCPUArch` value representing the target CPU architecture.
/// Returns: `NSTDCPUArch arch` - The target CPU architecture.
NSTDAPI NSTDCPUArch nstd_core_platform_arch();

/// Returns an `NSTDOperatingSystem` value representing the target OS.
/// Returns: `NSTDOperatingSystem os` - The target OS.
NSTDAPI NSTDOperatingSystem nstd_core_platform_os();

#ifdef __cplusplus
}
#endif
#endif