using OpenSteam.Models;
using System.Diagnostics;
using System.IO;
using System.Net.Http;
using System.Text;
using System.Text.Json;
using System.Text.RegularExpressions;
using System.Windows;

namespace OpenSteam.Services
{

    public static class SteamUtils
    {
        private static string _cachedSteamPath;
        private static List<Game> _memoryGameList;
        private static readonly HttpClient _httpClient = new HttpClient();

        public static void Reset()
        {
            try
            {
                Process[] processes = Process.GetProcessesByName("steam");

                if (processes.Length > 0)
                {
                    foreach (Process proceso in processes)
                    {
                        try
                        {
                            proceso.Kill();
                        }
                        catch { }
                    }
                }

                bool disableWeb = Properties.Settings.Default.DisableWebHelper;

                if (disableWeb)
                {
                    string steamPath = GetSteamPath();
                    string steamExe = Path.Combine(steamPath, "steam.exe");

                    Process.Start(steamExe, "-no-browser +open steam://open/minigameslist");
                }
                else
                {
                    ProcessStartInfo psi = new ProcessStartInfo
                    {
                        FileName = "steam://flushconfig",
                        UseShellExecute = true
                    };
                    Process.Start(psi);
                }
            }
            catch (Exception ex)
            {
                System.Windows.MessageBox.Show($"Something went wrong: {ex.Message}");
            }
        }

        public static async Task StopSteam()
        {
            try
                {
                    Process[] processes = Process.GetProcessesByName("steam");

                    if (processes.Length > 0)
                    {
                        foreach (Process proceso in processes)
                        {
                            try
                            {
                                proceso.Kill();
                            }
                            catch { }
                        }
                    }
                } catch { MessageBox.Show("Try closing Steam manually before continuing"); }
        }
        public static List<Game> GetInstalledGames()
        {
            var installedGames = new List<Game>();
            string steamPath = GetSteamPath();
            if (string.IsNullOrEmpty(steamPath)) return installedGames;

            var libraryPaths = new List<string> { steamPath };

            // Parse libraryfolders.vdf
            string vdfPath = Path.Combine(steamPath, "steamapps", "libraryfolders.vdf");
            if (File.Exists(vdfPath))
            {
                try
                {
                    string vdfContent = File.ReadAllText(vdfPath);
                    var matches = Regex.Matches(vdfContent, "\"path\"\\s+\"([^\"]+)\"");
                    foreach (Match match in matches)
                    {
                        string path = match.Groups[1].Value.Replace("\\\\", "\\");
                        if (!libraryPaths.Contains(path) && Directory.Exists(path))
                        {
                            libraryPaths.Add(path);
                        }
                    }
                }
                catch { }
            }

            foreach (var libPath in libraryPaths)
            {
                string steamAppsPath = Path.Combine(libPath, "steamapps");
                if (Directory.Exists(steamAppsPath))
                {
                    var acfFiles = Directory.GetFiles(steamAppsPath, "appmanifest_*.acf");
                    foreach (var acf in acfFiles)
                    {
                        try
                        {
                            string content = File.ReadAllText(acf);
                            var appidMatch = Regex.Match(content, "\"appid\"\\s+\"(\\d+)\"");
                            var nameMatch = Regex.Match(content, "\"name\"\\s+\"([^\"]+)\"");

                            if (appidMatch.Success && nameMatch.Success)
                            {
                                string appid = appidMatch.Groups[1].Value;
                                // Ignore common redistributables or non-game apps if possible
                                if (appid == "228980" || appid == "250820") continue; 

                                installedGames.Add(new Game
                                {
                                    appid = appid,
                                    name = nameMatch.Groups[1].Value
                                });
                            }
                        }
                        catch { }
                    }
                }
            }

            return installedGames.GroupBy(g => g.appid).Select(g => g.First()).OrderBy(g => g.name).ToList();
        }

