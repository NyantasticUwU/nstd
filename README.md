# About
NSTD is a library that is meant to be cross-platform, and fairly safe (any function that can fail
will return an error code indicating if it has or hasn't) while having a plethora of features and
functionality. The goal is to have one API for any platform, any language, and any use case.

# Platform support
The `core` module can be used ***ANYWHERE***. It doesn't rely on ***ANY*** other libraries,
including standard libraries. Other modules will work on *most* platforms and have been tested to
build for Windows, macOS, Linux, and Redox.

# Module overview
- `nstd`
    - `alloc`
        - `heap`
    - `audio`
    - `collections`
        - `vec`
    - `core`
        - `char_types`
        - `def`
        - `float_types`
        - `int_types`
        - `math`
        - `platform`
        - `pointer`
        - `range`
        - `slice`
        - `str`
    - `env`
    - `events`
    - `fs`
    - `gl`
    - `gui`
    - `image`
    - `input`
        - `key`
        - `mouse`
        - `touch`
    - `io`
    - `math`
    - `net`
    - `os`
    - `proc`
    - `str`
    - `thread`
    - `time`

# How to build
```
cargo build --release --features ""
```
Where after "--features", inside the quotation marks, you would list each module seperated by spaces
and prefixed with "nstd_". nstd_core is built by default. For building nstd as a C library, you
should also use the "clib" feature, to build the module for C ABI.

Example:
```
cargo build --release --features "clib nstd_io nstd_str"
```
Alternatively you can also use
```
cargo build --release --all-features
```
to build with all modules.
