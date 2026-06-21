using OpenSteam.Models;
using OpenSteam.Properties;
using OpenSteam.Services;
using System.Diagnostics;
using System.IO;
using System.Windows;
using System.Windows.Input;
using System.Windows.Media;

namespace OpenSteam.Views
{
    public partial class MainWindow : Window
    {
        public MainWindow()
        {
            InitializeComponent();
            if (Settings.Default.InitialMessage == true)
            {
                ShowInitialMessage();
            }
            else
            {
                ShowHome();
            }

            State();
            var version = Update.GetVersion();
            txtVersion.Text = $"v{version} | .NET 9 Edition";
            _ = Update.CheckForUpdates();
            _ = Update.GetNews();

            this.Closing += MainWindow_Closing;

            if (Properties.Settings.Default.AutoPatchLaunch)
            {
                Attach attach = new Attach();
                if (Properties.Settings.Default.LuaPath == "Lua")
                {
                    _ = attach.PatchSteam(SteamUtils.GetSteamPath(), false, 0);
                }
                else { _ = attach.PatchSteam(SteamUtils.GetSteamPath(), false, 1); }
                State();
            }

            // Load Settings state
            AutoPatch_.IsChecked = Properties.Settings.Default.AutoPatchLaunch;
            DisableWebHelper_.IsChecked = Properties.Settings.Default.DisableWebHelper;
            CloseSteamPatch_.IsChecked = Properties.Settings.Default.CloseSteamBefore;
            DeleteAutoPatch_.IsChecked = Properties.Settings.Default.DeleteOnClose;
            DisableNFSWAlert_.IsChecked = Properties.Settings.Default.DisableNFSWAlert;
            DisableFilter_.IsChecked = Properties.Settings.Default.FilterManager;
            //VersionToggle.IsChecked = Properties.Settings.Default.LuaPath == "Lua";

            //Config Archive
            this.Loaded += MainWindow_Loaded;


        }
        private void MainWindow_Loaded(object sender, RoutedEventArgs e)
        {

            if (Properties.Settings.Default.LuaPath == "Lua")
            {
                VersionToggle.IsChecked = true;
            }
            else
            {
                VersionToggle.IsChecked = false;
                UpdateLuaFolders();
            }
        }
        public void ShowInitialMessage()
        {
            HomeGrid.Visibility = Visibility.Collapsed;
            SettingsGrid.Visibility = Visibility.Collapsed;
            DynamicContent.Visibility = Visibility.Visible;
            DynamicContent.Content = new InitialMessage();
        }

        public void ShowHome()
        {
            DynamicContent.Visibility = Visibility.Collapsed;
            DynamicContent.Content = null;
            HomeGrid.Visibility = Visibility.Visible;
            SettingsGrid.Visibility = Visibility.Collapsed;
        }

        public void State()
        {
            if (File.Exists(Path.Combine(SteamUtils.GetSteamPath(), "xinput1_4.dll")) || File.Exists(Path.Combine(SteamUtils.GetSteamPath(), "hid.dll")) || File.Exists(Path.Combine(SteamUtils.GetSteamPath(), "dwmapi.dll")))
            {
                ParcheEstado.Text = "Status: System Ready";
                StatusDot.Fill = Brushes.LimeGreen;
            }
            else
            {
                ParcheEstado.Text = "Status: System Not Ready (You need patch)";
                StatusDot.Fill = Brushes.Red;
            }
        }

        // Navigation Handlers
        private void NavHome_Click(object sender, RoutedEventArgs e)
        {
            HomeGrid.Visibility = Visibility.Visible;
            SettingsGrid.Visibility = Visibility.Collapsed;
            DynamicContent.Visibility = Visibility.Collapsed;
            DynamicContent.Content = null;
        }

        private void NavByPass_Click(object sender, RoutedEventArgs e)
        {
            HomeGrid.Visibility = Visibility.Collapsed;
            SettingsGrid.Visibility = Visibility.Collapsed;
            DynamicContent.Visibility = Visibility.Visible;
            DynamicContent.Content = new OnlineByPass();
        }

        private void NavOnline_Click(object sender, RoutedEventArgs e)
        {
            HomeGrid.Visibility = Visibility.Collapsed;
            SettingsGrid.Visibility = Visibility.Collapsed;
            DynamicContent.Visibility = Visibility.Visible;
            DynamicContent.Content = new OnlineLua();
        }

