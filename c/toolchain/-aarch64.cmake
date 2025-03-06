# CMake toolchain file for Raspberry Pi aarch64

# Assume Linux target
set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR aarch64)

# Set path to the aarch64 cross compilers installed via MSYS2
set(CMAKE_C_COMPILER   /usr/bin/aarch64-linux-gnu-gcc)
set(CMAKE_CXX_COMPILER /usr/bin/aarch64-linux-gnu-g++)

# Set sysroot to the minimal sysroot directory
set(CMAKE_SYSROOT "./sysroot")

# Pass additional flags as needed
set(CMAKE_C_FLAGS "--sysroot=${CMAKE_SYSROOT}" CACHE STRING "" FORCE)
set(CMAKE_CXX_FLAGS "--sysroot=${CMAKE_SYSROOT}" CACHE STRING "" FORCE)