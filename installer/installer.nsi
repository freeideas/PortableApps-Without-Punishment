; PortableApps Without Punishment Installer
; NSIS Script for GUI installation

!define PRODUCT_NAME "PortableApps Without Punishment"
!define PRODUCT_VERSION "1.0"

; Include Modern UI
!include "MUI2.nsh"
!include "FileFunc.nsh"
!include "LogicLib.nsh"

; General settings
Name "${PRODUCT_NAME}"
OutFile "..\releases\PortableApps Without Punishment.exe"
InstallDir "$TEMP\NoPunish"
RequestExecutionLevel user
ShowInstDetails show

; Interface settings
!define MUI_ICON "${NSISDIR}\Contrib\Graphics\Icons\modern-install.ico"
!define MUI_HEADERIMAGE
!define MUI_HEADERIMAGE_BITMAP "${NSISDIR}\Contrib\Graphics\Header\orange.bmp"
!define MUI_WELCOMEFINISHPAGE_BITMAP "${NSISDIR}\Contrib\Graphics\Wizard\orange.bmp"

; Pages
!define MUI_WELCOMEPAGE_TITLE "Welcome to ${PRODUCT_NAME}"
!define MUI_WELCOMEPAGE_TEXT "This wizard will help you remove the annoying 'application was not closed properly' warnings from your PortableApps.$\r$\n$\r$\nNo more punishment for improper shutdowns!$\r$\n$\r$\nClick Next to continue."
!insertmacro MUI_PAGE_WELCOME

; Custom page for directory selection
Page custom SelectPortableAppsDir ValidateSelection

!define MUI_PAGE_HEADER_TEXT "Installation Progress"
!define MUI_PAGE_HEADER_SUBTEXT "Please wait while your PortableApps are being patched..."
!insertmacro MUI_PAGE_INSTFILES

!define MUI_FINISHPAGE_TITLE "Installation Complete"
!define MUI_FINISHPAGE_TEXT "${PRODUCT_NAME} has successfully patched your PortableApps.$\r$\n$\r$\nYour applications will no longer punish you!"
!insertmacro MUI_PAGE_FINISH

; Languages
!insertmacro MUI_LANGUAGE "English"

; Variables
Var PortableAppsDir
Var Dialog
Var Label
Var DirRequest
Var BrowseButton

; Sections
Section "MainSection"
    ; Extract embedded files to temp directory
    SetOutPath "$INSTDIR"
    File "..\builds\go\NoPunishReplacer.exe"
    File "..\builds\c\UniversalLauncher.exe"
    
    ; Show what we're doing
    DetailPrint "Patching PortableApps in: $PortableAppsDir"
    DetailPrint ""
    
    ; Run NoPunishReplacer with the selected directory
    nsExec::ExecToLog '"$INSTDIR\NoPunishReplacer.exe" "$PortableAppsDir" "$INSTDIR\UniversalLauncher.exe"'
    Pop $0
    
    ${If} $0 == "0"
        DetailPrint ""
        DetailPrint "Success! Your PortableApps have been patched."
    ${Else}
        DetailPrint ""
        DetailPrint "Some apps may not have been patched. Check the log above."
    ${EndIf}
    
    ; Clean up temp files
    Delete "$INSTDIR\NoPunishReplacer.exe"
    Delete "$INSTDIR\UniversalLauncher.exe"
    RMDir "$INSTDIR"
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
    
    ; Set default path if PortableApps directory exists in common locations
    ${If} ${FileExists} "C:\PortableApps\*.*"
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