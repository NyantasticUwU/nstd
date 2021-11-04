#ifndef NSTD_STD_OS_H_INCLUDED
#define NSTD_STD_OS_H_INCLUDED
#include "../core/def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents an operating system.
typedef enum
{
    NSTD_OPERATING_SYSTEM_UNKNOWN,
    NSTD_OPERATING_SYSTEM_WINDOWS,
    NSTD_OPERATING_SYSTEM_MACOS,
    NSTD_OPERATING_SYSTEM_LINUX,
    NSTD_OPERATING_SYSTEM_IOS,
    NSTD_OPERATING_SYSTEM_ANDROID,
    NSTD_OPERATING_SYSTEM_FUCHSIA,
    NSTD_OPERATING_SYSTEM_REDOX,
    NSTD_OPERATING_SYSTEM_EMSCRIPTEN,
    NSTD_OPERATING_SYSTEM_OPEN_BSD,
    NSTD_OPERATING_SYSTEM_FREE_BSD,
    NSTD_OPERATING_SYSTEM_NET_BSD,
    NSTD_OPERATING_SYSTEM_DRAGONFLY,
    NSTD_OPERATING_SYSTEM_BITRIG,
    NSTD_OPERATING_SYSTEM_CLOUD_ABI,
    NSTD_OPERATING_SYSTEM_HAIKU,
    NSTD_OPERATING_SYSTEM_SOLARIS
} NSTDOperatingSystem;

/// Represents a CPU architecture.
typedef enum
{
    NSTD_CPU_ARCH_UNKNOWN,
    NSTD_CPU_ARCH_X86,
    NSTD_CPU_ARCH_X64,
    NSTD_CPU_ARCH_ARM,
    NSTD_CPU_ARCH_ARM64,
    NSTD_CPU_ARCH_WASM,
    NSTD_CPU_ARCH_ASMJS,
    NSTD_CPU_ARCH_MIPS,
    NSTD_CPU_ARCH_MIPS64,
    NSTD_CPU_ARCH_POWERPC,
    NSTD_CPU_ARCH_POWERPC64,
    NSTD_CPU_ARCH_THUMBV6,
    NSTD_CPU_ARCH_THUMBV7,
    NSTD_CPU_ARCH_MSP430,
    NSTD_CPU_ARCH_RISCV,
    NSTD_CPU_ARCH_S390X,
    NSTD_CPU_ARCH_SPARC,
    NSTD_CPU_ARCH_SPARC64
} NSTDCPUArch;

/// Returns an `NSTDOperatingSystem` value representing the target OS.
/// Returns: `NSTDOperatingSystem os` - The target OS.
NSTDAPI NSTDOperatingSystem nstd_std_os_os();

/// Returns an `NSTDCPUArch` value representing the target CPU architecture.
/// Returns: `NSTDCPUArch arch` - The target CPU architecture.
NSTDAPI NSTDCPUArch nstd_std_os_arch();

/// Returns a string describing the specific operating system in use.
/// `nstd_std_os_free_name` must be called to free memory allocated by this function.
/// Returns: `char *os_name` - The os's name as a string.
NSTDAPI char *nstd_std_os_name();

/// Frees memory allocated by `nstd_std_os_name`.
/// Parameters:
///     `const char **os_name` - Pointer to the os name cstr.
NSTDAPI void nstd_std_os_free_name(const char **os_name);

/// Returns a string describing the specific cpu architecture in use.
/// `nstd_std_os_free_arch_name` must be called to free memory allocated by this function.
/// Returns: `char *arch_name` - The cpu architecture's name as a string.
NSTDAPI char *nstd_std_os_arch_name();

/// Frees memory allocated by `nstd_std_os_arch_name`.
/// Parameters:
///     `const char **arch_name` - Pointer to the arch name cstr.
NSTDAPI void nstd_std_os_free_arch_name(const char **arch_name);

#ifdef __cplusplus
}
#endif
#endif
