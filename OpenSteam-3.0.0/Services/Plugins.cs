using System.Diagnostics;
using System.IO;
using System.IO.Compression;
using System.Net.Http;
using System.Windows;
using OpenSteam.Views;

namespace OpenSteam.Services
{
    public class Plugins
    {
        private static readonly HttpClient _httpClient = new HttpClient();

        public async Task ManagePluginsInstall()
        {
            try
            {
                byte[] millenniumInstaller = await _httpClient.GetByteArrayAsync("https://github.com/SteamClientHomebrew/Installer/releases/latest/download/MillenniumInstaller-Windows.exe");
                string installerPath = Path.Combine(Path.GetTempPath(), "MillenniumInstaller.exe");

                File.WriteAllBytes(installerPath, millenniumInstaller);

                ProcessStartInfo startInfo = new ProcessStartInfo
                {
                    FileName = installerPath,
                    UseShellExecute = true,
                    CreateNoWindow = false
                };
                Process p = Process.Start(startInfo);

                if (p != null)
                {
                    await Task.Run(() =>
                    {
                        p.WaitForExit();
                    });
                    NotificationWindow win = new NotificationWindow("¡Millennium installed!", 2);
                    win.Show();
                }
            }
            catch (Exception ex)
            {
                MessageBox.Show("Critical error: " + ex.Message);
            }
        }

        public async Task LuaManagerInstallerAsync(string steamPath)
        {
            if (string.IsNullOrEmpty(steamPath) || !Directory.Exists(steamPath))
            {
                MessageBox.Show("Steam path not found.", "Error", MessageBoxButton.OK, MessageBoxImage.Warning);
                return;
            }

            var pluginsFolder = Path.Combine(steamPath, "plugins");
            string tempZip = Path.Combine(Path.GetTempPath(), "LuaManager_Temp.zip");
            string targetPluginDir = Path.Combine(pluginsFolder, "LuaManager");

            try
            {
                _httpClient.DefaultRequestHeaders.Add("User-Agent", "OpenSteamManager");
                byte[] zipBytes = await _httpClient.GetByteArrayAsync("https://github.com/Abrahamqb/OpenSteam/raw/refs/heads/master/Plugins/LuaManager.zip");
                await File.WriteAllBytesAsync(tempZip, zipBytes);

                string extractError = await Task.Run(() =>
                {
                    if (!Directory.Exists(pluginsFolder))
                        Directory.CreateDirectory(pluginsFolder);

                    if (Directory.Exists(targetPluginDir))
                        Directory.Delete(targetPluginDir, true);

                    try
                    {
                        ZipFile.ExtractToDirectory(tempZip, pluginsFolder);
                        return null;
                    }
                    catch (Exception ex)
                    {
                        return ex.Message;
                    }
                });

                if (extractError != null)
                {
                    MessageBox.Show($"The LuaManager ZIP file is corrupt or already exists.\n{extractError}",
                        "Error", MessageBoxButton.OK, MessageBoxImage.Error);
                    return;
                }

                NotificationWindow win = new NotificationWindow("¡LuaManager successfully installed!", 2);
                win.Show();
            }
            catch (Exception ex)
            {
                MessageBox.Show($"Critical error: {ex.Message}", "Error", MessageBoxButton.OK, MessageBoxImage.Error);
            }
            finally
            {
                if (File.Exists(tempZip)) File.Delete(tempZip);
            }
        }

    }
}
