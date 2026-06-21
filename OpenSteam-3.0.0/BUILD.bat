@echo off
title Compilador OpenSteam - .NET
set "SOLUTION_NAME=OpenSteam.sln"
set "OUTPUT_DIR=.\Build_Output"

echo ===========================================
echo   Compilando %SOLUTION_NAME%
echo ===========================================

:: 1. Limpiar compilaciones anteriores
echo [+] Limpiando archivos temporales...
dotnet clean %SOLUTION_NAME% -c Release > nul

:: 2. Restaurar dependencias
echo [+] Restaurando paquetes NuGet...
dotnet restore %SOLUTION_NAME%

:: 3. Compilar y Publicar
:: -c Release: Optimiza el código
:: -o %OUTPUT_DIR%: Guarda el resultado en la carpeta especificada
echo [+] Compilando proyecto en modo Release...
:: dotnet build %SOLUTION_NAME% -c Release --no-restore
dotnet publish %SOLUTION_NAME% -c Release -r win-x64 --self-contained true -p:PublishSingleFile=true

if %errorlevel% neq 0 (
    echo.
    echo [X] ERROR: La compilacion ha fallado.
    pause
    exit /b %errorlevel%
)

echo.
echo ===========================================
echo   [OK] PROCESO COMPLETADO
echo   Archivos en: %OUTPUT_DIR%
echo ===========================================
start .\bin\Release\net9.0-windows\win-x64\OpenSteam.exe
pause