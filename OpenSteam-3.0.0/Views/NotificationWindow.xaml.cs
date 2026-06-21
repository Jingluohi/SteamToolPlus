using System.Windows;
using System.Windows.Media.Animation;
using System.Windows.Threading;

namespace OpenSteam.Views
{
    public partial class NotificationWindow : Window
    {
        public NotificationWindow(string mensaje, int segundos)
        {
            InitializeComponent();
            TxtMensaje.Text = mensaje;

            DispatcherTimer timer = new DispatcherTimer();
            timer.Interval = TimeSpan.FromSeconds(segundos);
            timer.Tick += async (s, e) =>
            {
                timer.Stop();
                await CloseWithAnimation();
            };
            timer.Start();
        }

        private async Task CloseWithAnimation()
        {
            Storyboard sb = (Storyboard)this.Resources["OnClosing"];
            if (sb != null)
            {
                sb.Begin();

                await Task.Delay(400);
            }
            this.Close();
        }
    }
}