        private void NavLibrary_Click(object sender, RoutedEventArgs e)
        {
            HomeGrid.Visibility = Visibility.Collapsed;
            SettingsGrid.Visibility = Visibility.Collapsed;
            DynamicContent.Visibility = Visibility.Visible;
            DynamicContent.Content = new LibrarySteam();
        }

        private void NavExtra_Click(object sender, RoutedEventArgs e)
        {
            HomeGrid.Visibility = Visibility.Collapsed;
            SettingsGrid.Visibility = Visibility.Collapsed;
            DynamicContent.Visibility = Visibility.Visible;
            DynamicContent.Content = new Extra();
        }

        private void NavSettings_Click(object sender, RoutedEventArgs e)
        {
            HomeGrid.Visibility = Visibility.Collapsed;
            SettingsGrid.Visibility = Visibility.Visible;
            DynamicContent.Visibility = Visibility.Collapsed;
            DynamicContent.Content = null;
        }

        private void NavInfo_Click(object sender, RoutedEventArgs e)
        {
            HomeGrid.Visibility = Visibility.Collapsed;
            SettingsGrid.Visibility = Visibility.Collapsed;
            DynamicContent.Visibility = Visibility.Visible;
            DynamicContent.Content = new Information();
        }

        // Home Handlers
        private async void patchButton_Click(object sender, RoutedEventArgs e)
        {
            if (Properties.Settings.Default.CloseSteamBefore)
            {
                try
                {
                    Process[] processes = Process.GetProcessesByName("steam");
                    if (processes.Length > 0)
                    {
                        foreach (Process proceso in processes)
                        {
                            try { proceso.Kill(); } catch { }
                        }
                    }
                }
                catch (Exception ex)
                {
                    Console.WriteLine("Error: " + ex.Message);
                }
            }
            SteamUtils.Reset();
            Attach attach = new Attach();
            if (Properties.Settings.Default.LuaPath == "Lua")
            {
                await attach.PatchSteam(SteamUtils.GetSteamPath(), false, 0);
            }
            else { await attach.PatchSteam(SteamUtils.GetSteamPath(), false, 1); }
            
            State();
        }

        private async void DeletePatchButton_Click(object sender, RoutedEventArgs e)
        {
            Attach attach = new Attach();
            await attach.PatchSteam(SteamUtils.GetSteamPath(), true, 0);
            State();
        }

        private async void Plugins_Click(object sender, RoutedEventArgs e)
        {
            Plugins plugins = new Plugins();
            await plugins.ManagePluginsInstall();
            await Task.Delay(1000);
            await plugins.LuaManagerInstallerAsync(SteamUtils.GetSteamPath());
        }

        private void ManualLua_Click(object sender, RoutedEventArgs e)
        {
            LuaLoaders luaLoaders = new LuaLoaders();
            luaLoaders.Load(SteamUtils.GetSteamPath());
        }

        private void ResetSteam_Click(object sender, RoutedEventArgs e)
        {
            SteamUtils.Reset();
        }

        // Window Controls
        private void Drag_MouseDown(object sender, MouseButtonEventArgs e)
        {
            if (e.ChangedButton == MouseButton.Left)
                this.DragMove();
        }

        private void ExitButton_Click(object sender, RoutedEventArgs e)
        {
            this.Close();
        }

