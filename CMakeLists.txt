cmake_minimum_required(VERSION 3.10)

project(abfs C)

# Include directories
include_directories(include)

# Source files
file(GLOB_RECURSE SOURCES "src/*.c")

# Create the executable
add_executable(abfs ${SOURCES})

# Optionally, you can specify the output directory for the executable
set_target_properties(abfs PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
)
