@echo off
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

:: 设置并读取局域网名称，steamid
set "current_name="
set "current_steamid="

for /f "tokens=1,2 delims==" %%a in ('type "%ini_file1%" 2^>nul') do (
    if "%%a"=="account_name" set "current_name=%%b"
    if "%%a"=="account_steamid" set "current_steamid=%%b"
)


:: 每次循环都清空选择变量
set "choice="

:: 显示当前配置和菜单
echo ==================================
echo      黑夜君临无缝联机实时配置
echo ==================================
echo.
echo 1. 局域网名称=!current_name!
echo 2. SteamID=!current_steamid!
echo 3. 打开存档文件夹
echo.
echo ================================
echo.
echo 选择要修改的项 (输入1或2或3)：
set /p "choice="

:: 处理直接回车的情况
if "!choice!"=="" (
    echo.
    echo 无效选择，请重新输入
    timeout /t 2 >nul
    goto main_loop
)

:: 根据选择1，2，3，4执行相应操作
if "!choice!"=="1" goto modify_name
if "!choice!"=="2" goto modify_steamid
if "!choice!"=="3" goto open_folder

:: 如果输入无效，重新显示菜单
echo.
echo 无效选择，请重新输入
timeout /t 2 >nul
goto main_loop

:: 循环的开始
:modify_name

:: 清空变量以防止使用之前的值
set "new_name="

echo 输入新的局域网名称(不支持中文)：
set /p "new_name="
if not "!new_name!"=="" (
    powershell -Command "(gc '%ini_file1%') -replace 'account_name=.*', 'account_name=%new_name%' | Out-File '%ini_file1%' -Encoding utf8" >nul 2>&1
    echo 局域网名称已修改为：!new_name!
) else (
    echo 局域网名称未作任何修改
)
timeout /t 2 >nul
goto main_loop


:: 循环的开始
:modify_steamid

:: 清空变量以防止使用之前的值
set "new_steamid="

echo 查看steamid方法(复制到浏览器打开)：https://search.bilibili.com/article?keyword=如何查看steamid
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

:: 打开黑夜君临存档文件夹
:open_folder
echo 正在打开艾尔登法环存档文件夹...
explorer "%USERPROFILE%\AppData\Roaming\Nightreign"
echo 文件夹已打开，2秒后返回主菜单...
timeout /t 2 >nul
goto main_loop