[![autoit crate](https://img.shields.io/crates/v/autoit.svg)](https://crates.io/crates/autoit) {{badges}}

# {{crate}}

{{readme}}

## Build

Only tested with nightly-x86_64-pc-windows-msvc. Not sure if it will work with gnu (mingw).

To build and run I have to set INCLUDE, LIB, LIBCLANG_PATH and PATH in a "VS2015 x64 Native Tools Command Prompt":

```batch
set INCLUDE=%INCLUDE%;c:\AutoItX;C:\Program Files (x86)\Windows Kits\10\Include\10.0.10150.0\ucrt

set LIB=%LIB%;C:\Program Files (x86)\Windows Kits\10\Lib\10.0.10150.0\ucrt\x64;C:\AutoItX

set PATH=%PATH%;c:\AutoItX

set LIBCLANG_PATH=D:\LLVM\bin
```

License: {{license}}