@echo off
chcp 65001 >nul
:: 这是一个注释符号，代表这一行是代码注释，不会显示到输出内容上。

:: 启用了延迟环境变量扩展功能
setlocal enabledelayedexpansion

:: 设置需要修改的ini文件路径
set "ini_file1=steam_settings\configs.user.ini"

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

:: 设置并读取steamid值
set "current_steamid="

for /f "tokens=1,2 delims==" %%a in ('type "%ini_file1%" 2^>nul') do (
    if "%%a"=="account_steamid" set "current_steamid=%%b"
)


:: 每次循环都清空选择变量
set "choice="

:: 显示当前配置和菜单

echo ================================
echo.
echo SteamID=!current_steamid!
echo.
echo ================================
echo.
echo.
echo.

:: 循环的开始

:modify_steamid

:: 清空变量以防止使用之前的值
set "new_steamid="


echo 注：SteamID是17位纯数字，没有其他情况。
echo.
echo 查看steamid方法(选中链接Ctrl+C到浏览器打开)：https://search.bilibili.com/article?keyword=如何查看steamid
echo 输入新的SteamID：
set /p "new_steamid="
if not "!new_steamid!"=="" (
    :: 使用正则表达式同时匹配有#和无#的情况，确保去掉注释符号并设置新值
    powershell -Command "(gc '%ini_file1%') -replace '^#?account_steamid=.*', 'account_steamid=%new_steamid%' | Out-File '%ini_file1%' -Encoding utf8" >nul 2>&1
    echo SteamID已修改为: !new_steamid!
) else (
    echo SteamID未作任何修改
)
timeout /t 2 >nul
goto main_loop