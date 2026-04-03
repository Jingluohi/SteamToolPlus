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

:: 设置并读取局域网名称，steamid
set "current_name="
set "current_steamid="

for /f "tokens=1,2 delims==" %%a in ('type "%ini_file1%" 2^>nul') do (
    if "%%a"=="account_name" set "current_name=%%b"
    if "%%a"=="account_steamid" set "current_steamid=%%b"
)

:: 如果未找到未注释的 account_steamid，则 current_steamid 保持为空（表示未配置）


:: 每次循环都清空选择变量
set "choice="

:: 显示当前配置和菜单
echo ==================================
echo      模拟器实时配置
echo ==================================
echo.
echo 1. 玩家名称=!current_name!
if defined current_steamid (
    echo 2. SteamID=!current_steamid!
) else (
    echo 2. SteamID=待填写
)
echo 3. 创建存放授权文件夹
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
if "!choice!"=="3" goto modify_shouquan

:: 如果输入无效，重新显示菜单
echo.
echo 无效选择，请重新输入
timeout /t 2 >nul
goto main_loop

:: 循环的开始
:: "必须存在的换行注释，否则报错。"
:modify_name

:: 清空变量以防止使用之前的值
set "new_name="

echo 输入新的局域网名称:
set /p "new_name="
if not "!new_name!"=="" (
    :: 使用 .NET 方法以 UTF-8 无 BOM 读写文件
    powershell -Command "$c=[System.IO.File]::ReadAllText('%ini_file1%', [System.Text.Encoding]::UTF8); $c=$c -replace 'account_name=.*', 'account_name=%new_name%'; [System.IO.File]::WriteAllText('%ini_file1%', $c, [System.Text.Encoding]::UTF8)" >nul 2>&1
    echo 玩家名称已修改为: !new_name!
) else (
    echo 玩家名称未作任何修改
)
timeout /t 2 >nul
goto main_loop


:: 循环的开始
:: "必须存在的换行注释，否则报错。"
:modify_steamid

:: 清空变量以防止使用之前的值
set "new_steamid="

echo.
echo 注：SteamID是17位纯数字，没有其他情况。
echo.
echo 查看steamid方法(复制到浏览器打开)：https://search.bilibili.com/article?keyword=如何查看steamid
echo 输入新的SteamID：
set /p "new_steamid="
if not "!new_steamid!"=="" (
    :: 写入未注释的行，值为用户输入（无论原行是注释还是未注释，均替换为未注释行）
    powershell -Command "$c=[System.IO.File]::ReadAllText('%ini_file1%', [System.Text.Encoding]::UTF8); $c=$c -replace '(?m)^\s*#?\s*account_steamid\s*=.*', 'account_steamid=%new_steamid%'; [System.IO.File]::WriteAllText('%ini_file1%', $c, [System.Text.Encoding]::UTF8)" >nul 2>&1
    echo SteamID已修改为: !new_steamid!
) else (
    echo SteamID未作任何修改
)
timeout /t 2 >nul
goto main_loop

:: 循环的开始
:: "必须存在的换行注释，否则报错。"
:modify_shouquan

echo.
echo 输入Steam账号好友代码
set "friendcode="
set /p friendcode=请输入Steam好友代码：
for /f "tokens=* delims= " %%a in ("!friendcode!") do set "friendcode=%%a"
if "!friendcode!"=="" (
    echo 好友代码不能为空，请重新输入。
    goto input
)

set "target=userdata\%friendcode%\3321460"

if not exist "%target%" (
    mkdir "%target%"
    if exist "%target%" (
        echo 文件夹已成功创建：%target%
    ) else (
        echo 创建失败，请检查权限或路径。
    )
) else (
    echo 文件夹已经存在：%target%
timeout /t 5 >nul
goto main_loop
)
timeout /t 5 >nul
goto main_loop
