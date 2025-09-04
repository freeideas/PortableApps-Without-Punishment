package main

import (
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("PortableApps Universal Launcher Replacer")
		fmt.Println("=========================================")
		fmt.Println()
		fmt.Println("Usage: replacer.exe <PortableAppsDirectory> [UniversalLauncher.exe]")
		fmt.Println()
		fmt.Println("Arguments:")
		fmt.Println("  PortableAppsDirectory - Root directory containing PortableApps (e.g., D:\\PortableApps)")
		fmt.Println("  UniversalLauncher.exe - Path to the universal launcher (default: looks in current dir)")
		fmt.Println()
		fmt.Println("Example:")
		fmt.Println("  replacer.exe D:\\PortableApps")
		fmt.Println("  replacer.exe D:\\PortableApps C:\\Tools\\UniversalLauncher.exe")
		os.Exit(1)
	}

	portableAppsDir := os.Args[1]
	universalLauncherPath := "UniversalLauncher.exe"

	if len(os.Args) >= 3 {
		universalLauncherPath = os.Args[2]
	}

	// Check if PortableApps directory exists
	if _, err := os.Stat(portableAppsDir); os.IsNotExist(err) {
		fmt.Printf("Error: Directory '%s' does not exist\n", portableAppsDir)
		os.Exit(1)
	}

	// Check if UniversalLauncher.exe exists
	if _, err := os.Stat(universalLauncherPath); os.IsNotExist(err) {
		fmt.Printf("Error: UniversalLauncher.exe not found at '%s'\n", universalLauncherPath)
		fmt.Println("Please ensure UniversalLauncher.exe is in the current directory or provide its path")
		os.Exit(1)
	}

	fmt.Println("PortableApps Universal Launcher Replacer")
	fmt.Println("=========================================")
	fmt.Printf("PortableApps Directory: %s\n", portableAppsDir)
	fmt.Printf("Universal Launcher: %s\n", universalLauncherPath)
	fmt.Println()

	// Find all portable apps
	apps := findPortableApps(portableAppsDir)
	
	if len(apps) == 0 {
		fmt.Println("No PortableApps found in the specified directory.")
		fmt.Println("Looking for pattern: */PortableApps/*Portable/*Portable.exe")
		os.Exit(0)
	}

	fmt.Printf("Found %d PortableApps:\n", len(apps))
	for i, app := range apps {
		fmt.Printf("  %d. %s\n", i+1, filepath.Base(app))
	}
	fmt.Println()

	// Process each app
	successCount := 0
	for _, appLauncher := range apps {
		if err := replaceAppLauncher(appLauncher, universalLauncherPath); err != nil {
			fmt.Printf("❌ Failed: %s - %v\n", filepath.Base(appLauncher), err)
		} else {
			fmt.Printf("✅ Success: %s\n", filepath.Base(appLauncher))
			successCount++
		}
	}

	fmt.Println()
	fmt.Printf("Summary: %d of %d apps successfully patched\n", successCount, len(apps))
	
	if successCount > 0 {
		fmt.Println()
		fmt.Println("🎉 PortableApps have been patched! No more 'not closed properly' warnings!")
	}
}

func findPortableApps(rootDir string) []string {
	var apps []string
	
	// Walk the directory tree
	filepath.Walk(rootDir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil // Continue walking even if there's an error
		}
		
		// Skip if not a file
		if info.IsDir() {
			return nil
		}
		
		// Check if it matches the pattern *Portable.exe
		if strings.HasSuffix(strings.ToLower(path), "portable.exe") {
			// Additional validation: check if it's in a proper PortableApp structure
			// Should have App/AppInfo directory structure
			appDir := filepath.Dir(path)
			appInfoPath := filepath.Join(appDir, "App", "AppInfo")
			
			if _, err := os.Stat(appInfoPath); err == nil {
				// Check if it's not already our wrapper (by checking for _original.exe)
				originalName := strings.TrimSuffix(filepath.Base(path), ".exe") + "_original.exe"
				originalPath := filepath.Join(filepath.Dir(path), originalName)
				
				if _, err := os.Stat(originalPath); os.IsNotExist(err) {
					// This is a valid, unpatched PortableApp
					apps = append(apps, path)
				}
			}
		}
		
		return nil
	})
	
	return apps
}

func replaceAppLauncher(launcherPath string, universalLauncherPath string) error {
	// Construct the original launcher name
	launcherDir := filepath.Dir(launcherPath)
	launcherName := filepath.Base(launcherPath)
	originalName := strings.TrimSuffix(launcherName, ".exe") + "_original.exe"
	originalPath := filepath.Join(launcherDir, originalName)
	
	// Step 1: Check if already replaced
	if _, err := os.Stat(originalPath); err == nil {
		return fmt.Errorf("already patched (found %s)", originalName)
	}
	
	// Step 2: Rename original launcher to *_original.exe
	if err := os.Rename(launcherPath, originalPath); err != nil {
		return fmt.Errorf("failed to rename original: %v", err)
	}
	
	// Step 3: Copy UniversalLauncher.exe to the original launcher name
	if err := copyFile(universalLauncherPath, launcherPath); err != nil {
		// Try to restore original on failure
		os.Rename(originalPath, launcherPath)
		return fmt.Errorf("failed to copy universal launcher: %v", err)
	}
	
	return nil
}

func copyFile(src, dst string) error {
	sourceFile, err := os.Open(src)
	if err != nil {
		return err
	}
	defer sourceFile.Close()
	
	destFile, err := os.Create(dst)
	if err != nil {
		return err
	}
	defer destFile.Close()
	
	_, err = io.Copy(destFile, sourceFile)
	if err != nil {
		return err
	}
	
	// Copy file permissions
	sourceInfo, err := os.Stat(src)
	if err != nil {
		return err
	}
	
	return os.Chmod(dst, sourceInfo.Mode())
}