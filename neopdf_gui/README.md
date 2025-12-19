# NeoPDF GUI Plotter

A C++/Qt6 application to plot and compare PDF sets using the `neopdf` library.

## Dependencies

- A C++ compiler (g++, clang, msvc)
- CMake (version 3.16 or higher)
- Qt6 (including the Charts module)
- Rust toolchain (for building the `neopdf_capi`)

## How to Build

1.  **Configure with CMake:**
    From the `neopdf_gui` directory, run:
    ```bash
    cmake -B build -DCMAKE_BUILD_TYPE=Release -DCMAKE_PREFIX_PATH=/usr/local/Cellar/qt/6.9.1/lib/cmake/Qt6
    ```
    This will configure the project and also trigger the build of the `neopdf_capi` Rust library.

2.  **Build the application:**
    ```bash
    cmake --build build
    ```

## How to Run

After a successful build, the executable will be located in the `build` directory.

```bash
./build/neopdf_gui
```
