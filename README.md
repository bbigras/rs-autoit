[![autoit crate](https://img.shields.io/crates/v/autoit.svg)](https://crates.io/crates/autoit) [![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/bbigras/rs-autoit.svg)](http://isitmaintained.com/project/bbigras/rs-autoit "Average time to resolve an issue")
[![Percentage of issues still open](http://isitmaintained.com/badge/open/bbigras/rs-autoit.svg)](http://isitmaintained.com/project/bbigras/rs-autoit "Percentage of issues still open")
[![Dependabot Status](https://api.dependabot.com/badges/status?host=github&repo=bbigras/rs-autoit)](https://dependabot.com)

# autoit

Rust binding for [AutoItX](https://www.autoitscript.com/site/autoit/)

**(Work in progress):** If you need any function just open an issue or a PR.

## Examples
```rust
use autoit::{init, mouse_move, mouse_get_pos};

init();

mouse_move(0, 0, Some(0));
assert_eq!(mouse_get_pos(), (0, 0));

mouse_move(50, 50, Some(0));
assert_eq!(mouse_get_pos(), (50, 50));
```

## Build

Only tested with nightly-x86_64-pc-windows-msvc. Not sure if it will work with gnu (mingw).

To build and run I have to set INCLUDE, LIB, LIBCLANG_PATH and PATH in a "VS2015 x64 Native Tools Command Prompt":

```batch
set INCLUDE=%INCLUDE%;c:\AutoItX;C:\Program Files (x86)\Windows Kits\10\Include\10.0.10150.0\ucrt

set LIB=%LIB%;C:\Program Files (x86)\Windows Kits\10\Lib\10.0.10150.0\ucrt\x64;C:\AutoItX

set PATH=%PATH%;c:\AutoItX

set LIBCLANG_PATH=D:\LLVM\bin
```

License: Unlicense/MIT
