; PortableApps Without Punishment Installer
; NSIS Script with toggle for Remove/Restore Punishment

!define PRODUCT_NAME "PortableApps Without Punishment"
!define PRODUCT_VERSION "1.0"
!define BUILD_DATE "2025-09-04-1908"
!define REGISTRY_KEY "HKCU\Software\PortableAppsWithoutPunishment"

; Include Modern UI
!include "MUI2.nsh"
!include "FileFunc.nsh"
!include "LogicLib.nsh"

; General settings
Name "${PRODUCT_NAME}"
OutFile "..\releases\PortableApps Without Punishment ${BUILD_DATE}.exe"
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
!define MUI_WELCOMEPAGE_TEXT "This wizard can either remove or restore the annoying 'application was not closed properly' warnings in your PortableApps.$\r$\n$\r$\n• Remove Punishment: Eliminate the warnings forever$\r$\n• Restore Punishment: Bring back the medieval shame$\r$\n$\r$\nVersion: ${PRODUCT_VERSION} (Built: ${BUILD_DATE})$\r$\n$\r$\nSupports silent mode: /S /RESTORE /D=path$\r$\n$\r$\nClick Next to continue."
!insertmacro MUI_PAGE_WELCOME

; Custom page for mode selection (Remove or Restore punishment)
Page custom SelectMode ValidateMode

; Custom page for directory selection
Page custom SelectPortableAppsDir ValidateSelection

!define MUI_PAGE_HEADER_TEXT "Installation Progress"
!define MUI_PAGE_HEADER_SUBTEXT "Please wait while your PortableApps are being patched..."
!insertmacro MUI_PAGE_INSTFILES

!define MUI_FINISHPAGE_TITLE "Operation Complete"
!define MUI_FINISHPAGE_TEXT "The operation completed successfully!"
!insertmacro MUI_PAGE_FINISH

; Languages
!insertmacro MUI_LANGUAGE "English"

; Variables
Var PortableAppsDir
Var Dialog
Var Label
Var DirRequest
Var BrowseButton
Var SilentMode
Var RestoreMode
Var RadioRemove
Var RadioRestore
Var FinishText

; Functions
Function .onInit
    ; Initialize variables
    StrCpy $SilentMode "false"
    StrCpy $RestoreMode "false"
    StrCpy $PortableAppsDir ""
    
    ; Parse command line arguments
    ${GetParameters} $0
    ${GetOptions} $0 "/S" $1
    IfErrors +2 0
        StrCpy $SilentMode "true"
    
    ; Check for restore mode parameter
    ${GetOptions} $0 "/RESTORE" $1
    IfErrors +2 0
        StrCpy $RestoreMode "true"
    
    ; Check for directory parameter
    ${GetOptions} $0 "/D=" $1
    IfErrors check_registry 0
        StrCpy $PortableAppsDir $1
        Goto done_init
    
    check_registry:
    ; Try to read last used directory from registry for convenience
    ReadRegStr $PortableAppsDir HKCU "Software\PortableAppsWithoutPunishment" "LastDirectory"
    IfErrors done_init 0
    
    done_init:
    ; If we have all parameters and silent mode, skip GUI
    ${If} $SilentMode == "true"
    ${AndIf} $PortableAppsDir != ""
        SetSilent silent
    ${EndIf}
FunctionEnd

; Custom function for mode selection
Function SelectMode
    nsDialogs::Create 1018
    Pop $Dialog
    
    ${If} $Dialog == error
        Abort
    ${EndIf}
    
    !insertmacro MUI_HEADER_TEXT "Choose Operation" "Select whether to remove or restore PortableApps punishment"
    
    ${NSD_CreateLabel} 0 20u 100% 30u "What would you like to do with your PortableApps?$\r$\n$\r$\nChoose 'Remove Punishment' to eliminate the annoying 'not closed properly' warnings, or 'Restore Punishment' to bring back the original behavior."
    Pop $Label
    
    ${NSD_CreateRadioButton} 20u 60u 200u 12u "Remove Punishment (Patch PortableApps)"
    Pop $RadioRemove
    ${NSD_CreateRadioButton} 20u 80u 200u 12u "Restore Punishment (Unpatch PortableApps)"
    Pop $RadioRestore
    
    ; Set default selection based on variable
    ${If} $RestoreMode == "true"
        ${NSD_Check} $RadioRestore
    ${Else}
        ${NSD_Check} $RadioRemove
    ${EndIf}
    
    nsDialogs::Show
