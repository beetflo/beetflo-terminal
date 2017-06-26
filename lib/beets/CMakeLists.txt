# Set the name and the supported language of the project
project(beets C)

# Set the minimum version of cmake required to build this project
cmake_minimum_required(VERSION 2.6)

# Use the package PkgConfig to detect GTK+ headers/library files
find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK3 REQUIRED gtk+-3.0)
# Setup CMake to use GTK+, tell the compiler where to look for headers
# and to the linker where to look for libraries
include_directories(src ${GTK3_INCLUDE_DIRS})
link_directories(${GTK3_LIBRARY_DIRS})

# Add other flags to the compiler
add_definitions(${GTK3_CFLAGS_OTHER})
# Add an executable compiled from hello.c

# add_library(beets_shared SHARED src/beets.c)
add_library(beets_static STATIC src/beets.c)

add_executable(beets src/beets.c)

# Link the target to the GTK+ libraries
target_link_libraries(beets_static ${GTK3_LIBRARIES})

install (TARGETS beets DESTINATION bin)
install (TARGETS beets_static DESTINATION lib)

