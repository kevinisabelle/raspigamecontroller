cmake_minimum_required(VERSION 3.10)

project(GamepadKi CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

find_package(PkgConfig REQUIRED)
pkg_check_modules(GLIB REQUIRED gio-2.0)

include_directories(${GLIB_INCLUDE_DIRS})
link_directories(${GLIB_LIBRARY_DIRS})
add_definitions(${GLIB_CFLAGS_OTHER})

file(GLOB SOURCES "src/*.cpp")

add_executable(gamepadki ${SOURCES})

target_include_directories(gamepadki PRIVATE ./include)
target_include_directories(gamepadki PRIVATE ./include/bluez)
target_link_libraries(gamepadki ${GLIB_LIBRARIES})