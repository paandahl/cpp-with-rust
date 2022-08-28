cmake_minimum_required(VERSION 3.22)
project(cpp_with_rust)

set(CMAKE_CXX_STANDARD 17)

set(BLOBSTORE_CARGO_MANIFEST ${CMAKE_SOURCE_DIR}/Cargo.toml)
set(BLOBSTORE_SOURCE_FILE ${CMAKE_SOURCE_DIR}/src/lib.rs)
set(BLOBSTORE_BRIDGE_CPP ${CMAKE_SOURCE_DIR}/target/cxxbridge/blobstore/src/lib.rs.cc)
set(BLOBSTORE_LIB ${CMAKE_SOURCE_DIR}/target/debug/libblobstore${CMAKE_STATIC_LIBRARY_SUFFIX})

# Add a custom command that builds the rust crate and generates C++ bridge code
add_custom_command(
        OUTPUT ${BLOBSTORE_BRIDGE_CPP} ${BLOBSTORE_LIB}
        COMMAND cargo build --manifest-path ${BLOBSTORE_CARGO_MANIFEST}
        DEPENDS ${BLOBSTORE_SOURCE_FILE}
        USES_TERMINAL
)

add_executable(cpp_with_rust src/main.cpp src/multibuf.cpp ${BLOBSTORE_BRIDGE_CPP})

target_include_directories(
        cpp_with_rust
        PRIVATE
        include/
        target/cxxbridge/blobstore/src/
)

target_link_libraries(cpp_with_rust ${BLOBSTORE_LIB})
