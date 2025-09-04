/*
 * Transparent PortableApps Launcher Wrapper v2
 * 
 * This version handles the INI file lookup issue where the launcher
 * looks for a configuration file matching its own name.
 * 
 * Solution: Automatically creates a copy of the INI file with the
 * launcher's new name if it doesn't exist.
 * 
 * Compile:
 *   i686-w64-mingw32-gcc -o NSISPortable.exe PALTransparentWrapper_v2.c -lshlwapi -mwindows -static
 */

#include <windows.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <shlwapi.h>

#define MAX_PATH_LENGTH 512
#define RENAMED_LAUNCHER_NAME "launcher.exe"

int FileExists(const char* path) {
    DWORD attrib = GetFileAttributesA(path);
    return (attrib != INVALID_FILE_ATTRIBUTES && !(attrib & FILE_ATTRIBUTE_DIRECTORY));
}

int DirectoryExists(const char* path) {
    DWORD attrib = GetFileAttributesA(path);
    return (attrib != INVALID_FILE_ATTRIBUTES && (attrib & FILE_ATTRIBUTE_DIRECTORY));
}

void EnsureINIFileExists(const char* appDir, const char* wrapperName) {
    char originalINIPath[MAX_PATH_LENGTH];
    char launcherINIPath[MAX_PATH_LENGTH];
    char wrapperBaseName[MAX_PATH_LENGTH];
    char* dot;
    
    // Get base name without extension from wrapper name
    strcpy(wrapperBaseName, wrapperName);
    dot = strrchr(wrapperBaseName, '.');
    if (dot && _stricmp(dot, ".exe") == 0) {
        *dot = '\0';
    }
    
    // Construct paths to both INI files
    snprintf(originalINIPath, sizeof(originalINIPath), 
             "%s\\App\\AppInfo\\Launcher\\%s.ini", appDir, wrapperBaseName);
    snprintf(launcherINIPath, sizeof(launcherINIPath), 
             "%s\\App\\AppInfo\\Launcher\\launcher.ini", appDir);
    
    // If original INI exists but launcher.ini doesn't, copy it
    if (FileExists(originalINIPath) && !FileExists(launcherINIPath)) {
        CopyFileA(originalINIPath, launcherINIPath, FALSE);
    }
}

void CleanupRuntimeData(const char* appDir) {
    char dataPath[MAX_PATH_LENGTH];
    char pattern[MAX_PATH_LENGTH];
    WIN32_FIND_DATAA findData;
    HANDLE hFind;
    char filePath[MAX_PATH_LENGTH];
    
    // Construct path to Data directory
    snprintf(dataPath, sizeof(dataPath), "%s\\Data", appDir);
    
    if (!DirectoryExists(dataPath)) {
        return;  // Data directory doesn't exist yet
    }
    
    // Clean up ALL runtime data files
    snprintf(pattern, sizeof(pattern), "%s\\PortableApps.comLauncherRuntimeData-*.ini", dataPath);
    hFind = FindFirstFileA(pattern, &findData);
    if (hFind != INVALID_HANDLE_VALUE) {
        do {
            snprintf(filePath, sizeof(filePath), "%s\\%s", dataPath, findData.cFileName);
            DeleteFileA(filePath);
        } while (FindNextFileA(hFind, &findData));
        FindClose(hFind);
    }
    
    // Clean up settings directory
    snprintf(pattern, sizeof(pattern), "%s\\settings", dataPath);
    if (DirectoryExists(pattern)) {
        char settingsPath[MAX_PATH_LENGTH];
        strcpy(settingsPath, pattern);
        
        // Clean lock files
        snprintf(pattern, sizeof(pattern), "%s\\*.lock", settingsPath);
        hFind = FindFirstFileA(pattern, &findData);
        if (hFind != INVALID_HANDLE_VALUE) {
            do {
                snprintf(filePath, sizeof(filePath), "%s\\%s", settingsPath, findData.cFileName);
                DeleteFileA(filePath);
            } while (FindNextFileA(hFind, &findData));
            FindClose(hFind);
        }
        
        // Clean PID files
        snprintf(pattern, sizeof(pattern), "%s\\*.pid", settingsPath);
        hFind = FindFirstFileA(pattern, &findData);
        if (hFind != INVALID_HANDLE_VALUE) {
            do {
                snprintf(filePath, sizeof(filePath), "%s\\%s", settingsPath, findData.cFileName);
                DeleteFileA(filePath);
            } while (FindNextFileA(hFind, &findData));
            FindClose(hFind);
        }
    }
    
    // Clean up temp directory
    snprintf(pattern, sizeof(pattern), "%s\\Temp", dataPath);
    if (DirectoryExists(pattern)) {
        char tempPattern[MAX_PATH_LENGTH];
        snprintf(tempPattern, sizeof(tempPattern), "%s\\*", pattern);
        hFind = FindFirstFileA(tempPattern, &findData);
        if (hFind != INVALID_HANDLE_VALUE) {
            do {
                if (strcmp(findData.cFileName, ".") != 0 && strcmp(findData.cFileName, "..") != 0) {
                    snprintf(filePath, sizeof(filePath), "%s\\%s", pattern, findData.cFileName);
                    if (!DeleteFileA(filePath)) {
                        RemoveDirectoryA(filePath);
                    }
                }
            } while (FindNextFileA(hFind, &findData));
            FindClose(hFind);
        }
    }
}

