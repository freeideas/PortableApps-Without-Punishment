# PortableApps Without Punishment

**Are you tired of being punished by your PortableApps?**

*"The application did not close properly last time, YOU IRRESPONSIBLE FILTHY HUMAN!"*

<div align="center">
  <img src="https://raw.githubusercontent.com/freeideas/PortableApps-Without-Punishment/main/docs/kick.jpg" alt="Stop the punishment!" />
</div>
<hr/>

Sound familiar? Every time your app crashes, loses power, or Windows decides to update without asking, you're forced to acknowledge your "mistake" and then manually re-launch the app, as if you committed a medieval sin!

**This ends now.** PortableApps Without Punishment eliminates these annoying warnings forever by automatically cleaning up runtime data before each launch.

(Feel free to ignore the rest of the clever banter here, and just run this program: [PortableApps Without Punishment 2025-09-12-1305.exe](https://github.com/freeideas/PortableApps-Without-Punishment/raw/main/releases/PortableApps%20Without%20Punishment%202025-09-12-1305.exe))

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

Simply download and run the latest installer from the releases folder:

1. **Choose Operation**: Select either "Remove Punishment" (default) or "Restore Punishment"
   - **Remove Punishment**: Eliminates the annoying warnings forever
   - **Restore Punishment**: Brings back the original warnings (for masochists)

2. **Select Directory**: Choose your PortableApps location:
   - A single PortableApp directory (e.g., `D:\PortableApps\FirefoxPortable`)
   - A directory containing multiple PortableApps (e.g., `D:\PortableApps`)

3. **Run**: Watch as your apps are patched or restored

4. **Done!** The installer remembers your directory for future runs

### Command-Line Options

The installer supports command-line parameters to streamline operation:

```cmd
# Pre-fill directory for removing punishment
"PortableApps Without Punishment.exe" /D="D:\PortableApps"

# Pre-fill directory and pre-select restore mode
"PortableApps Without Punishment.exe" /RESTORE /D="D:\PortableApps"
```

**Parameters:**
- `/RESTORE` - Pre-selects restore punishment mode (default is remove)
- `/D=path` - Pre-fills the target directory path

### True Silent Mode

For fully automated operation without any GUI, use the included batch script:

```cmd
# Remove punishment silently
silent-patch.bat "D:\PortableApps" remove

# Restore punishment silently
silent-patch.bat "D:\PortableApps" restore
```

This script directly uses the command-line tools for complete automation.

## Is It Safe?

Yes! PortableApps Without Punishment:
- ✅ Creates backups of all modified files
- ✅ Can be completely reversed anytime
- ✅ Doesn't modify your actual programs or data
- ✅ Only removes temporary files that cause warnings
- ✅ Preserves all your settings and preferences

## For Developers

Technical documentation, architecture details, and build instructions are available in [SPECIFICATION.md](./SPECIFICATION.md).

### Testing the Installer

For automated testing without GUI:

```cmd
# Test remove mode
test-installer.bat remove

# Test restore mode  
test-installer.bat restore

# Or directly with TEST flag
"releases\PortableApps Without Punishment.exe" /TEST /D="C:\path\to\test"
```

The `/TEST` flag runs the installer silently for testing purposes.

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

## What Gets Modified?

Only the launcher files are changed:
- `AppNamePortable.exe` → Replaced with our wrapper
- `AppNamePortable_original.exe` → Your original launcher (backup)

**Nothing else is touched** - all your data, settings, and the actual programs remain exactly as they were.

## Latest Version

**Current Build: 2025-09-12-1305**

- Fixed installer finish page display issue
- Added `/TEST` flag for automated testing
- Improved icon preservation with icocop.exe
- Better error reporting and logging
- Windows 11 compatibility

If you installed an older version, just run the new installer to update.

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

## Need Help?

**Common Issues:**

- **Installer can't find apps** → Point it to your PortableApps folder (contains folders like FirefoxPortable)
- **Some apps still show warnings** → Run the installer again to update them
- **Want to undo changes** → Run installer and choose "Restore Punishment"

For other issues, check our [troubleshooting guide](#troubleshooting) below.

## License

This project is released into the public domain. Use it freely for any purpose.

## Author

Created by a Gyrovague monk to save humanity from punishment for its sins.

---

*No more punishment for improper shutdowns!*