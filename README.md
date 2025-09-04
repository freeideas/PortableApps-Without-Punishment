# PortableApps Without Punishment

**Are you tired of being punished by your PortableApps?**

*"The application did not close properly last time it was run and will now clean up."*

Sound familiar? Every time your app crashes, loses power, or Windows decides to update without asking, you're forced to acknowledge your "mistake" and wait through the cleanup process. It's like digital detention for a crime you didn't commit!

**This ends now.** PortableApps Without Punishment eliminates these annoying warnings forever by automatically cleaning up runtime data before each launch.

![Stop the punishment!](https://raw.githubusercontent.com/yourusername/PortableAppsWithoutPunishment/main/docs/kick.png)

## Problem

PortableApps applications track their runtime state using temporary files. When these applications crash, are force-closed, or experience power loss, these runtime files remain on disk. The next time you launch the app, you're greeted with an annoying warning message about improper shutdown - even though the application works perfectly fine. Then you are punished by having to manually launch the app again -- as if you are doing penance for a sin you committed.

## Solution

PortableApps Without Punishment is a simple installer that automatically patches all your PortableApps to eliminate the warning messages. It works by replacing each app's launcher with a transparent wrapper that cleans up runtime files before launching.

The UniversalLauncher acts as a transparent replacement that:
- **Preserves configuration** by backing up the INI file to `[AppName]_original.ini`
- **Silently cleans up** all runtime data files that trigger the warning
- **Launches the original** PortableApps launcher (renamed to `[AppName]_original.exe`)
- **Passes all arguments** to ensure full compatibility

## Features

- **Universal Design**: Works with ANY PortableApps application
- **GUI Installer**: User-friendly installation process
- **Batch Processing**: Patch all your PortableApps at once
- **Fully Transparent**: Users won't know it's there (except no more warnings!)
- **Preserves Functionality**: All command-line arguments and features work normally
- **Lightweight**: UniversalLauncher is only ~45KB with minimal overhead
- **Self-Aware**: Automatically detects which app it's wrapping based on filename

## Installation

Simply download and run `PortableApps Without Punishment.exe` from the releases folder:

1. The installer will open a dialog to select your PortableApps location
2. Choose either:
   - A single PortableApp directory (e.g., `D:\PortableApps\FirefoxPortable`)
   - A directory containing multiple PortableApps (e.g., `D:\PortableApps`)
3. Click Install and watch as your apps are patched
4. Done! No more annoying warnings

## How It Works

The Universal Launcher performs these steps each time it runs:

1. **Configuration Backup**: Copies `App/AppInfo/Launcher/[AppName].ini` to `[AppName]_original.ini` (overwrites if exists)
2. **Cleanup Phase**: Removes all `PortableApps.comLauncherRuntimeData-*.ini` files and other state tracking files (`.lock`, `.pid`, `.tmp`) from the Data directory that trigger the "not closed properly" warning
3. **Launch Original**: Executes `[AppName]_original.exe` (the renamed original launcher) with all provided arguments
4. **Exit Cleanly**: Closes immediately after launching, adding zero overhead to the running application

## Building from Source

### Requirements
- MinGW-w64 compiler (for C code cross-compilation on Linux/Mac)
- Go compiler (for building NoPunishReplacer)
- NSIS (for building the GUI installer)

### Compile Commands

#### UniversalLauncher (C)

##### Linux/Mac (Cross-compile for Windows):
```bash
i686-w64-mingw32-gcc -o UniversalLauncher.exe c-src/UniversalLauncher.c -lshlwapi -mwindows -static
```

##### Windows (MinGW):
```cmd
gcc -o UniversalLauncher.exe c-src/UniversalLauncher.c -lshlwapi -mwindows -static
```

##### Windows (Visual Studio):
```cmd
cl /Fe:UniversalLauncher.exe c-src/UniversalLauncher.c shlwapi.lib user32.lib /link /SUBSYSTEM:WINDOWS
```

#### NoPunishReplacer (Go):
```bash
GOOS=windows GOARCH=amd64 go build -o NoPunishReplacer.exe go-src/replacer.go
```

#### GUI Installer (NSIS):
```bash
makensis installer/installer.nsi
```

## Project Structure

```
PortableAppsWithoutPunishment/
├── README.md                    # This file
├── c-src/                       # C source code
│   └── UniversalLauncher.c      # Universal launcher that replaces each PortableApp launcher
├── go-src/                      # Go source code
│   └── replacer.go              # Command-line tool that finds and replaces PortableApps launchers
├── installer/                   # NSIS installer files
│   └── installer.nsi            # NSIS installer script
├── releases/                    # Pre-built installer for distribution
│   └── PortableApps Without Punishment.exe  # The installer
└── docs/                        # Additional documentation
    └── BUILD.md                 # Detailed build instructions
```

## Compatibility

Tested and working with:
- Firefox Portable
- Chrome Portable
- Notepad++ Portable
- VLC Portable
- 7-Zip Portable
- GIMP Portable
- LibreOffice Portable
- And all other PortableApps.com format applications

## Technical Details

### Files Cleaned

The wrapper removes these file types to prevent the warning:
- `PortableApps.comLauncherRuntimeData-*.ini` - Main runtime tracking files
- `*.lock` files in Data/settings/ - Process lock files
- `*.pid` files - Process ID tracking files
- `*.tmp` and `*.temp` files - Temporary state files
- Contents of Data/Temp/ directory - Temporary working files

### Safety

The wrapper only removes temporary runtime files. It never touches:
- Your actual settings and preferences
- User data files
- Application files
- Configuration that should persist

## Troubleshooting

### "Original launcher not found" error
- Ensure the original launcher was renamed to `[AppName]_original.exe`
- Check that both files are in the same directory

### Application doesn't start
- Verify the Universal Launcher has the exact name of the original launcher
- Check that `[AppName]_original.exe` is the actual PortableApps launcher, not the main program

### Configuration not loading
- The Universal Launcher automatically backs up the INI file to `[AppName]_original.ini`
- Check that `App/AppInfo/Launcher/[AppName].ini` exists

## Contributing

Improvements and bug fixes are welcome! The codebase is intentionally simple and well-commented for easy modification.

### Key areas for contribution:
- Additional cleanup patterns for specific apps
- Improved error handling and logging
- Enhanced app detection patterns
- Support for non-standard PortableApp structures

## License

This project is released into the public domain. Use it freely for any purpose.

## Author

Created to save humanity from clicking "OK" on unnecessary warning dialogs, one PortableApp at a time.

---

*No more punishment for improper shutdowns!*