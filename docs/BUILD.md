# Build Instructions

This document provides detailed instructions for building the PortableApps Transparent Launcher Wrapper from source.

## Prerequisites

### Windows
- **MinGW-w64** or **Visual Studio** (2017 or later)
- Windows SDK (included with Visual Studio)

### Linux/Mac (for cross-compilation)
- **MinGW-w64** cross-compiler
- Install on Ubuntu/Debian: `sudo apt-get install mingw-w64`
- Install on Fedora: `sudo dnf install mingw64-gcc`
- Install on macOS: `brew install mingw-w64`

## Source Files

The main source file is located at:
```
src/PALTransparentWrapper_Final.c
```

This is the universal version that works with any PortableApps application.

### Other versions (for reference):
- `PALTransparentWrapper.c` - Original basic version
- `PALTransparentWrapper_v2.c` - Added INI file handling
- `PALTransparentWrapper_v3.c` - Improved cleanup patterns
- `PALTransparentWrapper_Final.c` - Final universal version

## Build Commands

### Linux/Mac Cross-Compilation

For 32-bit Windows executable (recommended for maximum compatibility):
```bash
i686-w64-mingw32-gcc -o bin/UniversalPALWrapper.exe \
    src/PALTransparentWrapper_Final.c \
    -lshlwapi -mwindows -static
```

For 64-bit Windows executable:
```bash
x86_64-w64-mingw32-gcc -o bin/UniversalPALWrapper64.exe \
    src/PALTransparentWrapper_Final.c \
    -lshlwapi -mwindows -static
```

### Windows - MinGW

```cmd
gcc -o bin\UniversalPALWrapper.exe ^
    src\PALTransparentWrapper_Final.c ^
    -lshlwapi -mwindows -static
```

### Windows - Visual Studio Command Prompt

```cmd
cl /Fe:bin\UniversalPALWrapper.exe ^
    src\PALTransparentWrapper_Final.c ^
    shlwapi.lib user32.lib ^
    /link /SUBSYSTEM:WINDOWS
```

### Windows - Visual Studio IDE

1. Create a new Empty C++ Project
2. Add `src/PALTransparentWrapper_Final.c` to the project
3. Project Properties:
   - Configuration Type: Application (.exe)
   - Character Set: Use Multi-Byte Character Set
   - Subsystem: Windows (/SUBSYSTEM:WINDOWS)
4. Add to Linker > Input > Additional Dependencies:
   - shlwapi.lib
   - user32.lib
5. Build the project (F7)

## Compilation Flags Explained

- `-mwindows`: Creates a Windows GUI application (no console window)
- `-static`: Statically links libraries (no external DLL dependencies)
- `-lshlwapi`: Links the Shell API library (for path operations)
- `/SUBSYSTEM:WINDOWS`: Visual Studio equivalent of `-mwindows`

## Build Optimization

### For smaller file size:
```bash
i686-w64-mingw32-gcc -o bin/UniversalPALWrapper.exe \
    src/PALTransparentWrapper_Final.c \
    -lshlwapi -mwindows -static -Os -s
```
- `-Os`: Optimize for size
- `-s`: Strip symbols

### For better performance:
```bash
i686-w64-mingw32-gcc -o bin/UniversalPALWrapper.exe \
    src/PALTransparentWrapper_Final.c \
    -lshlwapi -mwindows -static -O3 -march=i686
```
- `-O3`: Maximum optimization
- `-march=i686`: Optimize for i686 architecture

## Testing Your Build

1. Copy the compiled executable to a PortableApps directory
2. Rename the original launcher (e.g., `NSISPortable.exe`) to `launcher.exe`
3. Rename your compiled wrapper to match the original (e.g., `NSISPortable.exe`)
4. Run the application and verify it launches without warnings

## Troubleshooting

### "undefined reference to `_imp__GetFileAttributesA@4`"
- Missing `-lshlwapi` flag
- On Visual Studio, add `shlwapi.lib` to linker inputs

### "undefined reference to `MessageBoxA`"
- Missing `-mwindows` flag or `user32.lib`
- On Visual Studio, add `user32.lib` to linker inputs

### Application opens a console window
- Missing `-mwindows` flag
- On Visual Studio, set Subsystem to Windows

### "cannot find -lshlwapi"
- MinGW installation is incomplete
- Try reinstalling MinGW-w64 with full Windows libraries

## Creating Application-Specific Builds

While the universal wrapper works for all apps, you can create specific versions:

```bash
# For NSIS Portable
i686-w64-mingw32-gcc -o bin/NSISPortable.exe \
    src/PALTransparentWrapper_Final.c \
    -lshlwapi -mwindows -static

# For Firefox Portable
i686-w64-mingw32-gcc -o bin/FirefoxPortable.exe \
    src/PALTransparentWrapper_Final.c \
    -lshlwapi -mwindows -static
```

The compiled executable name determines which app it wraps.

## Automated Build Script

Create a `build.sh` script for Linux/Mac:

```bash
#!/bin/bash
# Build script for PAL Transparent Wrapper

SOURCE="src/PALTransparentWrapper_Final.c"
OUTPUT="bin/UniversalPALWrapper.exe"
COMPILER="i686-w64-mingw32-gcc"
FLAGS="-lshlwapi -mwindows -static -Os -s"

echo "Building PAL Transparent Wrapper..."
mkdir -p bin
$COMPILER -o $OUTPUT $SOURCE $FLAGS

if [ $? -eq 0 ]; then
    echo "Build successful! Output: $OUTPUT"
    echo "Size: $(du -h $OUTPUT | cut -f1)"
else
    echo "Build failed!"
    exit 1
fi
```

Make it executable: `chmod +x build.sh`

## Continuous Integration

For GitHub Actions, use this workflow:

```yaml
name: Build Wrapper

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    
    - name: Install MinGW
      run: sudo apt-get update && sudo apt-get install -y mingw-w64
    
    - name: Build
      run: |
        mkdir -p bin
        i686-w64-mingw32-gcc -o bin/UniversalPALWrapper.exe \
          src/PALTransparentWrapper_Final.c \
          -lshlwapi -mwindows -static -Os -s
    
    - name: Upload artifact
      uses: actions/upload-artifact@v2
      with:
        name: UniversalPALWrapper
        path: bin/UniversalPALWrapper.exe
```

## Verification

After building, verify your executable:

```bash
# Check file type
file bin/UniversalPALWrapper.exe

# Check dependencies (should show no external DLLs with -static)
i686-w64-mingw32-objdump -p bin/UniversalPALWrapper.exe | grep DLL

# Check size (should be around 240KB)
ls -lh bin/UniversalPALWrapper.exe
```

## Contributing

When submitting builds:
1. Use the standard build flags shown above
2. Test with at least 3 different PortableApps
3. Include build environment details in pull requests
4. Sign commits if possible