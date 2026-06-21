using Microsoft.Win32;
using System.IO;
using System.Net.Http;
using System.Text;
using System.Text.Json;
using System.Windows;
using OpenSteam.Views;

namespace OpenSteam.Services
{
    public class LuaLoaders
    {
        private static readonly HttpClient _staticHttpClient = new HttpClient(); // For static methods
        private readonly HttpClient _instanceHttpClient = new HttpClient(); // For instance methods

        public void Load(string path)
        {
            if (string.IsNullOrEmpty(path))
            {
                MessageBox.Show("The Steam path was not detected.");
                return;
            }
            string luaPathSteam = Path.Combine(path, "config", Properties.Settings.Default.LuaPath);
            OpenFileDialog luaLoader = new OpenFileDialog
            {
                Filter = "Lua Files|*.lua",
                Title = "select Lua"
            };

            if (luaLoader.ShowDialog() == true)
            {
                try
                {
                    if (!Directory.Exists(luaPathSteam))
                    {
                        Directory.CreateDirectory(luaPathSteam);
                    }

                    string destinationFile = Path.Combine(luaPathSteam, luaLoader.SafeFileName);
                    File.Copy(luaLoader.FileName, destinationFile, true);
                    SteamUtils.FixManifests(SteamUtils.GetSteamPath());
                    NotificationWindow win = new NotificationWindow("¡Lua successfully loaded!", 2);
                    win.Show();
                }
                catch (UnauthorizedAccessException)
                {
                    MessageBox.Show("Error: You do not have permission to write to the Steam folder. Run as administrator.");
                }
                catch (Exception ex)
                {
                    MessageBox.Show($"Something went wrong: {ex.Message}");
                }
            }
        }

        public static async Task<string> SteamLuaGenerator(int appId, string path, int cacheDays = 7)
        {
            string apiUrl = $"https://api.steamproof.net/apps/depots?ids={appId}";
            string apiJson = await _staticHttpClient.GetStringAsync(apiUrl);

            string cacheDir = Path.Combine(path, "cache");
            Directory.CreateDirectory(cacheDir);

            string cacheFile = Path.Combine(cacheDir, "depotkeys.json");

            string[] keysUrls = new[]
            {
                "https://gitlab.com/steamautocracks/manifesthub/-/raw/main/depotkeys.json",
                "https://api.993499094.xyz/depotkeys.json"
            };

            var depotKeys = new Dictionary<string, string>();
            bool shouldRefresh = true;

            if (File.Exists(cacheFile))
            {
                DateTime lastWriteUtc = File.GetLastWriteTimeUtc(cacheFile);
                bool isExpired = lastWriteUtc < DateTime.UtcNow.AddDays(-cacheDays);

                if (!isExpired)
                {
                    try
                    {
                        string cachedJson = await File.ReadAllTextAsync(cacheFile, Encoding.UTF8);
                        depotKeys = JsonSerializer.Deserialize<Dictionary<string, string>>(cachedJson)
                                    ?? new Dictionary<string, string>();
                        shouldRefresh = false;
                    }
                    catch
                    {
                        shouldRefresh = true;
                    }
                }
            }


            if (shouldRefresh)
            {

                foreach (string url in keysUrls)
                {
                    try
                    {
                        string jsonFromServer = await _staticHttpClient.GetStringAsync(url);
                        var downloadedKeys = JsonSerializer.Deserialize<Dictionary<string, string>>(jsonFromServer);

                        if (downloadedKeys != null)
                        {
                            foreach (var kvp in downloadedKeys)
                            {
                                depotKeys.TryAdd(kvp.Key, kvp.Value);
                            }
                        }
                    }
                    catch
                    {
                        continue;
                    }
                }


                string finalCacheJson = JsonSerializer.Serialize(depotKeys);
                await File.WriteAllTextAsync(cacheFile, finalCacheJson, Encoding.UTF8);
            }

            using var doc = JsonDocument.Parse(apiJson);

            var apps = doc.RootElement.GetProperty("apps");
            if (apps.GetArrayLength() == 0)
                throw new Exception("API NOT WORK");

            var app = apps[0];
            var depots = app.GetProperty("depots");

            var sb = new StringBuilder();
            sb.AppendLine($"-- Open Steam Lua Generator {System.Reflection.Assembly.GetExecutingAssembly().GetName().Version} --");
            sb.AppendLine($"addappid({appId})");

            foreach (var depot in depots.EnumerateArray())
            {
                int depotId = depot.GetProperty("depotId").GetInt32();
                string depotIdKey = depotId.ToString();

                if (depotKeys.TryGetValue(depotIdKey, out var depotKey) && !string.IsNullOrWhiteSpace(depotKey))
                    sb.AppendLine($"addappid({depotId},1,\"{depotKey}\")");
                else
                    sb.AppendLine($"addappid({depotId},0,\"\")");

                if (depot.TryGetProperty("manifests", out var manifests) &&
                    manifests.ValueKind == JsonValueKind.Object)
                {
                    string? manifestId = null;

                    if (manifests.TryGetProperty("public", out var publicManifest) &&
                        publicManifest.TryGetProperty("manifestId", out var publicManifestId))
                    {
                        manifestId = publicManifestId.GetString();
                    }
                    else
                    {
                        foreach (var branch in manifests.EnumerateObject())
                        {
                            if (branch.Value.TryGetProperty("manifestId", out var anyManifestId))
                            {
                                manifestId = anyManifestId.GetString();
                                if (!string.IsNullOrWhiteSpace(manifestId))
                                    break;
                            }
                        }
                    }

                    if (!string.IsNullOrWhiteSpace(manifestId))
                        sb.AppendLine($"setManifestid({depotId},\"{manifestId}\")");
                }
            }

            string outputFile = Path.Combine(path, $"{appId}.lua");
            await File.WriteAllTextAsync(outputFile, sb.ToString(), Encoding.UTF8);

            return sb.ToString();
        }

        public async Task OnlineLoad(string ID, string path)
        {
            // Use the instance HttpClient
            _instanceHttpClient.DefaultRequestHeaders.Add("User-Agent", "OpenSteam-Manager/1.0");

            string luaPathSteam = Path.Combine(path, "config", Properties.Settings.Default.LuaPath);
            string ManifestPathSteam = Path.Combine(path, "depotcache");
            string tempZip = Path.Combine(Path.GetTempPath(), $"Lua_{ID}.zip");

            try
            {
                int appid = int.Parse(ID);
                var lua = await SteamLuaGenerator(appid, luaPathSteam);

                var result = await SteamUtils.FixManifests(path);

                NotificationWindow win = new NotificationWindow(
                    $"✔ Lua & Manifest loaded",
                    3
                );
                win.Show();

                await Task.Delay(1000);

                SteamUtils.Reset();
            }
            catch (Exception ex)
            {
                var result = await SteamUtils.FixManifests(path);
                MessageBox.Show($"Something went wrong: {ex.Message}", "Error");
            }
            finally
            {
                if (File.Exists(tempZip)) File.Delete(tempZip);
            }
        }

    }
}
