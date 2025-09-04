# PortableApps Without Punishment

**Are you tired of being punished by your PortableApps?**

*"The application did not close properly last time, YOU IRRESPONSIBLE FILTHY HUMAN!"*

<div align="center">
  <img src="https://raw.githubusercontent.com/freeideas/PortableApps-Without-Punishment/main/docs/kick.jpg" alt="Stop the punishment!" />
</div>
<hr/>

Sound familiar? Every time your app crashes, loses power, or Windows decides to update without asking, you're forced to acknowledge your "mistake" and then manually re-launch the app, as if you committed a medieval sin!

**This ends now.** PortableApps Without Punishment eliminates these annoying warnings forever by automatically cleaning up runtime data before each launch.

(Feel free to ignore the rest of the clever banter here, and just run this program: [PortableApps Without Punishment 2025-09-04-1859.exe](https://github.com/freeideas/PortableApps-Without-Punishment/raw/main/releases/PortableApps%20Without%20Punishment%202025-09-04-1859.exe))

## Problem

PortableApps applications track their runtime state using temporary files. When these applications crash, are force-closed, or experience power loss, these runtime files remain on disk. The next time you launch the app, you're greeted with an annoying warning message about improper shutdown - even though the application works perfectly fine. Then you must endure the shame of manually launching the app again.

## Solution

PortableApps Without Punishment is a simple installer that automatically patches all your PortableApps to eliminate the warning messages. It works by replacing each app's launcher with a transparent wrapper that cleans up runtime files before launching.

The UniversalLauncher acts as a transparent replacement that:
- **Preserves configuration** by backing up the INI file to `[AppName]_original.ini`
- **Silently cleans up** all runtime data files that trigger the warning
- **Launches the original** PortableApps launcher (renamed to `[AppName]_original.exe`)
- **Passes all arguments** to ensure full compatibility

## Features

- **Toggle Operation**: Single installer can both remove AND restore punishment
- **Universal Design**: Works with ANY PortableApps application
- **Smart Updates**: Automatically updates previously patched apps with fixed launchers
- **GUI & Silent Mode**: User-friendly installer with command-line automation support
- **Memory Safe**: Built with Rust for guaranteed memory safety and reliability
- **No Installation Required**: Single executable, no files left behind
- **Directory Memory**: Remembers your PortableApps location for convenience
- **Fully Transparent**: Users won't know it's there (except no more warnings!)
- **Preserves Functionality**: All command-line arguments and features work normally
- **Self-Aware**: Automatically detects which app it's wrapping based on filename

## Usage

Simply download and run the latest `PortableApps Without Punishment YYYY-MM-DD-HHMM.exe` from the releases folder:

1. **Choose Operation**: Select either "Remove Punishment" (default) or "Restore Punishment"
   - **Remove Punishment**: Eliminates the annoying warnings forever
   - **Restore Punishment**: Brings back the original warnings (for masochists)

2. **Select Directory**: Choose your PortableApps location:
   - A single PortableApp directory (e.g., `D:\PortableApps\FirefoxPortable`)
   - A directory containing multiple PortableApps (e.g., `D:\PortableApps`)

3. **Run**: Watch as your apps are patched or restored

4. **Done!** The installer remembers your directory for future runs

### Silent Mode

For automation or batch processing:

```cmd
# Remove punishment (default)
"PortableApps Without Punishment.exe" /S /D="D:\PortableApps"

# Restore punishment
"PortableApps Without Punishment.exe" /S /RESTORE /D="D:\PortableApps"
```

**Parameters:**
- `/S` - Silent mode (no GUI)
- `/RESTORE` - Restore punishment mode (default is remove)
- `/D=path` - Target directory path

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
│   │   └── src/main.rs         # Fixed: no console window, proper INI backup
│   ├── replacer/                # Command-line tool that finds and patches PortableApps
│   │   └── src/main.rs         # Fixed: updates already-patched apps
│   ├── restore-punishment/      # Tool that reverses the patching process
│   │   └── src/main.rs         # Restores original launchers and punishment
│   └── Cargo.toml               # Rust workspace configuration
├── installer/                   # NSIS installer files
│   └── installer.nsi            # Toggle installer (Remove/Restore modes)
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

## Recent Fixes & Improvements

**Version 2025-09-04-1859 includes major fixes:**

- **Fixed Console Window Flash**: Universal Launcher now builds as Windows GUI application, eliminating the brief console window that appeared when launching apps
- **Fixed INI Backup Issue**: Corrected INI file backup to create `[AppName]_original.ini` instead of incorrect `launcher.ini`
- **Smart Update System**: Replacer now updates already-patched apps with fixed components instead of skipping them
- **Toggle Operation**: Single installer can both remove and restore punishment
- **Silent Mode Support**: Command-line automation with `/S /RESTORE /D=path` parameters

**If you used a previous version that had issues:**
Simply run the new installer in "Remove Punishment" mode on your PortableApps directory. It will automatically update all previously patched apps with the fixed Universal Launcher.

## Troubleshooting

### "Original launcher not found" error
- Run the latest installer to update with fixed components
- Ensure the original launcher was renamed to `[AppName]_original.exe`
- Check that both files are in the same directory

### Application doesn't start
- Try running the installer again to update components
- Verify the Universal Launcher has the exact name of the original launcher
- Check that `[AppName]_original.exe` is the actual PortableApps launcher

### Console window still appears
- This was fixed in the latest version - run the installer again to update
- The Universal Launcher now builds as Windows GUI application

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