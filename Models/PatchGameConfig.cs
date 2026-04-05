using System;
using System.Collections.Generic;
using Newtonsoft.Json;

namespace SteamToolPlus.Models;

/// <summary>
/// 补丁类型枚举
/// </summary>
public enum PatchType
{
    NoSteam,        // 免 Steam 启动
    LAN,            // 局域网联机
    SteamOnline,    // Steam 联机
    DEncrypted      // D 加密虚拟机
}

/// <summary>
/// 游戏配置信息类（用于 JSON 序列化/反序列化）
/// </summary>
public class PatchGameConfig
{
    /// <summary>
    /// 游戏标签（唯一标识符，如 "re4", "er"）
    /// </summary>
    [JsonProperty("tag")]
    public string Tag { get; set; } = "";
    
    /// <summary>
    /// 游戏英文名称
    /// </summary>
    [JsonProperty("game_name")]
    public string GameName { get; set; } = "";
    
    /// <summary>
    /// 游戏中文名称
    /// </summary>
    [JsonProperty("chinese_name")]
    public string? ChineseName { get; set; }
    
    /// <summary>
    /// 游戏 Steam AppID
    /// </summary>
    [JsonProperty("game_id")]
    public string GameId { get; set; } = "";
    
    /// <summary>
    /// 补丁源路径（相对路径）
    /// </summary>
    [JsonProperty("patch_source_path")]
    public string PatchSourcePath { get; set; } = "";
    
    /// <summary>
    /// 补丁类型
    /// </summary>
    [JsonProperty("patch_type")]
    public PatchType PatchType { get; set; }
    
    /// <summary>
    /// 是否为 D 加密游戏
    /// </summary>
    [JsonProperty("is_d_encrypted")]
    public bool IsDEncrypted { get; set; }
    
    /// <summary>
    /// 网格视图封面图片文件名（可选，默认根据 GameId 生成）
    /// </summary>
    [JsonProperty("cover_image")]
    public string? CoverImage { get; set; }
    
    /// <summary>
    /// 列表视图封面图片文件名（可选，默认根据 GameId 生成）
    /// </summary>
    [JsonProperty("list_cover_image")]
    public string? ListCoverImage { get; set; }
    
    /// <summary>
    /// 拼音首字母（用于搜索，可选）
    /// </summary>
    [JsonProperty("pinyin_initials")]
    public string? PinyinInitials { get; set; }

    /// <summary>
    /// 游戏详情 UI 配置（可选，默认显示所有常用选项）
    /// </summary>
    [JsonProperty("ui_config")]
    public GameDetailUIConfig? UIConfig { get; set; }

    /// <summary>
    /// 获取 UI 配置（如果未配置则返回默认值）
    /// </summary>
    public GameDetailUIConfig GetUIConfig()
    {
        return UIConfig ?? new GameDetailUIConfig();
    }

    /// <summary>
    /// 获取网格视图封面图片的 Pack URI
    /// </summary>
    public string GetCoverImagePackUri()
    {
        string imageName = CoverImage ?? $"{GameId}.jpg";
        return $"pack://application:,,,/Resources/pic/inside/{imageName}";
    }
    
    /// <summary>
    /// 获取列表视图封面图片的 Pack URI
    /// </summary>
    public string GetListCoverImagePackUri()
    {
        string imageName = ListCoverImage ?? $"{GameId}.jpg";
        return $"pack://application:,,,/Resources/pic/outside/{imageName}";
    }
}

/// <summary>
/// 游戏配置管理器（单例模式）
/// </summary>
public class GameConfigManager
{
    private static readonly Lazy<GameConfigManager> _instance = new(() => new GameConfigManager());
    public static GameConfigManager Instance => _instance.Value;
    
    private readonly List<PatchGameConfig> _games = new();
    private readonly Dictionary<string, PatchGameConfig> _gameByTag = new();
    private readonly Dictionary<string, PatchGameConfig> _gameByGameId = new();
    
    /// <summary>
    /// 所有游戏配置列表
    /// </summary>
    public IReadOnlyList<PatchGameConfig> Games => _games.AsReadOnly();
    
    /// <summary>
    /// 私有构造函数，防止外部实例化
    /// </summary>
    private GameConfigManager()
    {
    }
    
    /// <summary>
    /// 从 JSON 文件加载游戏配置
    /// </summary>
    /// <param name="jsonFilePath">JSON 配置文件路径</param>
    public void LoadFromJson(string jsonFilePath)
    {
        if (!System.IO.File.Exists(jsonFilePath))
        {
            throw new System.IO.FileNotFoundException($"游戏配置文件不存在：{jsonFilePath}");
        }
        
        string jsonContent = System.IO.File.ReadAllText(jsonFilePath, System.Text.Encoding.UTF8);
        var games = JsonConvert.DeserializeObject<List<PatchGameConfig>>(jsonContent);
        
        if (games == null)
        {
            throw new InvalidOperationException("游戏配置文件格式错误");
        }
        
        _games.Clear();
        _gameByTag.Clear();
        _gameByGameId.Clear();
        
        foreach (var game in games)
        {
            _games.Add(game);
            _gameByTag[game.Tag] = game;
            _gameByGameId[game.GameId] = game;
        }
    }
    
