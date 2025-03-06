cmake_minimum_required(VERSION 3.10)

project(GamepadKi C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED True)

# Find the GLib package using pkg-config
find_package(PkgConfig REQUIRED)
pkg_check_modules(GLIB REQUIRED gio-2.0)

# Include GLib headers
include_directories(${GLIB_INCLUDE_DIRS})

# Link GLib libraries
link_directories(${GLIB_LIBRARY_DIRS})
add_definitions(${GLIB_CFLAGS_OTHER})

# Collect all source files in the src directory
file(GLOB SOURCES "src/*.c")

# Create an executable target from the collected source files
add_executable(gamepadki ${SOURCES})

target_include_directories(gamepadki PRIVATE ./include)
target_include_directories(gamepadki PRIVATE ./include/bluez)

# Link the executable with GLib libraries
target_link_libraries(gamepadki ${GLIB_LIBRARIES})