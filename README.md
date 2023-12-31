# crankit-sys

![rustc](https://img.shields.io/badge/rustc-1.72+-blue?logo=rust)

Rust bindings for the [playdate C API](https://sdk.play.date/inside-playdate-with-c),
generated at build-time based on the playdate SDK found at `$PLAYDATE_SDK_PATH`.

## Status: Discontinued

This project is discontinued in favor of [`playdate-sys`](https://crates.io/crates/playdate-sys)


## Requirements

The bindings are generated at build-time and require:

* [Playdate SDK](https://play.date/dev/#cardSDK) must be installed
* An environment variable `PLAYDATE_SDK_PATH` must point to the playdate SDK installation folder
* `clang` must be installed

## MSRV

The minimum supported rust version is currently `1.72`.

It will be updated when required, and that will not be considered a breaking change (it can happen in a minor version).

## Unlicense

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org/>
