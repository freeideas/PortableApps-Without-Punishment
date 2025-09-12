// RestorePunishment - Restores original PortableApps launchers
// Reverses the patching done by PortableApps Without Punishment

#![cfg_attr(windows, windows_subsystem = "windows")]

use anyhow::{Context, Result};
use regex::Regex;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[cfg(windows)]
use winapi::um::winuser::{MessageBoxW, MB_ICONERROR, MB_OK, MB_ICONQUESTION, MB_YESNO, IDYES};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }
    
    let portableapps_dir = &args[1];
    
    // Validate path
    if !Path::new(portableapps_dir).exists() {
        show_error(&format!("Directory '{}' does not exist", portableapps_dir));
        std::process::exit(1);
    }
    
    println!("PortableApps Punishment Restorer");
    println!("================================");
    println!("PortableApps Directory: {}", portableapps_dir);
    println!();
    
    // Find all patched PortableApps
    let apps = find_patched_apps(portableapps_dir)?;
    
    if apps.is_empty() {
        println!("No patched PortableApps found in the specified directory.");
        println!("Looking for pattern: */PortableApps/*Portable/*_original.exe");
        return Ok(());
    }
    
    println!("Found {} patched PortableApps:", apps.len());
    for (i, app) in apps.iter().enumerate() {
        let app_name = app.parent()
            .and_then(|p| p.file_name())
            .unwrap_or_default()
            .to_string_lossy();
        println!("  {}. {}", i + 1, app_name);
    }
    println!();
    
    // Ask for confirmation
    if !confirm_restore() {
        println!("Restoration cancelled by user.");
        return Ok(());
    }
    
    // Process each app
    let mut success_count = 0;
    for original_launcher in &apps {
        match restore_app_launcher(original_launcher) {
            Ok(_) => {
                let app_name = original_launcher.parent()
                    .and_then(|p| p.file_name())
                    .unwrap_or_default()
                    .to_string_lossy();
                println!("✅ Restored: {}", app_name);
                success_count += 1;
            }
            Err(e) => {
                let app_name = original_launcher.parent()
                    .and_then(|p| p.file_name())
                    .unwrap_or_default()
                    .to_string_lossy();
                println!("❌ Failed: {} - {}", app_name, e);
            }
        }
    }
    
    println!();
    println!("Summary: {} of {} apps successfully restored to original state", success_count, apps.len());
    
    if success_count > 0 {
        println!();
        println!("💀 PortableApps punishment has been restored!");
        println!("   Your apps will now show 'not closed properly' warnings again.");
    }
    
    Ok(())
}

fn print_usage() {
    println!("PortableApps Punishment Restorer");
    println!("================================");
    println!();
    println!("Usage: restore-punishment.exe <PortableAppsDirectory>");
    println!();
    println!("Arguments:");
    println!("  PortableAppsDirectory - Root directory containing patched PortableApps");
    println!();
    println!("Example:");
    println!("  restore-punishment.exe D:\\PortableApps");
    println!();
    println!("This tool reverses the patching done by PortableApps Without Punishment,");
    println!("restoring the original launchers and bringing back the punishment warnings.");
}

fn find_patched_apps<P: AsRef<Path>>(root_dir: P) -> Result<Vec<PathBuf>> {
    let mut apps = Vec::new();
    let original_regex = Regex::new(r"(?i)portable_original\.exe$").unwrap();
    
    for entry in WalkDir::new(root_dir) {
        let entry = entry?;
        let path = entry.path();
        
        // Skip directories
        if !path.is_file() {
            continue;
        }
        
        // Check if it matches *Portable_original.exe pattern
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if original_regex.is_match(file_name) {
                // Additional validation: check for proper PortableApp structure
                if let Some(app_dir) = path.parent() {
                    let app_info_path = app_dir.join("App").join("AppInfo");
                    if app_info_path.exists() {
                        apps.push(path.to_path_buf());
                    }
                }
            }
        }
    }
    
    Ok(apps)
}

fn restore_app_launcher<P: AsRef<Path>>(original_launcher_path: P) -> Result<()> {
    let original_launcher_path = original_launcher_path.as_ref();
    let launcher_dir = original_launcher_path.parent()
        .context("Failed to get launcher directory")?;
    
    // Get the base name by removing "_original" suffix
    let original_name = original_launcher_path.file_stem()
        .context("Failed to get original filename")?
        .to_string_lossy();
    
    let base_name = if let Some(stripped) = original_name.strip_suffix("_original") {
        stripped
    } else {
        return Err(anyhow::anyhow!("File doesn't match expected '_original.exe' pattern"));
    };
    
    let current_launcher = launcher_dir.join(format!("{}.exe", base_name));
    let backup_ini = launcher_dir.join("App/AppInfo/Launcher").join(format!("{}_original.ini", base_name));
    
    // Check if current launcher exists (it should be our universal launcher)
    if !current_launcher.exists() {
        return Err(anyhow::anyhow!("Current launcher {} not found", current_launcher.display()));
    }
    
    // Step 1: Remove the universal launcher
    fs::remove_file(&current_launcher)
        .context("Failed to remove universal launcher")?;
    
    // Step 2: Restore original launcher
    fs::rename(original_launcher_path, &current_launcher)
        .context("Failed to restore original launcher")?;
    
    // Step 3: Remove backup INI file if it exists
    if backup_ini.exists() {
        let _ = fs::remove_file(&backup_ini);
    }
    
    Ok(())
}

fn confirm_restore() -> bool {
    #[cfg(windows)]
    {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        
        let message = "Are you sure you want to restore punishment to your PortableApps?\n\nThis will bring back the 'not closed properly' warnings.\n\nYou can remove punishment again by running the installer.";
        let title = "Confirm Punishment Restoration";
        
        let message_wide: Vec<u16> = OsStr::new(message)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let title_wide: Vec<u16> = OsStr::new(title)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        
        unsafe {
            let result = MessageBoxW(
                std::ptr::null_mut(),
                message_wide.as_ptr(),
                title_wide.as_ptr(),
                MB_YESNO | MB_ICONQUESTION,
            );
            result == IDYES
        }
    }
    
    #[cfg(not(windows))]
    {
        println!("Are you sure you want to restore punishment to your PortableApps?");
        println!();
        println!("This will bring back the 'not closed properly' warnings.");
        println!();
        println!("You can remove punishment again by running the installer.");
        println!();
        print!("Continue? (y/N): ");
        
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim().to_lowercase();
                let confirmed = input == "y" || input == "yes";
                if confirmed {
                    logger.log("User confirmed restoration");
                } else {
                    logger.log("User cancelled restoration");
                }
                confirmed
            }
            Err(_) => false,
        }
    }
}

#[cfg(windows)]
fn show_error(message: &str) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    
    let title = "PortableApps Restoration Error";
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