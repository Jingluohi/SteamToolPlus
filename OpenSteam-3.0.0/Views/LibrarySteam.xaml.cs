using OpenSteam.Services;
using OpenSteam.Models;
using System.Collections.ObjectModel;
using System.Diagnostics;
using System.IO;
using System.Text.RegularExpressions;
using System.Windows;
using System.Windows.Controls;

namespace OpenSteam.Views
{
    public partial class LibrarySteam : UserControl
    {
        private string luaPath;
        private string steamPath;
        private List<Game> fullGameList = new List<Game>();
        private ObservableCollection<Game> installedLuaGames = new ObservableCollection<Game>();

        public LibrarySteam()
        {
            InitializeComponent();
            LuaListBox.ItemsSource = installedLuaGames;

            steamPath = SteamUtils.GetSteamPath();

            if (steamPath != null)
            {
                luaPath = Path.Combine(steamPath, "config", Properties.Settings.Default.LuaPath);
                if (!Directory.Exists(luaPath))
                {
                    Directory.CreateDirectory(luaPath);
                }
                _ = LoadData();
            }
            else
            {
                MessageBox.Show("Steam was not found on this system.");
            }
        }

        private async Task LoadData()
        {
            try
            {
                fullGameList = await SteamUtils.DownloadGameListAsync();
            }
            catch { }

            await RefreshLuaList();
        }

        private string GetGameNameLocal(string appId)
        {
            // 1. Try local ACF (Native)
            try
            {
                string acfPath = Path.Combine(steamPath, "steamapps", $"appmanifest_{appId}.acf");
                if (File.Exists(acfPath))
                {
                    string content = File.ReadAllText(acfPath);
                    var match = Regex.Match(content, "\"name\"\\s+\"([^\"]+)\"");
                    if (match.Success) return match.Groups[1].Value;
                }
            }
            catch { }

            // 2. Try JSON Cache (Fast)
            var game = fullGameList.FirstOrDefault(g => g.appid == appId);
            if (game != null)
            {
                return game.name;
            }

            return appId;
        }

        private async Task RefreshLuaList()
        {
            installedLuaGames.Clear();
            if (!Directory.Exists(luaPath)) return;

            string[] files = Directory.GetFiles(luaPath, "*.lua");

            foreach (string file in files)
            {
                string id = Path.GetFileNameWithoutExtension(file);
                string realName = GetGameNameLocal(id);

                installedLuaGames.Add(new Game
                {
                    appid = id,
                    name = realName
                });
            }
        }

        private void BtnDelete_Click(object sender, RoutedEventArgs e)
        {
            if (LuaListBox.SelectedItems.Count == 0) return;

            if (MessageBox.Show("Delete selected files?", "Delete", MessageBoxButton.YesNo, MessageBoxImage.Question) == MessageBoxResult.Yes)
            {
                var itemsToRemove = LuaListBox.SelectedItems.Cast<Game>().ToList();
                foreach (var game in itemsToRemove)
                {
                    try
                    {
                        string path = Path.Combine(luaPath, game.appid + ".lua");
                        if (File.Exists(path)) File.Delete(path);
                        installedLuaGames.Remove(game);
                    }
                    catch (Exception ex)
                    {
                        MessageBox.Show($"Error deleting file for {game.name}: {ex.Message}");
                    }
                }
            }
        }

        private void BtnOpenSteam_Click(object sender, RoutedEventArgs e)
        {
            if (LuaListBox.SelectedItems.Count == 0) return;
            foreach (Game game in LuaListBox.SelectedItems)
            {
                try
                {
                    Process.Start(new ProcessStartInfo($"https://store.steampowered.com/app/{game.appid}") { UseShellExecute = true });
                }
                catch { }
            }
        }
    }
}
