@echo off
chcp 65001 >nul
echo ==========================================
echo SteamTool Plus Development Server
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

REM Check if dependencies are installed
if not exist "node_modules" (
    echo Dependencies not found. Installing...
    call npm install
    if errorlevel 1 (
        echo Error: Failed to install dependencies
        pause
        exit /b 1
    )
)

echo Starting development server...
echo This will open the application in development mode.
echo.
echo Press Ctrl+C to stop the server
echo.

REM Run Tauri development server
call npm run tauri:dev

if errorlevel 1 (
    echo.
    echo Error: Failed to start development server
    pause
    exit /b 1
)

pause
