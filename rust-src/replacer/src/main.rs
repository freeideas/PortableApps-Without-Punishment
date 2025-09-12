// PortableApps Without Punishment Replacer - Rust Edition
// Finds and patches PortableApps to eliminate "not closed properly" warnings

use anyhow::{Context, Result};
use regex::Regex;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;
use chrono::Local;

#[cfg(windows)]
use winapi::um::winuser::{MessageBoxW, MB_ICONERROR, MB_OK};

const UNIVERSAL_LAUNCHER_NAME: &str = "UniversalLauncher.exe";

struct Logger {
    file: Option<std::fs::File>,
    console: bool,
}

impl Logger {
    fn new(log_path: Option<&str>) -> Result<Self> {
        let file = if let Some(path) = log_path {
            Some(OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)
                .with_context(|| format!("Failed to create log file: {}", path))?)
        } else {
            None
        };
        
        Ok(Logger {
            file,
            console: true,
        })
    }
    
    fn log(&mut self, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_line = format!("[{}] {}", timestamp, message);
        
        if self.console {
            println!("{}", message);
        }
        
        if let Some(ref mut file) = self.file {
            writeln!(file, "{}", log_line).ok();
            file.flush().ok();
        }
    }
    
    fn log_raw(&mut self, message: &str) {
        if self.console {
            println!("{}", message);
        }
        
        if let Some(ref mut file) = self.file {
            writeln!(file, "{}", message).ok();
            file.flush().ok();
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }
    
    let portableapps_dir = &args[1];
    let mut universal_launcher_path = UNIVERSAL_LAUNCHER_NAME.to_string();
    let mut log_file_path = None;
    
    // Parse arguments
    let mut i = 2;
    while i < args.len() {
        if args[i] == "--log" && i + 1 < args.len() {
            log_file_path = Some(args[i + 1].clone());
            i += 2;
        } else if !args[i].starts_with("--") {
            universal_launcher_path = args[i].clone();
            i += 1;
        } else {
            i += 1;
        }
    }
    
    // Initialize logger
    let mut logger = Logger::new(log_file_path.as_deref())?;
    
    // Validate paths
    if !Path::new(portableapps_dir).exists() {
        let error_msg = format!("Directory '{}' does not exist", portableapps_dir);
        logger.log(&error_msg);
        show_error(&error_msg);
        std::process::exit(1);
    }
    
    if !Path::new(&universal_launcher_path).exists() {
        let error_msg = format!(
            "UniversalLauncher.exe not found at '{}'\nPlease ensure UniversalLauncher.exe is in the current directory or provide its path",
            universal_launcher_path
        );
        logger.log(&error_msg);
        show_error(&error_msg);
        std::process::exit(1);
    }
    
    logger.log("PortableApps Without Punishment Replacer");
    logger.log("=========================================");
    logger.log(&format!("PortableApps Directory: {}", portableapps_dir));
    logger.log(&format!("Universal Launcher: {}", universal_launcher_path));
    if let Some(ref log_path) = log_file_path {
        logger.log(&format!("Log File: {}", log_path));
    }
    logger.log("");
    
    // Find all PortableApps
    logger.log("Searching for PortableApps...");
    let apps = find_portable_apps(portableapps_dir, &mut logger)?;
    
    if apps.is_empty() {
        logger.log("No PortableApps found in the specified directory.");
        logger.log("Looking for pattern: */PortableApps/*Portable/*Portable.exe");
        return Ok(());
    }
    
    logger.log(&format!("Found {} PortableApps:", apps.len()));
    for (i, app) in apps.iter().enumerate() {
        logger.log(&format!("  {}. {}", i + 1, app.file_name().unwrap_or_default().to_string_lossy()));
    }
    logger.log("");
    
    // Process each app
    let mut success_count = 0;
    let mut updated_count = 0;
    logger.log("Processing applications...");
    for app_launcher in &apps {
        match replace_app_launcher(app_launcher, &universal_launcher_path, &mut logger) {
            Ok(action) => {
                let app_name = app_launcher.file_name().unwrap_or_default().to_string_lossy();
                match action.as_str() {
                    "updated" => {
                        logger.log(&format!("Updated: {} (already patched, universal launcher updated)", app_name));
                        updated_count += 1;
                    }
                    "patched" => {
                        logger.log(&format!("Patched: {} (first-time patch)", app_name));
                        success_count += 1;
                    }
                    _ => {
                        logger.log(&format!("Success: {}", app_name));
                        success_count += 1;
                    }
                }
            }
            Err(e) => {
                logger.log(&format!("Failed: {} - {}", app_launcher.file_name().unwrap_or_default().to_string_lossy(), e));
            }
        }
    }
    
    logger.log("");
    let total_processed = success_count + updated_count;
    if updated_count > 0 {
        logger.log(&format!("Summary: {} new patches, {} updates, {} total processed of {} apps found", 
                success_count, updated_count, total_processed, apps.len()));
    } else {
        logger.log(&format!("Summary: {} of {} apps successfully patched", success_count, apps.len()));
    }
    
    if total_processed > 0 {
        logger.log("");
        if updated_count > 0 {
            logger.log("SUCCESS: PortableApps have been patched/updated! No more 'not closed properly' warnings!");
        } else {
            logger.log("SUCCESS: PortableApps have been patched! No more 'not closed properly' warnings!");
        }
    }
    
    logger.log("");
    logger.log("Operation completed.");
    
    Ok(())
}

