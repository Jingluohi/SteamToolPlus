using System.Diagnostics;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Input;

namespace OpenSteam.Views
{
    public partial class Extra : UserControl
    {
        public Extra()
        {
            InitializeComponent();
        }

        public enum ExtraUrlOption
        {
            SteamCMD = 1,
            NLGL = 2,
            CreamInstaller = 3,
            OnlineFix = 4,
            SteamAchievementManager = 5
        }

        public string URL(ExtraUrlOption option)
        {
            switch (option)
            {
                case ExtraUrlOption.SteamCMD: return "https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip";
                case ExtraUrlOption.NLGL: return "https://github.com/onajlikezz/Nightlight-Game-Launcher/releases/tag/NLLauncherV4";
                case ExtraUrlOption.CreamInstaller:
                    MessageBox.Show("Redirecting to CreamInstaller GitHub page.", "Information", MessageBoxButton.OK, MessageBoxImage.Information);
                    return "https://github.com/CyberSys/CreamInstaller";
                case ExtraUrlOption.OnlineFix:
                    MessageBox.Show("Redirecting to Online-fix.me.", "Information", MessageBoxButton.OK, MessageBoxImage.Information);
                    return "https://online-fix.me/";
                case ExtraUrlOption.SteamAchievementManager:
                    if(MessageBoxResult.Yes != MessageBox.Show("Using this option may infect your account. Do you wish to continue?", "Warning", MessageBoxButton.YesNo, MessageBoxImage.Warning))
                    {
                        return string.Empty;
                    }
                    MessageBox.Show("Redirecting to Steam Archievement Manager.", "Information", MessageBoxButton.OK, MessageBoxImage.Information);
                    return "https://github.com/gibbed/SteamAchievementManager";
                default: return string.Empty;
            }
        }

        private void OpenExternalUrl(ExtraUrlOption option)
        {
            try
            {
                Process.Start(new ProcessStartInfo { FileName = URL(option), UseShellExecute = true });
            }
            catch { }
        }

        private void Steamcmd(object sender, MouseButtonEventArgs e) => OpenExternalUrl(ExtraUrlOption.SteamCMD);
        private void nlgl(object sender, MouseButtonEventArgs e) => OpenExternalUrl(ExtraUrlOption.NLGL);
        private void craminstaller(object sender, MouseButtonEventArgs e) => OpenExternalUrl(ExtraUrlOption.CreamInstaller);
        private void onlinefix(object sender, MouseButtonEventArgs e) => OpenExternalUrl(ExtraUrlOption.OnlineFix);
        private void steamachievementmanager(object sender, MouseButtonEventArgs e) => OpenExternalUrl(ExtraUrlOption.SteamAchievementManager);
    }
}