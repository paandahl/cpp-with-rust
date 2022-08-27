cmake_minimum_required(VERSION 3.22)
project(cpp_with_rust)

set(CMAKE_CXX_STANDARD 17)

set(CARGO_MANIFEST ${CMAKE_SOURCE_DIR}/Cargo.toml)
set(CARGO_TARGET_DIR ${CMAKE_SOURCE_DIR}/target)

if(CMAKE_BUILD_TYPE STREQUAL Debug)
    set(CARGO_PROFILE dev)
    set(CARGO_BUILD_FOLDER ${CARGO_TARGET_DIR}/debug)
elseif(CMAKE_BUILD_TYPE STREQUAL RelWithDebInfo)
    set(CARGO_PROFILE release-with-debug)
    set(CARGO_BUILD_FOLDER ${CARGO_TARGET_DIR}/release-with-debug)
else()
    set(CARGO_PROFILE release)
    set(CARGO_BUILD_FOLDER ${CARGO_TARGET_DIR}/release)
endif()

set(BLOBSTORE_SOURCE_FILE ${CMAKE_SOURCE_DIR}/src/lib.rs)
set(BLOBSTORE_BRIDGE_CPP ${CARGO_TARGET_DIR}/cxxbridge/blobstore/src/lib.rs.cc)
set(BLOBSTORE_LIB ${CARGO_BUILD_FOLDER}/${CMAKE_STATIC_LIBRARY_PREFIX}blobstore${CMAKE_STATIC_LIBRARY_SUFFIX})

# Add a custom command that builds the rust crate and generates C++ bridge code
add_custom_command(
        OUTPUT ${BLOBSTORE_BRIDGE_CPP} ${BLOBSTORE_LIB}
        COMMAND cargo build --profile ${CARGO_PROFILE} --manifest-path ${CARGO_MANIFEST}
        DEPENDS ${BLOBSTORE_SOURCE_FILE}
        USES_TERMINAL
)

add_executable(cpp_with_rust src/main.cpp src/multibuf.cpp ${BLOBSTORE_BRIDGE_CPP})

target_include_directories(
        cpp_with_rust
        PRIVATE
        include/
        ${CARGO_TARGET_DIR}/cxxbridge/blobstore/src/
)

target_link_libraries(cpp_with_rust ${BLOBSTORE_LIB})

# Windows-only configuration
if(WIN32)
    target_link_libraries(cpp_with_rust userenv ws2_32 bcrypt)
    set_target_properties(
            cpp_with_rust
            PROPERTIES
            MSVC_RUNTIME_LIBRARY "MultiThreadedDLL"
            RUNTIME_OUTPUT_DIRECTORY_DEBUG ${CMAKE_BINARY_DIR}
            RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}
    )
endif()
