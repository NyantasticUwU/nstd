#ifndef NSTD_CORE_POINTER_H_INCLUDED
#define NSTD_CORE_POINTER_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a reference to any type.
typedef struct
{
    /// Raw pointer to the referenced object.
    void *ptr;
    /// Size in bytes of the referenced object.
    NSTDUSize size;
} NSTDPointer;

/// Creates a new instance of `NSTDPointer`.
/// Parameters:
///     `void *const obj` - The object to reference.
///     `const NSTDUSize size` - The size in bytes of `obj`.
/// Returns: `NSTDPointer ptr` - The pointer type.
NSTDAPI NSTDPointer nstd_core_pointer_new(void *const obj, const NSTDUSize size);

/// Overwrites the current referenced object's data with `obj`.
/// Parameters:
///     `NSTDPointer *const ptr` - The pointer.
///     `const void *const obj` - The object to overwrite with.
NSTDAPI void nstd_core_pointer_write(NSTDPointer *const ptr, const void *const obj);

#ifdef __cplusplus
}
#endif
#endif