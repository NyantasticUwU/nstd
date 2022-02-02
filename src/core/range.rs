use core::ops::Range;

/// Generates the `NSTD*Range[*]` struct.
macro_rules! nstd_gen_range_struct {
    ($name: ident, $type: ty) => {
        /// Represents a signed range.
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, Hash)]
        pub struct $name {
            /// Start of the range (included).
            pub start: $type,
            /// End of the range (exluded).
            pub end: $type,
        }
        impl Into<Range<$type>> for $name {
            #[inline]
            fn into(self) -> Range<$type> {
                self.start..self.end
            }
        }
    };
}
nstd_gen_range_struct!(NSTDIRange8, i8);
nstd_gen_range_struct!(NSTDURange8, u8);
nstd_gen_range_struct!(NSTDIRange16, i16);
nstd_gen_range_struct!(NSTDURange16, u16);
nstd_gen_range_struct!(NSTDIRange32, i32);
nstd_gen_range_struct!(NSTDURange32, u32);
nstd_gen_range_struct!(NSTDIRange64, i64);
nstd_gen_range_struct!(NSTDURange64, u64);
nstd_gen_range_struct!(NSTDIRange, isize);
nstd_gen_range_struct!(NSTDURange, usize);