        public static string GetSteamPath()
        {
            if (!string.IsNullOrEmpty(_cachedSteamPath))
            {
                return _cachedSteamPath;
            }

            string registryPath = Microsoft.Win32.Registry.GetValue(@"HKEY_CURRENT_USER\Software\Valve\Steam", "SteamPath", null) as string;
            if (registryPath != null)
            {
                _cachedSteamPath = registryPath.Replace("/", "\\");
                return _cachedSteamPath;
            }
            string defaultPath = @"C:\Program Files (x86)\Steam";
            if (Directory.Exists(defaultPath))
            {
                _cachedSteamPath = defaultPath;
                return _cachedSteamPath;
            }
            return null;
        }

        private const string JsonUrl = "https://raw.githubusercontent.com/SteamTools-Team/GameList/refs/heads/main/games.json";
        private static readonly string CacheDirectory = Path.Combine(Path.GetTempPath(), "OpenSteamData");
        private static readonly string CacheFilePath = Path.Combine(CacheDirectory, "games.json");
        public static async Task<List<Game>> DownloadGameListAsync()
        {
            if (_memoryGameList != null && _memoryGameList.Count > 0) return _memoryGameList;

            try
            {
                if (File.Exists(CacheFilePath))
                {
                    DateTime lastWriteTime = File.GetLastWriteTime(CacheFilePath);
                    if (lastWriteTime.Date == DateTime.Today)
                    {
                        string localJson = await File.ReadAllTextAsync(CacheFilePath);
                        _memoryGameList = DeserializeGames(localJson);
                        return _memoryGameList;
                    }
                }

                string jsonContent = await _httpClient.GetStringAsync(JsonUrl);

                if (!Directory.Exists(CacheDirectory))
                    Directory.CreateDirectory(CacheDirectory);

                await File.WriteAllTextAsync(CacheFilePath, jsonContent);

                _memoryGameList = DeserializeGames(jsonContent);
                return _memoryGameList;
            }
            catch (Exception ex)
            {

                if (File.Exists(CacheFilePath))
                {
                    string oldJson = await File.ReadAllTextAsync(CacheFilePath);
                    _memoryGameList = DeserializeGames(oldJson);
                    return _memoryGameList;
                }

                return new List<Game>();
            }
        }


        private static List<Game> DeserializeGames(string json)
        {
            var options = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
            return JsonSerializer.Deserialize<List<Game>>(json, options) ?? new List<Game>();
        }

        public static List<Game> GetFilteredGames(string searchInput, List<Game> fullGameList)
        {
            if (fullGameList == null || fullGameList.Count == 0)
                return new List<Game>();

            if (string.IsNullOrWhiteSpace(searchInput))
                return fullGameList;
            string cleanInput = searchInput.Trim();

            bool isNumeric = true;
            foreach (char c in cleanInput)
            {
                if (!char.IsDigit(c)) { isNumeric = false; break; }
            }

            var query = fullGameList.AsEnumerable();

            if (isNumeric)
            {

                return query.Where(g => g.appid == cleanInput).ToList();
            }
            else
            {

                return query.Where(g =>
                    g.name != null &&
                    g.name.Contains(cleanInput, StringComparison.OrdinalIgnoreCase))
                    .ToList();
            }
        }

