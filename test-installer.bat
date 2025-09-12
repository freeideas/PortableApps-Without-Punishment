@echo off
REM Test script for the PortableApps Without Punishment installer
REM Usage: test-installer.bat [remove|restore]

setlocal
set "MODE=%~1"
set "TEST_DIR=%~dp0test"
set "INSTALLER=%~dp0releases\PortableApps Without Punishment 2025-09-12-1305.exe"

if not exist "%TEST_DIR%" (
    echo Error: Test directory not found: %TEST_DIR%
    exit /b 1
)

echo ====================================
echo Testing PortableApps Without Punishment Installer
echo ====================================
echo Test Directory: %TEST_DIR%
echo Mode: %MODE%
echo.

if /i "%MODE%"=="restore" (
    echo Running installer in RESTORE mode with TEST flag...
    "%INSTALLER%" /TEST /RESTORE /D="%TEST_DIR%"
) else (
    echo Running installer in REMOVE mode with TEST flag...
    "%INSTALLER%" /TEST /D="%TEST_DIR%"
)

if %ERRORLEVEL%==0 (
    echo.
    echo Test completed successfully!
    echo Check the latest log in: %%TEMP%%\PortableApps_NoPunish_*.log
) else (
    echo.
    echo Test failed with error code: %ERRORLEVEL%
)