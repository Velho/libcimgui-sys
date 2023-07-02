# cimgui-sys

FFI rust bindings for the Dear ImGui library.
Makes use of the [cimgui - Github](https://github.com/cimgui/cimgui)
cimgui is a thing c-api wrapper generated for the imgui.
This wrapper has been configured to make use of the SDL2 backend primarily, but others can be used as well.

These bindings are generated using the documentation from <https://rust-lang.github.io/rust-bindgen/library-usage.html>

## Dependencies

| | version |
|-|-|
| Dear ImGui | 1.89.5 |

## Building

Project was tested using,

|||
|-|-|
| MSVC | 17 (2022) |
| CMake | 3.25.0 |
| LLVM | 16.0.4 |

Linux **TBD**

1. Clone git repository

    `git clone libcimgui-sys`

2. Update git submodules

    `git submodule update --init --recursive`

3. Build submodules

    * By default the project is configured to be built under
    _deps/cimgui/.build_

    * Configure the project using CMake

    `cmake -E make_directory deps/cimgui/.build`

    `cmake -S deps/cimgui/ -B deps/cimgui/.build`

    * Build cimgui

    `cmake --build deps/cimgui/.build`

4. Build project

    `cargo build`

