namespace OpenSteam.Models
{
    public class Game
    {
        public string appid { get; set; }
        public string name { get; set; }
        public string type { get; set; }
        public List<string> tags { get; set; }
        public bool nsfw { get; set; }
        public bool drm { get; set; }
        public bool IsDemo => type?.ToLower() == "demo" || name.ToLower().Contains("demo");
        public string CoverUrl => $"https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/{appid}/library_600x900_2x.jpg";
    }
}