fn print_usage() {
    println!("PortableApps Universal Launcher Replacer");
    println!("=========================================");
    println!();
    println!("Usage: replacer.exe <PortableAppsDirectory> [UniversalLauncher.exe] [--log <logfile>]");
    println!();
    println!("Arguments:");
    println!("  PortableAppsDirectory - Root directory containing PortableApps (e.g., D:\\PortableApps)");
    println!("  UniversalLauncher.exe - Path to the universal launcher (default: looks in current dir)");
    println!("  --log <logfile>      - Write detailed log to specified file");
    println!();
    println!("Example:");
    println!("  replacer.exe D:\\PortableApps");
    println!("  replacer.exe D:\\PortableApps C:\\Tools\\UniversalLauncher.exe");
    println!("  replacer.exe D:\\PortableApps --log C:\\Temp\\replacer.log");
}

fn find_portable_apps<P: AsRef<Path>>(root_dir: P, logger: &mut Logger) -> Result<Vec<PathBuf>> {
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
                        logger.log(&format!("  Found: {}", path.display()));
                        apps.push(path.to_path_buf());
                    }
                }
            }
        }
    }
    
    Ok(apps)
}

fn find_icocop_exe(logger: &mut Logger) -> Option<PathBuf> {
    // Try to find icocop.exe in the same directory as the replacer
    if let Ok(current_exe) = env::current_exe() {
        if let Some(current_dir) = current_exe.parent() {
            let icocop_path = current_dir.join("icocop.exe");
            logger.log(&format!("  Looking for icocop.exe at: {}", icocop_path.display()));
            if icocop_path.exists() {
                logger.log("  Found icocop.exe");
                return Some(icocop_path);
            } else {
                logger.log("  icocop.exe not found at expected location");
            }
        }
    }
    
    // No fallback to PATH - we only use the bundled version
    logger.log("  Warning: icocop.exe not bundled with installer");
    None
}

fn copy_with_icon<P: AsRef<Path>>(source_exe: P, universal_launcher_path: &str, target_path: P, logger: &mut Logger) -> Result<()> {
    if let Some(icocop_path) = find_icocop_exe(logger) {
        // Use icocop.exe to copy universal launcher with icon from source exe
        // Arguments: ICON_SOURCE TARGET_EXE OUTPUT_EXE
        logger.log(&format!("  Using icocop.exe to copy icon from {} to {}", 
            source_exe.as_ref().display(), target_path.as_ref().display()));
        
        let output = Command::new(&icocop_path)
            .arg(&source_exe.as_ref())           // ICON_SOURCE (original exe with icons)
            .arg(universal_launcher_path)        // TARGET_EXE (universal launcher to copy)
            .arg(target_path.as_ref())           // OUTPUT_EXE (final output with icons)
            .output()
            .with_context(|| format!("Failed to execute icocop.exe at {}", icocop_path.display()))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            logger.log(&format!("  ERROR: icocop.exe failed with exit code {:?}", output.status.code()));
            if !stderr.is_empty() {
                logger.log(&format!("  STDERR: {}", stderr));
            }
            if !stdout.is_empty() {
                logger.log(&format!("  STDOUT: {}", stdout));
            }
            return Err(anyhow::anyhow!(
                "icocop.exe failed with exit code {:?}: {}",
                output.status.code(),
                stderr
            ));
        }
    } else {
        // Fallback to regular copy if icocop.exe is not available
        logger.log("  Warning: icocop.exe not found, copying without icon preservation");
        fs::copy(universal_launcher_path, target_path.as_ref())
            .context("failed to copy universal launcher (icocop.exe not found)")?;
    }
    Ok(())
}

fn replace_app_launcher<P: AsRef<Path>>(launcher_path: P, universal_launcher_path: &str, logger: &mut Logger) -> Result<String> {
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
        // App is already patched - copy universal launcher with icon from original
        logger.log(&format!("  App already patched, updating: {}", launcher_path.display()));
        copy_with_icon(&original_path, universal_launcher_path, &launcher_path.to_path_buf(), logger)
            .context("failed to update universal launcher")?;
        return Ok("updated".to_string());
    }
    
    // Step 1: Rename original launcher to *_original.exe (first-time patch)
    logger.log(&format!("  Renaming {} to {}", launcher_path.display(), original_path.display()));
    fs::rename(launcher_path, &original_path)
        .with_context(|| format!("failed to rename original launcher"))?;
    
    // Step 2: Copy UniversalLauncher.exe with original icon to the original launcher name
    if let Err(e) = copy_with_icon(&original_path, universal_launcher_path, &launcher_path.to_path_buf(), logger) {
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