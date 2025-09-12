@echo off
REM Silent patching script for PortableApps Without Punishment
REM Usage: silent-patch.bat [directory] [remove|restore]

setlocal
set "TARGET_DIR=%~1"
set "MODE=%~2"

if "%TARGET_DIR%"=="" (
    echo Usage: silent-patch.bat [directory] [remove^|restore]
    echo Example: silent-patch.bat "C:\PortableApps" remove
    exit /b 1
)

if not exist "%TARGET_DIR%" (
    echo Error: Directory does not exist: %TARGET_DIR%
    exit /b 1
)

set "SCRIPT_DIR=%~dp0"
set "LOGFILE=%TEMP%\PortableApps_NoPunish_silent_%DATE:~-4%-%DATE:~4,2%-%DATE:~7,2%_%TIME:~0,2%%TIME:~3,2%.log"
set "LOGFILE=%LOGFILE: =0%"

if /i "%MODE%"=="restore" (
    echo Restoring punishment to PortableApps in: %TARGET_DIR%
    "%SCRIPT_DIR%builds\rust\restore-punishment.exe" "%TARGET_DIR%" --log "%LOGFILE%"
) else (
    echo Removing punishment from PortableApps in: %TARGET_DIR%
    echo Using universal launcher: %SCRIPT_DIR%builds\rust\universal-launcher.exe
    "%SCRIPT_DIR%builds\rust\replacer.exe" "%TARGET_DIR%" "%SCRIPT_DIR%builds\rust\universal-launcher.exe" --log "%LOGFILE%"
)

if %ERRORLEVEL%==0 (
    echo Success! Log file: %LOGFILE%
) else (
    echo Operation failed. Check log file: %LOGFILE%
    exit /b %ERRORLEVEL%
)