int main(int argc, char* argv[]) {
    char exePath[MAX_PATH_LENGTH];
    char appDir[MAX_PATH_LENGTH];
    char originalLauncher[MAX_PATH_LENGTH];
    char wrapperName[MAX_PATH_LENGTH];
    char* lastSlash;
    STARTUPINFOA si;
    PROCESS_INFORMATION pi;
    char commandLine[MAX_PATH_LENGTH * 2];
    int i;
    
    // Get the path and name of this executable
    if (GetModuleFileNameA(NULL, exePath, sizeof(exePath)) == 0) {
        MessageBoxA(NULL, "Failed to get executable path", "PortableApps Launcher", MB_OK | MB_ICONERROR);
        return 1;
    }
    
    // Extract directory and wrapper name
    strcpy(appDir, exePath);
    lastSlash = strrchr(appDir, '\\');
    if (lastSlash) {
        strcpy(wrapperName, lastSlash + 1);  // Get wrapper filename
        *lastSlash = '\0';  // Terminate to get directory
    } else {
        strcpy(wrapperName, "Unknown.exe");
    }
    
    // Construct path to the renamed launcher
    snprintf(originalLauncher, sizeof(originalLauncher), "%s\\%s", appDir, RENAMED_LAUNCHER_NAME);
    
    // Check if renamed launcher exists
    if (!FileExists(originalLauncher)) {
        char errorMsg[MAX_PATH_LENGTH * 2];
        snprintf(errorMsg, sizeof(errorMsg), 
                "Original launcher not found:\n%s\n\n"
                "The original PortableApps launcher should be renamed to '%s'\n"
                "and placed in the same directory as this wrapper.", 
                originalLauncher, RENAMED_LAUNCHER_NAME);
        MessageBoxA(NULL, errorMsg, "PortableApps Launcher", MB_OK | MB_ICONERROR);
        return 1;
    }
    
    // Ensure the launcher.ini file exists (copy from original if needed)
    EnsureINIFileExists(appDir, wrapperName);
    
    // Clean up runtime data silently
    CleanupRuntimeData(appDir);
    
    // Build command line
    strcpy(commandLine, "\"");
    strcat(commandLine, originalLauncher);
    strcat(commandLine, "\"");
    
    for (i = 1; i < argc; i++) {
        strcat(commandLine, " ");
        if (strchr(argv[i], ' ')) {
            strcat(commandLine, "\"");
            strcat(commandLine, argv[i]);
            strcat(commandLine, "\"");
        } else {
            strcat(commandLine, argv[i]);
        }
    }
    
    // Launch the original launcher
    ZeroMemory(&si, sizeof(si));
    si.cb = sizeof(si);
    ZeroMemory(&pi, sizeof(pi));
    
    if (CreateProcessA(NULL, commandLine, NULL, NULL, FALSE, 0, NULL, appDir, &si, &pi)) {
        CloseHandle(pi.hProcess);
        CloseHandle(pi.hThread);
        return 0;
    } else {
        char errorMsg[MAX_PATH_LENGTH * 2];
        snprintf(errorMsg, sizeof(errorMsg), 
                "Failed to launch:\n%s\n\nError code: %lu", 
                originalLauncher, GetLastError());
        MessageBoxA(NULL, errorMsg, "PortableApps Launcher", MB_OK | MB_ICONERROR);
        return 1;
    }
}