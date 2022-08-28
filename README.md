# Minimal application mixing C++ and Rust

This example uses [cxx](https://github.com/dtolnay/cxx) 
to generate bindings between
C++ and Rust, and integrates the two parts through CMake.

It is basically an inverted version of cxx's 
[demo code](https://github.com/dtolnay/cxx/tree/master/demo),
using C++ for the entry point and a MultiBuf class, while 
implementing a simple blobstore-library in Rust.

## How it works

In [lib.rs](src/lib.rs) we add bridge declarations for our Rust types:

```rust
#[cxx::bridge(namespace = "org::blobstore")]
mod ffi {
    // Rust types and signatures exposed to C++.
    extern "Rust" {
        type BlobstoreClient;
        fn new_blobstore_client() -> Box<BlobstoreClient>;
        fn put(&mut self, parts: Pin<&mut MultiBuf>) -> u64;
        ...
    }
}

fn new_blobstore_client() -> Box<BlobstoreClient> {
    Box::new(BlobstoreClient { blobs: HashMap::new() })
}

struct BlobstoreClient {
    blobs: HashMap<u64, Blob>,
}

impl BlobstoreClient {
    fn put(&mut self, mut parts: Pin<&mut MultiBuf>) -> u64 {
        ...
    }
}
```

In [build.rs](build.rs) we add logic to generate C++ bridging code from the declarations:

```rust
fn main() {
    cxx_build::bridge("src/lib.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
```

In [CMakeLists.txt](CMakeLists.txt) we add a custom command to trigger the Rust build:

```cmake
add_custom_command(
        OUTPUT ${BLOBSTORE_BRIDGE_CPP} ${BLOBSTORE_LIB}
        COMMAND cargo build --manifest-path ${BLOBSTORE_CARGO_MANIFEST}
        ...
)
```

In [main.cpp](src/main.cpp) we include the generated C++ header, construct Rust types,
and call their methods:

```c++
#include "lib.rs.h"

int main() {
    auto client = org::blobstore::new_blobstore_client();
    ...
    const auto blobid = client->put(buf);
}
```

The application also consumes C++ types from Rust (`MultiBuf`), and leverages shared types between the two 
languages (`BlobMetadata`).

To learn more about the bridging layer, check out 
[cxx's documentation](https://cxx.rs/).

## Building and running the code

```shell
  git clone git@github.com:paandahl/cpp-with-rust.git
  mkdir cpp-with-rust/build
  cd cpp-with-rust/build

  cmake ..
  cmake --build .
  ./cpp_with_rust
```

> **_NOTE:_** If you are using Windows, run these commands in the 
> *Developer PowerShell for VS*.

## Technical notes

* As opposed to the original 
  [cxx demo](https://github.com/dtolnay/cxx/tree/master/demo),
  `build.rs` only generates the C++ bridging code, without
  compiling it. Instead, we pass it in to the CMake build
  by referencing it in `add_executable()`.
* For simplicity, this example always builds the Rust 
  code in debug mode. 
  [See here](https://github.com/paandahl/cpp-with-rust/compare/main...cargo-release-builds) 
  for suggested changes to adhere to the specified `CMAKE_BUILD_TYPE`, and 
  moving the cargo output to within the CMake build tree.

## License

The code is available under the [MIT license](https://opensource.org/licenses/MIT).