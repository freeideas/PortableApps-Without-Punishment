// Universal PortableApps Launcher - Rust Edition
// Eliminates "application did not close properly" warnings by cleaning up runtime data

#![cfg_attr(windows, windows_subsystem = "windows")]

use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[cfg(not(windows))]
use std::process::Command;

#[cfg(windows)]
use winapi::um::{
    errhandlingapi::GetLastError,
    processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW},
    winuser::{MessageBoxW, MB_ICONERROR, MB_OK},
};

fn main() -> Result<()> {
    // Capture original working directory before any operations
    let original_cwd = env::current_dir().context("Failed to get current working directory")?;
    
    let current_exe = env::current_exe().context("Failed to get current executable path")?;
    let app_dir = current_exe.parent().context("Failed to get app directory")?;
    let wrapper_name = current_exe
        .file_stem()
        .context("Failed to get wrapper filename")?
        .to_string_lossy();
    
    let original_launcher = app_dir.join(format!("{}_original.exe", wrapper_name));
    
    if !original_launcher.exists() {
        show_error(&format!(
            "Original launcher not found:\n{}\n\nThis wrapper expects the original PortableApps launcher to be renamed to '{}_original.exe'.\nPlease use the PortableApps Without Punishment installer.",
            original_launcher.display(),
            wrapper_name
        ));
        std::process::exit(1);
    }
    
    // Step 1: Clean up runtime data
    cleanup_runtime_data(app_dir)?;
    
    // Step 2: Copy the appropriate INI file
    copy_ini_file(app_dir, &wrapper_name)?;
    
    // Step 3: Launch the original with all arguments, preserving original working directory
    launch_original(&original_launcher, &original_cwd)?;
    
    Ok(())
}

fn cleanup_runtime_data(app_dir: &Path) -> Result<()> {
    let data_dir = app_dir.join("Data");
    if !data_dir.exists() {
        return Ok(());
    }
    
    // Clean up runtime data files matching pattern
    let pattern = format!("{}/**/PortableApps.comLauncherRuntimeData-*.ini", data_dir.display());
    if let Ok(paths) = glob::glob(&pattern) {
        for entry in paths {
            if let Ok(path) = entry {
                let _ = fs::remove_file(path);
            }
        }
    }
    
    // Clean up settings directory
    let settings_dir = data_dir.join("settings");
    if settings_dir.exists() {
        for entry in WalkDir::new(&settings_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if matches!(ext.to_str(), Some("lock" | "pid" | "tmp" | "temp")) {
                    let _ = fs::remove_file(path);
                }
            }
        }
    }
    
    // Clean up PID files in Data root
    for entry in WalkDir::new(&data_dir).max_depth(1).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "pid" {
                let _ = fs::remove_file(path);
            }
        }
    }
    
    // Clean up temp directory
    let temp_dir = data_dir.join("Temp");
    if temp_dir.exists() {
        for entry in WalkDir::new(&temp_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path != temp_dir {
                if path.is_file() {
                    let _ = fs::remove_file(path);
                } else if path.is_dir() {
                    let _ = fs::remove_dir_all(path);
                }
            }
        }
    }
    
    Ok(())
}

fn copy_ini_file(app_dir: &Path, wrapper_name: &str) -> Result<()> {
    let source_ini = app_dir.join("App/AppInfo/Launcher").join(format!("{}.ini", wrapper_name));
    let backup_ini = app_dir.join("App/AppInfo/Launcher").join(format!("{}_original.ini", wrapper_name));
    
    // Always create/update the backup INI file for the original launcher to use
    if source_ini.exists() {
        fs::copy(&source_ini, &backup_ini)
            .with_context(|| format!("Failed to backup INI file from {} to {}", 
                source_ini.display(), backup_ini.display()))?;
    }
    
    Ok(())
}

fn launch_original(original_launcher: &Path, original_cwd: &Path) -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    
    #[cfg(windows)]
    {
        // Use Windows API for more control
        launch_with_winapi(original_launcher, &args, original_cwd)
    }
    
    #[cfg(not(windows))]
    {
        // Use standard process spawning for other platforms
        Command::new(original_launcher)
            .args(&args)
            .current_dir(original_cwd)
            .spawn()
            .context("Failed to launch original launcher")?;
        Ok(())
    }
}

#[cfg(windows)]
fn launch_with_winapi(original_launcher: &Path, args: &[String], original_cwd: &Path) -> Result<()> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    
    let command_line = if args.is_empty() {
        format!("\"{}\"", original_launcher.display())
    } else {
        let quoted_args: Vec<String> = args.iter()
            .map(|arg| if arg.contains(' ') { 
                format!("\"{}\"", arg) 
            } else { 
                arg.clone() 
            })
            .collect();
        format!("\"{}\" {}", original_launcher.display(), quoted_args.join(" "))
    };
    
    let command_line_wide: Vec<u16> = OsStr::new(&command_line)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    
    let original_cwd_wide: Vec<u16> = original_cwd.as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    
    unsafe {
        let mut startup_info: STARTUPINFOW = std::mem::zeroed();
        startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
        
        let mut process_info: PROCESS_INFORMATION = std::mem::zeroed();
        
        let success = CreateProcessW(
            std::ptr::null(),
            command_line_wide.as_ptr() as *mut u16,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0, // Don't inherit handles
            0, // No creation flags
            std::ptr::null_mut(),
            original_cwd_wide.as_ptr(),
            &mut startup_info,
            &mut process_info,
        );
        
        if success == 0 {
            let error = GetLastError();
            show_error(&format!(
                "Failed to launch the original launcher:\n{}\n\nWindows error code: {}",
                original_launcher.display(),
                error
            ));
            std::process::exit(1);
        }
        
        // Close handles
        winapi::um::handleapi::CloseHandle(process_info.hProcess);
        winapi::um::handleapi::CloseHandle(process_info.hThread);
    }
    
    Ok(())
}

#[cfg(windows)]
fn show_error(message: &str) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    
    let title = "PortableApps Launcher";
    let message_wide: Vec<u16> = OsStr::new(message)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let title_wide: Vec<u16> = OsStr::new(title)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    
    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            message_wide.as_ptr(),
            title_wide.as_ptr(),
            MB_OK | MB_ICONERROR,
        );
    }
}

#[cfg(not(windows))]
fn show_error(message: &str) {
    eprintln!("Error: {}", message);
}