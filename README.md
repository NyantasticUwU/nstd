# About
NSTD is a library that is meant to be cross-platform, and fairly safe (any function that can fail
will return an error code indicating if it has or hasn't) while having a plethora of features and
functionality. The goal is to have one API for any platform, any language, and any use case.

# Platform support
The `core` module can be used ***ANYWHERE***. It doesn't rely on ***ANY*** other libraries,
including standard libraries (except for the headers `stddef.h` and `stdint.h`). Other modules will
work on *most* platforms and have been tested to build for Windows, macOS, Linux, and Redox.

# Module overview
- `nstd` - Cross platform CFFI based library.
    - `alloc` - Heap allocation.
        - `allocator` - Custom vtable struct for memory allocation.
        - `heap` - Similar to Rust's Box.
    - `audio` - Audio IO.
    - `collections` - Collection types.
        - `bit_mask` - Customizable bit mask with a small memory footprint.
        - `rc` - Shared memory through a reference counter.
        - `stack` - A growable and shrinkable stack array type.
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
        - `file` - File IO.
    - `gl` - Low level graphics library.
        - `buffer` - GPU VRAM buffers.
        - `command` - Device commands.
            - `buffer` - GPU Command buffers.
            - `encoder` - GPU Command encoders.
        - `def` - Commonly used graphics related types.
        - `device` - Module for working with graphics devices.
            - `handle` - A handle to a graphics device.
            - `info` - Information about a graphics device.
        - `instance` - An instance of `wgpu`.
        - `render` - Rendering.
            - `pass` - Contains functions for working with a render pass.
            - `pipeline` - A rendering pipeline.
        - `shader` - GPU shader programs.
            - `module` - Shader modules.
        - `state` - The graphics library's state machine.
        - `surface` - Interaction with the display's surface.
            - `config` - The surface's configuration.
            - `texture` - Surface textures.
        - `texture` - Module for working with textures.
            - `view` - Texture views.
    - `gui` - Graphical user interface library.
        - `def` - Commonly used GUI related types.
        - `display` - A display/monitor handle.
    - `image` - Raw image data IO.
    - `input` - Keyboard/Mouse input.
        - `key` - Keyboard types.
        - `mouse` - Mouse types.
    - `io` - Standard IO.
        - `input_stream` - Input stream type.
        - `io_stream` - Stream type for both input and output.
        - `output_stream` - Output stream type.
        - `stderr` - The standard error stream.
        - `stdin` - The standard input stream.
        - `stdout` - The standard output stream.
        - `stream` - Base stream type.
    - `math` - High level math.
    - `net` - Networking.
        - `tcp` - TCP networking.
        - `udp` - UDP networking.
    - `os` - Operating system specific functionality.
        - `linux` - OS support for Linux.
            - `alloc` - Low level memory allocation for Linux.
        - `windows` - OS support for Windows.
            - `alloc` - Low level memory allocation for Windows.
                - `heap` - Windows heap management.
            - `def` - Commonly used Windows typedefs.
            - `io` - Windows standard IO.
                - `handle` - Handle type for Windows standard IO streams.
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
features such as `nstd_os_alloc` which can be enabled seperately, `nstd_os` is not required.
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
