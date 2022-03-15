#ifndef NSTD_STRING_H_INCLUDED
#define NSTD_STRING_H_INCLUDED
#include "core/def.h"
#include "core/slice.h"
#include "core/str.h"
#include "nstd.h"
#include "vec.h"
NSTDCPPSTART

/// Represents a dynamic-sized array of UTF-8 chars.
typedef struct
{
    /// The internal UTF-8 encoded buffer.
    NSTDVec bytes;
} NSTDString;

/// Creates a new `NSTDString` instance.
/// Returns: `NSTDString string` - The new string.
NSTDAPI NSTDString nstd_string_new();

/// Creates an `NSTDString` from existing data.
/// Parameters:
///     `const NSTDVec *const bytes` - The existing raw data.
/// Returns: `NSTDString string` - The new `NSTDString` object.
NSTDAPI NSTDString nstd_string_from_existing(const NSTDVec *const bytes);

/// Creates a new `NSTDString` from a raw C string.
/// Parameters:
///     `const NSTDChar *const cstr` - The C string.
/// Returns: `NSTDString string` - The new NSTD string.
NSTDAPI NSTDString nstd_string_from_cstring(const NSTDChar *const cstr);

/// Creates a string view from an `NSTDString`.
/// Parameters:
///     `const NSTDString *const string` - The string.
/// Returns: `NSTDStr str` - The new string view.
NSTDAPI NSTDStr nstd_string_as_str(const NSTDString *const string);

/// Creates an `NSTDSlice` from an `NSTDString`.
/// Parameters:
///     `const NSTDString *const string` - The string.
/// Returns: `NSTDSlice slice` - The new slice.
NSTDAPI NSTDSlice nstd_string_as_slice(const NSTDString *const string);

/// Gets the length of a string.
/// Parameters:
///     `const NSTDString *const string` - The string.
/// Returns: `NSTDUSize len` - The length of the UTF-8 encoded string, -1 on error.
NSTDAPI NSTDUSize nstd_string_len(const NSTDString *const string);

/// Returns the number of bytes used by this string.
/// Parameters:
///     `const NSTDString *const string` - The string.
/// Returns: `NSTDUSize len` - The number of bytes in the string.
NSTDAPI NSTDUSize nstd_string_byte_len(const NSTDString *const string);

/// Pushes an `NSTDUnichar` to an `NSTDString`.
/// Parameters:
///     `NSTDString *const string` - The string.
///     `const NSTDUnichar chr` - The unicode character to push to the string.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_string_push(NSTDString *const string, const NSTDUnichar chr);

/// Removes an `NSTDUnichar` from the end of an `NSTDString`.
/// Parameters:
///     `NSTDString *const string` - The string.
/// Returns: `NSTDUnichar chr` - The unichar that was popped off the string, fill char on error.
NSTDAPI NSTDUnichar nstd_string_pop(NSTDString *const string);

/// Extends an `NSTDString` by an `NSTDSlice` of `NSTDUnichar`s.
/// Parameters:
///     `NSTDString *const string` - The string.
///     `const NSTDSlice *const chars` - `NSTDSlice` of `NSTDUnichar`s.
NSTDAPI void nstd_string_extend(NSTDString *const string, const NSTDSlice *const chars);

/// Converts an `NSTDFloat32` to a c-string.
/// To keep this from leaking memory please call `nstd_string_free`.
/// Parameters:
///     `const NSTDFloat32 num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_string_from_f32(const NSTDFloat32 num);
/// Converts a `NSTDFloat64` to a c-string.
/// To keep this from leaking memory please call `nstd_string_free`.
/// Parameters:
///     `const NSTDFloat64 num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_string_from_f64(const NSTDFloat64 num);

/// Converts a `NSTDInt8` to a c-string.
/// To keep this from leaking memory please call `nstd_string_free`.
/// Parameters:
///     `const NSTDInt8 num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_string_from_i8(const NSTDInt8 num);
/// Converts an `NSTDUInt8` to a c-string.
/// To keep this from leaking memory please call `nstd_string_free`.
/// Parameters:
///     `const NSTDUInt8 num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_string_from_u8(const NSTDUInt8 num);

/// Converts a `NSTDInt16` to a c-string.
/// To keep this from leaking memory please call `nstd_string_free`.
/// Parameters:
///     `const NSTDInt16 num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_string_from_i16(const NSTDInt16 num);
/// Converts an `NSTDUInt16` to a c-string.
/// To keep this from leaking memory please call `nstd_string_free`.
/// Parameters:
///     `const NSTDUInt16 num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_string_from_u16(const NSTDUInt16 num);

/// Converts a `NSTDInt32` to a c-string.
/// To keep this from leaking memory please call `nstd_string_free`.
/// Parameters:
///     `const NSTDInt32 num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_string_from_i32(const NSTDInt32 num);
/// Converts an `NSTDUInt32` to a c-string.
/// To keep this from leaking memory please call `nstd_string_free`.
/// Parameters:
///     `const NSTDUInt32 num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_string_from_u32(const NSTDUInt32 num);

/// Converts a `NSTDInt64` to a c-string.
/// To keep this from leaking memory please call `nstd_string_free`.
/// Parameters:
///     `const NSTDInt64 num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_string_from_i64(const NSTDInt64 num);
/// Converts an `NSTDUInt64` to a c-string.
/// To keep this from leaking memory please call `nstd_string_free`.
/// Parameters:
///     `const NSTDUInt64 num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_string_from_u64(const NSTDUInt64 num);

/// Converts an `NSTDISize` to a c-string.
/// To keep this from leaking memory please call `nstd_string_free`.
/// Parameters:
///     `const NSTDISize num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_string_from_isize(const NSTDISize num);
/// Converts an `NSTDUSize` to a c-string.
/// To keep this from leaking memory please call `nstd_string_free`.
/// Parameters:
///     `const NSTDUSize num` - The number to be converted.
/// Returns: `NSTDString str` - The number represented as a string.
NSTDAPI NSTDString nstd_string_from_usize(const NSTDUSize num);

/// Frees an `NSTDString` instance.
/// Parameters:
///     `NSTDString *const string` - Pointer to a string.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_string_free(NSTDString *const string);

NSTDCPPEND
#endif
