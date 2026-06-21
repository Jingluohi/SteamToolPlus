using SharpCompress.Archives;
using SharpCompress.Common;
using SharpCompress.Readers;
using Microsoft.Win32;
using System;
using System.IO;
using System.Net.Http;
using System.Threading;
using System.Threading.Tasks;
using System.Windows;
using OpenSteam.Views;

namespace OpenSteam.Services
{
    class OnlineFixScrapping
    {
        private static readonly HttpClient _staticHttpClient = new HttpClient();

        public static async Task GetFixes(string gameName, string password)
        {

            //Alert
            MessageBox.Show("If you receive a timeout error, it's because the API backend isn't optimized; it's designed for testing purposes. You can continue ;)");
            NotificationWindow win = new NotificationWindow("Downloading Online Fix", 5);
            win.Show();


            string tempZip = Path.Combine(Path.GetTempPath(), $"of_fix_{Guid.NewGuid().ToString().Substring(0, 8)}.rar");
            string tempExtracted = Path.Combine(Path.GetTempPath(), "of_temp_extract");

            try
            {
                string apiUrl = $"https://scrappingfix.vercel.app/download?game={Uri.EscapeDataString(gameName)}";
                byte[] fixFile = await _staticHttpClient.GetByteArrayAsync(apiUrl);
                await File.WriteAllBytesAsync(tempZip, fixFile);

                if (Directory.Exists(tempExtracted)) Directory.Delete(tempExtracted, true);
                Directory.CreateDirectory(tempExtracted);

                var readerOptions = new ReaderOptions { Password = password };

                using (Stream stream = File.OpenRead(tempZip))
                using (var reader = ReaderFactory.OpenReader(stream, readerOptions))
                {
                    while (reader.MoveToNextEntry())
                    {
                        if (!reader.Entry.IsDirectory)
                        {
                            reader.WriteEntryToDirectory(tempExtracted, new ExtractionOptions
                            {
                                ExtractFullPath = true,
                                Overwrite = true
                            });
                        }
                    }
                }

                string gamePath = SearchGamePath(gameName);

                if (Directory.Exists(gamePath) && gamePath != "Game not found")
                {
                    foreach (var file in Directory.GetFiles(tempExtracted, "*", SearchOption.AllDirectories))
                    {
                        string relativePath = Path.GetRelativePath(tempExtracted, file);
                        string destinationPath = Path.Combine(gamePath, relativePath);

                        Directory.CreateDirectory(Path.GetDirectoryName(destinationPath));

                        if (File.Exists(destinationPath))
                        {
                            File.SetAttributes(destinationPath, FileAttributes.Normal);
                            File.Delete(destinationPath);
                        }

                        File.Copy(file, destinationPath, true);
                        File.SetAttributes(destinationPath, FileAttributes.Normal);

                        try { File.Delete(destinationPath + ":Zone.Identifier"); } catch { }
                    }

                    Application.Current.Dispatcher.Invoke(() => MessageBox.Show("Fix successfully applied!"));
                }
                else
                {
                    MessageBox.Show("The game route could not be found.");
                }
            }
            catch (Exception ex)
            {
                MessageBox.Show($"Error: {ex.Message}");
                NotificationWindow Error = new NotificationWindow("Try disabling your antivirus.", 5);
                Error.ShowDialog();
            }
            finally
            {
                try
                {
                    if (File.Exists(tempZip)) File.Delete(tempZip);
                    if (Directory.Exists(tempExtracted)) Directory.Delete(tempExtracted, true);
                }
                catch { }
            }
        }

        public static string SearchGamePath(string gameName)
        {
            string manualPath = null;
            Thread t = new Thread(() =>
            {
                var dialog = new Microsoft.Win32.OpenFolderDialog
                {
                    Title = $"Select the root folder of {gameName.Replace("%20", " ")}",
                    InitialDirectory = @"C:\Program Files (x86)\Steam\steamapps\common"
                };
                if (dialog.ShowDialog() == true) manualPath = dialog.FolderName;
            });
            t.SetApartmentState(ApartmentState.STA);
            t.Start();
            t.Join();
            return manualPath ?? "Game not found";
        }
    }
}