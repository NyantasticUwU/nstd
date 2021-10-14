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
        - `sys`
    - `std`: Higher level modules.
        - `audio`
        - `def`
        - `env`
        - `events`
        - `fs`
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
```
> git clone https://github.com/NyantasticUwU/nstd.git
> cd nstd
> mkdir lib
```
Repeat this next step for "lib" in (audio, env, events, fs, gui, image, io, math, net, os, proc,
str, thread, time), or alternatively use "build.py".
```
> cd src/std/lib
> cargo build --release
> cd ../../../
```
The static libraries will be built to "src/std/lib/target/release". Move them into the "lib"
directory that was created earlier. Note that on Windows and Linux, "build.py" will do this for you.

Now let us continue. We will be using CMake, and the CMakeLists.txt file includes test code at the
bottom of the file, you can comment those out with the '#' char.
```
> mkdir build
> cd build
> cmake ..
```