        private void MainWindow_Closing(object sender, System.ComponentModel.CancelEventArgs e)
        {
            if (Properties.Settings.Default.DeleteOnClose)
            {
                try
                {
                    Attach attach = new Attach();
                    attach.PatchSteam(SteamUtils.GetSteamPath(), true, 0);
                    State();
                }
                catch (Exception ex)
                {
                    Console.WriteLine("Error: " + ex.Message);
                }
            }
        }
        private void UpdateLuaFolders()
        {
            if (VersionToggle == null) return;
            Color Transparent = Color.FromArgb(25, 255, 255, 255);
            Color Default = Color.FromArgb(255, 0, 122, 204);
            try
            {
                string steamPath = SteamUtils.GetSteamPath();
                if (string.IsNullOrEmpty(steamPath)) return;

                string steamConfigPath = Path.Combine(steamPath, "config");
                string modernFolder = Path.Combine(steamConfigPath, "Lua");
                string oldFolder = Path.Combine(steamConfigPath, "stplug-in");

                if (!Directory.Exists(steamConfigPath))
                {
                    Directory.CreateDirectory(steamConfigPath);
                }

                if (VersionToggle.IsChecked == true)
                {
                    DefaultTxt.Foreground = new SolidColorBrush(Transparent);
                    AlternativeTxt.Foreground = new SolidColorBrush(Default);
                    Properties.Settings.Default.LuaPath = "Lua";
                    Properties.Settings.Default.Save();

                    if (Directory.Exists(oldFolder))
                    {
                        CopyDirectory(oldFolder, modernFolder);
                        Directory.Delete(oldFolder, true);
                    }
                    else if (!Directory.Exists(modernFolder))
                    {
                        Directory.CreateDirectory(modernFolder);
                    }
                }
                else
                {
                    DefaultTxt.Foreground = new SolidColorBrush(Default);
                    AlternativeTxt.Foreground = new SolidColorBrush(Transparent);
                    Properties.Settings.Default.LuaPath = "stplug-in";
                    Properties.Settings.Default.Save();

                    if (Directory.Exists(modernFolder))
                    {
                        CopyDirectory(modernFolder, oldFolder);
                        Directory.Delete(modernFolder, true);
                    }
                    else if (!Directory.Exists(oldFolder))
                    {
                        Directory.CreateDirectory(oldFolder);
                    }
                }
            }
            catch (IOException ex)
            {
                MessageBox.Show($"Error updating folders: {ex.Message}\n Make sure Steam is closed and run it as administrator.", "Folder Error", MessageBoxButton.OK, MessageBoxImage.Warning);
            }
            catch (Exception ex)
            {
                MessageBox.Show($"An unexpected error occurred: {ex.Message}", "Error", MessageBoxButton.OK, MessageBoxImage.Error);
            }
        }

        private static void CopyDirectory(string sourceDir, string destinationDir)
        {
            Directory.CreateDirectory(destinationDir);

            foreach (string file in Directory.GetFiles(sourceDir))
            {
                string destFile = Path.Combine(destinationDir, Path.GetFileName(file));
                File.Copy(file, destFile, true);
            }

            foreach (string subDir in Directory.GetDirectories(sourceDir))
            {
                string destSubDir = Path.Combine(destinationDir, Path.GetFileName(subDir));
                CopyDirectory(subDir, destSubDir);
            }
        }

        // Settings Handlers
        private void CleanCache_Click(object sender, RoutedEventArgs e) => SettingsFunction.CleanSteamCache();
        private async void SteamBackup_Click(object sender, RoutedEventArgs e) => await SettingsFunction.SteamFolderBackup();
        private void ConfigBackup_Click(object sender, RoutedEventArgs e) => SettingsFunction.BackupSteamConfig();
        private void Folder_Click(object sender, RoutedEventArgs e) => SettingsFunction.OpenFolder();

        private void DeleteAutoPatch(object sender, RoutedEventArgs e)
        {
            Properties.Settings.Default.DeleteOnClose = DeleteAutoPatch_.IsChecked ?? false;
            Properties.Settings.Default.Save();
        }

        private void AutoPatch(object sender, RoutedEventArgs e)
        {
            Properties.Settings.Default.AutoPatchLaunch = AutoPatch_.IsChecked ?? false;
            Properties.Settings.Default.Save();
        }

        private void CloseSteamPatch(object sender, RoutedEventArgs e)
        {
            Properties.Settings.Default.CloseSteamBefore = CloseSteamPatch_.IsChecked ?? false;
            Properties.Settings.Default.Save();
        }

        private void DisableWebHelper(object sender, RoutedEventArgs e)
        {
            Properties.Settings.Default.DisableWebHelper = DisableWebHelper_.IsChecked ?? false;
            Properties.Settings.Default.Save();
        }

        private void DisableNFSWAlert(object sender, RoutedEventArgs e)
        {
            Properties.Settings.Default.DisableNFSWAlert = DisableNFSWAlert_.IsChecked ?? false;
            Properties.Settings.Default.Save();
        }

        private void DisableFilterToOnlineLua(object sender, RoutedEventArgs e)
        {
            Properties.Settings.Default.FilterManager = DisableFilter_.IsChecked ?? false;
            Properties.Settings.Default.Save();
        }

        private void VersionToggle_Checked(object sender, RoutedEventArgs e)
        {
            if (!this.IsLoaded || VersionToggle == null) return;
            UpdateLuaFolders();
        }
    }
}