FunctionEnd

Function ValidateMode
    ${NSD_GetState} $RadioRestore $0
    ${If} $0 == 1
        StrCpy $RestoreMode "true"
    ${Else}
        StrCpy $RestoreMode "false"
    ${EndIf}
FunctionEnd

; Sections
Section "MainSection"
    ; Create temporary directory
    SetOutPath "$INSTDIR"
    
    ${If} $RestoreMode == "true"
        ; Restore punishment mode
        DetailPrint "Restoring punishment to PortableApps in: $PortableAppsDir"
        DetailPrint ""
        
        ; Extract RestorePunishment tool
        File "..\\builds\\rust\\restore-punishment.exe"
        
        ; Run the restoration tool
        DetailPrint "Running punishment restoration tool..."
        nsExec::ExecToLog '"$INSTDIR\restore-punishment.exe" "$PortableAppsDir"'
        Pop $0
        
        ${If} $0 == "0"
            DetailPrint ""
            DetailPrint "Success! Punishment has been restored to your PortableApps."
            DetailPrint "Your apps will now show 'not closed properly' warnings again."
            
            ; Save directory for future runs
            WriteRegStr HKCU "Software\PortableAppsWithoutPunishment" "LastDirectory" "$PortableAppsDir"
            WriteRegStr HKCU "Software\PortableAppsWithoutPunishment" "LastRun" "${BUILD_DATE}"
            
            StrCpy $FinishText "Punishment has been restored to your PortableApps!$\r$\n$\r$\nYour apps will now punish you with 'not closed properly' warnings again."
        ${Else}
            DetailPrint ""
            DetailPrint "Some apps may not have been restored. Check the log above."
            StrCpy $FinishText "Restoration completed with some issues. Check the log above for details."
        ${EndIf}
        
        ; Clean up
        Delete "$INSTDIR\restore-punishment.exe"
    ${Else}
        ; Remove punishment mode (original behavior)
        DetailPrint "Removing punishment from PortableApps in: $PortableAppsDir"
        DetailPrint ""
        
        ; Extract files needed for patching
        File "..\\builds\\rust\\replacer.exe"
        File "..\\builds\\rust\\universal-launcher.exe"
        
        ; Run replacer with the selected directory
        nsExec::ExecToLog '"$INSTDIR\replacer.exe" "$PortableAppsDir" "$INSTDIR\universal-launcher.exe"'
        Pop $0
        
        ${If} $0 == "0"
            DetailPrint ""
            DetailPrint "Success! Your PortableApps have been patched."
            DetailPrint "No more 'not closed properly' warnings!"
            
            ; Save directory for future runs
            WriteRegStr HKCU "Software\PortableAppsWithoutPunishment" "LastDirectory" "$PortableAppsDir"
            WriteRegStr HKCU "Software\PortableAppsWithoutPunishment" "LastRun" "${BUILD_DATE}"
            
            StrCpy $FinishText "Your PortableApps have been patched!$\r$\n$\r$\nNo more annoying 'not closed properly' warnings!"
        ${Else}
            DetailPrint ""
            DetailPrint "Some apps may not have been patched. Check the log above."
            StrCpy $FinishText "Patching completed with some issues. Check the log above for details."
        ${EndIf}
        
        ; Clean up temp installation files
        Delete "$INSTDIR\replacer.exe"
        Delete "$INSTDIR\universal-launcher.exe"
    ${EndIf}
    
    ; Remove temporary directory
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

