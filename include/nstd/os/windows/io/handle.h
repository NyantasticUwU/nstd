#ifndef NSTD_OS_WINDOWS_IO_HANDLE_H_INCLUDED
#define NSTD_OS_WINDOWS_IO_HANDLE_H_INCLUDED
#include "../../../core/def.h"
#include "../../../nstd.h"
#include "../def.h"
NSTDCPPSTART

/// Represents a handle to a standard IO stream.
typedef NSTDUInt32 NSTDOSWindowsIOHandle;

/// Gets the `NSTDOSWindowsHandle` of a `NSTDOSWindowsIOHandle`.
///
/// # Parameters
///
/// - `const NSTDOSWindowsIOHandle stream` - An IO handle.
///
/// # Returns
///
/// `NSTDOSWindowsHandle handle` - The Window's handle.
NSTDAPI NSTDOSWindowsHandle nstd_os_windows_io_handle_as_handle(const NSTDOSWindowsIOHandle stream);

NSTDCPPEND
#endif
