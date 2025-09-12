# PortableApps Without Punishment - Technical Specification

## System Architecture

### Components
- **universal-launcher.exe**: Runtime wrapper for PortableApps
- **replacer.exe**: Deployment tool that patches apps
- **restore-punishment.exe**: Reversal tool
- **installer.exe**: NSIS GUI wrapper
- **icocop.exe**: Icon preservation utility (dependency)

## Universal Launcher

### Purpose
Transparent wrapper that eliminates "not closed properly" warnings by cleaning runtime state before launching the original PortableApp.

### Operation
1. Identifies target app from its own filename
2. Backs up INI configuration to `*_original.ini`
3. Removes runtime tracking files from Data directory:
   - `PortableApps.comLauncherRuntimeData-*.ini`
   - `*.lock`, `*.pid`, `*.tmp` files
   - `Temp/` directory contents
4. Executes `[AppName]_original.exe` with all arguments
5. Exits immediately

### Build Configuration
- Windows GUI subsystem (no console window)
- Self-identifying based on filename
- No external configuration required

## Replacer Tool

### Purpose
Finds and patches PortableApps in a directory tree.

### Operation
1. Recursively searches for `*Portable.exe` files
2. For each app found:
   - Creates backup: `[AppName]_original.exe`
   - Deploys universal-launcher as `[AppName].exe`
   - Uses icocop.exe to preserve original icons
3. Updates previously patched apps with new launcher version
4. Logs all operations to timestamped file

### Command Interface
```
replacer.exe <directory> <universal-launcher-path> [--log <logfile>]
```

## Restore Tool

### Purpose
Reverses all modifications made by replacer.

### Operation
1. Finds all `*_original.exe` files
2. Restores original launcher
3. Removes universal launcher
4. Cleans backup files

## NSIS Installer

### Purpose
GUI wrapper for command-line tools with user-friendly interface.

### Modes
- **Remove Punishment**: Deploys universal launcher to apps
- **Restore Punishment**: Reverses all modifications

### Command-Line Interface
```
installer.exe [/S] [/RESTORE] [/D=<path>]
```
- `/S`: Silent mode
- `/RESTORE`: Restore mode (default is remove)
- `/D=<path>`: Target directory

### Registry Usage
Stores last used directory at:
```
HKCU\Software\PortableAppsWithoutPunishment\LastDirectory
```

## File Structure

### Patched App Layout
```
AppNamePortable/
├── AppNamePortable.exe          (universal launcher)
├── AppNamePortable_original.exe (original launcher backup)
└── Data/
    └── [runtime files cleaned on each launch]
```

### Build Requirements
- Rust toolchain with Windows target
- NSIS for installer
- All components statically linked