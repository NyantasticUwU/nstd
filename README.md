# About
NSTD is a library that is meant to be cross-platform, and fairly safe (any function that can fail
will return an error code indicating if it has or hasn't) while having a plethora of features and
functionality. The goal is to have one API for any platform, any language, and any use case.

# Platform support
The `core` module can be used ***ANYWHERE***. It doesn't rely on ***ANY*** other libraries,
including standard libraries. Other modules will work on *most* platforms and have been tested to
build for Windows, macOS, Linux, and Redox.

# Module overview
- `nstd` - Cross platform CFFI based library.
    - `alloc` - Heap allocation.
        - `allocator` - Custom vtable struct for memory allocation.
        - `heap` - Similar to Rust's Box.
    - `audio` - Audio IO.
    - `collections` - Collection types.
        - `bit_mask` - Customizable bit mask with a small memory footprint.
        - `vec` - A dynamically sized array.
    - `core` - Contains modules that don't require an operating system to be used.
        - `char_types` - Functions specific to character types.
        - `cstr` - C string examination.
        - `def` - Commonly used typedefs.
        - `float_types` - Functions specific to floating point types.
        - `int_types` - Functions specific to integer types.
        - `math` - Low level math.
        - `platform` - Platform identification.
        - `pointer` - Pointer type.
        - `range` - Range types.
        - `slice` - View into a memory.
        - `str` - UTF-8 string slice type.
    - `env` - Environment specific functionality and identification.
    - `events` - Event loops.
    - `fs` - File system.
    - `gl` - Low level graphics library.
    - `gui` - Graphical user interface library.
    - `image` - Raw image data IO.
    - `input` - Keyboard/Mouse/Touchpad input.
        - `key` - Keyboard types.
        - `mouse` - Mouse types.
        - `touch` - Touchpad types.
    - `io` - Standard IO.
    - `math` - High level math.
    - `net` - Networking.
    - `os` - Operating system specific functionality.
        - `linux` - OS support for Linux.
            - `alloc` - Low level memory allocation for Linux.
        - `windows` - OS support for Windows.
            - `alloc` - Low level memory allocation for Windows.
            - `def` - Commonly used Windows typedefs.
            - `io` - Windows standard IO.
    - `proc` - Process management.
    - `string` - Dynamically sized UTF-8 encoded string.
    - `sys` - System identification.
    - `thread` - Threading API.
    - `time` - Library for getting system times.

# How to build
```
cargo build --release --features ""
```
Where after "--features", inside the quotation marks, you would list each module seperated by spaces
and prefixed with "nstd_", though the `nstd_os` module is a bit different, and has it's own
features such as `nstd_os_alloc` which can be enabled seperately along with the `nstd_os` feature.
`nstd_core` is built by default. For building nstd as a C library, you should also use the "clib"
feature, to build the module for C ABI.

Example:
```
cargo build --release --features "clib nstd_alloc nstd_os nstd_os_io nstd_string"
```
Alternatively you can also use
```
cargo build --release --all-features
```
to build with all modules.
