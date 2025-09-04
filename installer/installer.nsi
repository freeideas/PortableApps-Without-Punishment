; PortableApps Without Punishment Installer
; NSIS Script for GUI installation with uninstaller support

!define PRODUCT_NAME "PortableApps Without Punishment"
!define PRODUCT_VERSION "1.0"
!define BUILD_DATE "2025-09-04-1848"
!define REGISTRY_KEY "HKCU\Software\PortableAppsWithoutPunishment"

; Include Modern UI
!include "MUI2.nsh"
!include "FileFunc.nsh"
!include "LogicLib.nsh"

; General settings
Name "${PRODUCT_NAME}"
OutFile "..\releases\PortableApps Without Punishment ${BUILD_DATE}.exe"
InstallDir "$LOCALAPPDATA\PortableAppsWithoutPunishment"
RequestExecutionLevel user
ShowInstDetails show
ShowUninstDetails show

; Interface settings
!define MUI_ICON "${NSISDIR}\Contrib\Graphics\Icons\modern-install.ico"
!define MUI_HEADERIMAGE
!define MUI_HEADERIMAGE_BITMAP "${NSISDIR}\Contrib\Graphics\Header\orange.bmp"
!define MUI_WELCOMEFINISHPAGE_BITMAP "${NSISDIR}\Contrib\Graphics\Wizard\orange.bmp"

; Pages
!define MUI_WELCOMEPAGE_TITLE "Welcome to ${PRODUCT_NAME}"
!define MUI_WELCOMEPAGE_TEXT "This wizard will help you remove the annoying 'application was not closed properly' warnings from your PortableApps.$\r$\n$\r$\nNo more punishment for improper shutdowns!$\r$\n$\r$\nVersion: ${PRODUCT_VERSION} (Built: ${BUILD_DATE})$\r$\n$\r$\nClick Next to continue."
!insertmacro MUI_PAGE_WELCOME

; Custom page for directory selection
Page custom SelectPortableAppsDir ValidateSelection

!define MUI_PAGE_HEADER_TEXT "Installation Progress"
!define MUI_PAGE_HEADER_SUBTEXT "Please wait while your PortableApps are being patched..."
!insertmacro MUI_PAGE_INSTFILES

!define MUI_FINISHPAGE_TITLE "Installation Complete"
!define MUI_FINISHPAGE_TEXT "${PRODUCT_NAME} has successfully patched your PortableApps.$\r$\n$\r$\nYour PortableApp(s) will no longer punish you!$\r$\n$\r$\nTo restore punishment later, run the uninstaller from Add/Remove Programs."
!insertmacro MUI_PAGE_FINISH

; Uninstaller pages
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

; Languages
!insertmacro MUI_LANGUAGE "English"

; Variables
Var PortableAppsDir
Var Dialog
Var Label
Var DirRequest
Var BrowseButton
Var SilentMode

; Functions
Function .onInit
    ; Parse command line arguments
    StrCpy $SilentMode "false"
    
    ; Check for silent mode flag
    ${GetParameters} $0
    ${GetOptions} $0 "/S" $1
    IfErrors +2 0
        StrCpy $SilentMode "true"
    
    ; Check for directory parameter
    ${GetOptions} $0 "/D=" $1
    IfErrors check_registry 0
        StrCpy $PortableAppsDir $1
        Goto done_init
    
    check_registry:
    ; Try to read last used directory from registry
    ReadRegStr $PortableAppsDir HKCU "Software\PortableAppsWithoutPunishment" "LastDirectory"
    IfErrors done_init 0
    
    done_init:
    ; If we have a directory and silent mode, skip GUI
    ${If} $SilentMode == "true"
    ${AndIf} $PortableAppsDir != ""
        SetSilent silent
    ${EndIf}
FunctionEnd

; Sections
Section "MainSection"
    ; Create installation directory
    SetOutPath "$INSTDIR"
    
    ; Extract files needed for patching
    File "..\\builds\\rust\\replacer.exe"
    File "..\\builds\\rust\\universal-launcher.exe"
    
    ; Show what we're doing
    DetailPrint "Patching PortableApps in: $PortableAppsDir"
    DetailPrint ""
    
    ; Run replacer with the selected directory
    nsExec::ExecToLog '"$INSTDIR\replacer.exe" "$PortableAppsDir" "$INSTDIR\universal-launcher.exe"'
    Pop $0
    
    ${If} $0 == "0"
        DetailPrint ""
        DetailPrint "Success! Your PortableApps have been patched."
        
        ; Create uninstaller
        WriteUninstaller "$INSTDIR\Uninstall.exe"
        
        ; Save directory info to registry
        WriteRegStr HKCU "Software\PortableAppsWithoutPunishment" "LastDirectory" "$PortableAppsDir"
        WriteRegStr HKCU "Software\PortableAppsWithoutPunishment" "LastRun" "${BUILD_DATE}"
        WriteRegStr HKCU "Software\PortableAppsWithoutPunishment" "InstallDir" "$INSTDIR"
        
        ; Add uninstall information to registry
        WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" "DisplayName" "${PRODUCT_NAME}"
        WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" "UninstallString" "$INSTDIR\Uninstall.exe"
        WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" "InstallLocation" "$INSTDIR"
        WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" "DisplayVersion" "${PRODUCT_VERSION}"
        WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" "Publisher" "PortableApps Without Punishment"
        WriteRegDWORD HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" "NoModify" 1
        WriteRegDWORD HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" "NoRepair" 1
    ${Else}
        DetailPrint ""
        DetailPrint "Some apps may not have been patched. Check the log above."
        ; Still create uninstaller for partial success
        WriteUninstaller "$INSTDIR\Uninstall.exe"
        WriteRegStr HKCU "Software\PortableAppsWithoutPunishment" "LastDirectory" "$PortableAppsDir"
        WriteRegStr HKCU "Software\PortableAppsWithoutPunishment" "InstallDir" "$INSTDIR"
    ${EndIf}
    
    ; Clean up temp installation files but keep uninstaller
    Delete "$INSTDIR\replacer.exe"
    Delete "$INSTDIR\universal-launcher.exe"
