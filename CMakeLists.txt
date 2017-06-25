cmake_minimum_required(VERSION 3.7)
project(app)

set(CMAKE_CXX_STANDARD 11)

set(SOURCE_FILES
        Cargo.toml)

add_executable(app ${SOURCE_FILES})