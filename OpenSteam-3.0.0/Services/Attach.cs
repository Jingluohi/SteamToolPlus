using OpenSteam.Services;
using OpenSteam.Views;
using System;
using System.IO;
using System.IO.Compression;
using System.Linq;
using System.Net.Http;
using System.Text.Json;
using System.Threading.Tasks;

public class Attach
{
    private static readonly HttpClient _httpClient = new HttpClient();

    public static string GetReleaseDownloadUrl(string jsonText)
    {
        if (string.IsNullOrWhiteSpace(jsonText))
            return null;
        try
        {
            using (JsonDocument doc = JsonDocument.Parse(jsonText))
            {
                JsonElement root = doc.RootElement;

                if (root.TryGetProperty("assets", out JsonElement assets))
                {
                    var releaseAsset = assets.EnumerateArray()
                        .FirstOrDefault(asset => asset.GetProperty("name").GetString().EndsWith("-Release.zip", StringComparison.OrdinalIgnoreCase));
                    if (releaseAsset.ValueKind != JsonValueKind.Undefined)
                    {
                        return releaseAsset.GetProperty("browser_download_url").GetString();
                    }
                }
            }
        }
        catch (JsonException)
        {
            Console.WriteLine("The provided text is not a valid JSON string.");
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Unexpected error: {ex.Message}");
        }
        return null;
    }

    public async Task PatchSteam(string path, bool Delet, int Mode)
    {
        if (Delet)
        {
            await SteamUtils.StopSteam();
            await Task.Delay(1000);

            string[] FilesDeleted = new[]
            {
            "xinput1_4.dll",
            "hid.dll",
            "dwmapi.dll",
            "OpenSteamTool.dll"
        };

            foreach (string file in FilesDeleted)
            {
                string fullPath = Path.Combine(path, file);
                try
                {
                    if (File.Exists(fullPath))
                    {
                        File.Delete(fullPath);
                    }
                }
                catch (Exception ex)
                {
                    Console.WriteLine($"Error eliminando {file}: {ex.Message}");
                }
            }

            NotificationWindow win = new NotificationWindow("¡Unpatched Steam!", 2);
            win.Show();
        }
        else
        {
            if (!Directory.Exists(path)) return;

            string tempPath = Path.Combine(path, "temp");
            if (!Directory.Exists(tempPath)) Directory.CreateDirectory(tempPath);

            string zipPath = Path.Combine(tempPath, "inject.zip");

            if (!_httpClient.DefaultRequestHeaders.Contains("User-Agent"))
            {
                _httpClient.DefaultRequestHeaders.Add("User-Agent", "OpenSteamManager");
            }

            switch (Mode)
            {
                case 0:
                    try
                    {
                        string apiUrl = "https://api.github.com/repos/OpenSteam001/OpenSteamTool/releases/latest";
                        string jsonResponse = await _httpClient.GetStringAsync(apiUrl);
                        string downloadUrl = GetReleaseDownloadUrl(jsonResponse);

                        if (string.IsNullOrEmpty(downloadUrl))
                        {
                            Console.WriteLine("Error: Could not find the stable -Release.zip link.");
                            return;
                        }

                        byte[] fileData = await _httpClient.GetByteArrayAsync(downloadUrl);
                        await File.WriteAllBytesAsync(zipPath, fileData);

                        using (ZipArchive archive = ZipFile.OpenRead(zipPath))
                        {
                            foreach (ZipArchiveEntry entry in archive.Entries)
                            {
                                if (entry.FullName.EndsWith(".dll", StringComparison.OrdinalIgnoreCase))
                                {
                                    string destinationPath = Path.Combine(path, entry.Name);
                                    entry.ExtractToFile(destinationPath, overwrite: true);
                                }
                            }
                        }

                        NotificationWindow win0 = new NotificationWindow("¡Steam Patched!", 2);
                        win0.Show();
                    }
                    catch (Exception ex)
                    {
                        Console.WriteLine($"Error en Modo 0: {ex.Message}");
                    }
                    break; 

                case 1:
                    try
                    {
                        byte[] fileData = await _httpClient.GetByteArrayAsync("https://github.com/Abrahamqb/OpenSteamMore-Dev/releases/latest/download/inject.zip");
                        await File.WriteAllBytesAsync(zipPath, fileData);

                        ZipFile.ExtractToDirectory(zipPath, path, true);

                        NotificationWindow win1 = new NotificationWindow("¡Steam Patched!", 2);
                        win1.Show();
                    }
                    catch (Exception ex)
                    {
                        Console.WriteLine($"Error en Modo 1: {ex.Message}");
                    }
                    break; 
            }

            try
            {
                if (File.Exists(zipPath)) File.Delete(zipPath);
                if (Directory.Exists(tempPath)) Directory.Delete(tempPath, true);
            }
            catch { }
        }
    }
}