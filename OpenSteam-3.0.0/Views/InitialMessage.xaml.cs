using OpenSteam.Properties;
using System.Windows;
using System.Windows.Controls;

namespace OpenSteam.Views
{
    /// <summary>
    /// Lógica de interacción para InitialMessage.xaml
    /// </summary>
    public partial class InitialMessage : UserControl
    {
        public InitialMessage()
        {
            InitializeComponent();
        }

        private void BtnDontShow_Click(object sender, System.Windows.RoutedEventArgs e)
        {
            Settings.Default.InitialMessage = false;
            Settings.Default.Save();

            var mainWindow = Window.GetWindow(this) as MainWindow;

            if (mainWindow != null)
            {
                mainWindow.ShowHome();
            }
        }
    }
}
