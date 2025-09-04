/*
 * Transparent PortableApps Launcher Wrapper
 * 
 * This program replaces the original PortableApps launcher executable.
 * It removes leftover runtime data files that cause the "application was 
 * not closed properly" message, then launches the original launcher which
 * has been renamed to "launcher.exe"
 * 
 * Installation:
 *   1. Rename the original PortableApps launcher (e.g., NSISPortable.exe) to "launcher.exe"
 *   2. Compile this wrapper with the original launcher's name
 *   3. Place it where the original launcher was
 * 
 * Compile examples:
 *   For NSIS Portable:
 *     i686-w64-mingw32-gcc -o NSISPortable.exe PALTransparentWrapper.c -lshlwapi -mwindows -static
 *   
 *   For Firefox Portable:
 *     i686-w64-mingw32-gcc -o FirefoxPortable.exe PALTransparentWrapper.c -lshlwapi -mwindows -static
 */

#include <windows.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <shlwapi.h>

#define MAX_PATH_LENGTH 512
#define ORIGINAL_LAUNCHER_NAME "launcher.exe"

int FileExists(const char* path) {
    DWORD attrib = GetFileAttributesA(path);
    return (attrib != INVALID_FILE_ATTRIBUTES && !(attrib & FILE_ATTRIBUTE_DIRECTORY));
}

int DirectoryExists(const char* path) {
    DWORD attrib = GetFileAttributesA(path);
    return (attrib != INVALID_FILE_ATTRIBUTES && (attrib & FILE_ATTRIBUTE_DIRECTORY));
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
        return;  // Data directory doesn't exist yet, nothing to clean
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
        
        // Clean temp marker files (sometimes used to track improper shutdown)
        snprintf(pattern, sizeof(pattern), "%s\\*.tmp", settingsPath);
        hFind = FindFirstFileA(pattern, &findData);
        if (hFind != INVALID_HANDLE_VALUE) {
            do {
                snprintf(filePath, sizeof(filePath), "%s\\%s", settingsPath, findData.cFileName);
                DeleteFileA(filePath);
            } while (FindNextFileA(hFind, &findData));
            FindClose(hFind);
        }
    }
    
    // Clean up any .pid files in Data directory root
    snprintf(pattern, sizeof(pattern), "%s\\*.pid", dataPath);
    hFind = FindFirstFileA(pattern, &findData);
    if (hFind != INVALID_HANDLE_VALUE) {
        do {
            snprintf(filePath, sizeof(filePath), "%s\\%s", dataPath, findData.cFileName);
            DeleteFileA(filePath);
        } while (FindNextFileA(hFind, &findData));
        FindClose(hFind);
    }
    
    // Clean up temp directories that may have been left
    snprintf(pattern, sizeof(pattern), "%s\\Temp", dataPath);
    if (DirectoryExists(pattern)) {
        char tempPattern[MAX_PATH_LENGTH];
        snprintf(tempPattern, sizeof(tempPattern), "%s\\*", pattern);
        hFind = FindFirstFileA(tempPattern, &findData);
        if (hFind != INVALID_HANDLE_VALUE) {
            do {
                if (strcmp(findData.cFileName, ".") != 0 && strcmp(findData.cFileName, "..") != 0) {
                    snprintf(filePath, sizeof(filePath), "%s\\%s", pattern, findData.cFileName);
                    // Try to delete as file first, then as directory if that fails
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
    char* lastSlash;
    STARTUPINFOA si;
    PROCESS_INFORMATION pi;
    char commandLine[MAX_PATH_LENGTH * 2];
    int i;
    
    // Get the path of this executable
    if (GetModuleFileNameA(NULL, exePath, sizeof(exePath)) == 0) {
        MessageBoxA(NULL, "Failed to get executable path", "PortableApps Launcher", MB_OK | MB_ICONERROR);
        return 1;
    }
    
    // Get the directory containing this executable
    strcpy(appDir, exePath);
    lastSlash = strrchr(appDir, '\\');
    if (lastSlash) {
        *lastSlash = '\0';
    }
    
    // Construct path to the original launcher (now named launcher.exe)
    snprintf(originalLauncher, sizeof(originalLauncher), "%s\\%s", appDir, ORIGINAL_LAUNCHER_NAME);
    
    // Check if original launcher exists
    if (!FileExists(originalLauncher)) {
        char errorMsg[MAX_PATH_LENGTH * 2];
        snprintf(errorMsg, sizeof(errorMsg), 
                "Original launcher not found:\n%s\n\n"
                "The original PortableApps launcher should be renamed to '%s'\n"
                "and placed in the same directory as this wrapper.", 
                originalLauncher, ORIGINAL_LAUNCHER_NAME);
        MessageBoxA(NULL, errorMsg, "PortableApps Launcher", MB_OK | MB_ICONERROR);
        return 1;
    }
    
    // Silently clean up runtime data
    CleanupRuntimeData(appDir);
    
    // Build command line with all original arguments
    strcpy(commandLine, "\"");
    strcat(commandLine, originalLauncher);
    strcat(commandLine, "\"");
    
    // Append all command line arguments
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
        // Successfully launched - close handles and exit
        CloseHandle(pi.hProcess);
        CloseHandle(pi.hThread);
        return 0;
    } else {
        // Failed to launch - show error
        char errorMsg[MAX_PATH_LENGTH * 2];
        snprintf(errorMsg, sizeof(errorMsg), 
                "Failed to launch the original launcher:\n%s\n\nWindows error code: %lu", 
                originalLauncher, GetLastError());
        MessageBoxA(NULL, errorMsg, "PortableApps Launcher", MB_OK | MB_ICONERROR);
        return 1;
    }
}