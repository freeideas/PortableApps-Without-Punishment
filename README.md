# PortableApps Without Punishment

**Are you tired of being punished by your PortableApps?**

*"The application did not close properly last time, YOU IRRESPONSIBLE FILTHY HUMAN!"*

<div align="center">
  <img src="https://raw.githubusercontent.com/freeideas/PortableApps-Without-Punishment/main/docs/kick.jpg" alt="Stop the punishment!" />
</div>
<hr/>

Sound familiar? Every time your app crashes, loses power, or Windows decides to update without asking, you're forced to acknowledge your "mistake" and then manually re-launch the app, as if you committed a medieval sin!

**This ends now.** PortableApps Without Punishment eliminates these annoying warnings forever by automatically cleaning up runtime data before each launch.

(Feel free to ignore the rest of the clever banter here, and just run this program: [PortableApps Without Punishment 2025-09-04-1836.exe](https://github.com/freeideas/PortableApps-Without-Punishment/raw/main/releases/PortableApps%20Without%20Punishment%202025-09-04-1836.exe))

## Problem

PortableApps applications track their runtime state using temporary files. When these applications crash, are force-closed, or experience power loss, these runtime files remain on disk. The next time you launch the app, you're greeted with an annoying warning message about improper shutdown - even though the application works perfectly fine. Then you are punished by having to manually launch the app again -- as if you are doing penance for a medieval sin!

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
- **Memory Safe**: Built with Rust for guaranteed memory safety and reliability
- **Self-Aware**: Automatically detects which app it's wrapping based on filename

## Installation

Simply download and run the latest `PortableApps Without Punishment YYYY-MM-DD-HHMM.exe` from the releases folder:

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
- Rust compiler with Windows cross-compilation support
- MinGW-w64 linker (for cross-compilation on Linux/Mac)  
- NSIS (for building the GUI installer)

### Compile Commands

#### Rust Components (Cross-compile for Windows):
```bash
# Install Windows target
rustup target add x86_64-pc-windows-gnu

# Build both tools
cd rust-src
cargo build --release --target x86_64-pc-windows-gnu

# Copy to builds directory
cp target/x86_64-pc-windows-gnu/release/*.exe ../builds/rust/
```

#### GUI Installer (NSIS):
```bash
makensis installer/installer.nsi
```

## Project Structure

```
PortableAppsWithoutPunishment/
├── README.md                    # This file
├── rust-src/                    # Rust source code
│   ├── universal-launcher/      # Universal launcher that replaces each PortableApp launcher
│   │   └── src/main.rs
│   ├── replacer/                # Command-line tool that finds and patches PortableApps
│   │   └── src/main.rs
│   └── Cargo.toml               # Rust workspace configuration
├── installer/                   # NSIS installer files
│   └── installer.nsi            # NSIS installer script
├── releases/                    # Pre-built installer for distribution
│   └── PortableApps Without Punishment.exe  # The installer
└── docs/                        # Additional documentation
    └── kick.jpg                 # Punishment illustration
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
- Enhanced app detection patterns
- Support for non-standard PortableApp structures
- Icon extraction and injection for perfect app impersonation

## License

This project is released into the public domain. Use it freely for any purpose.

## Author

Created by a Gyrovague monk to save humanity from punishment for its sins.

---

*No more punishment for improper shutdowns!*