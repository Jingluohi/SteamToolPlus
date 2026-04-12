using System;
using System.IO;
using System.Text;

namespace SteamToolPlus;

internal static class FileLogger
{
    private static readonly object SyncRoot = new();
    private static readonly string LogDirectory = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "logs");
    private static readonly string LogFilePath = Path.Combine(LogDirectory, $"{DateTime.Now:yyyy-MM-dd-HH-mm-ss}.log");

    internal static string CurrentLogFilePath => LogFilePath;

    internal static void Info(string message)
    {
        Write("INFO", message);
    }

    internal static void Error(string message, Exception? exception = null)
    {
        if (exception == null)
        {
            Write("ERROR", message);
            return;
        }

        Write("ERROR", $"{message}{Environment.NewLine}{exception}");
    }

    private static void Write(string level, string message)
    {
        try
        {
            lock (SyncRoot)
            {
                Directory.CreateDirectory(LogDirectory);
                File.AppendAllText(
                    LogFilePath,
                    $"[{DateTime.Now:yyyy-MM-dd HH:mm:ss.fff}] [{level}] {message}{Environment.NewLine}",
                    Encoding.UTF8);
            }
        }
        catch
        {
            // 避免日志写入失败影响主流程
        }
    }
}
