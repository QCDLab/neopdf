cmake_minimum_required(VERSION 3.16)
project(neopdf_gui LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Automatically run MOC, UIC, and RCC
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)

# Find Qt6
find_package(Qt6 REQUIRED COMPONENTS Core Gui Widgets Charts)

# Find the neopdf_capi header directory
find_path(
    NEOPDF_CAPI_INCLUDE_DIR
    NAMES NeoPDF.hpp neopdf_capi.h
    PATHS /usr/local/include/neopdf_capi
    NO_DEFAULT_PATH
)
if(NOT NEOPDF_CAPI_INCLUDE_DIR)
    message(FATAL_ERROR "Could not find NeoPDF.hpp/neopdf_capi.h")
endif()

# Find the neopdf_capi library
find_library(
    NEOPDF_CAPI_LIBRARY
    NAMES neopdf_capi
    PATHS /usr/local/lib
    NO_DEFAULT_PATH
)
if(NOT NEOPDF_CAPI_LIBRARY)
    message(FATAL_ERROR "Could not find libneopdf_capi")
endif()

message(STATUS "Found neopdf_capi include dir: ${NEOPDF_CAPI_INCLUDE_DIR}")
message(STATUS "Found neopdf_capi library: ${NEOPDF_CAPI_LIBRARY}")

# --- End of finding neopdf_capi ---

# Define the executable
add_executable(neopdf_gui
    main.cpp
    MainWindow.cpp
    MainWindow.hpp
)

# Link against Qt6 libraries
target_link_libraries(neopdf_gui
    PRIVATE
    Qt6::Core
    Qt6::Gui
    Qt6::Widgets
    Qt6::Charts
)

# Include directories
target_include_directories(neopdf_gui
    PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${NEOPDF_CAPI_INCLUDE_DIR}
)

# Link against the neopdf_capi library
target_link_libraries(neopdf_gui
    PRIVATE
    ${NEOPDF_CAPI_LIBRARY}
)

# Add preprocessor definition to disable std::filesystem integration in Qt
# to solve macOS SDK compatibility issues with conda-forge compilers.
target_compile_definitions(neopdf_gui PRIVATE QT_NO_FILESYSTEM)

install(TARGETS neopdf_gui
    RUNTIME DESTINATION bin
)
