using System.Diagnostics;
using System.IO;
using System.Net.Http;
using System.Windows;
using OpenSteam.Views;

namespace OpenSteam.Services
{
    public static class Update
    {
        private static readonly HttpClient _httpClient = new HttpClient();

        public static string GetVersion()
        {
            var version = System.Reflection.Assembly.GetExecutingAssembly().GetName().Version;
            return $"{version.Major}.{version.Minor}.{version.Build}";
        }

        public static async Task CheckForUpdates()
        {
            try
            {
                _httpClient.DefaultRequestHeaders.Add("User-Agent", "OpenSteamManager");

                string latestVersionString = await _httpClient.GetStringAsync("https://raw.githubusercontent.com/Abrahamqb/OpenSteam/refs/heads/master/version.txt");
                latestVersionString = latestVersionString.Trim();

                Version latestVersion = new Version(latestVersionString);
                Version currentVersion = System.Reflection.Assembly.GetExecutingAssembly().GetName().Version;

                if (latestVersion > currentVersion)
                {
                    if (MessageBoxResult.Yes == MessageBox.Show($"A new version is available: v{latestVersion}\n\nYou are using: v{currentVersion.Major}.{currentVersion.Minor}.{currentVersion.Build} \nDo you want to update?", "Update Available", MessageBoxButton.YesNo, MessageBoxImage.Information))
                    {
                        NotificationWindow win = new NotificationWindow("Updating... May take a few seconds :) ", 5);
                        win.Show();
                        await DownloadAndInstallUpdate();
                    }
                }
            }
            catch (Exception ex)
            {
                MessageBox.Show("Error checking for updates: " + ex.Message, "Error", MessageBoxButton.OK, MessageBoxImage.Error);
            }
        }

        private static async Task DownloadAndInstallUpdate()
        {
            string appPath = Process.GetCurrentProcess().MainModule.FileName;
            string appDir = Path.GetDirectoryName(appPath);
            string newAppPath = Path.Combine(appDir, "OpenSteam_new.exe");

            byte[] data = await _httpClient.GetByteArrayAsync("https://github.com/abrahamqb/OpenSteam/releases/latest/download/OpenSteam.exe");
            await File.WriteAllBytesAsync(newAppPath, data);

            string batchCode = $@"
@echo off
timeout /t 2 /nobreak > nul
del ""{appPath}""
ren ""{newAppPath}"" ""{Path.GetFileName(appPath)}""
start """" ""{appPath}""
del ""%~f0""
";

            string batchPath = Path.Combine(appDir, "updater.bat");
            File.WriteAllText(batchPath, batchCode);


            Process.Start(new ProcessStartInfo { FileName = batchPath, CreateNoWindow = true, UseShellExecute = false });
            Application.Current.Shutdown();
        }
        public static async Task GetNews()
        {
            try
            {
                string TempPath = Path.Combine(Path.GetTempPath(), "OpenSteamData");
                string NameNews = "/News.txt";
                await Task.Run(() =>
                {
                    if (!File.Exists(TempPath + NameNews))
                    {
                        Directory.CreateDirectory(TempPath);
                        File.Create(TempPath + NameNews).Close();
                    }
                });

                _httpClient.DefaultRequestHeaders.Add("User-Agent", "OpenSteamManager");
                string NewsInfo = await _httpClient.GetStringAsync("https://raw.githubusercontent.com/Abrahamqb/OpenSteamMore-Dev/refs/heads/main/News");
                NewsInfo = NewsInfo.Trim().Replace("\r\n", "\n").Normalize();
                if (string.IsNullOrEmpty(NewsInfo))
                {
                    return;
                }

                if (NewsInfo != File.ReadAllText(TempPath + NameNews))
                {
                    MessageBox.Show($"{NewsInfo}", "Dev", MessageBoxButton.OK, MessageBoxImage.Information);
                    File.WriteAllText(TempPath + NameNews, NewsInfo);
                }
            }
            catch
            {

            }
        }

    }
}