    /// <summary>
    /// 根据标签获取游戏配置
    /// </summary>
    /// <param name="tag">游戏标签</param>
    /// <returns>游戏配置对象，如果不存在则返回 null</returns>
    public PatchGameConfig? GetGameByTag(string tag)
    {
        return _gameByTag.TryGetValue(tag, out var game) ? game : null;
    }
    
    /// <summary>
    /// 根据 GameID 获取游戏配置
    /// </summary>
    /// <param name="gameId">游戏 AppID</param>
    /// <returns>游戏配置对象，如果不存在则返回 null</returns>
    public PatchGameConfig? GetGameByGameId(string gameId)
    {
        return _gameByGameId.TryGetValue(gameId, out var game) ? game : null;
    }
    
    /// <summary>
    /// 根据补丁类型筛选游戏
    /// </summary>
    /// <param name="patchType">补丁类型</param>
    /// <returns>符合条件的游戏列表</returns>
    public List<PatchGameConfig> GetGamesByPatchType(PatchType patchType)
    {
        return _games.FindAll(g => g.PatchType == patchType);
    }
    
    /// <summary>
    /// 搜索游戏（支持名称、GameID、拼音首字母匹配）
    /// </summary>
    /// <param name="searchTerm">搜索关键词</param>
    /// <returns>匹配的游戏列表</returns>
    public List<PatchGameConfig> SearchGames(string searchTerm)
    {
        if (string.IsNullOrWhiteSpace(searchTerm))
        {
            return new List<PatchGameConfig>();
        }
        
        string term = searchTerm.Trim().ToLower();
        var results = new List<PatchGameConfig>();
        
        foreach (var game in _games)
        {
            // 1. 精确匹配 GameID（完全匹配或部分匹配）
            if (game.GameId.Contains(term))
            {
                results.Add(game);
                continue;
            }
            
            // 2. 匹配英文名称（包含匹配）
            if (game.GameName.ToLower().Contains(term))
            {
                results.Add(game);
                continue;
            }
            
            // 3. 匹配中文名称（包含匹配）
            if (!string.IsNullOrEmpty(game.ChineseName) && game.ChineseName.Contains(term))
            {
                results.Add(game);
                continue;
            }
            
            // 4. 匹配拼音首字母
            if (!string.IsNullOrEmpty(game.PinyinInitials) && game.PinyinInitials.ToLower().Contains(term))
            {
                results.Add(game);
                continue;
            }
        }
        
        return results;
    }
}

/// <summary>
/// 游戏详情 UI 配置
/// </summary>
public class GameDetailUIConfig
{
    /// <summary>
    /// 是否显示 overlay 选项
    /// </summary>
    [JsonProperty("show_overlay")]
    public bool ShowOverlay { get; set; } = true;

    /// <summary>
    /// 是否显示成就导入选项
    /// </summary>
    [JsonProperty("show_achievement_import")]
    public bool ShowAchievementImport { get; set; } = true;

    /// <summary>
    /// 是否显示统计数据导入选项
    /// </summary>
    [JsonProperty("show_stats_import")]
    public bool ShowStatsImport { get; set; } = true;

    /// <summary>
    /// 是否显示 HTTP 文件夹选项
    /// </summary>
    [JsonProperty("show_http_folder")]
    public bool ShowHttpFolder { get; set; } = true;

    /// <summary>
    /// 是否显示头像选项
    /// </summary>
    [JsonProperty("show_avatar")]
    public bool ShowAvatar { get; set; } = true;

    /// <summary>
    /// 是否显示语言选项
    /// </summary>
    [JsonProperty("show_language")]
    public bool ShowLanguage { get; set; } = true;

    /// <summary>
    /// 是否显示 MOD 文件夹选项
    /// </summary>
    [JsonProperty("show_mods")]
    public bool ShowMods { get; set; } = true;

    /// <summary>
    /// 是否显示手柄配置选项
    /// </summary>
    [JsonProperty("show_controller")]
    public bool ShowController { get; set; } = true;

    /// <summary>
    /// 是否显示 ticket 选项
    /// </summary>
    [JsonProperty("show_ticket")]
    public bool ShowTicket { get; set; } = false;

    /// <summary>
    /// 是否显示自定义保存名称选项
    /// </summary>
    [JsonProperty("show_custom_save_name")]
    public bool ShowCustomSaveName { get; set; } = false;

    /// <summary>
    /// 是否显示实验版模式选项
    /// </summary>
    [JsonProperty("show_experimental_mode")]
    public bool ShowExperimentalMode { get; set; } = true;

    /// <summary>
    /// 使用说明文件路径（相对路径）
    /// </summary>
    [JsonProperty("instructions_file")]
    public string? InstructionsFile { get; set; }

    /// <summary>
    /// 特殊说明文本
    /// </summary>
    [JsonProperty("special_instructions")]
    public string? SpecialInstructions { get; set; }
}