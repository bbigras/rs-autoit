# rs-autoit
Rust binding for [AutoItX](https://www.autoitscript.com/site/autoit/)

**(Work in progress):** If you need any function just open an issue or a PR.

## Build

Only tested with nightly-x86_64-pc-windows-msvc. Not sure if it will work with gnu (mingw).

To build and run I have to set INCLUDE, LIB and PATH:

```batch
set INCLUDE=%INCLUDE%;c:\AutoItX;C:\Program Files (x86)\Windows Kits\10\Include\10.0.10150.0\ucrt

set LIB=%LIB%;C:\Program Files (x86)\Windows Kits\10\Lib\10.0.10150.0\ucrt\x64;C:\AutoItX

set PATH=%PATH%;c:\AutoItX
```
