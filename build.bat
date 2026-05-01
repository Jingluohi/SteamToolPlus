@echo off
chcp 65001 >nul
echo ==========================================
echo SteamTool Plus Build Script
echo ==========================================
echo.

REM Check if Node.js is installed
node --version >nul 2>&1
if errorlevel 1 (
    echo Error: Node.js is not installed or not in PATH
    echo Please install Node.js from https://nodejs.org/
    pause
    exit /b 1
)

REM Check if Rust is installed
rustc --version >nul 2>&1
if errorlevel 1 (
    echo Error: Rust is not installed or not in PATH
    echo Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

echo [1/4] Installing dependencies...
call npm install
if errorlevel 1 (
    echo Error: Failed to install dependencies
    pause
    exit /b 1
)

echo.
echo [2/4] Building frontend...
call npm run build
if errorlevel 1 (
    echo Error: Failed to build frontend
    pause
    exit /b 1
)

echo.
echo [3/4] Building Tauri application...
call npm run tauri build -- --no-bundle
if errorlevel 1 (
    echo Error: Failed to build Tauri application
    pause
    exit /b 1
)

echo.
echo [4/4] Copying files to PublicOut...

REM Create PublicOut directory
if not exist "PublicOut" mkdir "PublicOut"

REM Copy exe file
copy "src-tauri\target\release\steam-tool-plus.exe" "PublicOut\SteamToolPlus.exe" >nul 2>&1
if errorlevel 1 (
    echo Warning: Could not find built executable
    echo Checking for alternative locations...
    dir "src-tauri\target\release\*.exe" /b 2>nul
) else (
    echo Copied: SteamToolPlus.exe
)

REM Copy resources directory if exists
if exist "resources" (
    if exist "PublicOut\resources" rmdir /s /q "PublicOut\resources"
    xcopy /e /i /y "resources" "PublicOut\resources" >nul
    if errorlevel 1 (
        echo Warning: Failed to copy resources
    ) else (
        echo Copied: resources\
    )
)

REM Create necessary directories
if not exist "PublicOut\config" mkdir "PublicOut\config"
if not exist "PublicOut\extensions" mkdir "PublicOut\extensions"
if not exist "PublicOut\logs" mkdir "PublicOut\logs"

echo Created: config, extensions, logs directories
echo.
echo ==========================================
echo Build completed successfully!
echo ==========================================
echo Output location: PublicOut\
echo   - SteamToolPlus.exe
echo   - resources\
echo   - config\
echo   - extensions\
echo   - logs\
echo ==========================================
echo.
pause
