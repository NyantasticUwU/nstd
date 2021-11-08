use core::ffi::c_void;

/// Copies bytes from `other` to `copycat`.
/// Parameters:
///     `NSTDCOREByte *const copycat` - Pointer to memory to be copied to.
///     `const NSTDCOREByte *const other` - Pointer to memory to be copied from.
///     `const NSTDCOREUSize size` - Number of bytes to copy.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_mem_copy(copycat: *mut u8, other: *const u8, size: usize) {
    core::ptr::copy(other, copycat, size);
}

/// Moves bytes from `from` to `to`. Zeroes out `from`'s memory.
/// Parameters:
///     `NSTDCOREByte *const from` - Memory to be moved from.
///     `NSTDCOREByte *const to` - Memory to be moved to.
///     `const NSTDCOREUSize size` - Number of bytes to move.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_mem_move(from: *mut u8, to: *mut u8, size: usize) {
    nstd_core_mem_copy(to, from, size);
    for i in 0..size {
        (*from.add(i)) = 0;
    }
}

/// Moves memory from `*ptr1` to `*ptr2` and vice versa.
/// Parameters:
///     `const void **const ptr1` - Pointer to first pointer's memory location.
///     `const void **const ptr2` - Pointer to second pointer's memory location.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_mem_switch(ptr1: *mut *const c_void, ptr2: *mut *const c_void) {
    let ptr3 = ptr1;
    *ptr1 = *ptr2;
    *ptr2 = *ptr3;
}

/// Fills a block of memory with `byte`.
/// Parameters:
///     `NSTDCOREByte *const ptr` - Pointer to block of memory.
///     `const NSTDCOREUSize size` - Size of block.
///     `const NSTDCOREByte byte` - Byte to fill with.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_mem_fill(ptr: *mut u8, size: usize, byte: u8) {
    for i in 0..size {
        (*ptr.add(i)) = byte;
    }
}

/// Zeros a memory range pointed to by `ptr`.
/// Parameters:
///     `NSTDCOREByte *const ptr` - Pointer to memory to be zeroed.
///     `NSTDCOREUSize start` - Starting index of memory to be zeroed.
///     `const NSTDCOREUSize end` - Ending index of memory to be zeroed. (Excluded).
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_mem_zero(ptr: *mut u8, mut start: usize, end: usize) {
    while start < end {
        (*ptr.add(start)) = 0;
        start += 1;
    }
}