        public static async Task<(int updated, int luaUpdated)> FixManifests(string steamPath)
        {
            const string API = "https://api.steamproof.net";
            string pluginDir = Path.Combine(steamPath, "config", Properties.Settings.Default.LuaPath);
            string depotCache = Path.Combine(steamPath, "depotcache");

            if (!Directory.Exists(pluginDir))
                throw new DirectoryNotFoundException($"No found: {pluginDir}");

            Directory.CreateDirectory(depotCache);

            var luaFiles = Directory.GetFiles(pluginDir, "*.lua");
            var needsUpdateIds = new List<string>();
            var luaData = new Dictionary<string, (string path, string content)>();

            foreach (var file in luaFiles)
            {
                string appId = Path.GetFileNameWithoutExtension(file);
                if (!Regex.IsMatch(appId, @"^\d+$")) continue;

                string content = await File.ReadAllTextAsync(file);

                var depotIds = Regex.Matches(content, @"addappid\((\d+)")
                                    .Select(m => m.Groups[1].Value)
                                    .ToList();

                luaData[appId] = (file, content);

                bool missing = depotIds.Any(d =>
                    !Directory.GetFiles(depotCache, $"{d}_*.manifest").Any()
                );

                if (missing || depotIds.Count == 0)
                    needsUpdateIds.Add(appId);
            }

            if (needsUpdateIds.Count == 0) return (0, 0);

            int totalManifestsDownloaded = 0;
            int luaFilesUpdated = 0;

            string idsQuery = string.Join(",", needsUpdateIds);
            string jsonResponse = await _httpClient.GetStringAsync($"{API}/apps/depots?ids={idsQuery}");
            using var doc = JsonDocument.Parse(jsonResponse);

            if (!doc.RootElement.TryGetProperty("apps", out var appsArray)) return (0, 0);

            foreach (var app in appsArray.EnumerateArray())
            {
                string appId = app.GetProperty("appId").ToString();
                if (!luaData.ContainsKey(appId)) continue;

                var appInfo = luaData[appId];
                var manifestEntries = new List<string>();

                try
                {
                    string dlInfoJson = await _httpClient.GetStringAsync($"{API}/app/{appId}/manifests/download");
                    using var dlDoc = JsonDocument.Parse(dlInfoJson);

                    if (dlDoc.RootElement.TryGetProperty("manifests", out var manifestList))
                    {
                        foreach (var m in manifestList.EnumerateArray())
                        {
                            string dId = m.GetProperty("depotId").ToString();
                            string mId = m.GetProperty("manifestId").ToString();
                            string dlUrl = m.GetProperty("url").ToString();

                            string fileName = $"{dId}_{mId}.manifest";
                            string fullPath = Path.Combine(depotCache, fileName);

                            if (!File.Exists(fullPath))
                            {
                                byte[] data = await _httpClient.GetByteArrayAsync(dlUrl);
                                await File.WriteAllBytesAsync(fullPath, data);
                                totalManifestsDownloaded++;
                            }
                        }
                    }
                }
                catch { }

                var depotsFromApi = app.GetProperty("depots").EnumerateArray();
                foreach (var depot in depotsFromApi)
                {
                    string dId = depot.GetProperty("depotId").ToString();
                    if (depot.TryGetProperty("manifests", out var manifests) &&
                        manifests.TryGetProperty("public", out var pub))
                    {
                        string mId = pub.GetProperty("manifestId").ToString();

                        if (depot.TryGetProperty("maxSize", out var sz) && sz.GetRawText() != "0")
                            manifestEntries.Add($"setManifestid({dId}, \"{mId}\", {sz})");
                        else
                            manifestEntries.Add($"setManifestid({dId}, \"{mId}\")");
                    }
                }

                if (manifestEntries.Count > 0)
                {
                    string cleanContent = appInfo.content;

                    cleanContent = Regex.Replace(cleanContent, @"\r?\n?setManifestid\([^\)]*\);?", "", RegexOptions.IgnoreCase);

                    cleanContent = Regex.Replace(cleanContent, @"(\r?\n-- SteamProof Manifests.*)", "", RegexOptions.Singleline);

                    cleanContent = cleanContent.TrimEnd();

                    StringBuilder sb = new StringBuilder();
                    sb.AppendLine(cleanContent);
                    sb.AppendLine();
                    sb.AppendLine($"-- SteamProof Manifests (updated {DateTime.UtcNow:yyyy-MM-dd HH:mm UTC})");

                    foreach (var entry in manifestEntries)
                        sb.AppendLine(entry);

                    await File.WriteAllTextAsync(appInfo.path, sb.ToString(), new UTF8Encoding(false));
                    luaFilesUpdated++;
                }
            }

            return (totalManifestsDownloaded, luaFilesUpdated);
        }
    }
}
