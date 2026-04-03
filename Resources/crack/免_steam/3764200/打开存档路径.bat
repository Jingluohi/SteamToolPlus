@echo off
chcp 65001 >nul

:: 打开生化危机9学习版存档文件夹
:open_folder


set "save_path=%USERPROFILE%\AppData\Roaming\GSE Saves\3764200\remote\win64_save"
if exist "%save_path%" (
    echo 正在打开生化危机9学习版存档文件夹...
    echo.
    explorer "%save_path%"
    echo 文件夹已打开，5秒后自动关闭...
timeout /t 5 >nul
) else (
    echo 存档文件夹不存在: %save_path%
    echo.
    echo 需要先用补丁进入游戏进行基础设置以生成存档文件夹。10秒后自动关闭...
timeout /t 10 >nul
)
