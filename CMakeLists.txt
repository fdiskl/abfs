cmake_minimum_required(VERSION 3.10)

project(abfs C)

# Include directories
include_directories(include)

# Common source files (without main)
file(GLOB_RECURSE COMMON_SOURCES "src/*.c")
list(FILTER COMMON_SOURCES EXCLUDE REGEX ".*main.c$")

# Main app
add_executable(abfs src/main.c ${COMMON_SOURCES})
set_target_properties(abfs PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
)

# Tests (each test with its own main)
file(GLOB TEST_FILES "tests/*.c")
foreach(TEST_FILE ${TEST_FILES})
    # Get filename without extension
    get_filename_component(TEST_NAME ${TEST_FILE} NAME_WE)

    # Create a test executable
    add_executable(${TEST_NAME} ${TEST_FILE} ${COMMON_SOURCES})
    set_target_properties(${TEST_NAME} PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin/tests
    )
endforeach()
