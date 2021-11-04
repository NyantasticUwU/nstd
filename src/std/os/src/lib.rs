use platforms::{
    target::{Arch, OS},
    TARGET_ARCH, TARGET_OS,
};
use std::{
    env::consts::{ARCH, OS},
    ffi::CString,
    os::raw::c_char,
    ptr,
};

/// Represents an operating system.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDOperatingSystem {
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
    NSTD_OPERATING_SYSTEM_SOLARIS,
}
impl From<OS> for NSTDOperatingSystem {
    #[inline]
    fn from(os: OS) -> Self {
        match os {
            OS::Windows => Self::NSTD_OPERATING_SYSTEM_WINDOWS,
            OS::MacOS => Self::NSTD_OPERATING_SYSTEM_MACOS,
            OS::Linux => Self::NSTD_OPERATING_SYSTEM_LINUX,
            OS::iOS => Self::NSTD_OPERATING_SYSTEM_IOS,
            OS::Android => Self::NSTD_OPERATING_SYSTEM_ANDROID,
            OS::Fuchsia => Self::NSTD_OPERATING_SYSTEM_FUCHSIA,
            OS::Redox => Self::NSTD_OPERATING_SYSTEM_REDOX,
            OS::Emscripten => Self::NSTD_OPERATING_SYSTEM_EMSCRIPTEN,
            OS::OpenBSD => Self::NSTD_OPERATING_SYSTEM_OPEN_BSD,
            OS::FreeBSD => Self::NSTD_OPERATING_SYSTEM_FREE_BSD,
            OS::NetBSD => Self::NSTD_OPERATING_SYSTEM_NET_BSD,
            OS::Dragonfly => Self::NSTD_OPERATING_SYSTEM_DRAGONFLY,
            OS::Bitrig => Self::NSTD_OPERATING_SYSTEM_BITRIG,
            OS::CloudABI => Self::NSTD_OPERATING_SYSTEM_CLOUD_ABI,
            OS::Haiku => Self::NSTD_OPERATING_SYSTEM_HAIKU,
            OS::Solaris => Self::NSTD_OPERATING_SYSTEM_SOLARIS,
            _ => Self::NSTD_OPERATING_SYSTEM_UNKNOWN,
        }
    }
}

/// Represents a CPU architecture.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDCPUArch {
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
    NSTD_CPU_ARCH_SPARC64,
}
impl From<Arch> for NSTDCPUArch {
    #[inline]
    fn from(arch: Arch) -> Self {
        match arch {
            Arch::X86 => Self::NSTD_CPU_ARCH_X86,
            Arch::X86_64 => Self::NSTD_CPU_ARCH_X64,
            Arch::ARM => Self::NSTD_CPU_ARCH_ARM,
            Arch::AARCH64 => Self::NSTD_CPU_ARCH_ARM64,
            Arch::WASM32 => Self::NSTD_CPU_ARCH_WASM,
            Arch::ASMJS => Self::NSTD_CPU_ARCH_ASMJS,
            Arch::MIPS => Self::NSTD_CPU_ARCH_MIPS,
            Arch::MIPS64 => Self::NSTD_CPU_ARCH_MIPS64,
            Arch::POWERPC => Self::NSTD_CPU_ARCH_POWERPC,
            Arch::POWERPC64 => Self::NSTD_CPU_ARCH_POWERPC64,
            Arch::THUMBV6 => Self::NSTD_CPU_ARCH_THUMBV6,
            Arch::THUMBV7 => Self::NSTD_CPU_ARCH_THUMBV7,
            Arch::MSP430 => Self::NSTD_CPU_ARCH_MSP430,
            Arch::RISCV => Self::NSTD_CPU_ARCH_RISCV,
            Arch::S390X => Self::NSTD_CPU_ARCH_S390X,
            Arch::SPARC => Self::NSTD_CPU_ARCH_SPARC,
            Arch::SPARC64 => Self::NSTD_CPU_ARCH_SPARC64,
            _ => Self::NSTD_CPU_ARCH_UNKNOWN,
        }
    }
}

/// Returns an `NSTDOperatingSystem` value representing the target OS.
/// Returns: `NSTDOperatingSystem os` - The target OS.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_os_os() -> NSTDOperatingSystem {
    NSTDOperatingSystem::from(TARGET_OS)
}

/// Returns an `NSTDCPUArch` value representing the target CPU architecture.
/// Returns: `NSTDCPUArch arch` - The target CPU architecture.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_os_arch() -> NSTDCPUArch {
    NSTDCPUArch::from(TARGET_ARCH)
}

/// Returns a string describing the specific operating system in use.
/// `nstd_std_os_free_name` must be called to free memory allocated by this function.
/// Returns: `char *os_name` - The os's name as a string.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_os_name() -> *mut c_char {
    static_nstd_create_cstr(OS)
}

/// Frees memory allocated by `nstd_std_os_name`.
/// Parameters:
///     `const char **os_name` - Pointer to the os name cstr.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_os_free_name(os_name: *mut *mut c_char) {
    static_nstd_deallocate_cstr(os_name);
}

/// Returns a string describing the specific cpu architecture in use.
/// `nstd_std_os_free_arch_name` must be called to free memory allocated by this function.
/// Returns: `char *arch_name` - The cpu architecture's name as a string.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_os_arch_name() -> *mut c_char {
    static_nstd_create_cstr(ARCH)
}

/// Frees memory allocated by `nstd_std_os_arch_name`.
/// Parameters:
///     `const char **arch_name` - Pointer to the arch name cstr.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_os_free_arch_name(arch_name: *mut *mut c_char) {
    static_nstd_deallocate_cstr(arch_name);
}

/// Creates a raw `*mut c_char` from a `&str`.
/// Parameters:
///     `rstr: &str` - Rust string slice to convert.
/// Returns: `cstr: *mut c_char` - The cstring version of `rstr`.
#[inline]
unsafe fn static_nstd_create_cstr(rstr: &str) -> *mut c_char {
    let mut bytes = String::from(rstr).into_bytes();
    bytes.push(0);
    CString::from_vec_unchecked(bytes).into_raw() as *mut c_char
}

/// Frees heap allocated rust c-string.
/// Parameters:
///     `cstr: *mut *mut c_char` - C-string to deallocate.
#[inline]
unsafe fn static_nstd_deallocate_cstr(cstr: *mut *mut c_char) {
    CString::from_raw(*cstr);
    *cstr = ptr::null_mut();
}
