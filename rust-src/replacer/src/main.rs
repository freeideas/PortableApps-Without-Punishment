// PortableApps Without Punishment Replacer - Rust Edition
// Finds and patches PortableApps to eliminate "not closed properly" warnings

use anyhow::{Context, Result};
use regex::Regex;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[cfg(windows)]
use winapi::um::winuser::{MessageBoxW, MB_ICONERROR, MB_OK};

const UNIVERSAL_LAUNCHER_NAME: &str = "UniversalLauncher.exe";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }
    
    let portableapps_dir = &args[1];
    let universal_launcher_path = if args.len() >= 3 {
        args[2].clone()
    } else {
        UNIVERSAL_LAUNCHER_NAME.to_string()
    };
    
    // Validate paths
    if !Path::new(portableapps_dir).exists() {
        show_error(&format!("Directory '{}' does not exist", portableapps_dir));
        std::process::exit(1);
    }
    
    if !Path::new(&universal_launcher_path).exists() {
        show_error(&format!(
            "UniversalLauncher.exe not found at '{}'\nPlease ensure UniversalLauncher.exe is in the current directory or provide its path",
            universal_launcher_path
        ));
        std::process::exit(1);
    }
    
    println!("PortableApps Without Punishment Replacer");
    println!("=========================================");
    println!("PortableApps Directory: {}", portableapps_dir);
    println!("Universal Launcher: {}", universal_launcher_path);
    println!();
    
    // Find all PortableApps
    let apps = find_portable_apps(portableapps_dir)?;
    
    if apps.is_empty() {
        println!("No PortableApps found in the specified directory.");
        println!("Looking for pattern: */PortableApps/*Portable/*Portable.exe");
        return Ok(());
    }
    
    println!("Found {} PortableApps:", apps.len());
    for (i, app) in apps.iter().enumerate() {
        println!("  {}. {}", i + 1, app.file_name().unwrap_or_default().to_string_lossy());
    }
    println!();
    
    // Process each app
    let mut success_count = 0;
    let mut updated_count = 0;
    for app_launcher in &apps {
        match replace_app_launcher(app_launcher, &universal_launcher_path) {
            Ok(action) => {
                let app_name = app_launcher.file_name().unwrap_or_default().to_string_lossy();
                match action.as_str() {
                    "updated" => {
                        println!("🔄 Updated: {} (already patched, universal launcher updated)", app_name);
                        updated_count += 1;
                    }
                    "patched" => {
                        println!("✅ Patched: {} (first-time patch)", app_name);
                        success_count += 1;
                    }
                    _ => {
                        println!("✅ Success: {}", app_name);
                        success_count += 1;
                    }
                }
            }
            Err(e) => {
                println!("❌ Failed: {} - {}", app_launcher.file_name().unwrap_or_default().to_string_lossy(), e);
            }
        }
    }
    
    println!();
    let total_processed = success_count + updated_count;
    if updated_count > 0 {
        println!("Summary: {} new patches, {} updates, {} total processed of {} apps found", 
                success_count, updated_count, total_processed, apps.len());
    } else {
        println!("Summary: {} of {} apps successfully patched", success_count, apps.len());
    }
    
    if total_processed > 0 {
        println!();
        if updated_count > 0 {
            println!("🎉 PortableApps have been patched/updated! No more 'not closed properly' warnings!");
        } else {
            println!("🎉 PortableApps have been patched! No more 'not closed properly' warnings!");
        }
    }
    
    Ok(())
}

fn print_usage() {
    println!("PortableApps Universal Launcher Replacer");
    println!("=========================================");
    println!();
    println!("Usage: replacer.exe <PortableAppsDirectory> [UniversalLauncher.exe]");
    println!();
    println!("Arguments:");
    println!("  PortableAppsDirectory - Root directory containing PortableApps (e.g., D:\\PortableApps)");
    println!("  UniversalLauncher.exe - Path to the universal launcher (default: looks in current dir)");
    println!();
    println!("Example:");
    println!("  replacer.exe D:\\PortableApps");
    println!("  replacer.exe D:\\PortableApps C:\\Tools\\UniversalLauncher.exe");
}

fn find_portable_apps<P: AsRef<Path>>(root_dir: P) -> Result<Vec<PathBuf>> {
    let mut apps = Vec::new();
    let portable_regex = Regex::new(r"(?i)portable\.exe$").unwrap();
    
    for entry in WalkDir::new(root_dir) {
        let entry = entry?;
        let path = entry.path();
        
        // Skip directories
        if !path.is_file() {
            continue;
        }
        
        // Check if it matches *Portable.exe pattern
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if portable_regex.is_match(file_name) {
                // Additional validation: check for proper PortableApp structure
                if let Some(app_dir) = path.parent() {
                    let app_info_path = app_dir.join("App").join("AppInfo");
                    if app_info_path.exists() {
                        // Include all valid PortableApps (patched or unpatched)
                        apps.push(path.to_path_buf());
                    }
                }
            }
        }
    }
    
    Ok(apps)
}

fn replace_app_launcher<P: AsRef<Path>>(launcher_path: P, universal_launcher_path: &str) -> Result<String> {
    let launcher_path = launcher_path.as_ref();
    let launcher_dir = launcher_path.parent()
        .context("Failed to get launcher directory")?;
    
    let _launcher_name = launcher_path.file_name()
        .context("Failed to get launcher filename")?
        .to_string_lossy();
    
    let base_name = launcher_path.file_stem()
        .context("Failed to get base filename")?
        .to_string_lossy();
    
    let original_name = format!("{}_original.exe", base_name);
    let original_path = launcher_dir.join(&original_name);
    
    // Handle already patched apps by updating the universal launcher
    if original_path.exists() {
        // App is already patched - just update the universal launcher
        fs::copy(universal_launcher_path, launcher_path)
            .context("failed to update universal launcher")?;
        return Ok("updated".to_string());
    }
    
    // Step 1: Rename original launcher to *_original.exe (first-time patch)
    fs::rename(launcher_path, &original_path)
        .with_context(|| format!("failed to rename original launcher"))?;
    
    // Step 2: Copy UniversalLauncher.exe to the original launcher name
    if let Err(e) = fs::copy(universal_launcher_path, launcher_path) {
        // Try to restore original on failure
        let _ = fs::rename(&original_path, launcher_path);
        return Err(e).context("failed to copy universal launcher");
    }
    
    // Step 3: Copy file permissions (on Unix-like systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        
        if let Ok(metadata) = fs::metadata(universal_launcher_path) {
            let permissions = metadata.permissions();
            let _ = fs::set_permissions(launcher_path, permissions);
        }
    }
    
    Ok("patched".to_string())
}

#[cfg(windows)]
fn show_error(message: &str) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    
    let title = "PortableApps Replacer";
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