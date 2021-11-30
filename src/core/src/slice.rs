use core::{ffi::c_void, ptr};

/// Represents a view into a sequence of data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NSTDSlice {
    pub size: usize,
    pub element_size: usize,
    pub data: *mut u8,
}
impl NSTDSlice {
    /// Gets the number of used bytes for this slice.
    #[inline]
    pub fn byte_count(&self) -> usize {
        self.size * self.element_size
    }

    /// Returns a pointer to the end of the slice.
    #[inline]
    pub unsafe fn end_unchecked(&self) -> *mut u8 {
        self.data.add(self.byte_count())
    }
}
impl<T> AsRef<[T]> for NSTDSlice {
    #[inline]
    fn as_ref(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.data as *const T, self.size) }
    }
}
impl<T> AsMut<[T]> for NSTDSlice {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.data as *mut T, self.size) }
    }
}

/// Creates a new slice from raw data.
/// Parameters:
///     `const NSTDUSize size` - Number of elements to slice.
///     `const NSTDUSize element_size` - Size of each element.
///     `NSTDByte *const data` - Pointer to the raw data.
/// Returns: `NSTDSlice slice` - The new slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_new(
    size: usize,
    element_size: usize,
    data: *mut u8,
) -> NSTDSlice {
    NSTDSlice {
        size,
        element_size,
        data,
    }
}

/// Gets a pointer to an element from a slice.
/// NOTE: The returned element pointer can quickly become a dangling pointer if the slice's memory
/// gets reallocated or deallocated, so it is advised to create a copy of the element after
/// getting it.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDUSize pos` - The position of the element to get.
/// Returns: `void *element` - Pointer to the element.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_get(slice: &NSTDSlice, pos: usize) -> *mut c_void {
    match slice.size > pos {
        true => slice.data.add(pos * slice.element_size) as *mut c_void,
        false => ptr::null_mut(),
    }
}

/// Gets the first element in the slice.
/// NOTE: This function follows the same behaviour rules as `nstd_core_slice_get`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
/// Returns: `void *element` - Pointer to the first element.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_first(slice: &NSTDSlice) -> *mut c_void {
    match slice.size > 0 {
        true => slice.data as *mut c_void,
        false => ptr::null_mut(),
    }
}

/// Gets the last element in the slice.
/// NOTE: This function follows the same behaviour rules as `nstd_core_slice_get`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
/// Returns: `void *element` - Pointer to the last element.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_last(slice: &NSTDSlice) -> *mut c_void {
    match slice.size > 0 {
        true => slice.end_unchecked().sub(slice.element_size) as *mut c_void,
        false => ptr::null_mut(),
    }
}

/// Checks if a slice contains `element`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const void *const element` - The element to search for.
/// Returns: `NSTDInt32 is_in` - Nonzero if the slice contains `element`.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_contains(
    slice: &NSTDSlice,
    element: *const c_void,
) -> i32 {
    let mut ptr = slice.data;
    let element = core::slice::from_raw_parts(element as *const u8, slice.element_size);
    for _ in 0..slice.size {
        let data = core::slice::from_raw_parts(ptr, slice.element_size);
        if data == element {
            return 1;
        }
        ptr = ptr.add(slice.element_size);
    }
    0
}

/// Checks if a slice starts with another slice.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDSlice *const pattern` - The slice pattern.
/// Returns: `NSTDInt32 starts_with` - Nonzero if `slice` starts with `pattern`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_starts_with(
    slice: &NSTDSlice,
    pattern: &NSTDSlice,
) -> i32 {
    let slice = core::slice::from_raw_parts(slice.data, slice.byte_count());
    let pattern = core::slice::from_raw_parts(pattern.data, pattern.byte_count());
    slice.starts_with(pattern) as i32
}

/// Checks if a slice ends with another slice.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDSlice *const pattern` - The slice pattern.
/// Returns: `NSTDInt32 ends_with` - Nonzero if `slice` ends with `pattern`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_ends_with(slice: &NSTDSlice, pattern: &NSTDSlice) -> i32 {
    let slice = core::slice::from_raw_parts(slice.data, slice.byte_count());
    let pattern = core::slice::from_raw_parts(pattern.data, pattern.byte_count());
    slice.ends_with(pattern) as i32
}

/// Fills a slice with `element`.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `const void *const element` - The element.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_fill(slice: &mut NSTDSlice, element: *const c_void) {
    let element = core::slice::from_raw_parts(element as *const u8, slice.element_size);
    let mut ptr = slice.data;
    for _ in 0..slice.size {
        let data = core::slice::from_raw_parts_mut(ptr, slice.element_size);
        data.copy_from_slice(element);
        ptr = ptr.add(slice.element_size);
    }
}

/// Swaps two elements in a slice.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `const NSTDUSize i` - The index of the first element.
///     `const NSTDUSize j` - The index of the second element.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_swap(slice: &mut NSTDSlice, i: usize, j: usize) {
    let i = slice.data.add(slice.element_size * i);
    let j = slice.data.add(slice.element_size * j);
    let slicei = core::slice::from_raw_parts_mut(i, slice.element_size);
    let slicej = core::slice::from_raw_parts_mut(j, slice.element_size);
    slicei.swap_with_slice(slicej);
}

/// Reverses a slice's elements.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_reverse(slice: &mut NSTDSlice) {
    let mut ptr = slice.data;
    let mut data = core::slice::from_raw_parts_mut(ptr, slice.byte_count());
    data.reverse();
    for _ in 0..slice.size {
        data = core::slice::from_raw_parts_mut(ptr, slice.element_size);
        data.reverse();
        ptr = ptr.add(slice.element_size);
    }
}

/// Shifts a slice `x` times to the right.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `const NSTDUSize x` - Number of times to shift the slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_shift_right(slice: &mut NSTDSlice, x: usize) {
    let data = core::slice::from_raw_parts_mut(slice.data, slice.byte_count());
    data.rotate_right(x % slice.size * slice.element_size);
}

/// Shifts a slice `x` times to the left.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `const NSTDUSize x` - Number of times to shift the slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_shift_left(slice: &mut NSTDSlice, x: usize) {
    let data = core::slice::from_raw_parts_mut(slice.data, slice.byte_count());
    data.rotate_left(x % slice.size * slice.element_size);
}

/// Copies elements from `s1` to `s2`. The slices must be the same size in bytes.
/// Parameters:
///     `NSTDSlice *const s1` - The slice to copy to.
///     `const NSTDSlice *const s2` - The slice to copy from.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_copy_from_slice(s1: &mut NSTDSlice, s2: &NSTDSlice) {
    let bc1 = s1.byte_count();
    let bc2 = s2.byte_count();
    if bc1 == bc2 {
        let s1 = core::slice::from_raw_parts_mut(s1.data, bc1);
        let s2 = core::slice::from_raw_parts(s2.data, bc2);
        s1.copy_from_slice(s2);
    }
}

/// Swaps the elements of `s1` and `s2`. The slices must be the same size in bytes.
/// Parameters:
///     `NSTDSlice *const s1` - The first slice.
///     `NSTDSlice *const s2` - The second slice.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_swap_with_slice(s1: &mut NSTDSlice, s2: &mut NSTDSlice) {
    let bc1 = s1.byte_count();
    let bc2 = s2.byte_count();
    if bc1 == bc2 {
        let s1 = core::slice::from_raw_parts_mut(s1.data, bc1);
        let s2 = core::slice::from_raw_parts_mut(s2.data, bc2);
        s1.swap_with_slice(s2);
    }
}