# Minimal application mixing C++ and Rust

This example uses [cxx](https://github.com/dtolnay/cxx) 
to generate bindings between
C++ and Rust, and integrates the two parts through CMake.

It is basically an inverted version of cxx's 
[demo code](https://github.com/dtolnay/cxx/tree/master/demo),
using C++ for the entry point and a MultiBuf class, while 
implementing a simple blobstore-library in Rust.

## Building and running the code

```shell
  mkdir build && cd build
  cmake ..
  make
  ./cpp_with_rust
```

## Technical notes

* As opposed to the original 
  [cxx demo](https://github.com/dtolnay/cxx/tree/master/demo),
  `build.rs` only generates the C++ bridging code, without
  compiling it. Instead we pass it in to the CMake build
  by referencing it in `add_executable()`.
