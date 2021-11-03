use std::{os::raw::c_void, ptr};

/// Represents a view into a sequence of data.
#[repr(C)]
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

    /// Returns a pointer to the end of the slicetor.
    #[inline]
    pub unsafe fn end_unchecked(&self) -> *mut u8 {
        self.data.add(self.byte_count())
    }
}

/// Creates a new slice from raw data.
/// Parameters:
///     `const NSTDSize size` - Number of elements to view.
///     `const NSTDSize element_size` - Size of each element.
///     `NSTDByte *const data` - Pointer to the raw data.
/// Returns: `NSTDSlice slice` - The new slice.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_collections_slice_new(
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
///     `const NSTDSize pos` - The position of the element to get.
/// Returns: `void *element` - Pointer to the element.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_collections_slice_get(
    slice: &NSTDSlice,
    pos: usize,
) -> *mut c_void {
    match slice.size > pos {
        true => slice.data.add(pos * slice.element_size) as *mut c_void,
        false => ptr::null_mut(),
    }
}

/// Gets the first element in the slice.
/// NOTE: This function follows the same behaviour rules as `nstd_std_collections_slice_get`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
/// Returns: `void *element` - Pointer to the first element.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_collections_slice_first(slice: &NSTDSlice) -> *mut c_void {
    match slice.size > 0 {
        true => slice.data as *mut c_void,
        false => ptr::null_mut(),
    }
}

/// Gets the last element in the slice.
/// NOTE: This function follows the same behaviour rules as `nstd_std_collections_slice_get`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
/// Returns: `void *element` - Pointer to the last element.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_collections_slice_last(slice: &NSTDSlice) -> *mut c_void {
    match slice.size > 0 {
        true => slice.end_unchecked().sub(slice.element_size) as *mut c_void,
        false => ptr::null_mut(),
    }
}
