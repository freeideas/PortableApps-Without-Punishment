/*
 * Transparent PortableApps Launcher Wrapper - Final Version
 * 
 * This wrapper is self-aware of its filename and automatically:
 * 1. Cleans up runtime data files
 * 2. Copies the matching INI file to launcher.ini
 * 3. Runs launcher.exe (the renamed original)
 * 
 * Installation:
 *   1. Rename original (e.g., NSISPortable.exe) to launcher.exe
 *   2. Place this wrapper with the original name (e.g., NSISPortable.exe)
 *   3. Done! The wrapper handles everything else automatically
 * 
 * Universal: Just rename this exe to match any PortableApp!
 * 
 * Compile:
 *   i686-w64-mingw32-gcc -o UniversalPALWrapper.exe PALTransparentWrapper_Final.c -lshlwapi -mwindows -static
 */

#include <windows.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <shlwapi.h>

#define MAX_PATH_LENGTH 512
#define LAUNCHER_NAME "launcher.exe"

int FileExists(const char* path) {
    DWORD attrib = GetFileAttributesA(path);
    return (attrib != INVALID_FILE_ATTRIBUTES && !(attrib & FILE_ATTRIBUTE_DIRECTORY));
}

int DirectoryExists(const char* path) {
    DWORD attrib = GetFileAttributesA(path);
    return (attrib != INVALID_FILE_ATTRIBUTES && (attrib & FILE_ATTRIBUTE_DIRECTORY));
}

void CopyINIFile(const char* appDir, const char* wrapperName) {
    char sourceINI[MAX_PATH_LENGTH];
    char targetINI[MAX_PATH_LENGTH];
    char baseName[MAX_PATH_LENGTH];
    char* dot;
    
    // Get base name without .exe extension
    strcpy(baseName, wrapperName);
    dot = strrchr(baseName, '.');
    if (dot && _stricmp(dot, ".exe") == 0) {
        *dot = '\0';
    }
    
    // Construct paths
    snprintf(sourceINI, sizeof(sourceINI), 
             "%s\\App\\AppInfo\\Launcher\\%s.ini", appDir, baseName);
    snprintf(targetINI, sizeof(targetINI), 
             "%s\\App\\AppInfo\\Launcher\\launcher.ini", appDir);
    
    // Only copy if source exists and is different from target
    if (FileExists(sourceINI) && _stricmp(sourceINI, targetINI) != 0) {
        // Copy the file, overwriting if it exists
        CopyFileA(sourceINI, targetINI, FALSE);
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
    
    // Clean up ALL runtime data files (PortableApps.comLauncherRuntimeData-*.ini)
    snprintf(pattern, sizeof(pattern), "%s\\PortableApps.comLauncherRuntimeData-*.ini", dataPath);
    hFind = FindFirstFileA(pattern, &findData);
    if (hFind != INVALID_HANDLE_VALUE) {
        do {
            snprintf(filePath, sizeof(filePath), "%s\\%s", dataPath, findData.cFileName);
            DeleteFileA(filePath);
        } while (FindNextFileA(hFind, &findData));
        FindClose(hFind);
    }
    
    // Clean up in settings subdirectory if it exists
    snprintf(pattern, sizeof(pattern), "%s\\settings", dataPath);
    if (DirectoryExists(pattern)) {
        char settingsPath[MAX_PATH_LENGTH];
        strcpy(settingsPath, pattern);
        
        // Clean various state/lock files
        const char* patterns[] = {"*.lock", "*.pid", "*.tmp", "*.temp", NULL};
        for (int i = 0; patterns[i]; i++) {
            snprintf(pattern, sizeof(pattern), "%s\\%s", settingsPath, patterns[i]);
            hFind = FindFirstFileA(pattern, &findData);
            if (hFind != INVALID_HANDLE_VALUE) {
                do {
                    snprintf(filePath, sizeof(filePath), "%s\\%s", settingsPath, findData.cFileName);
                    DeleteFileA(filePath);
                } while (FindNextFileA(hFind, &findData));
                FindClose(hFind);
            }
        }
    }
    
    // Clean up PID files in Data root
    snprintf(pattern, sizeof(pattern), "%s\\*.pid", dataPath);
    hFind = FindFirstFileA(pattern, &findData);
    if (hFind != INVALID_HANDLE_VALUE) {
        do {
            snprintf(filePath, sizeof(filePath), "%s\\%s", dataPath, findData.cFileName);
            DeleteFileA(filePath);
        } while (FindNextFileA(hFind, &findData));
        FindClose(hFind);
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
                    // Try to delete as file first
                    if (!DeleteFileA(filePath)) {
                        // If that fails, try as directory
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
    char launcherPath[MAX_PATH_LENGTH];
    char wrapperName[MAX_PATH_LENGTH];
    char* lastSlash;
    STARTUPINFOA si;
    PROCESS_INFORMATION pi;
    char commandLine[MAX_PATH_LENGTH * 2];
    int i;
    
    // Get the full path of this executable
    if (GetModuleFileNameA(NULL, exePath, sizeof(exePath)) == 0) {
        MessageBoxA(NULL, "Failed to get executable path", "PortableApps Launcher", MB_OK | MB_ICONERROR);
        return 1;
    }
    
    // Extract directory and wrapper filename
    strcpy(appDir, exePath);
    lastSlash = strrchr(appDir, '\\');
    if (lastSlash) {
        strcpy(wrapperName, lastSlash + 1);  // Get wrapper filename
        *lastSlash = '\0';  // Terminate to get directory
    } else {
        strcpy(wrapperName, "Unknown.exe");
    }
    
    // Construct path to launcher.exe
    snprintf(launcherPath, sizeof(launcherPath), "%s\\%s", appDir, LAUNCHER_NAME);
    
    // Check if launcher.exe exists
    if (!FileExists(launcherPath)) {
        char errorMsg[MAX_PATH_LENGTH * 2];
        snprintf(errorMsg, sizeof(errorMsg), 
                "Original launcher not found:\n%s\n\n"
                "Please rename the original PortableApps launcher to 'launcher.exe'\n"
                "and place this wrapper with the original name.", 
                launcherPath);
        MessageBoxA(NULL, errorMsg, "PortableApps Launcher", MB_OK | MB_ICONERROR);
        return 1;
    }
    
    // Step 1: Clean up runtime data
    CleanupRuntimeData(appDir);
    
    // Step 2: Copy the appropriate INI file to launcher.ini
    CopyINIFile(appDir, wrapperName);
    
    // Step 3: Build command line with all arguments
    strcpy(commandLine, "\"");
    strcat(commandLine, launcherPath);
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
    
    // Step 4: Launch the original launcher
    ZeroMemory(&si, sizeof(si));
    si.cb = sizeof(si);
    ZeroMemory(&pi, sizeof(pi));
    
    if (CreateProcessA(NULL, commandLine, NULL, NULL, FALSE, 0, NULL, appDir, &si, &pi)) {
        // Successfully launched - close handles and exit
        CloseHandle(pi.hProcess);
        CloseHandle(pi.hThread);
        return 0;
    } else {
        // Failed to launch
        char errorMsg[MAX_PATH_LENGTH * 2];
        snprintf(errorMsg, sizeof(errorMsg), 
                "Failed to launch the original launcher:\n%s\n\nWindows error code: %lu", 
                launcherPath, GetLastError());
        MessageBoxA(NULL, errorMsg, "PortableApps Launcher", MB_OK | MB_ICONERROR);
        return 1;
    }
}