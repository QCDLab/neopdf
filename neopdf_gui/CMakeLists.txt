cmake_minimum_required(VERSION 3.16)
project(neopdf_gui VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)

find_package(Qt6 REQUIRED COMPONENTS Core Gui Widgets Charts)

set(CARGO_C_INSTALL_PREFIX "$ENV{CARGO_C_INSTALL_PREFIX}")
if(DEFINED CARGO_C_INSTALL_PREFIX AND NOT CARGO_C_INSTALL_PREFIX STREQUAL "")
    list(APPEND NEOPDF_CAPI_INCLUDE_SEARCH_PATHS "${CARGO_C_INSTALL_PREFIX}/include/neopdf_capi")
    list(APPEND NEOPDF_CAPI_LIBRARY_SEARCH_PATHS "${CARGO_C_INSTALL_PREFIX}/lib")
else()
    message(FATAL_ERROR "CARGO_C_INSTALL_PREFIX must be specified.")
endif()

find_path(
    NEOPDF_CAPI_INCLUDE_DIR
    NAMES NeoPDF.hpp neopdf_capi.h
    PATHS ${NEOPDF_CAPI_INCLUDE_SEARCH_PATHS}
    NO_DEFAULT_PATH
)
if(NOT NEOPDF_CAPI_INCLUDE_DIR)
    message(FATAL_ERROR "Could not find NeoPDF.hpp/neopdf_capi.h.")
endif()

find_library(
    NEOPDF_CAPI_LIBRARY
    NAMES neopdf_capi
    PATHS ${NEOPDF_CAPI_LIBRARY_SEARCH_PATHS}
    NO_DEFAULT_PATH
)
if(NOT NEOPDF_CAPI_LIBRARY)
    message(FATAL_ERROR "Could not find libneopdf_capi.")
endif()

message(STATUS "Found neopdf_capi library: ${NEOPDF_CAPI_LIBRARY}")
message(STATUS "Found neopdf_capi include dir: ${NEOPDF_CAPI_INCLUDE_DIR}")

add_executable(neopdf_gui
    main.cpp
    MainWindow.cpp
    MainWindow.hpp
)

if(APPLE)
    set_target_properties(neopdf_gui PROPERTIES
        MACOSX_BUNDLE TRUE
    )
endif()

target_link_libraries(neopdf_gui
    PRIVATE
    Qt6::Core
    Qt6::Gui
    Qt6::Widgets
    Qt6::Charts
)

target_include_directories(neopdf_gui
    PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${NEOPDF_CAPI_INCLUDE_DIR}
)

target_link_libraries(neopdf_gui
    PRIVATE
    ${NEOPDF_CAPI_LIBRARY}
)

target_compile_definitions(neopdf_gui PRIVATE QT_NO_FILESYSTEM)

install(TARGETS neopdf_gui
    BUNDLE DESTINATION .
    RUNTIME DESTINATION bin
)

if(WIN32)
    set(QT_DEPLOY_TOOL windeployqt)
elseif(APPLE)
    set(QT_DEPLOY_TOOL macdeployqt)
endif()

if(QT_DEPLOY_TOOL)
    find_program(QT_DEPLOY_TOOL_PATH ${QT_DEPLOY_TOOL} HINTS ${Qt6_BIN_DIR})
    if(QT_DEPLOY_TOOL_PATH)
        if(WIN32)
            set(APP_PATH "\"${CMAKE_INSTALL_PREFIX}/bin/neopdf_gui.exe\"")
        elseif(APPLE)
            set(APP_PATH "\"${CMAKE_INSTALL_PREFIX}/neopdf_gui.app\"")
        endif()
        install(CODE "execute_process(COMMAND ${QT_DEPLOY_TOOL_PATH} ${APP_PATH})")
    endif()
endif()

set(CPACK_PACKAGE_VERSION ${PROJECT_VERSION})
set(CPACK_GENERATOR "TGZ")
if(APPLE)
    set(CPACK_GENERATOR "DragNDrop")
endif()

include(CPack)
