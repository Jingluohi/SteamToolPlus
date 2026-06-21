using OpenSteam.Services;
using OpenSteam.Models;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;

namespace OpenSteam.Views
{
    /// <summary>
    /// Lógica de interacción para OnlineByPass.xaml
    /// </summary>
    public partial class OnlineByPass : UserControl
    {
        public OnlineByPass()
        {
            InitializeComponent();
            LoadInstalledGames();
        }

        private void LoadInstalledGames()
        {
            try
            {
                var games = SteamUtils.GetInstalledGames();
                GamesListBox.ItemsSource = games;
            }
            catch (Exception ex)
            {
                MessageBox.Show($"Error loading Steam library: {ex.Message}", "Error", MessageBoxButton.OK, MessageBoxImage.Error);
            }
        }

        private async void BtnSearchOnlineFix_Click(object sender, RoutedEventArgs e)
        {
            if (GamesListBox.SelectedItem is Game selectedGame)
            {
                BtnSearchOnlineFix.IsEnabled = false;
                BtnSearchOnlineFix.Opacity = 0.6;

                try
                {
                    MessageBox.Show("For now, this feature is limited and may contain many errors. Therefore, it's normal that when you apply a patch, it may be outdated, already patched, or the game may simply not open.", "Experimental", MessageBoxButton.OK, MessageBoxImage.Information);
                    await OnlineFixScrapping.GetFixes(Uri.EscapeDataString(selectedGame.name), "online-fix.me");
                }
                catch (Exception ex)
                {
                    MessageBox.Show($"Error encoding game name: {ex.Message}", "Error", MessageBoxButton.OK, MessageBoxImage.Error);
                    return;
                }
                finally
                {
                    BtnSearchOnlineFix.IsEnabled = true;
                    BtnSearchOnlineFix.Opacity = 1;
                }
            }
            else
            {
                MessageBox.Show("Please select a game first.", "Selection Required", MessageBoxButton.OK, MessageBoxImage.Information);
            }
        }
    }
}
