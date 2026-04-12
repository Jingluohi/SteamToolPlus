﻿using Application = System.Windows.Application;
using System.Windows;
using MessageBox = System.Windows.MessageBox;

namespace SteamToolPlus;

public partial class App : Application
{
    protected override void OnStartup(StartupEventArgs e)
    {
        base.OnStartup(e);
        System.Text.Encoding.RegisterProvider(System.Text.CodePagesEncodingProvider.Instance);

        var appVersion = typeof(App).Assembly.GetName().Version?.ToString() ?? "unknown";
        FileLogger.Info($"应用启动，版本：{appVersion}，日志文件：{FileLogger.CurrentLogFilePath}");

        // 添加全局异常处理
        AppDomain.CurrentDomain.UnhandledException += (sender, args) =>
        {
            if (args.ExceptionObject is Exception ex)
            {
                FileLogger.Error("应用程序发生严重错误", ex);
            }
            else
            {
                FileLogger.Error($"应用程序发生严重错误：{args.ExceptionObject}");
            }

            MessageBox.Show($"应用程序发生严重错误：{args.ExceptionObject}\n\n程序将退出。", "致命错误", MessageBoxButton.OK, MessageBoxImage.Error);
            Current.Shutdown();
            Environment.Exit(1);
        };

        DispatcherUnhandledException += (sender, args) =>
        {
            FileLogger.Error("UI 线程发生错误", args.Exception);
            MessageBox.Show($"UI 线程发生错误：{args.Exception.Message}\n\n{args.Exception.StackTrace}", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
            args.Handled = true;
            Current.Shutdown();
            Environment.Exit(1);
        };
    }
}
