using System.Diagnostics;
using System.IO;
using System.Windows;

namespace OpenSteam.Services
{
    public static class SettingsFunction
    {
        public static void CleanSteamCache()
        {
            string steamPath = SteamUtils.GetSteamPath();
            if (string.IsNullOrEmpty(steamPath)) return;

            string appCache = Path.Combine(steamPath, "appcache");

            try
            {
                if (Directory.Exists(appCache))
                {
                    Directory.Delete(appCache, true);
                    MessageBox.Show("Cache folder deleted. Restart Steam to take effect.");
                }
            }
            catch (Exception ex)
            {
                MessageBox.Show($"Error cleaning cache: {ex.Message}");
            }
        }

        public static void OpenFolder()
        {
            string steamPath = SteamUtils.GetSteamPath();
            string appsPath = Path.Combine(steamPath);

            if (Directory.Exists(appsPath))
                Process.Start("explorer.exe", appsPath);
        }

        public static void BackupSteamConfig()
        {
            string steamPath = SteamUtils.GetSteamPath();
            string configSource = Path.Combine(steamPath, "config");
            string backupDest = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "Backup_Config");

            try
            {
                if (Directory.Exists(configSource))
                {
                    if (!Directory.Exists(backupDest)) Directory.CreateDirectory(backupDest);

                    foreach (string file in Directory.GetFiles(configSource))
                    {
                        File.Copy(file, Path.Combine(backupDest, Path.GetFileName(file)), true);
                    }
                    MessageBox.Show("Config backup created successfully!");
                    Process.Start("explorer.exe", backupDest);
                }
            }
            catch (Exception ex)
            {
                MessageBox.Show($"Backup failed: {ex.Message}");
            }
        }
        public static async Task SteamFolderBackup()
        {
            string steamPath = SteamUtils.GetSteamPath();
            string backupDest = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.MyDocuments), "BackupSteam");

            // List of folders to ignore
            var foldersToExclude = new List<string> { "steamapps", "common", "public" };

            try
            {
                if (Directory.Exists(steamPath))
                {
                    var result = MessageBox.Show("Do you want to back up your Steam folder? This may take several minutes.",
                                                 "Backup Steam", MessageBoxButton.YesNo, MessageBoxImage.Question);

                    if (result == MessageBoxResult.Yes)
                    {
                        if (!Directory.Exists(backupDest)) Directory.CreateDirectory(backupDest);

                        await Task.Run(() =>
                        {
                            foreach (string file in Directory.GetFiles(steamPath))
                            {
                                try
                                {
                                    string destFile = Path.Combine(backupDest, Path.GetFileName(file));
                                    File.Copy(file, destFile, true);
                                }
                                catch { }
                            }

                            // 2. Copy directories while respecting the exclusion list
                            foreach (string dirPath in Directory.GetDirectories(steamPath))
                            {
                                string folderName = Path.GetFileName(dirPath);

                                if (foldersToExclude.Contains(folderName.ToLower())) continue;

                                string destDir = Path.Combine(backupDest, folderName);
                                CopyDirectory(dirPath, destDir);
                            }
                        });

                        MessageBox.Show("Steam backup completed successfully!", "Success", MessageBoxButton.OK, MessageBoxImage.Information);
                        Process.Start("explorer.exe", backupDest);
                    }
                }
                else
                {
                    MessageBox.Show("Steam path not found.", "Error", MessageBoxButton.OK, MessageBoxImage.Error);
                }
            }
            catch (Exception ex)
            {
                MessageBox.Show($"Backup failed: {ex.Message}", "Error", MessageBoxButton.OK, MessageBoxImage.Error);
            }
        }

        private static void CopyDirectory(string sourceDir, string destDir)
        {
            Directory.CreateDirectory(destDir);

            // Copy all files in the current directory
            foreach (string file in Directory.GetFiles(sourceDir))
            {
                try
                {
                    string destFile = Path.Combine(destDir, Path.GetFileName(file));
                    File.Copy(file, destFile, true);
                }
                catch { }
            }

            // Recursive call for subdirectories
            foreach (string subDir in Directory.GetDirectories(sourceDir))
            {
                string destSubDir = Path.Combine(destDir, Path.GetFileName(subDir));
                CopyDirectory(subDir, destSubDir);
            }
        }
    }
}