SectionEnd

; Custom functions
Function SelectPortableAppsDir
    nsDialogs::Create 1018
    Pop $Dialog
    
    ${If} $Dialog == error
        Abort
    ${EndIf}
    
    !insertmacro MUI_HEADER_TEXT "Select PortableApps Location" "Choose the directory containing your PortableApps"
    
    ${NSD_CreateLabel} 0 0 100% 40u "Please select either:$\r$\n- A single PortableApp directory (e.g., D:\PortableApps\FirefoxPortable)$\r$\n- A directory containing multiple PortableApps (e.g., D:\PortableApps)$\r$\n$\r$\nAll PortableApps found will be patched automatically."
    Pop $Label
    
    ${NSD_CreateLabel} 0 50u 50u 12u "Directory:"
    Pop $Label
    
    ${NSD_CreateDirRequest} 55u 48u 180u 12u ""
    Pop $DirRequest
    
    ${NSD_CreateBrowseButton} 240u 48u 50u 12u "Browse..."
    Pop $BrowseButton
    ${NSD_OnClick} $BrowseButton BrowseForFolder
    
    ; Set path from variable (registry or command line) or defaults
    ${If} $PortableAppsDir != ""
        ${NSD_SetText} $DirRequest "$PortableAppsDir"
    ${ElseIf} ${FileExists} "C:\PortableApps\*.*"
        ${NSD_SetText} $DirRequest "C:\PortableApps"
    ${ElseIf} ${FileExists} "D:\PortableApps\*.*"
        ${NSD_SetText} $DirRequest "D:\PortableApps"
    ${ElseIf} ${FileExists} "E:\PortableApps\*.*"
        ${NSD_SetText} $DirRequest "E:\PortableApps"
    ${EndIf}
    
    nsDialogs::Show
FunctionEnd

Function BrowseForFolder
    nsDialogs::SelectFolderDialog "Select PortableApps Directory" ""
    Pop $0
    ${If} $0 != error
        ${NSD_SetText} $DirRequest $0
    ${EndIf}
FunctionEnd

Function ValidateSelection
    ${NSD_GetText} $DirRequest $PortableAppsDir
    
    ; Check if directory exists
    ${If} ${FileExists} "$PortableAppsDir\*.*"
        ; Directory exists, we're good
    ${Else}
        MessageBox MB_OK|MB_ICONSTOP "Please select a valid directory containing PortableApps."
        Abort
    ${EndIf}
FunctionEnd

; Uninstaller section
Section "Uninstall"
    ; Read the stored PortableApps directory
    ReadRegStr $PortableAppsDir HKCU "Software\PortableAppsWithoutPunishment" "LastDirectory"
    
    ${If} $PortableAppsDir == ""
        MessageBox MB_OK|MB_ICONSTOP "Cannot find the PortableApps directory that was previously patched.$\r$\n$\r$\nUninstallation cannot proceed automatically."
        Abort
    ${EndIf}
    
    DetailPrint "Restoring punishment to PortableApps in: $PortableAppsDir"
    DetailPrint ""
    
    ; Extract RestorePunishment tool to temp directory
    SetOutPath "$TEMP"
    File "..\\builds\\rust\\restore-punishment.exe"
    
    ; Run the restoration tool
    DetailPrint "Running punishment restoration tool..."
    nsExec::ExecToLog '"$TEMP\restore-punishment.exe" "$PortableAppsDir"'
    Pop $0
    
    ${If} $0 == "0"
        DetailPrint ""
        DetailPrint "Success! Punishment has been restored to your PortableApps."
        DetailPrint "Your apps will now show 'not closed properly' warnings again."
    ${Else}
        DetailPrint ""
        DetailPrint "Some apps may not have been restored. Check the log above."
    ${EndIf}
    
    ; Clean up temp files
    Delete "$TEMP\restore-punishment.exe"
    
    ; Remove registry entries
    DeleteRegKey HKCU "Software\PortableAppsWithoutPunishment"
    DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}"
    
    ; Remove uninstaller and installation directory
    Delete "$INSTDIR\Uninstall.exe"
    RMDir "$INSTDIR"
    
    DetailPrint ""
    DetailPrint "Uninstallation complete. PortableApps punishment has been restored!"
SectionEnd