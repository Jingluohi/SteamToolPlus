using OpenSteam.Properties;
using OpenSteam.Services;
using OpenSteam.Models;
using System.Diagnostics;
using System.Windows;
using System.Windows.Controls;

namespace OpenSteam.Views
{
    public partial class OnlineLua : UserControl
    {
        public OnlineLua()
        {
            InitializeComponent();
            LoadData();
        }

        private List<Game> CachedList = new List<Game>();

        private async void LoadData()
        {
            ButtonSearch.IsEnabled = false;
            ButtonSearch.Opacity = 0.6;
            ButtonText.Visibility = Visibility.Collapsed;
            ButtonProgress.Visibility = Visibility.Visible;

            try
            {
                CachedList = await SteamUtils.DownloadGameListAsync();
            }
            catch (Exception ex)
            {
                MessageBox.Show($"Failed to load game data: {ex.Message}", "Error", MessageBoxButton.OK, MessageBoxImage.Error);
            }
            finally
            {
                ButtonSearch.IsEnabled = true;
                ButtonSearch.Opacity = 1.0;
                ButtonText.Visibility = Visibility.Visible;
                ButtonProgress.Visibility = Visibility.Collapsed;
            }
        }

        private async void Search_Click(object sender, RoutedEventArgs e)
        {

            LuaLoaders luaLoaders = new LuaLoaders();
            string steamPath = SteamUtils.GetSteamPath();
            string userInput = SearchBox.Text;

            if (string.IsNullOrWhiteSpace(userInput))
            {
                MessageBox.Show("Please enter an AppID or Name first.", "Warning", MessageBoxButton.OK, MessageBoxImage.Warning);
                return;
            }

            ButtonSearch.IsEnabled = false;
            ButtonSearch.Opacity = 0.6;
            ButtonText.Visibility = Visibility.Collapsed;
            ButtonProgress.Visibility = Visibility.Visible;

            try
            {
                if (Properties.Settings.Default.FilterManager)
                {
                    var results = await Task.Run(() => SteamUtils.GetFilteredGames(userInput, CachedList));

                    if (results == null || !results.Any())
                    {
                        MessageBox.Show("No games found with that ID or Name. You can try disabling the filter in the settings (Only works with appid)", "Not Found", MessageBoxButton.OK, MessageBoxImage.Information);
                        return;
                    }

                    Game selectedGame = results.First();

                    if (selectedGame.nsfw && !Properties.Settings.Default.DisableNFSWAlert)
                    {
                        var res = MessageBox.Show("This game is marked as NSFW. Continue?", "NSFW Warning",
                                                 MessageBoxButton.YesNo, MessageBoxImage.Warning);
                        if (res == MessageBoxResult.No) return;
                    }

                    if (selectedGame.drm)
                    {
                        var res = MessageBox.Show("This game has DRM. It may not work. Continue?", "DRM Warning",
                                                 MessageBoxButton.YesNo, MessageBoxImage.Warning);
                        if (res == MessageBoxResult.No) return;
                    }

                    await luaLoaders.OnlineLoad(selectedGame.appid, steamPath);
                }
                else
                {
                    await luaLoaders.OnlineLoad(userInput, steamPath);
                }
                
            }
            catch (Exception ex)
            {
                MessageBox.Show($"An error occurred: {ex.Message}", "Error", MessageBoxButton.OK, MessageBoxImage.Error);
            }
            finally
            {
                ButtonSearch.IsEnabled = true;
                ButtonSearch.Opacity = 1.0;
                ButtonText.Visibility = Visibility.Visible;
                ButtonProgress.Visibility = Visibility.Collapsed;
            }
        }

    }
}