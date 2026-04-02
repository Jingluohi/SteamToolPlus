@echo off
:: 这是一个注释符号，代表这一行是代码注释，不会显示到输出内容上。

:: 启用了延迟环境变量扩展功能
setlocal enabledelayedexpansion

:: 设置需要修改的ini文件路径
set "ini_file1=SeamlessCoop\ersc_settings.ini"

:: 循环的开始
:main_loop

:: 清除已输出的内容
cls

:: 检测文件是否存在
set "missing_files="
if not exist "%ini_file1%" set "missing_files=!missing_files!"%ini_file1%" "

:: 输出不存在的文件名
if not "!missing_files!"=="" (
    echo 错误: 以下配置文件不存在:
    echo !missing_files!
    echo.
    echo 请确保文件存在后再运行此脚本。
    echo.
    pause
    exit /b 1
)

:: 设置并读取联机密码
set "current_cooppassword="

for /f "tokens=2 delims==" %%a in ('findstr /i "cooppassword" "%ini_file1%" 2^>nul') do (
    set "current_cooppassword=%%a"
    set "current_cooppassword=!current_cooppassword: =!"
)

:: 每次循环都清空选择变量
set "choice="

:: 显示当前配置和菜单
echo =================================
echo.
echo 当前联机密码=!current_cooppassword!
echo.
echo ================================
echo.



:: 清空变量以防止使用之前的值
set "new_cooppassword="

echo 输入新的联机密码:
set /p "new_cooppassword="
if not "!new_cooppassword!"=="" (
    :: 使用 .NET 方法以 UTF-8 无 BOM 读写文件
    powershell -Command "$c=[System.IO.File]::ReadAllText('%ini_file1%', [System.Text.Encoding]::UTF8); $c=$c -replace 'cooppassword = .*', 'cooppassword = %new_cooppassword%'; [System.IO.File]::WriteAllText('%ini_file1%', $c, [System.Text.Encoding]::UTF8)" >nul 2>&1
    echo 联机密码已修改为: !new_cooppassword!
) else (
    echo 联机密码未作任何修改
)
timeout /t 2 >nul
goto main_loop
