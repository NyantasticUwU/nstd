# What is this?
Big ol' CFFI based library for anything cross platform.

# Why does this exist?
The library is meant to be cross-platform, and fairly safe (any function that can fail will return
an error code indicating if it has or hasn't) while having a plethora of features and functionality.
The goal is to have one API for any platform, any language, and any use case.

# Where can it be used?
The `core` module can be used ***ANYWHERE***. It doesn't rely on ***ANY*** other libraries,
including standard libraries. The `std` module will work on *most* platforms and has been tested to
build for Windows, macOS, and Linux.

# Module overview:
- `nstd`: Everything NSTD.
    - `core`: Modules that don't rely on any other libraries.
        - `arch`
        - `char_types`
        - `def`
        - `float_types`
        - `int_types`
        - `mem`
        - `platform`
        - `slice`
    - `std`: Higher level modules.
        - `alloc`
        - `audio`
        - `collections`
            - `vec`
        - `def`
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

# How to build:
## Single binary:
```
cargo build --release --features ""
```
Where after "--features", inside the quotation marks, you would list each module seperated by spaces
and prefixed with "nstd_". nstd_core is built by default.

Example:
```
cargo build --release --features "nstd_collections nstd_io nstd_thread"
```
Alternatively you can also use
```
cargo build --release --all-features
```
to build with all modules.
## Individual binaries:
```
python3 build.py build --release
```
This will place each binary into the module's "target/release" directory, so in
"src/std/alloc/target/release" you would find the "nstd_alloc.[dll|so]" and "nstd_alloc.[lib|a]"
files.
