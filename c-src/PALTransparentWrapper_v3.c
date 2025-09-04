/*
 * Transparent PortableApps Launcher Wrapper v3
 * 
 * This version uses a different naming strategy:
 * - Original launcher renamed to: AppNamePortable_Original.exe
 * - Our wrapper keeps the original name: AppNamePortable.exe
 * 
 * This way the original launcher keeps its identity and can find its INI file.
 * 
 * Compile:
 *   i686-w64-mingw32-gcc -o NSISPortable.exe PALTransparentWrapper_v3.c -lshlwapi -mwindows -static
 */

#include <windows.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <shlwapi.h>

#define MAX_PATH_LENGTH 512
#define ORIGINAL_SUFFIX "_Original.exe"

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
        return;
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
        
        // Clean various lock/state files
        const char* patterns[] = {"*.lock", "*.pid", "*.tmp", NULL};
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
    char baseName[MAX_PATH_LENGTH];
    char* lastSlash;
    char* dot;
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
        strcpy(wrapperName, lastSlash + 1);
        *lastSlash = '\0';
    } else {
        strcpy(wrapperName, "Unknown.exe");
    }
    
    // Get base name without .exe extension
    strcpy(baseName, wrapperName);
    dot = strrchr(baseName, '.');
    if (dot && _stricmp(dot, ".exe") == 0) {
        *dot = '\0';
    }
    
    // Construct path to the original launcher (AppName_Original.exe)
    snprintf(originalLauncher, sizeof(originalLauncher), 
             "%s\\%s%s", appDir, baseName, ORIGINAL_SUFFIX);
    
    // Check if original launcher exists
    if (!FileExists(originalLauncher)) {
        // Try legacy naming (launcher.exe) for backward compatibility
        snprintf(originalLauncher, sizeof(originalLauncher), "%s\\launcher.exe", appDir);
        
        if (!FileExists(originalLauncher)) {
            char errorMsg[MAX_PATH_LENGTH * 3];
            snprintf(errorMsg, sizeof(errorMsg), 
                    "Original launcher not found!\n\n"
                    "Expected one of:\n"
                    "- %s%s\n"
                    "- launcher.exe\n\n"
                    "Please rename the original PortableApps launcher to one of these names.", 
                    baseName, ORIGINAL_SUFFIX);
            MessageBoxA(NULL, errorMsg, "PortableApps Launcher", MB_OK | MB_ICONERROR);
            return 1;
        }
    }
    
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