using OpenSteam.Services;
using OpenSteam.Models;
using System.Windows.Controls;

namespace OpenSteam.Views
{
    public partial class Information : UserControl
    {
        public Information()
        {
            InitializeComponent();
            var version = Update.GetVersion();
            InfoVersion.Text = $"v{version} | .NET 9 Edition | Jbrequi (Abrahamqb)";
        }
    }
}