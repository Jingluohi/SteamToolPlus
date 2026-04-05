using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Input;
using System.Windows.Media;
using Microsoft.Win32;
using Newtonsoft.Json;
using MessageBox = System.Windows.MessageBox;
using Button = System.Windows.Controls.Button;
using CheckBox = System.Windows.Controls.CheckBox;
using Orientation = System.Windows.Controls.Orientation;
using OpenFileDialog = Microsoft.Win32.OpenFileDialog;
using System.Threading;
using SteamToolPlus.Models;

namespace SteamToolPlus;

/// <summary>
/// Steam 游戏整合工具主窗口
/// 功能：
/// 1. 游戏本体下载（基于 ddv20.exe）
/// 2. 游戏补丁注入（基于 Goldberg Emulator）
/// 3. 局域网联机优化
/// 4. 成就解锁
/// 5. 存档管理
/// 6. 高级功能配置
/// </summary>
public partial class MainWindow : Window
{
    #region 全局变量
    
    /// <summary>
    /// 应用程序当前路径
    /// </summary>
    private string AppPath;
    
    /// <summary>
    /// 游戏保存基础路径
    /// </summary>
    private string BaseSavePath = "";
    
    /// <summary>
    /// 用户是否自定义了保存路径
    /// </summary>
    private bool UserCustomPath = false;
    
    /// <summary>
    /// 下载进程对象
    /// </summary>
    private Process? DownloadProcess;
    
    /// <summary>
    /// 下载是否已暂停
    /// </summary>
    private bool IsPaused = false;
    
    /// <summary>
    /// 颜色缓存（避免重复创建 SolidColorBrush 对象）
    /// </summary>
    private static readonly Dictionary<string, System.Windows.Media.SolidColorBrush> _colorCache = new();
    
    /// <summary>
    /// 获取缓存的颜色（线程安全）
    /// </summary>
    private static System.Windows.Media.SolidColorBrush GetCachedColor(string hexColor)
    {
        if (!_colorCache.TryGetValue(hexColor, out var color))
        {
            color = new System.Windows.Media.SolidColorBrush(
                (System.Windows.Media.Color)System.Windows.Media.ColorConverter.ConvertFromString(hexColor));
            _colorCache[hexColor] = color;
        }
        
        return color;
    }
    
    /// <summary>
    /// 所有游戏卡片的字典（动态管理，无需硬编码）
    /// Key: 游戏标签（如 "er", "re4"），Value: (网格卡片，列表卡片)
    /// </summary>
    private Dictionary<string, (FrameworkElement? gridCard, FrameworkElement? listCard)> _allGameCards = new();
    /// <summary>
    /// 游戏卡片数据源列表
    /// </summary>
    private List<GameCardViewModel> _gameCardViewModels = new();

    /// <summary>
    /// 筛选后的游戏卡片数据源列表
    /// </summary>
    private List<GameCardViewModel> _filteredGameCardViewModels = new();
    /// <summary>
    /// 初始化所有游戏卡片（动态扫描，无需硬编码）
    /// </summary>
    private void InitializeGameCards()
    {
        _allGameCards.Clear();
        
        // 动态查找所有以"Card"开头的网格视图卡片
        var gridCards = FindVisualChildren<FrameworkElement>(this)
            .Where(c => c.Name.StartsWith("Card") && !c.Name.StartsWith("ListCard"))
            .ToList();
        
        // 动态查找所有以"ListCard"开头的列表视图卡片
        var listCards = FindVisualChildren<FrameworkElement>(this)
            .Where(c => c.Name.StartsWith("ListCard"))
            .ToList();
        
        // 根据 Tag 属性配对卡片
        foreach (var gridCard in gridCards)
        {
            var tag = gridCard.Tag?.ToString();
            if (!string.IsNullOrEmpty(tag))
            {
                var listCard = listCards.FirstOrDefault(c => c.Tag?.ToString() == tag);
                _allGameCards[tag] = (gridCard, listCard);
                
                // 动态加载卡片封面图片
                LoadCardCoverImage(gridCard, tag, isGridView: true);
                if (listCard != null)
                    LoadCardCoverImage(listCard, tag, isGridView: false);
            }
        }
    }
    
    /// <summary>
    /// 动态加载游戏卡片的封面图片（根据 Tag 自动查找对应图片）
    /// </summary>
    private void LoadCardCoverImage(FrameworkElement card, string tag, bool isGridView)
    {
        try
        {
            var game = GetGameInfoByTag(tag);
            if (game == null) return;
            
            // 使用 pack URI 加载嵌入资源
            string packUri = isGridView ? game.GetCoverImagePackUri() : game.GetListCoverImagePackUri();
            var bitmap = new System.Windows.Media.Imaging.BitmapImage();
            bitmap.BeginInit();
            bitmap.UriSource = new Uri(packUri);
            
            if (isGridView)
            {
                bitmap.DecodePixelWidth = 275;
                bitmap.DecodePixelHeight = 160;
            }
            else
            {
                bitmap.DecodePixelWidth = 100;
            }
            
            bitmap.CacheOption = System.Windows.Media.Imaging.BitmapCacheOption.OnLoad;
            bitmap.EndInit();
            bitmap.Freeze();
            Dispatcher.Invoke(() =>
            {
                var imageControl = FindVisualChildren<System.Windows.Controls.Image>(card).FirstOrDefault();
                if (imageControl != null)
                    imageControl.Source = bitmap;
            });
        }
        catch { }
    }
    
    /// <summary>
    /// 递归查找视觉树中的所有子元素
    /// </summary>
    private static IEnumerable<T> FindVisualChildren<T>(DependencyObject depObj) where T : DependencyObject
    {
        if (depObj == null) yield break;
        
        for (int i = 0; i < VisualTreeHelper.GetChildrenCount(depObj); i++)
        {
            var child = VisualTreeHelper.GetChild(depObj, i);
            
            if (child is T result)
                yield return result;
            
            foreach (var childOfChild in FindVisualChildren<T>(child))
                yield return childOfChild;
        }
    }
    
    /// <summary>
    /// 设置所有游戏卡片的可见性（动态方法，无需硬编码）
    /// </summary>
    /// <param name="isVisible">是否可见</param>
    private void SetAllGameCardsVisibility(bool isVisible)
    {
        Visibility visibility = isVisible ? Visibility.Visible : Visibility.Collapsed;
        
        foreach (var cardPair in _allGameCards.Values)
        {
            if (cardPair.gridCard != null)
                cardPair.gridCard.Visibility = visibility;
            
            if (cardPair.listCard != null)
                cardPair.listCard.Visibility = visibility;
        }
    }
    
    /// <summary>
    /// 根据游戏标签设置卡片可见性（动态方法，无需硬编码）
    /// </summary>
    /// <param name="gameTag">游戏标签</param>
    /// <param name="isVisible">是否可见</param>
    private void SetGameCardVisibility(string gameTag, bool isVisible)
    {
        if (!_allGameCards.TryGetValue(gameTag, out var cardPair))
            return;
        
        Visibility visibility = isVisible ? Visibility.Visible : Visibility.Collapsed;
        
        if (cardPair.gridCard != null)
            cardPair.gridCard.Visibility = visibility;
        
        if (cardPair.listCard != null)
            cardPair.listCard.Visibility = visibility;
    }
    
    /// <summary>
    /// 根据补丁类型动态显示游戏（完全动态，无需硬编码）
    /// </summary>
    /// <param name="patchType">补丁类型</param>
    private void ShowGamesByPatchType(PatchType patchType)
    {
        // 遍历所有游戏卡片
        foreach (var kvp in _allGameCards)
        {
            string gameTag = kvp.Key;
            var game = GetGameInfoByTag(gameTag);
            
            if (game == null)
                continue;
            
            // 检查游戏是否匹配当前补丁类型
            bool shouldShow = IsGameMatchPatchType(game, patchType);
            
            // 如果处于搜索状态，还需要检查是否匹配搜索词
            if (isSearching && !string.IsNullOrEmpty(currentSearchTerm))
            {
                bool matchesSearch = IsGameMatchSearchTerm(game, currentSearchTerm);
                shouldShow = shouldShow && matchesSearch;
            }
            
            // 设置卡片可见性
            SetGameCardVisibility(gameTag, shouldShow);
        }
    }
    
    /// <summary>
    /// 判断游戏是否匹配补丁类型
    /// </summary>
    private bool IsGameMatchPatchType(PatchGameInfo game, PatchType patchType)
    {
        if (patchType == PatchType.DEncrypted)
        {
            // D 加密游戏：检查游戏是否是 D 加密类型
            return game.PatchType == PatchType.DEncrypted;
        }
        else
        {
            // 其他类型：检查 PatchType
            return game.PatchType == patchType;
        }
    }

    #region 搜索功能
    
    /// <summary>
    /// 搜索框获得焦点
    /// </summary>
    private void TxtGameSearch_GotFocus(object sender, RoutedEventArgs e)
    {
        if (TxtGameSearch.Text == "搜索游戏（名称或 ID）...")
        {
            TxtGameSearch.Text = "";
            TxtGameSearch.Foreground = GetCachedColor("#FFFFFFFF");
        }
    }
    
    /// <summary>
    /// 搜索框失去焦点
    /// </summary>
    private void TxtGameSearch_LostFocus(object sender, RoutedEventArgs e)
    {
        if (string.IsNullOrWhiteSpace(TxtGameSearch.Text))
        {
            TxtGameSearch.Text = "搜索游戏（名称或 ID）...";
            TxtGameSearch.Foreground = GetCachedColor("#FFB4BED6");
        }
    }
    
    /// <summary>
    /// 搜索框文本变化
    /// </summary>
    private void TxtGameSearch_TextChanged(object sender, TextChangedEventArgs e)
    {
        if (!string.IsNullOrWhiteSpace(TxtGameSearch.Text) && TxtGameSearch.Text != "搜索游戏（名称或 ID）...")
        {
            SearchGames(TxtGameSearch.Text);
        }
        else
        {
            if (BtnPatchTypeAll != null && BtnPatchTypeAll.Background == GetCachedColor("#FF0099F2"))
                BtnPatchTypeAll_Click(null, null);
            else if (BtnPatchTypeNoSteam != null && BtnPatchTypeNoSteam.Background == GetCachedColor("#FF0099F2"))
                BtnPatchTypeNoSteam_Click(null, null);
            else if (BtnPatchTypeLAN != null && BtnPatchTypeLAN.Background == GetCachedColor("#FF0099F2"))
                BtnPatchTypeLAN_Click(null, null);
            else if (BtnPatchTypeSteamOnline != null && BtnPatchTypeSteamOnline.Background == GetCachedColor("#FF0099F2"))
                BtnPatchTypeSteamOnline_Click(null, null);
            else if (BtnPatchTypeDEncrypted != null && BtnPatchTypeDEncrypted.Background == GetCachedColor("#FF0099F2"))
                BtnPatchTypeDEncrypted_Click(null, null);
        }
    }
    
    /// <summary>
    /// 搜索按钮点击事件
    /// </summary>
    private void BtnSearch_Click(object sender, RoutedEventArgs e)
    {
        if (!string.IsNullOrWhiteSpace(TxtGameSearch.Text) && TxtGameSearch.Text != "搜索游戏（名称或 ID）...")
        {
            SearchGames(TxtGameSearch.Text);
        }
    }

    /// <summary>
    /// 执行游戏搜索（优化版：支持模糊搜索、拼音首字母、部分匹配）
    /// </summary>
    /// <param name="searchText">搜索文本</param>
    /// <summary>
    /// 搜索游戏（基于配置文件）
    /// </summary>
    private void SearchGames(string searchText)
    {
        if (string.IsNullOrEmpty(searchText))
        {
            isSearching = false;
            currentSearchTerm = string.Empty;
            // 恢复显示所有游戏
            _filteredGameCardViewModels = new List<GameCardViewModel>(_gameCardViewModels);
            UpdateGameCardsDisplay();
            AppendDownloadLog("🔍 已清除搜索条件");
            return;
        }

        isSearching = true;
        currentSearchTerm = searchText.Trim().ToLower();

        // 使用配置管理器的搜索功能
        var searchResults = GameConfigManager.Instance.SearchGames(currentSearchTerm);

        // 转换为视图模型（使用 FromConfig 确保封面图片正确）
        _filteredGameCardViewModels.Clear();
        foreach (var game in searchResults)
        {
            _filteredGameCardViewModels.Add(GameCardViewModel.FromConfig(game));
        }

        UpdateGameCardsDisplay();

        AddToSearchHistory(searchText);
        AppendDownloadLog($"🔍 搜索：{searchText}（找到 {_filteredGameCardViewModels.Count} 个游戏）");
    }

 
    
    /// <summary>
    /// 获取游戏中文名（从配置管理器获取）
    /// </summary>
    /// <param name="gameName">游戏英文名</param>
    /// <returns>游戏中文名</returns>
    private string? GetGameChineseName(string gameName)
    {
        // 从配置管理器获取
        var config = GameConfigManager.Instance.Games.FirstOrDefault(g => g.GameName == gameName);
        return config?.ChineseName;
    }
    
    /// <summary>
    /// 判断游戏是否匹配搜索词（支持多种匹配方式）
    /// </summary>
    /// <param name="game">游戏信息</param>
    /// <param name="searchTerm">搜索关键词</param>
    /// <returns>是否匹配</returns>
    private bool IsGameMatchSearchTerm(PatchGameInfo game, string searchTerm)
    {
        // 1. 精确匹配 GameID（完全匹配或部分匹配）
        if (game.GameId.Contains(searchTerm))
            return true;
        
        // 2. 匹配英文名称（包含匹配）
        if (game.GameName.ToLower().Contains(searchTerm))
            return true;
        
        // 3. 匹配中文名称（包含匹配）
        string? chineseName = GetGameChineseName(game.GameName);
        if (!string.IsNullOrEmpty(chineseName))
        {
            if (chineseName.Contains(searchTerm))
                return true;
            
            // 4. 匹配中文拼音首字母（从配置管理器获取）
            var config = GameConfigManager.Instance.Games.FirstOrDefault(g => g.GameName == game.GameName);
            if (config != null && !string.IsNullOrEmpty(config.PinyinInitials))
            {
                if (config.PinyinInitials.ToLower().Contains(searchTerm))
                    return true;
            }
        }
        
        // 5. 模糊匹配：单词级别的匹配（支持空格分隔的多个关键词）
        string[] searchWords = searchTerm.Split(new[] { ' ', ' ' }, StringSplitOptions.RemoveEmptyEntries);
        if (searchWords.Length > 1)
        {
            bool allWordsMatch = true;
            foreach (string word in searchWords)
            {
                bool wordMatch = game.GameName.ToLower().Contains(word) ||
                               (!string.IsNullOrEmpty(chineseName) && chineseName.Contains(word));
                if (!wordMatch)
                {
                    allWordsMatch = false;
                    break;
                }
            }
            if (allWordsMatch)
                return true;
        }
        
        return false;
    }
    
    /// <summary>
    /// 获取游戏名称的拼音首字母（从配置管理器获取）
    /// </summary>
    /// <param name="gameName">游戏英文名称</param>
    /// <returns>拼音首字母字符串</returns>
    private string? GetPinyinInitials(string gameName)
    {
        // 从配置管理器获取
        var config = GameConfigManager.Instance.Games.FirstOrDefault(g => g.GameName == gameName);
        return config?.PinyinInitials?.ToLower();
    }
    
    /// <summary>
    /// 添加搜索到历史记录
    /// </summary>
    /// <param name="searchText">搜索文本</param>
    private void AddToSearchHistory(string searchText)
    {
        // 避免重复添加
        if (SearchHistory.Contains(searchText))
        {
            SearchHistory.Remove(searchText);
        }
        
        // 添加到历史记录开头
        SearchHistory.Insert(0, searchText);
        
        // 限制历史记录数量
        if (SearchHistory.Count > MaxSearchHistory)
        {
            SearchHistory.RemoveAt(SearchHistory.Count - 1);
        }
    }
    
    #endregion
    
    /// <summary>
    /// 下载是否正在进行中
    /// </summary>
    private bool IsDownloading = false;
    
    /// <summary>
    /// 游戏的Steam AppID
    /// </summary>
    private string? GameAppId;
    
    /// <summary>
    /// 游戏名称
    /// </summary>
    private string? GameName;
    
    /// <summary>
    /// 清单文件列表
    /// </summary>
    private List<string> ManifestList = new();
    
    
    
    
    /// <summary>
    /// 搜索历史记录
    /// </summary>
    private readonly List<string> SearchHistory = new();
    
    /// <summary>
    /// 最大搜索历史记录数
    /// </summary>
    private const int MaxSearchHistory = 10;
    
    /// <summary>
    /// 是否处于搜索状态
    /// </summary>
    private bool isSearching = false;
    
    /// <summary>
    /// 当前搜索词
    /// </summary>
    private string currentSearchTerm = string.Empty;
    
    /// <summary>
    /// 成就列表
    /// </summary>
    private List<AchievementItem> AchievementsList = new();
    
    /// <summary>
    /// 是否为单文件夹模式
    /// </summary>
    private bool IsSingleFolderMode = true;
    
    /// <summary>
    /// 批量下载游戏列表
    /// </summary>
    private List<GameInfo> BatchGameList = new();
    
    /// <summary>
    /// 当前批量下载的游戏对象
    /// </summary>
    private GameInfo? CurrentBatchGame;
    
    /// <summary>
    /// 补丁类别按钮字典
    /// </summary>
    private Dictionary<string, Button> PatchCategoryBtns = new();
    
    /// <summary>
    /// 下载日志缓冲区，用于批量更新 UI
    /// </summary>
    private StringBuilder DownloadLogBuffer = new();
    
    /// <summary>
    /// 日志更新的线程锁
    /// </summary>
    private readonly object LogLock = new object();
    
    /// <summary>
    /// 当前选中的游戏信息
    /// </summary>
    private PatchGameInfo? CurrentSelectedGame;
    
    /// <summary>
    /// 当前游戏的补丁源路径
    /// </summary>
    private string? CurrentPatchSourcePath;
    
    #endregion

    /// <summary>
    /// 添加日志（统一方法，支持下载日志和补丁日志）
    /// </summary>
    /// <param name="message">日志消息</param>
    /// <param name="isPatchLog">是否为补丁日志</param>
    private void AppendLog(string message, bool isPatchLog = false)
    {
        if (isPatchLog)
        {
            // 补丁日志同时输出到 TxtInjectLog 和 TxtPatchLog
            if (TxtInjectLog != null)
            {
                Dispatcher.BeginInvoke(() =>
                {
                    TxtInjectLog.AppendText($"[{DateTime.Now:HH:mm:ss}] {message}\n");
                    TxtInjectLog.ScrollToEnd();
                }, System.Windows.Threading.DispatcherPriority.Background);
            }
            
            if (TxtPatchLog != null)
            {
                Dispatcher.BeginInvoke(() =>
                {
                    TxtPatchLog.AppendText($"[{DateTime.Now:HH:mm:ss}] {message}\n");
                    TxtPatchLog.ScrollToEnd();
                }, System.Windows.Threading.DispatcherPriority.Background);
            }
        }
        else
        {
            // 下载日志输出到 TxtDownloadLog
            if (TxtDownloadLog != null)
            {
                Dispatcher.BeginInvoke(() =>
                {
                    TxtDownloadLog.AppendText($"[{DateTime.Now:HH:mm:ss}] {message}\n");
                    TxtDownloadLog.ScrollToEnd();
                }, System.Windows.Threading.DispatcherPriority.Background);
            }
        }
    }

    /// <summary>
    /// 添加下载日志
    /// </summary>
    private void AppendDownloadLog(string message) => AppendLog(message, false);

    /// <summary>
    /// 添加补丁日志（用于免 Steam 启动补丁界面）
    /// </summary>
    private void AppendPatchLog(string message) => AppendLog(message, true);

    /// <summary>
    /// 显示提示消息框
    /// </summary>
    private void ShowInfo(string message, string title = "提示") => 
        MessageBox.Show(message, title, MessageBoxButton.OK, MessageBoxImage.Information);

    /// <summary>
    /// 显示警告消息框
    /// </summary>
    private void ShowWarning(string message, string title = "提示") => 
        MessageBox.Show(message, title, MessageBoxButton.OK, MessageBoxImage.Warning);

    /// <summary>
    /// 显示错误消息框
    /// </summary>
    private void ShowError(string message, string title = "错误") => 
        MessageBox.Show(message, title, MessageBoxButton.OK, MessageBoxImage.Error);

    /// <summary>
    /// 显示确认对话框
    /// </summary>
    private MessageBoxResult ShowConfirm(string message, string title = "确认") => 
        MessageBox.Show(message, title, MessageBoxButton.YesNo, MessageBoxImage.Question);

    /// <summary>
    /// 主窗口构造函数
    /// </summary>
    public MainWindow()
    {
        InitializeComponent();
        
        // 设置窗口图标（使用 pack URI 加载嵌入资源）
        try
        {
            this.Icon = new System.Windows.Media.Imaging.BitmapImage(new Uri("pack://application:,,,/SteamToolPlus;component/Resources/icon.jpg"));
        }
        catch
        {
            // 如果图标加载失败，使用默认图标或不显示图标
        }
        
        AppPath = AppDomain.CurrentDomain.BaseDirectory;
        if (AppPath.EndsWith("\\"))
            AppPath = AppPath.TrimEnd('\\');

        // 初始化补丁类别按钮字典
        PatchCategoryBtns["basic"] = BtnBasic;
        PatchCategoryBtns["save"] = BtnSave;
        PatchCategoryBtns["network"] = BtnNetwork;
        PatchCategoryBtns["achievement"] = BtnAchievement;
        PatchCategoryBtns["advanced"] = BtnAdvanced;

        // 绑定窗口控制事件
        this.StateChanged += MainWindow_StateChanged;

        // 绑定各种控件的事件处理
        ChkEnableOverlay.Checked += (s, e) => UpdateOverlayControls(true);
        ChkEnableOverlay.Unchecked += (s, e) => UpdateOverlayControls(false);

        ChkImportAchievements.Checked += (s, e) => TxtAchievementFile.IsEnabled = true;
        ChkImportAchievements.Unchecked += (s, e) => TxtAchievementFile.IsEnabled = false;

        ChkImportStats.Checked += (s, e) => TxtStatsFile.IsEnabled = true;
        ChkImportStats.Unchecked += (s, e) => TxtStatsFile.IsEnabled = false;

        ChkEnableHttp.Checked += (s, e) => TxtHttpFolder.IsEnabled = true;
        ChkEnableHttp.Unchecked += (s, e) => TxtHttpFolder.IsEnabled = false;

        ChkEnableAvatar.Checked += (s, e) => TxtAvatarPath.IsEnabled = true;
        ChkEnableAvatar.Unchecked += (s, e) => TxtAvatarPath.IsEnabled = false;

        ChkEnableLanguage.Checked += (s, e) => TxtLanguages.IsEnabled = true;
        ChkEnableLanguage.Unchecked += (s, e) => TxtLanguages.IsEnabled = false;

        ChkEnableMods.Checked += (s, e) => TxtModsFolder.IsEnabled = true;
        ChkEnableMods.Unchecked += (s, e) => TxtModsFolder.IsEnabled = false;

        ChkEnableController.Checked += (s, e) => TxtControllerConfig.IsEnabled = true;
        ChkEnableController.Unchecked += (s, e) => TxtControllerConfig.IsEnabled = false;

        ChkEnableTicket.Checked += (s, e) => TxtTicket.IsEnabled = true;
        ChkEnableTicket.Unchecked += (s, e) => TxtTicket.IsEnabled = false;

        RbCustomSave.Checked += (s, e) => TxtCustomSavePath.IsEnabled = true;
        RbCustomSave.Unchecked += (s, e) => TxtCustomSavePath.IsEnabled = false;

        ChkCustomSavesName.Checked += (s, e) => TxtSavesFolderName.IsEnabled = true;
        ChkCustomSavesName.Unchecked += (s, e) => TxtSavesFolderName.IsEnabled = false;

        RbNormalMode.Checked += (s, e) => TxtModeTip.Text = "常规模式：适用于 90% 以上的 Steam 游戏，替换 steam_api.dll 即可生效";
        RbExperimentalMode.Checked += (s, e) =>
        {
            TxtModeTip.Text = "实验版模式：适用于无 steam_api.dll、常规模式失效、CPY 破解补丁的游戏";
            ShowInfo("您已选择实验版模式（steamclient.dll）");
        };

        // 窗口加载完成后初始化
        Loaded += (s, e) => AppStartInit();
    }

    /// <summary>
    /// 应用程序启动初始化
    /// </summary>
    private void AppStartInit()
    {
        try
        {
            AppendDownloadLog("=" + new string('=', 49));
            // AppendDownloadLog("🚀 程序启动，正在初始化...");
            // AppendDownloadLog($"📂 程序运行目录：{AppPath}");
            
            // 加载游戏配置
            try
            {
                string configPath = Path.Combine(AppPath, "Resources", "games_config.json");
                if (File.Exists(configPath))
                {
                    GameConfigManager.Instance.LoadFromJson(configPath);
                    // AppendDownloadLog($"✅ 加载游戏配置：{GameConfigManager.Instance.Games.Count} 个游戏");
                }
                else
                {
                    // AppendDownloadLog("⚠️ 未找到 games_config.json，将使用硬编码的游戏信息");
                }
            }
            catch (Exception ex)
            {
                // AppendDownloadLog($"❌ 加载游戏配置失败：{ex.Message}");
                MessageBox.Show($"加载游戏配置失败：{ex.Message}", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
            }

            // 自动检测 ddv20.exe
            AutoDetectDdv20();
            // 初始化保存目录
            InitSaveDirectory();
            
            // 动态加载游戏卡片
            LoadGameCards();
            // AppendDownloadLog($"✅ 游戏卡片初始化完成，共 {_gameCardViewModels.Count} 个游戏");
            
            // AppendDownloadLog("✅ 初始化完成");
            // AppendDownloadLog("=" + new string('=', 49));

            // 默认显示特定游戏补丁界面
            PatchSpecialPanel.Visibility = Visibility.Visible;
            PatchPanel.Visibility = Visibility.Collapsed;
        }
        catch (Exception ex)
        {
            MessageBox.Show($"程序初始化失败：{ex.Message}\n\n堆栈跟踪：{ex.StackTrace}", "严重错误", MessageBoxButton.OK, MessageBoxImage.Error);
        }
    }

    /// <summary>
    /// 动态加载游戏卡片（从配置文件）
    /// </summary>
    private void LoadGameCards()
    {
        _gameCardViewModels.Clear();

        // 从配置管理器获取所有游戏
        var games = GameConfigManager.Instance.Games;

        foreach (var game in games)
        {
            var viewModel = GameCardViewModel.FromConfig(game);
            _gameCardViewModels.Add(viewModel);
        }

        // 按游戏名称排序
        _gameCardViewModels = _gameCardViewModels.OrderBy(g => g.GameName).ToList();

        // 初始化筛选列表为所有游戏
        _filteredGameCardViewModels = new List<GameCardViewModel>(_gameCardViewModels);

        // 绑定到 ItemsControl
        GameCardsPanel.ItemsSource = _gameCardViewModels;
        ListGameCardsPanel.ItemsSource = _gameCardViewModels;

        // AppendDownloadLog($"✅ 动态加载游戏卡片完成：{_gameCardViewModels.Count} 个");
    }

    /// <summary>
    /// 更新游戏卡片显示
    /// </summary>
    private void UpdateGameCardsDisplay()
    {
        // 更新网格视图
        GameCardsPanel.ItemsSource = null;
        GameCardsPanel.ItemsSource = _filteredGameCardViewModels;

        // 更新列表视图
        ListGameCardsPanel.ItemsSource = null;
        ListGameCardsPanel.ItemsSource = _filteredGameCardViewModels;
    }

    /// <summary>
    /// 自动检测 ddv20.exe
    /// </summary>
    private void AutoDetectDdv20()
    {
        string ddv20Path = Path.Combine(AppPath, "Resources", "ddv20.exe");
        if (File.Exists(ddv20Path))
        {
            TxtDdExePath.Text = ddv20Path;
            AppendDownloadLog("✅ 自动识别到 ddv20.exe");
        }
        else
        {
            AppendDownloadLog("⚠️ 未找到 ddv20.exe，请手动选择");
            MessageBox.Show("请将 ddv20.exe 放在 Resources 文件夹中", "提示", MessageBoxButton.OK, MessageBoxImage.Warning);
        }
    }

    /// <summary>
    /// 初始化保存目录（优先 D:\SteamGame，其次程序目录，最后文档目录）
    /// </summary>
    private void InitSaveDirectory()
    {
        string[] paths = {
            @"D:\SteamGame",
            Path.Combine(AppPath, "SteamGame"),
            Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.MyDocuments), "SteamGame")
        };

        foreach (string testPath in paths)
        {
            try
            {
                Directory.CreateDirectory(testPath);
                string testFile = Path.Combine(testPath, "write_test.tmp");
                File.WriteAllText(testFile, "test", Encoding.UTF8);
                File.Delete(testFile);
                BaseSavePath = testPath;
                TxtSaveDir.Text = testPath;
                AppendDownloadLog($"📁 游戏保存目录：{testPath}");
                return;
            }
            catch
            {
                // 尝试下一个路径
            }
        }

        MessageBox.Show("无法找到可用的保存目录，请以管理员身份运行", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
    }

    /// <summary>
    /// 更新导航按钮选中状态（使用颜色缓存优化）
    /// </summary>
    /// <param name="selectedButton">当前选中的按钮</param>
    private void UpdateNavButtonSelected(System.Windows.Controls.Button selectedButton)
    {
        // 重置所有导航按钮的颜色和背景
        BtnPatchSpecial.Foreground = GetCachedColor("#FF909BB8");
        BtnPatch.Foreground = GetCachedColor("#FF909BB8");
        BtnGameDownload.Foreground = GetCachedColor("#FF909BB8");
        
        BtnPatchSpecial.Background = System.Windows.Media.Brushes.Transparent;
        BtnPatch.Background = System.Windows.Media.Brushes.Transparent;
        BtnGameDownload.Background = System.Windows.Media.Brushes.Transparent;
        
        // 设置选中按钮的颜色
        selectedButton.Foreground = GetCachedColor("#FFFFFFFF");
    }

    /// <summary>
    /// 特定游戏补丁按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void BtnPatchSpecial_Click(object sender, RoutedEventArgs e)
    {
        UpdateNavButtonSelected(BtnPatchSpecial);

        // 隐藏所有其他面板，显示特定游戏补丁界面
        PatchSpecialPanel.Visibility = Visibility.Visible;
        GameDetailPanel.Visibility = Visibility.Collapsed;
        DownloadPanel.Visibility = Visibility.Collapsed;
        PatchPanel.Visibility = Visibility.Collapsed;
    }

    /// <summary>
    /// 下载功能按钮点击事件（已废弃，保留以防需要恢复）
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void BtnDownload_Click(object sender, RoutedEventArgs e)
    {
        UpdateNavButtonSelected(BtnPatchSpecial);

        // 隐藏所有其他面板，显示特定游戏补丁界面
        PatchSpecialPanel.Visibility = Visibility.Visible;
        GameDetailPanel.Visibility = Visibility.Collapsed;
        DownloadPanel.Visibility = Visibility.Collapsed;
        PatchPanel.Visibility = Visibility.Collapsed;
    }

    /// <summary>
    /// 切换网格/列表视图
    /// </summary>
    private void SetViewMode(bool isGridView)
    {
        GridViewScrollViewer.Visibility = isGridView ? Visibility.Visible : Visibility.Collapsed;
        ListViewScrollViewer.Visibility = isGridView ? Visibility.Collapsed : Visibility.Visible;
        
        if (isGridView)
        {
            BtnGridView.Background = GetCachedColor("#FF0099F2");
            BtnGridView.Foreground = GetCachedColor("#FFFFFFFF");
            BtnListView.Background = GetCachedColor("#FF1A2445");
            BtnListView.Foreground = GetCachedColor("#FFB4BED6");
        }
        else
        {
            BtnListView.Background = GetCachedColor("#FF0099F2");
            BtnListView.Foreground = GetCachedColor("#FFFFFFFF");
            BtnGridView.Background = GetCachedColor("#FF1A2445");
            BtnGridView.Foreground = GetCachedColor("#FFB4BED6");
        }
    }

    private void BtnGridView_Click(object sender, RoutedEventArgs e) => SetViewMode(true);
    private void BtnListView_Click(object sender, RoutedEventArgs e) => SetViewMode(false);

    /// <summary>
    /// 返回游戏列表按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void BtnBackToList_Click(object sender, RoutedEventArgs e)
    {
        // 隐藏详情页面，显示游戏列表
        GameDetailPanel.Visibility = Visibility.Collapsed;
        PatchSpecialPanel.Visibility = Visibility.Visible;
        
        // 重置注入按钮状态
        BtnInjectPatch.IsEnabled = false;
        TxtGamePath.Text = "";
        InjectProgressPanel.Visibility = Visibility.Collapsed;
    }

    
    /// <summary>
    /// 游戏卡片点击事件（网格视图）
    /// </summary>
    private void GameCard_MouseLeftButtonDown(object sender, MouseButtonEventArgs e)
    {
        if (sender is Border border && border.Tag is string gameTag)
        {
            var game = GameConfigManager.Instance.GetGameByTag(gameTag);
            if (game != null)
            {
                ShowGameDetail(game);
            }
            else
            {
                MessageBox.Show($"未找到游戏配置：{gameTag}", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
            }
        }
    }

    /// <summary>
    /// 游戏卡片点击事件（列表视图）
    /// </summary>
    private void ListGameCard_MouseLeftButtonDown(object sender, MouseButtonEventArgs e)
    {
        if (sender is Border border && border.Tag is string gameTag)
        {
            var game = GameConfigManager.Instance.GetGameByTag(gameTag);
            if (game != null)
            {
                ShowGameDetail(game);
            }
            else
            {
                MessageBox.Show($"未找到游戏配置：{gameTag}", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
            }
        }
    }

    /// <summary>
    /// ScrollViewer 滚动事件处理 - 实现平滑滚动
    /// </summary>
    private void ScrollViewer_ScrollChanged(object sender, ScrollChangedEventArgs e)
    {
        // 此方法用于处理 ScrollViewer 的滚动事件
        // 平滑滚动效果已通过 XAML 中的属性配置实现
    }

    /// <summary>
    /// 补丁类型筛选按钮点击事件
    /// </summary>
    private void FilterGamesByPatchType(object sender, RoutedEventArgs e, PatchType? patchType = null, string logPrefix = "")
    {
        if (sender is not Button selectedButton) return;
        UpdatePatchTypeButtons(selectedButton);

        var searchResults = isSearching && !string.IsNullOrEmpty(currentSearchTerm)
            ? GameConfigManager.Instance.SearchGames(currentSearchTerm)
            : GameConfigManager.Instance.Games;

        _filteredGameCardViewModels = patchType.HasValue
            ? searchResults.Where(g => (PatchType)g.PatchType == patchType.Value).Select(GameCardViewModel.FromConfig).ToList()
            : searchResults.Select(GameCardViewModel.FromConfig).ToList();

        // AppendDownloadLog($"{logPrefix}（{_filteredGameCardViewModels.Count} 个游戏）");
        UpdateGameCardsDisplay();
    }

    private void BtnPatchTypeAll_Click(object sender, RoutedEventArgs e) => 
        FilterGamesByPatchType(sender, e, null, "🎮 显示所有游戏");

    private void BtnPatchTypeNoSteam_Click(object sender, RoutedEventArgs e) => 
        FilterGamesByPatchType(sender, e, PatchType.NoSteam, "🚫 显示免 Steam 启动游戏");

    private void BtnPatchTypeLAN_Click(object sender, RoutedEventArgs e) => 
        FilterGamesByPatchType(sender, e, PatchType.LAN, "🌐 显示局域网联机游戏");

    private void BtnPatchTypeSteamOnline_Click(object sender, RoutedEventArgs e) => 
        FilterGamesByPatchType(sender, e, PatchType.SteamOnline, "🌍 显示 Steam 联机游戏");

    private void BtnPatchTypeDEncrypted_Click(object sender, RoutedEventArgs e) => 
        FilterGamesByPatchType(sender, e, PatchType.DEncrypted, "🔒 显示 D 加密虚拟机游戏");

    /// <summary>
    /// 更新补丁类型按钮样式
    /// </summary>
    private void UpdatePatchTypeButtons(Button selectedButton)
    {
        var buttons = new[] { BtnPatchTypeAll, BtnPatchTypeNoSteam, BtnPatchTypeLAN, BtnPatchTypeSteamOnline, BtnPatchTypeDEncrypted };
        
        foreach (var btn in buttons)
        {
            btn.Background = GetCachedColor("#FF1A2445");
            btn.Foreground = GetCachedColor("#FFB4BED6");
        }
        
        selectedButton.Background = GetCachedColor("#FF0099F2");
        selectedButton.Foreground = GetCachedColor("#FFFFFFFF");
    }



    /// <summary>
    /// 根据标签获取游戏信息
    /// </summary>
    /// <param name="tag">游戏标签</param>
    /// <returns>游戏信息对象</returns>
    private PatchGameInfo? GetGameInfoByTag(string tag)
    {
        // 优先从配置管理器获取
        var config = GameConfigManager.Instance.GetGameByTag(tag);
        if (config != null)
        {
            return new PatchGameInfo
            {
                GameName = config.GameName,
                GameId = config.GameId,
                PatchSourcePath = config.PatchSourcePath,
                PatchType = (PatchType)config.PatchType,
                IsDEncrypted = config.IsDEncrypted
            };
        }
        // 如果配置中没有找到，返回 null
        AppendDownloadLog($"⚠️ 警告：游戏标签 '{tag}' 在配置文件中不存在");
        return null;

    }

    /// <summary>
    /// 选择游戏路径按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void BtnSelectGamePath_Click(object sender, RoutedEventArgs e)
    {
        var folderDialog = new System.Windows.Forms.FolderBrowserDialog
        {
            Description = "选择游戏安装文件夹",
            UseDescriptionForTitle = true,
            ShowNewFolderButton = false
        };

        if (folderDialog.ShowDialog() == System.Windows.Forms.DialogResult.OK)
        {
            TxtGamePath.Text = folderDialog.SelectedPath;
            BtnInjectPatch.IsEnabled = true;
            BtnRestoreGame.IsEnabled = true;
        }
    }

    /// <summary>
    /// 显示游戏详情（使用配置文件）
    /// </summary>
    private void ShowGameDetail(PatchGameConfig game)
    {
        CurrentSelectedGame = new PatchGameInfo
        {
            GameName = game.GameName,
            GameId = game.GameId,
            PatchSourcePath = game.PatchSourcePath,
            PatchType = (PatchType)game.PatchType,
            IsDEncrypted = game.IsDEncrypted
        };
        CurrentPatchSourcePath = game.PatchSourcePath;

        // 更新基本信息
        TxtDetailGameName.Text = game.GameName;
        TxtDetailGameId.Text = $"Game ID: {game.GameId}";
        TxtDetailGameNameCn.Text = game.ChineseName ?? "";

        // 加载封面图片
        try
        {
            string coverUri = $"pack://application:,,,/SteamToolPlus;component/Resources/pic/inside/{game.GameId}.jpg";
            ImgDetailCover.Source = new System.Windows.Media.Imaging.BitmapImage(new Uri(coverUri));
        }
        catch
        {
            // 如果加载失败，使用默认图片
        }

        // 清空路径和日志
        TxtGamePath.Text = "";
        TxtInjectLog.Text = "";
        InjectProgressPanel.Visibility = Visibility.Collapsed;
        BtnInjectPatch.IsEnabled = false;
        BtnRestoreGame.IsEnabled = false;

        // 加载 UI 配置
        LoadGameDetailUI(game);

        // 加载补丁文件列表
        LoadPatchFilesList(game.PatchSourcePath);
        LoadUsageInstructions(game.PatchSourcePath);

        // 切换界面
        PatchSpecialPanel.Visibility = Visibility.Collapsed;
        GameDetailPanel.Visibility = Visibility.Visible;
    }

    /// <summary>
    /// 根据游戏配置动态加载详情界面 UI
    /// </summary>
    private void LoadGameDetailUI(PatchGameConfig game)
    {
        var uiConfig = game.GetUIConfig();

        // 根据 UI 配置显示/隐藏相应控件
        SetControlVisibility(ChkEnableOverlay, uiConfig.ShowOverlay);
        SetControlVisibility(ChkImportAchievements, uiConfig.ShowAchievementImport);
        SetControlVisibility(ChkImportStats, uiConfig.ShowStatsImport);
        SetControlVisibility(ChkEnableHttp, uiConfig.ShowHttpFolder);
        SetControlVisibility(ChkEnableAvatar, uiConfig.ShowAvatar);
        SetControlVisibility(ChkEnableLanguage, uiConfig.ShowLanguage);
        SetControlVisibility(ChkEnableMods, uiConfig.ShowMods);
        SetControlVisibility(ChkEnableController, uiConfig.ShowController);
        SetControlVisibility(ChkEnableTicket, uiConfig.ShowTicket);
        SetControlVisibility(ChkCustomSavesName, uiConfig.ShowCustomSaveName);
        SetControlVisibility(RbExperimentalMode, uiConfig.ShowExperimentalMode);

        // 如果有特殊说明，显示出来
        if (!string.IsNullOrEmpty(uiConfig.SpecialInstructions))
        {
            TxtSpecialInstructions.Text = uiConfig.SpecialInstructions;
            SpecialInstructionsPanel.Visibility = Visibility.Visible;
        }
        else
        {
            SpecialInstructionsPanel.Visibility = Visibility.Collapsed;
        }

        // 如果有自定义使用说明文件路径，使用它
        if (!string.IsNullOrEmpty(uiConfig.InstructionsFile))
        {
            // TxtInstructionsPath.Text = uiConfig.InstructionsFile;
            // 注意：TxtInstructionsPath 控件未在 XAML 中定义，暂时注释
        }
    }

    /// <summary>
    /// 设置控件可见性（辅助方法）
    /// </summary>
    private void SetControlVisibility(UIElement control, bool isVisible)
    {
        if (control != null)
        {
            control.Visibility = isVisible ? Visibility.Visible : Visibility.Collapsed;
        }
    }

    /// <summary>
    /// 加载补丁文件列表
    /// </summary>
    /// <param name="sourcePath">补丁源路径</param>
    private void LoadPatchFilesList(string sourcePath)
    {
        PatchFilesList.Children.Clear();
        
        // 调试：输出实际路径
        System.Diagnostics.Debug.WriteLine($"正在检查补丁路径：{sourcePath}");
        System.Diagnostics.Debug.WriteLine($"路径是否存在：{Directory.Exists(sourcePath)}");
        
        if (!Directory.Exists(sourcePath))
        {
            var errorText = new TextBlock
            {
                Text = "❌ 补丁文件夹不存在",
                Foreground = new System.Windows.Media.SolidColorBrush(
                    (System.Windows.Media.Color)System.Windows.Media.ColorConverter.ConvertFromString("#FFF44336")),
                FontSize = 13,
                Margin = new Thickness(0, 5, 0, 5)
            };
            PatchFilesList.Children.Add(errorText);
            BtnInjectPatch.IsEnabled = false;
            return;
        }
        
        // 只获取顶层文件和文件夹
        var files = Directory.GetFiles(sourcePath, "*.*", SearchOption.TopDirectoryOnly);
        var directories = Directory.GetDirectories(sourcePath, "*", SearchOption.TopDirectoryOnly);
        int totalItems = files.Length + directories.Length;
        
        if (totalItems == 0)
        {
            var emptyText = new TextBlock
            {
                Text = "⚠️ 补丁文件夹为空",
                Foreground = GetCachedColor("#FFF57C00"),
                FontSize = 13,
                Margin = new Thickness(0, 5, 0, 5)
            };
            PatchFilesList.Children.Add(emptyText);
            BtnInjectPatch.IsEnabled = false;
            return;
        }
        
        // 显示文件数量
        var countText = new TextBlock
        {
            Text = $"共找到 {totalItems} 个项目（{files.Length} 个文件，{directories.Length} 个文件夹）",
            Foreground = GetCachedColor("#FF4CAF50"),
            FontSize = 13,
            FontWeight = System.Windows.FontWeights.Bold,
            Margin = new Thickness(0, 0, 0, 10)
        };
        PatchFilesList.Children.Add(countText);
        
        // 先显示文件夹
        foreach (string dir in directories.OrderBy(d => d))
        {
            string dirName = new System.IO.DirectoryInfo(dir).Name;
            var dirText = new TextBlock
            {
                Text = $"📁 {dirName}\\",
                Foreground = GetCachedColor("#FF64B8FF"),
                FontSize = 12,
                FontFamily = new System.Windows.Media.FontFamily("Consolas"),
                FontWeight = System.Windows.FontWeights.Bold,
                Margin = new Thickness(0, 3, 0, 3)
            };
            PatchFilesList.Children.Add(dirText);
        }
        
        // 显示文件列表
        foreach (string file in files.OrderBy(f => f))
        {
            string fileName = System.IO.Path.GetFileName(file);
            var fileText = new TextBlock
            {
                Text = $"📄 {fileName}",
                Foreground = GetCachedColor("#FFB4BED6"),
                FontSize = 12,
                FontFamily = new System.Windows.Media.FontFamily("Consolas"),
                Margin = new Thickness(0, 3, 0, 3)
            };
            PatchFilesList.Children.Add(fileText);
        }
        
        BtnInjectPatch.IsEnabled = true;
    }

    /// <summary>
    /// 加载使用说明
    /// </summary>
    /// <param name="patchSourcePath">补丁源路径</param>
    private void LoadUsageInstructions(string patchSourcePath)
    {
        TxtUsageInstructions.Text = "";
        
        try
        {
            if (Directory.Exists(patchSourcePath))
            {
                // 查找包含"使用"二字的 txt 文件
                var txtFiles = Directory.GetFiles(patchSourcePath, "*.txt", SearchOption.TopDirectoryOnly);
                
                // 筛选出包含"使用"的文件，并按文件名排序
                var usageFiles = txtFiles
                    .Where(f => Path.GetFileName(f).Contains("使用"))
                    .OrderBy(f => Path.GetFileName(f))
                    .ToList();
                
                if (usageFiles.Count == 0)
                {
                    TxtUsageInstructions.Text = ""; // 空白文本框
                    return;
                }
                
                // 遍历所有包含"使用"的文件
                for (int i = 0; i < usageFiles.Count; i++)
                {
                    var txtFile = usageFiles[i];
                    var fileName = Path.GetFileName(txtFile);
                    
                    // 添加文件分隔线（第一个文件前不加）
                    if (i > 0)
                    {
                        TxtUsageInstructions.Text += "\n\n─────────────────────────────────────\n\n";
                    }
                    
                    // 添加文件名作为标题
                    TxtUsageInstructions.Text += $"【{fileName}】\n\n";
                    
                    // 读取并添加文件内容
                    var content = File.ReadAllText(txtFile, System.Text.Encoding.UTF8);
                    TxtUsageInstructions.Text += content;
                }
            }
            else
            {
                TxtUsageInstructions.Text = ""; // 路径不存在也保持空白
            }
        }
        catch (Exception ex)
        {
            TxtUsageInstructions.Text = $"❌ 加载使用说明失败：{ex.Message}";
        }
    }

    /// <summary>
    /// 注入补丁按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private async void BtnInjectPatch_Click(object sender, RoutedEventArgs e)
    {
        if (CurrentSelectedGame == null || string.IsNullOrEmpty(TxtGamePath.Text))
        {
            MessageBox.Show("请先选择游戏安装路径！", "错误", MessageBoxButton.OK, MessageBoxImage.Warning);
            return;
        }
        
        string targetPath = TxtGamePath.Text;
        string sourcePath = CurrentPatchSourcePath;
        
        if (!Directory.Exists(targetPath))
        {
            MessageBox.Show("游戏路径不存在，请重新选择！", "错误", MessageBoxButton.OK, MessageBoxImage.Warning);
            return;
        }
        
        // 确认对话框
        var result = MessageBox.Show(
            $"确定要为 {CurrentSelectedGame.GameName} 注入补丁吗？\n\n" +
            $"源路径：{sourcePath}\n" +
            $"目标路径：{targetPath}\n\n" +
            $"注意：重复的文件将被备份为 .bak 文件",
            "确认注入",
            MessageBoxButton.YesNo,
            MessageBoxImage.Question);
        
        if (result != MessageBoxResult.Yes)
            return;
        
        // 开始注入
        InjectProgressPanel.Visibility = Visibility.Visible;
        BtnInjectPatch.IsEnabled = false;
        TxtInjectLog.Text = "";
        
        try
        {
            await InjectPatchFiles(sourcePath, targetPath);
        }
        catch (Exception ex)
        {
            AppendPatchLog($"❌ 错误：{ex.Message}");
            MessageBox.Show($"注入失败：{ex.Message}", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
            BtnInjectPatch.IsEnabled = true;
        }
    }

    /// <summary>
    /// 执行补丁文件注入
    /// </summary>
    /// <param name="sourcePath">源路径</param>
    /// <param name="targetPath">目标路径</param>
    private async Task InjectPatchFiles(string sourcePath, string targetPath)
    {
        var files = Directory.GetFiles(sourcePath, "*.*", SearchOption.AllDirectories);
        int totalFiles = files.Length;
        int processedFiles = 0;
        int backedUpFiles = 0;
        
        // 创建统一的备份文件夹
        string backupFolder = Path.Combine(targetPath, "_patch_backup");
        Directory.CreateDirectory(backupFolder);
        
        TxtInjectStatus.Text = "开始注入补丁...";
        TxtInjectProgress.Text = $"0/{totalFiles}";
        InjectProgressBar.Maximum = totalFiles;
        InjectProgressBar.Value = 0;
        
        AppendPatchLog($"开始注入补丁到：{targetPath}");
        AppendPatchLog($"备份文件夹：_patch_backup");
        AppendPatchLog($"共 {totalFiles} 个文件");
        
        // 步骤 1：生成 steam_interfaces.txt（在复制补丁文件之前）
        await GenerateSteamInterfacesFile(targetPath, backupFolder);
        
        // 步骤 2：复制补丁文件
        await Task.Run(() =>
        {
            foreach (string sourceFile in files)
            {
                // 计算相对路径
                string relativePath = sourceFile.Substring(sourcePath.Length + 1);
                string targetFile = Path.Combine(targetPath, relativePath);
                
                // 确保目标目录存在
                string? targetDir = Path.GetDirectoryName(targetFile);
                if (!string.IsNullOrEmpty(targetDir) && !Directory.Exists(targetDir))
                {
                    Directory.CreateDirectory(targetDir);
                }
                
                // 检查文件是否存在
                if (File.Exists(targetFile))
                {
                    // 文件存在，先备份到统一备份文件夹
                    // 保持原有目录结构，避免文件名冲突
                    string backupSubDir = Path.GetDirectoryName(relativePath) ?? "";
                    string backupSubPath = Path.Combine(backupFolder, backupSubDir);
                    Directory.CreateDirectory(backupSubPath);
                    
                    string backupFile = Path.Combine(backupSubPath, Path.GetFileName(targetFile) + ".bak");
                    try
                    {
                        File.Copy(targetFile, backupFile, true);
                        backedUpFiles++;
                        AppendPatchLog($"⚠️ 备份：{relativePath} → _patch_backup/{relativePath}.bak");
                    }
                    catch (Exception ex)
                    {
                        AppendPatchLog($"❌ 备份失败：{relativePath} - {ex.Message}");
                    }
                }
                
                // 复制文件
                try
                {
                    File.Copy(sourceFile, targetFile, true);
                    processedFiles++;
                    AppendPatchLog($"✅ 复制：{relativePath}");
                }
                catch (Exception ex)
                {
                    AppendPatchLog($"❌ 复制失败：{relativePath} - {ex.Message}");
                }
                
                // 更新进度（节流优化）
                int current = processedFiles;
                int backedUp = backedUpFiles;
                Dispatcher.Invoke(() =>
                {
                    TxtInjectStatus.Text = $"正在处理... (已备份 {backedUp} 个文件)";
                    TxtInjectProgress.Text = $"{current}/{totalFiles}";
                    InjectProgressBar.Value = current;
                }, System.Windows.Threading.DispatcherPriority.Background);
            }
        });
        
        // 完成
        AppendPatchLog($"注入完成！共处理 {processedFiles}/{totalFiles} 个文件，备份 {backedUpFiles} 个文件");
        AppendPatchLog($"📁 备份位置：_patch_backup/");
        TxtInjectStatus.Text = "✅ 注入完成";
        TxtInjectStatus.Foreground = new System.Windows.Media.SolidColorBrush(
            (System.Windows.Media.Color)System.Windows.Media.ColorConverter.ConvertFromString("#FF4CAF50"));
        
        MessageBox.Show(
            $"补丁注入完成！\n\n" +
            $"共处理：{processedFiles}/{totalFiles} 个文件\n" +
            $"备份文件：{backedUpFiles} 个\n" +
            $"备份位置：_patch_backup/",
            "完成",
            MessageBoxButton.OK,
            MessageBoxImage.Information);
        
        BtnInjectPatch.IsEnabled = true;
    }

    /// <summary>
    /// 生成 steam_interfaces.txt 文件（用于免 Steam 通用补丁注入）
    /// </summary>
    /// <param name="gamePath">游戏路径</param>
    /// <param name="backupFolder">备份文件夹路径</param>
    private async Task GenerateSteamInterfacesFile(string gamePath, string backupFolder)
    {
        try
        {
            AppendPatchLog("🔍 正在检测原始 steam_api DLL...");
            
            // 查找原始游戏的 steam_api DLL
            string? originalSteamApi = null;
            string[] possibleApiFiles = {
                Path.Combine(gamePath, "steam_api64.dll"),
                Path.Combine(gamePath, "steam_api.dll")
            };
            
            foreach (string apiFile in possibleApiFiles)
            {
                if (File.Exists(apiFile))
                {
                    // 优先使用备份的原始 DLL
                    string backupPath = Path.Combine(backupFolder, Path.GetFileName(apiFile) + ".bak");
                    if (File.Exists(backupPath))
                    {
                        originalSteamApi = backupPath;
                        AppendPatchLog($"📦 找到备份的原始 DLL：{Path.GetFileName(apiFile)}.bak");
                    }
                    else
                    {
                        originalSteamApi = apiFile;
                        AppendPatchLog($"📦 找到 DLL：{Path.GetFileName(apiFile)}");
                    }
                    break;
                }
            }
            
            if (originalSteamApi == null)
            {
                AppendPatchLog("⚠️ 未找到 steam_api DLL，跳过 steam_interfaces.txt 生成");
                return;
            }
            
            // 确定使用的 generate_interfaces 工具
            string toolPath = Path.Combine(AppPath, "Resources", "gbe_fork", "tools", "generate_interfaces");
            string exeName = Environment.Is64BitProcess ? "generate_interfaces_x64.exe" : "generate_interfaces_x32.exe";
            string generateInterfacesExe = Path.Combine(toolPath, exeName);
            
            if (!File.Exists(generateInterfacesExe))
            {
                AppendPatchLog($"❌ 未找到 generate_interfaces 工具：{exeName}");
                return;
            }
            
            AppendPatchLog($"🔧 准备调用 generate_interfaces 工具...");
            
            // 在临时目录生成 steam_interfaces.txt
            string tempInterfacesFile = Path.Combine(Path.GetTempPath(), "steam_interfaces.txt");
            
            // 调用工具
            var processInfo = new System.Diagnostics.ProcessStartInfo
            {
                FileName = generateInterfacesExe,
                Arguments = $"\"{originalSteamApi}\"",
                WorkingDirectory = toolPath,
                UseShellExecute = false,
                CreateNoWindow = true,
                RedirectStandardOutput = true,
                RedirectStandardError = true
            };
            
            using (var process = System.Diagnostics.Process.Start(processInfo))
            {
                if (process != null)
                {
                    string output = await process.StandardOutput.ReadToEndAsync();
                    string error = await process.StandardError.ReadToEndAsync();
                    await Task.Run(() => process.WaitForExit());
                    
                    if (!string.IsNullOrEmpty(output))
                    {
                        AppendPatchLog($"📝 generate_interfaces 输出：{output.Trim()}");
                    }
                    
                    if (!string.IsNullOrEmpty(error))
                    {
                        AppendPatchLog($"⚠️ generate_interfaces 错误：{error.Trim()}");
                    }
                    
                    if (process.ExitCode != 0)
                    {
                        AppendPatchLog($"❌ generate_interfaces 执行失败，退出码：{process.ExitCode}");
                        return;
                    }
                }
            }
            
            // 检查生成的文件
            if (!File.Exists(tempInterfacesFile))
            {
                AppendPatchLog("❌ steam_interfaces.txt 生成失败");
                return;
            }
            
            // 确保 steam_settings 文件夹存在
            string steamSettingsDir = Path.Combine(gamePath, "steam_settings");
            Directory.CreateDirectory(steamSettingsDir);
            
            // 复制生成的文件到 steam_settings 文件夹
            string targetInterfacesFile = Path.Combine(steamSettingsDir, "steam_interfaces.txt");
            File.Copy(tempInterfacesFile, targetInterfacesFile, true);
            
            // 清理临时文件
            try { File.Delete(tempInterfacesFile); } catch { }
            
            AppendPatchLog($"✅ steam_interfaces.txt 已生成并复制到 steam_settings 文件夹");
        }
        catch (Exception ex)
        {
            AppendPatchLog($"❌ 生成 steam_interfaces.txt 失败：{ex.Message}");
        }
    }

    /// <summary>
    /// 从备份的原始 DLL 生成 steam_interfaces.txt（专为一键配置场景）
    /// </summary>
    /// <param name="gamePath">游戏路径</param>
    /// <param name="backupDllPath">备份的原始 DLL 路径</param>
    /// <param name="dllName">DLL 文件名</param>
    private async Task GenerateSteamInterfacesFileFromBackup(string gamePath, string backupDllPath, string dllName)
    {
        try
        {
            AppendPatchLog($"🔍 使用备份的原始 DLL 生成 steam_interfaces.txt...");
            
            // 检查备份文件是否存在
            if (!File.Exists(backupDllPath))
            {
                AppendPatchLog($"❌ 备份文件不存在：{backupDllPath}");
                return;
            }
            
            AppendPatchLog($"📦 使用备份文件：{Path.GetFileName(backupDllPath)}");
            
            // 确定使用的 generate_interfaces 工具
            string toolPath = Path.Combine(AppPath, "Resources", "gbe_fork", "tools", "generate_interfaces");
            string exeName = Environment.Is64BitProcess ? "generate_interfaces_x64.exe" : "generate_interfaces_x32.exe";
            string generateInterfacesExe = Path.Combine(toolPath, exeName);
            
            if (!File.Exists(generateInterfacesExe))
            {
                AppendPatchLog($"❌ 未找到 generate_interfaces 工具：{exeName}");
                return;
            }
            
            AppendPatchLog($"🔧 准备调用 generate_interfaces 工具...");
            
            // 将备份的 DLL 复制到临时目录进行处理（避免在原位置生成文件）
            string tempDir = Path.Combine(Path.GetTempPath(), "steam_interfaces_gen_" + Guid.NewGuid().ToString());
            Directory.CreateDirectory(tempDir);
            
            string tempDllPath = Path.Combine(tempDir, dllName);
            File.Copy(backupDllPath, tempDllPath, true);
            
            AppendPatchLog($"📂 临时工作目录：{tempDir}");
            
            // 调用工具，传入临时目录的 DLL 路径
            // 注意：generate_interfaces 会在它的工作目录生成 steam_interfaces.txt
            // 所以我们需要将工作目录设置为临时目录，或者从工具目录复制生成的文件
            var processInfo = new System.Diagnostics.ProcessStartInfo
            {
                FileName = generateInterfacesExe,
                Arguments = $"\"{tempDllPath}\"",
                WorkingDirectory = tempDir,  // 改为临时目录，这样生成的文件会在临时目录
                UseShellExecute = false,
                CreateNoWindow = true,
                RedirectStandardOutput = true,
                RedirectStandardError = true
            };
            
            using (var process = System.Diagnostics.Process.Start(processInfo))
            {
                if (process != null)
                {
                    string output = await process.StandardOutput.ReadToEndAsync();
                    string error = await process.StandardError.ReadToEndAsync();
                    await Task.Run(() => process.WaitForExit());
                    
                    if (!string.IsNullOrEmpty(output))
                    {
                        AppendPatchLog($"📝 generate_interfaces 输出：{output.Trim()}");
                    }
                    
                    if (!string.IsNullOrEmpty(error))
                    {
                        AppendPatchLog($"⚠️ generate_interfaces 错误：{error.Trim()}");
                    }
                    
                    if (process.ExitCode != 0)
                    {
                        AppendPatchLog($"❌ generate_interfaces 执行失败，退出码：{process.ExitCode}");
                        // 清理临时目录
                        try { Directory.Delete(tempDir, true); } catch { }
                        return;
                    }
                }
            }
            
            // 检查生成的文件（在临时目录中）
            string generatedInterfacesFile = Path.Combine(tempDir, "steam_interfaces.txt");
            if (!File.Exists(generatedInterfacesFile))
            {
                AppendPatchLog("❌ steam_interfaces.txt 生成失败 - 未在临时目录找到生成的文件");
                // 清理临时目录
                try { Directory.Delete(tempDir, true); } catch { }
                return;
            }
            
            // 确保 steam_settings 文件夹存在
            string steamSettingsDir = Path.Combine(gamePath, "steam_settings");
            Directory.CreateDirectory(steamSettingsDir);
            
            // 复制生成的文件到 steam_settings 文件夹
            string targetInterfacesFile = Path.Combine(steamSettingsDir, "steam_interfaces.txt");
            File.Copy(generatedInterfacesFile, targetInterfacesFile, true);
            
            AppendPatchLog($"✅ steam_interfaces.txt 已生成并复制到 steam_settings 文件夹");
            
            // 清理临时目录
            try { Directory.Delete(tempDir, true); } catch { }
        }
        catch (Exception ex)
        {
            AppendPatchLog($"❌ 生成 steam_interfaces.txt 失败：{ex.Message}");
        }
    }

    /// <summary>
    /// 补丁功能按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void BtnPatch_Click(object sender, RoutedEventArgs e)
    {
        UpdateNavButtonSelected(BtnPatch);

        // 隐藏游戏详情面板，显示补丁注入主界面
        GameDetailPanel.Visibility = Visibility.Collapsed;
        PatchSpecialPanel.Visibility = Visibility.Collapsed;
        DownloadPanel.Visibility = Visibility.Collapsed;
        PatchPanel.Visibility = Visibility.Visible;

        ShowPatchCategory("basic");
    }

    /// <summary>
    /// 游戏本体下载按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void BtnGameDownload_Click(object sender, RoutedEventArgs e)
    {
        UpdateNavButtonSelected(BtnGameDownload);

        // 隐藏所有其他面板，显示游戏本体下载界面
        PatchSpecialPanel.Visibility = Visibility.Collapsed;
        GameDetailPanel.Visibility = Visibility.Collapsed;
        DownloadPanel.Visibility = Visibility.Visible;
        PatchPanel.Visibility = Visibility.Collapsed;
    }

    /// <summary>
    /// B 站链接点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void BilibiliLink_MouseLeftButtonDown(object sender, MouseButtonEventArgs e)
    {
        Process.Start(new ProcessStartInfo { FileName = "https://space.bilibili.com/405707676", UseShellExecute = true });
    }

    /// <summary>
    /// 百度网盘下载链接点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void PanDownloadLink_MouseLeftButtonDown(object sender, MouseButtonEventArgs e)
    {
        Process.Start(new ProcessStartInfo { FileName = "https://pan.baidu.com/s/1XbcZOLQcn4500z-SL1RDug?pwd=v1xm", UseShellExecute = true });
    }

    /// <summary>
    /// 选择 ddv20.exe 按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void SelectDdExe_Click(object sender, RoutedEventArgs e)
    {
        var dialog = new OpenFileDialog { Title = "选择ddv20.exe", Filter = "可执行程序|ddv20.exe;*.exe" };
        if (dialog.ShowDialog() == true)
        {
            TxtDdExePath.Text = dialog.FileName;
            AppendDownloadLog("✅ 已选择ddv20.exe");
        }
    }

    /// <summary>
    /// 单文件夹模式单选按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void RbSingleFolder_Checked(object sender, RoutedEventArgs e)
    {
        IsSingleFolderMode = true;
        BatchGameList.Clear();
        CurrentBatchGame = null;
        AppendDownloadLog("📁 已切换到单文件夹下载模式");
        AppendDownloadLog("💡 请选择包含 game.json、*.manifest 和*.vdf 的单个游戏配置文件夹");
        if (TxtGameConfigDir == null || TxtGameInfo == null || BtnStartDownload == null) return;
        TxtGameConfigDir.Text = "";
        TxtGameInfo.Text = "游戏名称：未加载 | AppID：未加载 | 可下载Depot数量：0个";
        BtnStartDownload.IsEnabled = false;
    }

    /// <summary>
    /// 批量下载模式单选按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void RbBatchDownload_Checked(object sender, RoutedEventArgs e)
    {
        IsSingleFolderMode = false;
        GameAppId = null;
        GameName = null;
        ManifestList.Clear();
        AppendDownloadLog("📦 已切换到批量下载模式");
        AppendDownloadLog("💡 请选择包含多个游戏配置文件夹的母文件夹");
        if (TxtGameConfigDir == null || TxtGameInfo == null || BtnStartDownload == null) return;
        TxtGameConfigDir.Text = "";
        TxtGameInfo.Text = "游戏名称：未加载 | AppID：未加载 | 可下载Depot数量：0个";
        BtnStartDownload.IsEnabled = false;
    }

    /// <summary>
    /// 选择游戏配置目录按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void SelectGameConfigDir_Click(object sender, RoutedEventArgs e)
    {
        if (IsSingleFolderMode)
        {
            var dialog = new OpenFolderDialog { Title = "选择包含 json/manifest/*.vdf 的游戏配置文件夹" };
            if (dialog.ShowDialog() == true) AutoParseSingleGameFolder(dialog.FolderName);
        }
        else
        {
            var dialog = new OpenFolderDialog { Title = "选择包含多个游戏配置文件夹的母文件夹" };
            if (dialog.ShowDialog() == true) AutoParseBatchFolder(dialog.FolderName);
        }
    }

    /// <summary>
    /// 选择保存目录按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void SelectSaveDir_Click(object sender, RoutedEventArgs e)
    {
        var dialog = new OpenFolderDialog { Title = "选择游戏保存目录" };
        if (dialog.ShowDialog() == true)
        {
            UserCustomPath = true;
            TxtSaveDir.Text = dialog.FolderName;
            AppendDownloadLog("✅ 已自定义保存目录");
            Directory.CreateDirectory(dialog.FolderName);
        }
    }

    /// <summary>
    /// 打开 ManifestHub GitHub 页面
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void OpenManifestHub_Click(object sender, RoutedEventArgs e)
    {
        try
        {
            // 使用默认浏览器打开 GitHub 页面
            System.Diagnostics.Process.Start(new System.Diagnostics.ProcessStartInfo
            {
                FileName = "https://github.com/SteamAutoCracks/ManifestHub",
                UseShellExecute = true
            });
            AppendDownloadLog("🔗 已打开清单库 GitHub 页面");
        }
        catch (Exception ex)
        {
            AppendDownloadLog($"❌ 无法打开浏览器：{ex.Message}");
        }
    }

    /// <summary>
    /// 打开百度网盘下载链接
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void OpenPanDownload_Click(object sender, RoutedEventArgs e)
    {
        try
        {
            // 使用默认浏览器打开百度网盘页面
            System.Diagnostics.Process.Start(new System.Diagnostics.ProcessStartInfo
            {
                FileName = "https://pan.baidu.com/s/1FTZyknIObyzMuLAJC-Uj9g?pwd=8uwx",
                UseShellExecute = true
            });
            AppendDownloadLog("📥 已打开百度网盘下载链接（提取码：8uwx）");
        }
        catch (Exception ex)
        {
            AppendDownloadLog($"❌ 无法打开浏览器：{ex.Message}");
        }
    }

    /// <summary>
    /// 切换到通过清单下载标签页
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void TabManifest_Click(object sender, RoutedEventArgs e)
    {
        // 切换内容显示
        ManifestDownloadContent.Visibility = Visibility.Visible;
        CloudDriveDownloadContent.Visibility = Visibility.Collapsed;

        // 更新按钮样式
        BtnTabManifest.Background = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromRgb(0x00, 0x99, 0xF2));
        BtnTabManifest.Foreground = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromRgb(0xFF, 0xFF, 0xFF));
        BtnTabManifest.FontWeight = FontWeights.Bold;

        BtnTabCloudDrive.Background = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromRgb(0x1A, 0x24, 0x45));
        BtnTabCloudDrive.Foreground = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromRgb(0xB4, 0xBE, 0xD6));
        BtnTabCloudDrive.FontWeight = FontWeights.Normal;
    }

    /// <summary>
    /// 切换到网盘下载标签页
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void TabCloudDrive_Click(object sender, RoutedEventArgs e)
    {
        // 切换内容显示
        ManifestDownloadContent.Visibility = Visibility.Collapsed;
        CloudDriveDownloadContent.Visibility = Visibility.Visible;

        // 更新按钮样式
        BtnTabManifest.Background = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromRgb(0x1A, 0x24, 0x45));
        BtnTabManifest.Foreground = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromRgb(0xB4, 0xBE, 0xD6));
        BtnTabManifest.FontWeight = FontWeights.Normal;

        BtnTabCloudDrive.Background = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromRgb(0x00, 0x99, 0xF2));
        BtnTabCloudDrive.Foreground = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromRgb(0xFF, 0xFF, 0xFF));
        BtnTabCloudDrive.FontWeight = FontWeights.Bold;
    }

    /// <summary>
    /// 自动解析单文件夹游戏配置
    /// </summary>
    /// <param name="folderPath">配置文件夹路径</param>
    private void AutoParseSingleGameFolder(string folderPath)
    {
        TxtGameConfigDir.Text = folderPath;
        AppendDownloadLog("=" + new string('=', 49));
        AppendDownloadLog("📂 已选择游戏清单文件夹（单文件夹模式）");
        AppendDownloadLog("🔍 开始解析...");

        GameAppId = null;
        GameName = null;
        ManifestList = new();

        try
        {
            var jsonFiles = Directory.GetFiles(folderPath, "*.json");
            if (jsonFiles.Length > 0)
            {
                string jsonPath = jsonFiles[0];
                object? gameJson = null;
                try
                {
                    string content = File.ReadAllText(jsonPath, Encoding.UTF8);
                    gameJson = JsonConvert.DeserializeObject(content);
                }
                catch
                {
                    try
                    {
                        string content = File.ReadAllText(jsonPath, Encoding.GetEncoding("GBK"));
                        gameJson = JsonConvert.DeserializeObject(content);
                    }
                    catch { }
                }

                if (gameJson != null)
                {
                    var jObj = gameJson as Newtonsoft.Json.Linq.JObject;
                    if (jObj != null)
                    {
                        GameAppId = jObj["appid"]?.ToString();
                        GameName = jObj["schinese_name"]?.ToString() ?? jObj["name"]?.ToString() ?? "未知游戏";
                    }
                }

                if (GameAppId != null && GameName != null)
                {
                    AppendDownloadLog($"✅ 解析成功：{GameName} (AppID: {GameAppId})");
                }
                else
                {
                    AppendDownloadLog("⚠️  找到.json 文件但未能提取有效信息");
                }
            }
            else
            {
                AppendDownloadLog("ℹ️  未找到.json 文件，将使用自动编号");
            }

            // 设置保存路径：如果有 AppID 就用 AppID，否则使用递增编号
            if (!UserCustomPath)
            {
                string gameSavePath;
                if (GameAppId != null)
                {
                    // 使用 AppID 作为保存目录
                    gameSavePath = Path.Combine(BaseSavePath, GameAppId);
                }
                else
                {
                    // 使用递增编号作为保存目录
                    gameSavePath = GetNextAvailableGamePath(BaseSavePath);
                }
                TxtSaveDir.Text = gameSavePath;
                Directory.CreateDirectory(gameSavePath);
                AppendDownloadLog($"📁 已自动设置保存路径：{gameSavePath}");
            }

            ManifestList = Directory.GetFiles(folderPath, "*.manifest").Select(Path.GetFileName).ToList()!;
            if (ManifestList.Count == 0)
            {
                AppendDownloadLog("❌ 错误：未找到.manifest文件");
                MessageBox.Show("未找到.manifest清单文件", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
                return;
            }
            AppendDownloadLog($"📦 找到 {ManifestList.Count} 个 Depot 清单");

            // 检查是否有.vdf 文件
            string[] vdfFiles = Directory.GetFiles(folderPath, "*.vdf");
            if (vdfFiles.Length == 0)
            {
                AppendDownloadLog("❌ 错误：未找到.vdf 文件");
                MessageBox.Show("未找到.vdf 文件（需要 key.vdf 或其他.vdf 文件）", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
                return;
            }
            AppendDownloadLog($"✅ 找到 {vdfFiles.Length} 个.vdf 文件");

            TxtGameInfo.Text = $"游戏名称：{GameName ?? "未知"} | AppID：{GameAppId ?? "自动编号"} | 可下载 Depot 数量：{ManifestList.Count}个";
            BtnStartDownload.IsEnabled = true;
            AppendDownloadLog("🎉 解析完成，可开始下载");
            AppendDownloadLog("=" + new string('=', 49));
        }
        catch (Exception ex)
        {
            AppendDownloadLog($"❌ 解析失败：{ex.Message}");
            MessageBox.Show($"解析失败：{ex.Message}", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
        }
    }

    /// <summary>
    /// 获取下一个可用的游戏保存路径（自动递增编号）
    /// </summary>
    /// <param name="basePath">基础路径</param>
    /// <returns>可用的保存路径</returns>
    private string GetNextAvailableGamePath(string basePath)
    {
        // 确保基础目录存在
        Directory.CreateDirectory(basePath);
        
        int index = 1;
        string gamePath;
        
        // 循环查找下一个可用的编号
        while (true)
        {
            gamePath = Path.Combine(basePath, index.ToString());
            if (!Directory.Exists(gamePath))
            {
                // 找到可用的路径
                return gamePath;
            }
            index++;
            
            // 防止无限循环（最多检查 9999）
            if (index > 9999)
            {
                throw new Exception("无法找到可用的保存路径（已尝试 1-9999）");
            }
        }
    }

    /// <summary>
    /// 自动解析批量下载文件夹
    /// </summary>
    /// <param name="folderPath">母文件夹路径</param>
    private void AutoParseBatchFolder(string folderPath)
    {
        TxtGameConfigDir.Text = folderPath;
        AppendDownloadLog("=" + new string('=', 49));
        AppendDownloadLog("📂 已选择批量下载文件夹（母文件夹）");
        AppendDownloadLog("🔍 开始扫描子文件夹...");

        BatchGameList.Clear();
        GameAppId = null;
        GameName = null;

        try
        {
            var subDirs = Directory.GetDirectories(folderPath);
            if (subDirs.Length == 0)
            {
                AppendDownloadLog("❌ 错误：未找到任何子文件夹");
                MessageBox.Show("未找到任何游戏配置子文件夹", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
                return;
            }

            AppendDownloadLog($"🔍 找到 {subDirs.Length} 个子文件夹，开始解析...");

            foreach (var subDir in subDirs)
            {
                string subDirName = Path.GetFileName(subDir);
                var jsonFiles = Directory.GetFiles(subDir, "*.json");
                if (jsonFiles.Length == 0) continue;

                string? appId = null;
                string? gameName = null;

                string jsonPath = jsonFiles[0];
                try
                {
                    string content = File.ReadAllText(jsonPath, Encoding.UTF8);
                    var jObj = JsonConvert.DeserializeObject(content) as Newtonsoft.Json.Linq.JObject;
                    if (jObj != null)
                    {
                        appId = jObj["appid"]?.ToString();
                        gameName = jObj["schinese_name"]?.ToString() ?? jObj["name"]?.ToString() ?? subDirName;
                    }
                }
                catch
                {
                    try
                    {
                        string content = File.ReadAllText(jsonPath, Encoding.GetEncoding("GBK"));
                        var jObj = JsonConvert.DeserializeObject(content) as Newtonsoft.Json.Linq.JObject;
                        if (jObj != null)
                        {
                            appId = jObj["appid"]?.ToString();
                            gameName = jObj["schinese_name"]?.ToString() ?? jObj["name"]?.ToString() ?? subDirName;
                        }
                    }
                    catch { }
                }

                var manifestFiles = Directory.GetFiles(subDir, "*.manifest");
                bool hasKeyVdf = File.Exists(Path.Combine(subDir, "key.vdf"));

                if (appId != null && manifestFiles.Length > 0 && hasKeyVdf)
                {
                    var gameInfo = new GameInfo
                    {
                        AppId = appId,
                        Name = gameName ?? subDirName,
                        FolderPath = subDir,
                        ManifestCount = manifestFiles.Length
                    };
                    BatchGameList.Add(gameInfo);
                    AppendDownloadLog($"✅ 发现游戏：{gameName} (AppID: {appId}, Depots: {manifestFiles.Length})");
                }
            }

            if (BatchGameList.Count == 0)
            {
                AppendDownloadLog("❌ 错误：未找到任何有效的游戏配置");
                MessageBox.Show("未找到任何有效的游戏配置（需要包含game.json、.manifest和key.vdf）", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
                return;
            }

            AppendDownloadLog($"🎉 共发现 {BatchGameList.Count} 个可下载的游戏");

            if (!UserCustomPath)
            {
                TxtSaveDir.Text = BaseSavePath;
                AppendDownloadLog($"📁 保存目录设置为：{BaseSavePath}");
            }

            TxtGameInfo.Text = $"批量模式 | 共 {BatchGameList.Count} 个游戏 | 请选择要下载的游戏";
            ShowBatchGameSelectionDialog();
        }
        catch (Exception ex)
        {
            AppendDownloadLog($"❌ 解析失败：{ex.Message}");
            MessageBox.Show($"解析失败：{ex.Message}", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
        }
    }

    /// <summary>
    /// 显示批量游戏选择对话框
    /// </summary>
    private void ShowBatchGameSelectionDialog()
    {
        if (BatchGameList.Count == 0) return;

        var dialog = new Window
        {
            Title = "选择要下载的游戏",
            Width = 550,
            Height = 450,
            WindowStartupLocation = WindowStartupLocation.CenterOwner,
            Owner = this,
            Background = GetCachedColor("#FF1B2838"),
            ResizeMode = ResizeMode.NoResize
        };

        var panel = new StackPanel { Margin = new Thickness(15) };

        var header = new TextBlock
        {
            Text = "请选择要下载的游戏（勾选需要的游戏）：",
            Foreground = GetCachedColor("#FFC7D5E0"),
            FontSize = 12,
            Margin = new Thickness(0, 0, 0, 10)
        };
        panel.Children.Add(header);

        var scrollViewer = new ScrollViewer { Height = 280, VerticalScrollBarVisibility = ScrollBarVisibility.Auto };
        var checkBoxPanel = new StackPanel { Background = GetCachedColor("#FF0D1117") };

        foreach (var game in BatchGameList)
        {
            var checkBox = new CheckBox
            {
                Content = $"☐ {game.Name} (AppID: {game.AppId}) - {game.ManifestCount}个 Depot",
                Foreground = GetCachedColor("#FFC7D5E0"),
                FontSize = 11,
                Margin = new Thickness(5, 5, 5, 5),
                Tag = game,
                IsChecked = false
            };
            checkBox.Click += (s, e) =>
            {
                var cb = s as CheckBox;
                if (cb != null)
                    cb.Content = cb.IsChecked == true ? cb.Content.ToString()!.Replace("☐", "☑") : cb.Content.ToString()!.Replace("☑", "☐");
            };
            checkBoxPanel.Children.Add(checkBox);
        }

        scrollViewer.Content = checkBoxPanel;
        panel.Children.Add(scrollViewer);

        var buttonPanel = new StackPanel { Orientation = Orientation.Horizontal, HorizontalAlignment = System.Windows.HorizontalAlignment.Right, Margin = new Thickness(0, 15, 0, 0) };

        var selectAllBtn = new Button { Content = "全选", Background = GetCachedColor("#FF3C7FC4"), Foreground = System.Windows.Media.Brushes.White, Padding = new Thickness(15, 5, 15, 5), Margin = new Thickness(0, 0, 10, 0), Cursor = System.Windows.Input.Cursors.Hand };
        selectAllBtn.Click += (s, e) =>
        {
            foreach (var child in checkBoxPanel.Children)
                if (child is CheckBox cb) { cb.IsChecked = true; cb.Content = cb.Content.ToString()!.Replace("☐", "☑"); }
        };

        var selectNoneBtn = new Button { Content = "全取消", Background = GetCachedColor("#FF455A64"), Foreground = System.Windows.Media.Brushes.White, Padding = new Thickness(15, 5, 15, 5), Margin = new Thickness(0, 0, 10, 0), Cursor = System.Windows.Input.Cursors.Hand };
        selectNoneBtn.Click += (s, e) =>
        {
            foreach (var child in checkBoxPanel.Children)
                if (child is CheckBox cb) { cb.IsChecked = false; cb.Content = cb.Content.ToString()!.Replace("☑", "☐"); }
        };

        var confirmBtn = new Button { Content = "确认下载", Background = GetCachedColor("#FF2E7D32"), Foreground = System.Windows.Media.Brushes.White, Padding = new Thickness(15, 5, 15, 5), Margin = new Thickness(0, 0, 10, 0), Cursor = System.Windows.Input.Cursors.Hand };
        confirmBtn.Click += (s, e) =>
        {
            var selectedGames = new List<GameInfo>();
            foreach (var child in checkBoxPanel.Children)
                if (child is CheckBox cb && cb.IsChecked == true && cb.Tag is GameInfo game) selectedGames.Add(game);

            if (selectedGames.Count == 0)
            { ShowWarning("请至少选择一个游戏"); return; }

            StartBatchDownload(selectedGames);
            dialog.Close();
        };

        var cancelBtn = new Button { Content = "取消", Background = GetCachedColor("#FF607D8B"), Foreground = System.Windows.Media.Brushes.White, Padding = new Thickness(15, 5, 15, 5), Cursor = System.Windows.Input.Cursors.Hand };
        cancelBtn.Click += (s, e) => dialog.Close();

        buttonPanel.Children.Add(selectAllBtn);
        buttonPanel.Children.Add(selectNoneBtn);
        buttonPanel.Children.Add(confirmBtn);
        buttonPanel.Children.Add(cancelBtn);
        panel.Children.Add(buttonPanel);

        dialog.Content = panel;
        dialog.ShowDialog();
    }

    /// <summary>
    /// 开始批量下载
    /// </summary>
    /// <param name="games">选择的游戏列表</param>
    private void StartBatchDownload(List<GameInfo> games)
    {
        if (games.Count == 0)
        { ShowWarning("请至少选择一个游戏"); return; }

        string saveDir = TxtSaveDir.Text;
        string ddv20Path = TxtDdExePath.Text;

        AppendDownloadLog($"🚀 开始批量下载，共选择 {games.Count} 个游戏");

        IsDownloading = true;
        IsPaused = false;

        BtnStartDownload.IsEnabled = false;
        BtnStartDownload.Content = "正在下载中...";
        BtnPauseDownload.IsEnabled = true;

        Task.Run(async () =>
        {
            int completed = 0;
            int total = games.Count;

            foreach (var game in games)
            {
                if (!IsDownloading) break;

                Dispatcher.BeginInvoke(() => TxtGameInfo.Text = $"正在下载：{game.Name} ({completed + 1}/{total})");
                AppendDownloadLog($"━━━ 开始下载：{game.Name} ━━━");

                string gameSavePath = Path.Combine(saveDir, game.AppId);
                try { Directory.CreateDirectory(gameSavePath); } catch { }

                bool success = await DownloadSingleGame(game.FolderPath, gameSavePath, game.AppId, game.Name, ddv20Path);

                completed++;
                int progress = (int)((completed / (double)total) * 100);
                AppendDownloadLog($"进度：{progress}%");

                if (success)
                    AppendDownloadLog($"✅ {game.Name} 下载完成");
                else
                    AppendDownloadLog($"⚠️ {game.Name} 下载失败或已取消");
            }

            Dispatcher.BeginInvoke(() =>
            {
                AppendDownloadLog("=" + new string('=', 49));
                AppendDownloadLog($"🎉 批量下载完成！成功 {completed}/{total} 个游戏");
                AppendDownloadLog($"📁 保存目录：{saveDir}");
                MessageBox.Show($"批量下载完成！\n成功 {completed}/{total} 个游戏\n保存目录：{saveDir}", "完成", MessageBoxButton.OK, MessageBoxImage.Information);

                IsDownloading = false;
                IsPaused = false;
                BtnStartDownload.IsEnabled = true;
                BtnStartDownload.Content = "▶ 开始下载";
                BtnPauseDownload.IsEnabled = false;
                BtnPauseDownload.Content = "⏸ 暂停下载";
            });
        });
    }

    /// <summary>
    /// 单游戏下载核心方法
    /// 使用同步方式读取进程输出，更可靠地获取ddv20.exe的实时输出
    /// </summary>
    /// <param name="configDir">配置目录</param>
    /// <param name="saveDir">保存目录</param>
    /// <param name="appId">游戏AppID</param>
    /// <param name="gameName">游戏名称</param>
    /// <param name="ddv20Path">ddv20.exe 路径</param>
    /// <returns>下载是否成功</returns>
    private async Task<bool> DownloadSingleGame(string configDir, string saveDir, string appId, string gameName, string ddv20Path)
    {
        try
        {
            AppendDownloadLog($"📋 ddv20路径: {ddv20Path}");
            AppendDownloadLog($"📋 配置目录: {configDir}");
            AppendDownloadLog($"📋 保存目录: {saveDir}");

            // 检查ddv20.exe是否存在
            if (!File.Exists(ddv20Path))
            {
                AppendDownloadLog($"❌ 错误：ddv20.exe不存在于 {ddv20Path}");
                return false;
            }

            // 检查配置目录是否存在
            if (!Directory.Exists(configDir))
            {
                AppendDownloadLog($"❌ 错误：配置目录不存在 {configDir}");
                return false;
            }

            // 验证是否有.vdf 文件
            string[] vdfFiles = Directory.GetFiles(configDir, "*.vdf");
            if (vdfFiles.Length == 0)
            {
                AppendDownloadLog($"❌ 错误：配置目录中未找到.vdf 文件 {configDir}");
                return false;
            }
            AppendDownloadLog($"✅ 找到 {vdfFiles.Length} 个.vdf 文件");

            // 构建 ddv20.exe 命令行参数
            // -lu China: 使用中国节点
            // --use-http: 使用HTTP下载
            // -o "保存目录": 指定保存路径
            // app -p "配置目录": 从配置目录读取app信息进行下载
            string arguments = $"-lu China --use-http -o \"{saveDir}\" app -p \"{configDir}\"";

            AppendDownloadLog($"📋 执行命令：{arguments}");

            // 确保保存目录存在
            Directory.CreateDirectory(saveDir);

            // 配置进程启动参数
            var psi = new ProcessStartInfo
            {
                FileName = ddv20Path,      // ddv20.exe 路径
                Arguments = arguments,      // 命令行参数
                UseShellExecute = true,    // 使用 shell 执行（显示控制台窗口）
                CreateNoWindow = false,    // 创建窗口（ddv20 需要可见窗口才能正常工作）
                WorkingDirectory = AppPath // 工作目录
            };

            AppendDownloadLog("🚀 正在启动ddv20.exe...");
            DownloadProcess = Process.Start(psi);  // 启动进程

            if (DownloadProcess == null)
            {
                AppendDownloadLog("❌ 错误：无法启动ddv20.exe");
                return false;
            }

            AppendDownloadLog("✅ ddv20.exe 已启动，请在打开的窗口中查看下载进度...");
            AppendDownloadLog("⚠️  注意：请勿关闭 ddv20 窗口，下载完成后会自动关闭");
            AppendDownloadLog("📋 主程序将保持响应，您可以继续使用其他功能...");

            // 启动后台任务监控进程
            await Task.Run(() => MonitorDownloadProcess());

            return DownloadProcess.ExitCode == 0;
        }
        catch (Exception ex)
        {
            AppendDownloadLog($"❌ 下载失败：{ex.Message}");
            return false;
        }
    }

    /// <summary>
    /// 后台监控下载进程
    /// </summary>
    private async Task MonitorDownloadProcess()
    {
        while (!DownloadProcess.HasExited)
        {
            if (!IsDownloading)
            {
                try { DownloadProcess.Kill(); AppendDownloadLog("⚠️  下载已被用户取消"); }
                catch { }
                return;
            }
            await Task.Delay(1000);
        }

        await Task.Delay(500);
        AppendDownloadLog($"📋 ddv20.exe 退出，退出码：{DownloadProcess.ExitCode}");
        AppendDownloadLog(DownloadProcess.ExitCode == 0 ? "✅ 下载完成！" : $"⚠️  下载结束，退出码：{DownloadProcess.ExitCode}");

        // 下载完成后自动清理生成的 JSON 文件
        if (DownloadProcess.ExitCode == 0)
        {
            await Task.Run(() => CleanupDownloadJsonFiles());
        }
    }

    /// <summary>
    /// 清理下载完成后生成的 JSON 文件
    /// </summary>
    private void CleanupDownloadJsonFiles()
    {
        try
        {
            AppendDownloadLog("🔍 正在扫描临时 JSON 文件...");
            
            // 获取程序根目录下所有的 json 文件（排除 game.json 等配置文件）
            string[] jsonFiles = Directory.GetFiles(AppPath, "*.json");
            
            if (jsonFiles.Length == 0)
            {
                AppendDownloadLog("✅ 没有需要清理的临时 JSON 文件");
                return;
            }

            // 过滤出以数字开头的 JSON 文件（ddv20 生成的临时文件）
            List<string> filesToDelete = new List<string>();
            foreach (string file in jsonFiles)
            {
                string fileName = Path.GetFileName(file);
                // ddv20 生成的文件通常以 "100% - 数字.json" 或 "数字%.json" 格式命名
                // 例如：100% - 2050651.json, 2109301.json
                if (System.Text.RegularExpressions.Regex.IsMatch(fileName, @"^100%\s*[-–]\s*\d+.*\.json$") ||
                    System.Text.RegularExpressions.Regex.IsMatch(fileName, @"^\d+%.*\.json$") ||
                    System.Text.RegularExpressions.Regex.IsMatch(fileName, @"^\d+\s*[-–]\s*\d+.*\.json$"))
                {
                    filesToDelete.Add(file);
                }
            }

            if (filesToDelete.Count == 0)
            {
                AppendDownloadLog("✅ 没有需要清理的临时 JSON 文件");
                return;
            }

            AppendDownloadLog($"🗑️  发现 {filesToDelete.Count} 个临时 JSON 文件，正在删除...");
            
            int deletedCount = 0;
            foreach (string file in filesToDelete)
            {
                try
                {
                    File.Delete(file);
                    deletedCount++;
                }
                catch (Exception ex)
                {
                    AppendDownloadLog($"⚠️  删除文件失败 {Path.GetFileName(file)}: {ex.Message}");
                }
            }

            AppendDownloadLog($"✅ 清理完成！已删除 {deletedCount}/{filesToDelete.Count} 个临时 JSON 文件");
        }
        catch (Exception ex)
        {
            AppendDownloadLog($"❌ 清理 JSON 文件时出错：{ex.Message}");
        }
    }

    /// <summary>
    /// 更新进度条
    /// </summary>
    /// <param name="line">ddv20.exe输出的一行文本</param>
    private void UpdateProgressBar(string line)
    {
        line = line.Trim();
        if (string.IsNullOrEmpty(line)) return;

        if (line.Contains("%"))
        {
            try
            {
                int percentIndex = line.LastIndexOf("%");
                string beforePercent = line.Substring(0, percentIndex);
                string[] parts = beforePercent.Split(' ', StringSplitOptions.RemoveEmptyEntries);
                if (parts.Length > 0)
                {
                    string lastPart = parts[parts.Length - 1].Replace(",", ".").Replace("。", ".");
                    if (double.TryParse(lastPart, out double percent) && percent >= 0 && percent <= 100)
                    {
                        AppendDownloadLog($"下载进度：{percent:F1}%");
                        return;
                    }
                }

                string percentStr = line.Substring(0, percentIndex);
                int lastSpaceIndex = percentStr.LastIndexOf(' ');
                if (lastSpaceIndex >= 0)
                {
                    string numStr = percentStr.Substring(lastSpaceIndex).Trim().Replace(",", ".").Replace("。", ".");
                    if (double.TryParse(numStr, out double percent2) && percent2 >= 0 && percent2 <= 100)
                    {
                        AppendDownloadLog($"下载进度：{percent2:F1}%");
                    }
                }
            }
            catch { }
        }

        if (line.Contains("Downloading") || line.Contains("download") || line.Contains("Progress"))
        {
            var match = System.Text.RegularExpressions.Regex.Match(line, @"(\d+(?:\.\d+)?)\s*%");
            if (match.Success && double.TryParse(match.Groups[1].Value, out double p))
            {
                AppendDownloadLog($"下载进度：{p:F1}%");
            }
        }
    }

    /// <summary>
    /// 开始下载按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void StartDownload_Click(object sender, RoutedEventArgs e)
    {
        if (!File.Exists(TxtDdExePath.Text))
        { MessageBox.Show("请先选择ddv20.exe", "错误", MessageBoxButton.OK, MessageBoxImage.Error); return; }

        if (!Directory.Exists(TxtGameConfigDir.Text))
        { MessageBox.Show("游戏配置文件夹不存在", "错误", MessageBoxButton.OK, MessageBoxImage.Error); return; }

        try { Directory.CreateDirectory(TxtSaveDir.Text); }
        catch { MessageBox.Show("保存目录无写入权限", "错误", MessageBoxButton.OK, MessageBoxImage.Error); return; }

        IsPaused = false;
        IsDownloading = true;

        if (IsSingleFolderMode)
            StartSingleDownload();
        else
            MessageBox.Show("批量下载请先在列表中选择游戏", "提示", MessageBoxButton.OK, MessageBoxImage.Information);
    }

    /// <summary>
    /// 开始单游戏下载
    /// </summary>
    private void StartSingleDownload()
    {
        string configDir = TxtGameConfigDir.Text;
        string saveDir = TxtSaveDir.Text;
        string appId = GameAppId ?? "";
        string gameName = GameName ?? "未知游戏";
        string ddv20Path = TxtDdExePath.Text;

        BtnStartDownload.IsEnabled = false;
        BtnStartDownload.Content = "正在下载中...";
        BtnPauseDownload.IsEnabled = true;
        AppendDownloadLog("🚀 开始下载（单文件夹模式）...");
        AppendDownloadLog($"📋 开始执行下载，配置目录={configDir}");

        Task.Run(async () =>
        {
            bool success = await DownloadSingleGame(configDir, saveDir, appId, gameName, ddv20Path);

            Dispatcher.BeginInvoke(() =>
            {
                AppendDownloadLog(new string('-', 50));

                if (success)
                {
                    AppendDownloadLog($"✅ 下载完成！保存路径：{saveDir}");
                    MessageBox.Show($"下载完成！\n保存路径：{saveDir}", "完成", MessageBoxButton.OK, MessageBoxImage.Information);
                }
                else
                {
                    AppendDownloadLog($"⚠️  下载结束，退出码：{DownloadProcess?.ExitCode}");
                    MessageBox.Show($"下载结束，退出码：{DownloadProcess?.ExitCode}", "提示", MessageBoxButton.OK, MessageBoxImage.Warning);
                }

                IsDownloading = false;
                IsPaused = false;
                DownloadProcess = null;
                BtnStartDownload.IsEnabled = true;
                BtnStartDownload.Content = "▶ 开始下载";
                BtnPauseDownload.IsEnabled = false;
                BtnPauseDownload.Content = "⏸ 暂停下载";
            });
        });
    }

    /// <summary>
    /// 暂停/恢复下载按钮点击事件
    /// 第一次点击：暂停（挂起进程）
    /// 第二次点击：终止进程
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void PauseResumeDownload_Click(object sender, RoutedEventArgs e)
    {
        // 如果没有下载进程，提示用户
        if (DownloadProcess == null || DownloadProcess.HasExited)
        {
            MessageBox.Show("当前没有正在运行的下载任务", "提示", MessageBoxButton.OK, MessageBoxImage.Warning);
            return;
        }

        try
        {
            if (!IsPaused)
            {
                // 第一次点击：尝试挂起进程的所有线程
                // 注意：挂起可能对某些进程不起作用（如ddv20这种控制台程序）
                bool anySuspended = false;
                foreach (ProcessThread thread in DownloadProcess.Threads)
                {
                    try
                    {
                        IntPtr handle = OpenThread(0x0002, false, (uint)thread.Id);
                        if (handle != IntPtr.Zero)
                        {
                            int result = NtSuspendThread(handle, IntPtr.Zero);
                            CloseHandle(handle);
                            if (result == 0) anySuspended = true;
                        }
                    }
                    catch { }
                }

                if (anySuspended)
                {
                    IsPaused = true;
                    BtnPauseDownload.Content = "■ 停止下载";
                    AppendDownloadLog("⚠️  下载已暂停（点击[停止下载]可终止）");
                }
                else
                {
                    // 如果挂起失败，直接终止进程
                    DownloadProcess.Kill();
                    IsDownloading = false;
                    AppendDownloadLog("⚠️  下载已终止");
                }
            }
            else
            {
                // 第二次点击：终止进程
                IsDownloading = false;  // 这会让DownloadSingleGame中的循环检测到并退出
                IsPaused = false;

                // 确保进程被终止
                if (!DownloadProcess.HasExited)
                {
                    try
                    {
                        DownloadProcess.Kill();
                        AppendDownloadLog("⚠️  下载已终止");
                    }
                    catch { }
                }

                // 等待进程完全退出
                try
                {
                    DownloadProcess.WaitForExit(5000);  // 等待最多5秒
                }
                catch { }

                Dispatcher.BeginInvoke(() =>
                {
                    BtnStartDownload.IsEnabled = true;
                    BtnStartDownload.Content = "▶ 开始下载";
                    BtnPauseDownload.IsEnabled = false;
                    BtnPauseDownload.Content = "⏸ 暂停下载";
                    AppendDownloadLog("⚠️  下载已被用户停止");
                });
            }
        }
        catch (Exception ex)
        {
            AppendDownloadLog($"❌ 操作失败：{ex.Message}");
            MessageBox.Show($"操作失败：{ex.Message}", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
        }
    }

    /// <summary>
    /// 挂起指定进程的所有线程（暂停）
    /// </summary>
    /// <param name="pid">进程ID</param>
    private void SuspendProcess(int pid)
    {
        IntPtr handle = OpenThread(0x0002, false, (uint)pid);
        if (handle != IntPtr.Zero) { NtSuspendThread(handle, IntPtr.Zero); CloseHandle(handle); }
    }

    /// <summary>
    /// 恢复指定进程的所有线程（继续）
    /// </summary>
    /// <param name="pid">进程ID</param>
    private void ResumeProcess(int pid)
    {
        IntPtr handle = OpenThread(0x0002, false, (uint)pid);
        if (handle != IntPtr.Zero) { NtResumeThread(handle, IntPtr.Zero); CloseHandle(handle); }
    }

    // Windows API：打开线程
    [System.Runtime.InteropServices.DllImport("kernel32.dll")]
    private static extern IntPtr OpenThread(uint dwDesiredAccess, bool bInheritHandle, uint dwThreadId);

    // Windows API：挂起线程
    [System.Runtime.InteropServices.DllImport("ntdll.dll")]
    private static extern int NtSuspendThread(IntPtr hThread, IntPtr PreviousSuspendCount);

    // Windows API：恢复线程
    [System.Runtime.InteropServices.DllImport("ntdll.dll")]
    private static extern int NtResumeThread(IntPtr hThread, IntPtr PreviousSuspendCount);

    // Windows API：关闭句柄
    [System.Runtime.InteropServices.DllImport("kernel32.dll")]
    private static extern bool CloseHandle(IntPtr hObject);

    /// <summary>
    /// 显示补丁类别按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void ShowPatchCategory_Click(object sender, RoutedEventArgs e)
    {
        if (sender is Button btn && btn.Tag is string category) ShowPatchCategory(category);
    }

    /// <summary>
    /// 显示指定的补丁类别
    /// </summary>
    /// <param name="category">类别名称</param>
    private void ShowPatchCategory(string category)
    {
        PatchBasicPanel.Visibility = Visibility.Collapsed;
        PatchSavePanel.Visibility = Visibility.Collapsed;
        PatchNetworkPanel.Visibility = Visibility.Collapsed;
        PatchAchievementPanel.Visibility = Visibility.Collapsed;
        PatchAdvancedPanel.Visibility = Visibility.Collapsed;

        // 重置所有按钮为默认样式（暗色背景，浅色文字，无边框）
        foreach (var kvp in PatchCategoryBtns)
        {
            kvp.Value.Background = GetCachedColor("#FF455A64");
            kvp.Value.Foreground = GetCachedColor("#FFB4BED6");
        }

        // 选中按钮设置为高亮样式（亮色背景，白色文字，金色边框）
        var selectedBtn = PatchCategoryBtns[category];
        selectedBtn.Background = GetCachedColor("#FF0099F2");
        selectedBtn.Foreground = GetCachedColor("#FFFFFFFF");
        
        // 显示对应的面板
        switch (category)
        {
            case "basic": PatchBasicPanel.Visibility = Visibility.Visible; break;
            case "save": PatchSavePanel.Visibility = Visibility.Visible; break;
            case "network": PatchNetworkPanel.Visibility = Visibility.Visible; break;
            case "achievement": PatchAchievementPanel.Visibility = Visibility.Visible; break;
            case "advanced": PatchAdvancedPanel.Visibility = Visibility.Visible; break;
        }
        
        AppendPatchLog($"📋 已切换到 {category} 配置页面");
    }

    /// <summary>
    /// 更新 Overlay 控件状态
    /// </summary>
    /// <param name="enable">是否启用</param>
    private void UpdateOverlayControls(bool enable)
    {
        TxtAccountName.IsEnabled = enable;
        TxtSteamId.IsEnabled = enable;
        ChkAchievementSound.IsEnabled = enable;
        ChkInviteSound.IsEnabled = enable;
    }

    /// <summary>
    /// 一键配置存档功能
    /// </summary>
    private void OneClickSaveConfig_Click(object sender, RoutedEventArgs e)
    {
        try
        {
            // 根据选择的存档模式生成配置文件
            string saveMode = "";
            if (RbDefaultSave.IsChecked == true)
                saveMode = "default";
            else if (RbPortableSave.IsChecked == true)
                saveMode = "portable";
            else if (RbCustomSave.IsChecked == true)
                saveMode = "custom";

            // 生成 configs.user.ini 配置
            StringBuilder config = new StringBuilder();
            config.AppendLine("[config]");
            
            if (saveMode == "portable")
            {
                config.AppendLine("local_save_path = ./saves");
                AppendPatchLog("✅ 已配置便携存档模式");
            }
            else if (saveMode == "custom" && !string.IsNullOrEmpty(TxtCustomSavePath.Text))
            {
                config.AppendLine($"local_save_path = {TxtCustomSavePath.Text}");
                AppendPatchLog($"✅ 已配置自定义存档路径：{TxtCustomSavePath.Text}");
            }
            else
            {
                AppendPatchLog("✅ 已配置默认存档模式");
            }

            if (ChkCustomSavesName.IsChecked == true && !string.IsNullOrEmpty(TxtSavesFolderName.Text))
            {
                config.AppendLine($"saves_folder_name = {TxtSavesFolderName.Text}");
                AppendPatchLog($"✅ 已配置存档文件夹名称：{TxtSavesFolderName.Text}");
            }

            // 保存配置文件
            SaveConfigFile("configs.user.ini", config.ToString());
            
            MessageBox.Show("存档配置已生成！\n配置文件将随补丁一起注入到游戏目录。", "配置完成", MessageBoxButton.OK, MessageBoxImage.Information);
            AppendPatchLog("✅ 存档配置完成");
        }
        catch (Exception ex)
        {
            MessageBox.Show($"配置失败：{ex.Message}", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
            AppendPatchLog($"❌ 存档配置失败：{ex.Message}");
        }
    }

    /// <summary>
    /// 一键配置局域网联机功能
    /// </summary>
    private void OneClickNetworkConfig_Click(object sender, RoutedEventArgs e)
    {
        try
        {
            StringBuilder mainConfig = new StringBuilder();
            mainConfig.AppendLine("[config]");
            
            // 强制局域网模式
            if (ChkForceLan.IsChecked == true)
            {
                mainConfig.AppendLine("force_lan = 1");
                AppendPatchLog("✅ 已启用强制局域网模式");
            }
            
            // 低延迟模式
            if (ChkLowLatency.IsChecked == true)
            {
                mainConfig.AppendLine("low_latency_lan = 1");
                AppendPatchLog("✅ 已启用低延迟联机模式");
            }

            // 保存主配置
            SaveConfigFile("configs.main.ini", mainConfig.ToString());

            // 生成自定义广播网段
            if (!string.IsNullOrEmpty(TxtBroadcastIps.Text))
            {
                File.WriteAllText(Path.Combine(AppPath, "Resources", "gbe_fork", "custom_broadcasts.txt"), TxtBroadcastIps.Text);
                AppendPatchLog("✅ 已配置自定义广播网段");
            }

            // 生成订阅组配置
            if (!string.IsNullOrEmpty(TxtSubscribedGroups.Text))
            {
                File.WriteAllText(Path.Combine(AppPath, "Resources", "gbe_fork", "subscribed_groups.txt"), TxtSubscribedGroups.Text);
                AppendPatchLog("✅ 已配置订阅组");
            }

            // 生成订阅组 Clans 配置
            if (!string.IsNullOrEmpty(TxtGroupsClans.Text))
            {
                File.WriteAllText(Path.Combine(AppPath, "Resources", "gbe_fork", "subscribed_groups_clans.txt"), TxtGroupsClans.Text);
                AppendPatchLog("✅ 已配置订阅组 Clans");
            }

            // 生成自动接受邀请配置
            if (ChkAutoAcceptInvite.IsChecked == true)
            {
                File.WriteAllText(Path.Combine(AppPath, "Resources", "gbe_fork", "auto_accept_invite.txt"), "");
                AppendPatchLog("✅ 已启用自动接受联机邀请");
            }

            MessageBox.Show("局域网联机配置已生成！\n请确保所有玩家：\n1. 同一游戏版本\n2. 同网络环境\n3. 关闭防火墙", "配置完成", MessageBoxButton.OK, MessageBoxImage.Information);
            AppendPatchLog("✅ 局域网联机配置完成");
        }
        catch (Exception ex)
        {
            MessageBox.Show($"配置失败：{ex.Message}", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
            AppendPatchLog($"❌ 局域网联机配置失败：{ex.Message}");
        }
    }

    /// <summary>
    /// 一键配置成就统计功能
    /// </summary>
    private void OneClickAchievementConfig_Click(object sender, RoutedEventArgs e)
    {
        try
        {
            // 生成成就配置文件
            if (ChkImportAchievements.IsChecked == true && !string.IsNullOrEmpty(TxtAchievementFile.Text) && File.Exists(TxtAchievementFile.Text))
            {
                File.Copy(TxtAchievementFile.Text, Path.Combine(AppPath, "Resources", "gbe_fork", "achievements.json"), true);
                AppendPatchLog($"✅ 已导入自定义成就文件：{TxtAchievementFile.Text}");
            }

            // 生成统计配置文件
            if (ChkImportStats.IsChecked == true && !string.IsNullOrEmpty(TxtStatsFile.Text) && File.Exists(TxtStatsFile.Text))
            {
                File.Copy(TxtStatsFile.Text, Path.Combine(AppPath, "Resources", "gbe_fork", "stats.json"), true);
                AppendPatchLog($"✅ 已导入自定义统计文件：{TxtStatsFile.Text}");
            }

            // 生成排行榜配置
            if (!string.IsNullOrEmpty(TxtLeaderboards.Text))
            {
                File.WriteAllText(Path.Combine(AppPath, "Resources", "gbe_fork", "leaderboards.txt"), TxtLeaderboards.Text);
                AppendPatchLog("✅ 已配置排行榜");
            }

            MessageBox.Show("成就统计配置已生成！\n配置文件将随补丁一起注入到游戏目录。", "配置完成", MessageBoxButton.OK, MessageBoxImage.Information);
            AppendPatchLog("✅ 成就统计配置完成");
        }
        catch (Exception ex)
        {
            MessageBox.Show($"配置失败：{ex.Message}", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
            AppendPatchLog($"❌ 成就统计配置失败：{ex.Message}");
        }
    }

    /// <summary>
    /// 一键配置高级功能
    /// </summary>
    private void OneClickAdvancedConfig_Click(object sender, RoutedEventArgs e)
    {
        try
        {
            StringBuilder mainConfig = new StringBuilder();
            mainConfig.AppendLine("[config]");
            
            // SteamHTTP 模拟
            if (ChkEnableHttp.IsChecked == true && !string.IsNullOrEmpty(TxtHttpFolder.Text))
            {
                mainConfig.AppendLine("http_folder = " + TxtHttpFolder.Text);
                AppendPatchLog($"✅ 已配置 SteamHTTP 模拟目录：{TxtHttpFolder.Text}");
            }

            // 头像配置
            if (ChkEnableAvatar.IsChecked == true && !string.IsNullOrEmpty(TxtAvatarPath.Text) && File.Exists(TxtAvatarPath.Text))
            {
                string avatarDest = Path.Combine(AppPath, "Resources", "gbe_fork", "account_avatar.png");
                File.Copy(TxtAvatarPath.Text, avatarDest, true);
                AppendPatchLog($"✅ 已配置自定义头像：{TxtAvatarPath.Text}");
            }

            // 语言配置
            if (ChkEnableLanguage.IsChecked == true && !string.IsNullOrEmpty(TxtLanguages.Text))
            {
                File.WriteAllText(Path.Combine(AppPath, "Resources", "gbe_fork", "supported_languages.txt"), TxtLanguages.Text);
                AppendPatchLog("✅ 已配置支持语言列表");
            }

            // Mods 配置
            if (ChkEnableMods.IsChecked == true && !string.IsNullOrEmpty(TxtModsFolder.Text))
            {
                mainConfig.AppendLine("mods_folder = " + TxtModsFolder.Text);
                AppendPatchLog($"✅ 已配置 Mods 目录：{TxtModsFolder.Text}");
            }

            // 手柄配置
            if (ChkEnableController.IsChecked == true)
            {
                mainConfig.AppendLine("enable_controller = 1");
                AppendPatchLog("✅ 已启用 XInput 手柄支持");
            }

            // 授权 Token 配置
            if (ChkEnableTicket.IsChecked == true && !string.IsNullOrEmpty(TxtTicket.Text))
            {
                mainConfig.AppendLine($"ticket = {TxtTicket.Text}");
                AppendPatchLog("✅ 已配置 EncryptedAppTicket");
            }

            // 保存主配置
            SaveConfigFile("configs.main.ini", mainConfig.ToString());

            MessageBox.Show("高级功能配置已生成！\n请谨慎使用高级功能，建议先备份原文件。", "配置完成", MessageBoxButton.OK, MessageBoxImage.Information);
            AppendPatchLog("✅ 高级功能配置完成");
        }
        catch (Exception ex)
        {
            MessageBox.Show($"配置失败：{ex.Message}", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
            AppendPatchLog($"❌ 高级功能配置失败：{ex.Message}");
        }
    }

    /// <summary>
    /// 保存配置文件到临时目录
    /// </summary>
    /// <param name="filename">文件名</param>
    /// <param name="content">配置内容</param>
    private void SaveConfigFile(string filename, string content)
    {
        string configPath = Path.Combine(AppPath, "Resources", "gbe_fork", filename);
        File.WriteAllText(configPath, content);
        AppendPatchLog($"✅ 已生成配置文件：{filename}");
    }

    /// <summary>
    /// 打开SteamDB按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void OpenSteamDB_Click(object sender, RoutedEventArgs e)
    {
        Process.Start(new ProcessStartInfo { FileName = "https://steamdb.info/", UseShellExecute = true });
    }

    /// <summary>
    /// 选择游戏目录按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void SelectGameDir_Click(object sender, RoutedEventArgs e)
    {
        var dialog = new OpenFolderDialog { Title = "选择游戏安装目录" };
        if (dialog.ShowDialog() == true) { TxtGameDir.Text = dialog.FolderName; AppendPatchLog($"已选择游戏目录：{dialog.FolderName}"); }
    }

    /// <summary>
    /// 选择游戏主程序按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void SelectGameExe_Click(object sender, RoutedEventArgs e)
    {
        var dialog = new OpenFileDialog { Title = "选择游戏主程序", Filter = "Windows可执行程序|*.exe" };
        if (dialog.ShowDialog() == true) { TxtGameExePath.Text = dialog.FileName; AppendPatchLog($"已选择游戏主程序：{dialog.FileName}"); }
    }

    /// <summary>
    /// 选择自定义存档路径按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void SelectCustomSavePath_Click(object sender, RoutedEventArgs e)
    {
        var dialog = new OpenFolderDialog { Title = "选择自定义存档文件夹" };
        if (dialog.ShowDialog() == true) { TxtCustomSavePath.Text = dialog.FolderName; AppendPatchLog($"已选择自定义存档路径：{dialog.FolderName}"); }
    }

    /// <summary>
    /// 选择成就文件按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void SelectAchievementFile_Click(object sender, RoutedEventArgs e)
    {
        var dialog = new OpenFileDialog { Title = "选择成就文件", Filter = "JSON文件|*.json|所有文件|*.*" };
        if (dialog.ShowDialog() == true) { TxtAchievementFile.Text = dialog.FileName; AppendPatchLog($"已选择成就文件：{dialog.FileName}"); }
    }

    /// <summary>
    /// 选择统计文件按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void SelectStatsFile_Click(object sender, RoutedEventArgs e)
    {
        var dialog = new OpenFileDialog { Title = "选择统计文件", Filter = "JSON文件|*.json|所有文件|*.*" };
        if (dialog.ShowDialog() == true) { TxtStatsFile.Text = dialog.FileName; AppendPatchLog($"已选择统计文件：{dialog.FileName}"); }
    }

    /// <summary>
    /// 选择HTTP模拟目录按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void SelectHttpFolder_Click(object sender, RoutedEventArgs e)
    {
        var dialog = new OpenFolderDialog { Title = "选择HTTP模拟目录" };
        if (dialog.ShowDialog() == true) { TxtHttpFolder.Text = dialog.FolderName; AppendPatchLog($"已选择HTTP模拟目录：{dialog.FolderName}"); }
    }

    /// <summary>
    /// 选择头像文件按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void SelectAvatarFile_Click(object sender, RoutedEventArgs e)
    {
        var dialog = new OpenFileDialog { Title = "选择头像文件", Filter = "图片文件|*.png;*.jpg|所有文件|*.*" };
        if (dialog.ShowDialog() == true) { TxtAvatarPath.Text = dialog.FileName; AppendPatchLog($"已选择头像文件：{dialog.FileName}"); }
    }

    /// <summary>
    /// 选择Mods目录按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void SelectModsFolder_Click(object sender, RoutedEventArgs e)
    {
        var dialog = new OpenFolderDialog { Title = "选择Mods目录" };
        if (dialog.ShowDialog() == true) { TxtModsFolder.Text = dialog.FolderName; AppendPatchLog($"已选择Mods目录：{dialog.FolderName}"); }
    }

    /// <summary>
    /// 添加成就按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void AddAchievement_Click(object sender, RoutedEventArgs e)
    {
        string achId = TxtAchievementId.Text.Trim();
        string achName = TxtAchievementName.Text.Trim();

        if (string.IsNullOrEmpty(achId) || string.IsNullOrEmpty(achName))
        { MessageBox.Show("成就ID和名称为必填项！", "错误", MessageBoxButton.OK, MessageBoxImage.Error); return; }

        var item = new AchievementItem { Id = achId, Name = achName, Description = TxtAchievementDesc.Text.Trim() ?? "无描述", Hidden = ChkAchievementHidden.IsChecked == true };
        AchievementsList.Add(item);
        LstAchievements.Items.Add($"ID: {achId} | 名称: {achName} | 描述: {item.Description} | 隐藏: {(item.Hidden ? "是" : "否")}");

        TxtAchievementId.Text = "";
        TxtAchievementName.Text = "";
        TxtAchievementDesc.Text = "";
        ChkAchievementHidden.IsChecked = false;

        AppendPatchLog($"✅ 已添加成就：ID={achId}，名称={achName}");
    }

    /// <summary>
    /// 删除成就按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void DeleteAchievement_Click(object sender, RoutedEventArgs e)
    {
        if (LstAchievements.SelectedIndex < 0)
        { MessageBox.Show("请先选中要删除的成就！", "提示", MessageBoxButton.OK, MessageBoxImage.Warning); return; }

        var deletedItem = AchievementsList[LstAchievements.SelectedIndex];
        AchievementsList.RemoveAt(LstAchievements.SelectedIndex);
        LstAchievements.Items.RemoveAt(LstAchievements.SelectedIndex);
        AppendPatchLog($"❌ 已删除成就：ID={deletedItem.Id}");
    }

    /// <summary>
    /// 清空成就按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void ClearAchievements_Click(object sender, RoutedEventArgs e)
    {
        if (AchievementsList.Count == 0)
        { MessageBox.Show("成就列表已为空！", "提示", MessageBoxButton.OK, MessageBoxImage.Information); return; }

        if (MessageBox.Show("是否清空所有成就？", "确认", MessageBoxButton.YesNo, MessageBoxImage.Question) == MessageBoxResult.Yes)
        { AchievementsList.Clear(); LstAchievements.Items.Clear(); AppendPatchLog("🧹 已清空所有成就列表"); }
    }

    /// <summary>
    /// 生成空成就文件按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void GenerateEmptyAchievement_Click(object sender, RoutedEventArgs e)
    {
        try
        {
            string gameDir = TxtGameDir.Text.Trim();
            if (string.IsNullOrEmpty(gameDir))
            { MessageBox.Show("请先选择游戏目录！", "错误", MessageBoxButton.OK, MessageBoxImage.Error); return; }

            if (!Directory.Exists(gameDir))
            { MessageBox.Show("游戏目录不存在！", "错误", MessageBoxButton.OK, MessageBoxImage.Error); return; }

            var achievementContent = new { format_version = 2, achievements = new List<object>(), stats = new List<object>() };
            string savePath = Path.Combine(gameDir, "achievements.json");
            File.WriteAllText(savePath, JsonConvert.SerializeObject(achievementContent, Formatting.Indented), Encoding.UTF8);

            TxtAchievementFile.Text = savePath;
            ChkImportAchievements.IsChecked = true;

            AppendPatchLog($"✅ 已生成空成就文件：{savePath}");
            MessageBox.Show($"空成就文件已生成！\n路径：{savePath}", "成功", MessageBoxButton.OK, MessageBoxImage.Information);
        }
        catch (Exception ex)
        {
            AppendPatchLog($"❌ 生成失败：{ex.Message}");
            MessageBox.Show($"生成失败：{ex.Message}", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
        }
    }

    /// <summary>
    /// 生成多成就文件按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void GenerateMultiAchievement_Click(object sender, RoutedEventArgs e)
    {
        try
        {
            string gameDir = TxtGameDir.Text.Trim();
            if (string.IsNullOrEmpty(gameDir))
            { MessageBox.Show("请先选择游戏目录！", "错误", MessageBoxButton.OK, MessageBoxImage.Error); return; }

            if (!Directory.Exists(gameDir))
            { MessageBox.Show("游戏目录不存在！", "错误", MessageBoxButton.OK, MessageBoxImage.Error); return; }

            if (AchievementsList.Count == 0)
            { MessageBox.Show("成就列表为空！请先添加成就", "提示", MessageBoxButton.OK, MessageBoxImage.Warning); return; }

            var achievementContent = new Dictionary<string, object>
            {
                { "format_version", 2 },
                { "achievements", AchievementsList.Select(a => new { apiname = a.Id, name = a.Name, description = a.Description, hidden = a.Hidden, icon = "", icon_gray = "" }).ToList() },
                { "stats", new List<object>() }
            };

            string savePath = Path.Combine(gameDir, "achievements.json");
            File.WriteAllText(savePath, JsonConvert.SerializeObject(achievementContent, Formatting.Indented), Encoding.UTF8);

            TxtAchievementFile.Text = savePath;
            ChkImportAchievements.IsChecked = true;

            AppendPatchLog($"✅ 已生成包含{AchievementsList.Count}个成就的文件：{savePath}");
            MessageBox.Show($"成就文件已生成！\n路径：{savePath}\n包含{AchievementsList.Count}个成就", "成功", MessageBoxButton.OK, MessageBoxImage.Information);
        }
        catch (Exception ex)
        {
            AppendPatchLog($"❌ 生成失败：{ex.Message}");
            MessageBox.Show($"生成失败：{ex.Message}", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
        }
    }

    /// <summary>
    /// 清空补丁日志按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void ClearPatchLog_Click(object sender, RoutedEventArgs e) => TxtPatchLog.Text = "";

    /// <summary>
    /// 一键完成全部配置按钮点击事件
    /// 根据Goldberg Emulator文档完成所有配置
    /// 1. 复制steam_api.dll或steamclient.dll到游戏目录
    /// 2. 创建steam_settings文件夹
    /// 3. 生成各种配置文件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private async void RunFullConfig_Click(object sender, RoutedEventArgs e)
    {
        try
        {
            string gameDir = TxtGameDir.Text.Trim();
            string appid = TxtAppId.Text.Trim();

            if (string.IsNullOrEmpty(gameDir))
            { MessageBox.Show("请选择游戏目录！", "错误", MessageBoxButton.OK, MessageBoxImage.Error); return; }

            if (string.IsNullOrEmpty(appid))
            { MessageBox.Show("请填写游戏AppID！", "错误", MessageBoxButton.OK, MessageBoxImage.Error); return; }

            if (!Directory.Exists(gameDir))
            { MessageBox.Show("游戏目录不存在！", "错误", MessageBoxButton.OK, MessageBoxImage.Error); return; }

            // 使用 Resources/gbe_fork 路径
            string gbeForkBasePath = Path.Combine(AppPath, "Resources", "gbe_fork");
            if (!Directory.Exists(gbeForkBasePath))
            {
                AppendPatchLog($"❌ 未找到 Resources/gbe_fork 目录：{gbeForkBasePath}");
                MessageBox.Show($"未找到 Resources/gbe_fork 目录！\n请确保程序根目录下有 Resources/gbe_fork 文件夹", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
                return;
            }

            AppendPatchLog("🚀 开始一键配置...");
            AppendPatchLog($"📂 游戏目录：{gameDir}");
            AppendPatchLog($"📂 gbe_fork 基础目录：{gbeForkBasePath}");

            bool isExperimentalMode = RbExperimentalMode.IsChecked == true;
            
            // 根据模式选择不同的 DLL 路径
            string dllName;
            string srcDllPath;
            
            if (isExperimentalMode)
            {
                // 实验版模式：使用 steamclient_experimental 目录
                dllName = "steamclient.dll";
                srcDllPath = Path.Combine(gbeForkBasePath, "steamclient_experimental", dllName);
                AppendPatchLog($"📌 使用实验版模式：steamclient_experimental/steamclient.dll");
            }
            else
            {
                // 常规模式：使用 experimental 目录（根据系统架构选择 x32 或 x64）
                dllName = "steam_api.dll";
                string experimentalPath = Path.Combine(gbeForkBasePath, "experimental");
                
                // 判断游戏是 32 位还是 64 位（优先检查 64 位 DLL）
                if (File.Exists(Path.Combine(experimentalPath, "x64", "steam_api64.dll")))
                {
                    srcDllPath = Path.Combine(experimentalPath, "x64", "steam_api64.dll");
                    dllName = "steam_api64.dll";
                    AppendPatchLog($"📌 使用常规模式（64 位）：experimental/x64/steam_api64.dll");
                }
                else if (File.Exists(Path.Combine(experimentalPath, "x32", "steam_api.dll")))
                {
                    srcDllPath = Path.Combine(experimentalPath, "x32", dllName);
                    AppendPatchLog($"📌 使用常规模式（32 位）：experimental/x32/steam_api.dll");
                }
                else
                {
                    AppendPatchLog($"❌ 未找到常规模式的 DLL 文件");
                    MessageBox.Show($"未找到 experimental 目录下的 DLL 文件！\n请检查 Resources/gbe_fork/experimental/x32 和 x64 文件夹", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
                    return;
                }
            }

            // 检查目标 DLL 是否存在并备份
            string destDllPath = Path.Combine(gameDir, dllName);
            string? originalDllBackupPath = null;
            
            if (File.Exists(destDllPath))
            {
                string backupPath = destDllPath + ".backup";
                File.Copy(destDllPath, backupPath, true);
                originalDllBackupPath = backupPath;
                AppendPatchLog($"📦 已备份原始 DLL：{dllName}.backup");
            }
            else
            {
                AppendPatchLog($"ℹ️ 未找到原始 DLL，可能是新游戏或未安装");
            }

            // 重要：在复制 Goldberg DLL 之前，先生成 steam_interfaces.txt（需要原始 DLL）
            if (!string.IsNullOrEmpty(originalDllBackupPath))
            {
                AppendPatchLog("🔧 正在生成 steam_interfaces.txt...");
                await GenerateSteamInterfacesFileFromBackup(gameDir, originalDllBackupPath, dllName);
            }
            else
            {
                AppendPatchLog("⚠️ 跳过 steam_interfaces.txt 生成（无原始 DLL）");
            }

            File.Copy(srcDllPath, destDllPath, true);
            AppendPatchLog($"✅ 已复制{dllName}到游戏目录");

            string steamSettingsDir = Path.Combine(gameDir, "steam_settings");
            Directory.CreateDirectory(steamSettingsDir);
            AppendPatchLog($"📁 已创建steam_settings目录");

            string appidFile = Path.Combine(steamSettingsDir, "steam_appid.txt");
            File.WriteAllText(appidFile, appid, Encoding.UTF8);
            AppendPatchLog($"✅ 已创建steam_appid.txt (AppID: {appid})");

            string savesFolderName = "GSE Saves";
            
            if (RbPortableSave.IsChecked == true)
            {
                string portableSaveDir = Path.Combine(gameDir, "Saves");
                Directory.CreateDirectory(portableSaveDir);
                
                string localSaveConfig = Path.Combine(steamSettingsDir, "configs.user.ini");
                StringBuilder saveConfig = new StringBuilder();
                saveConfig.AppendLine("[user_settings]");
                saveConfig.AppendLine($"local_save_path={portableSaveDir}");
                File.WriteAllText(localSaveConfig, saveConfig.ToString(), Encoding.UTF8);
                AppendPatchLog($"✅ 已配置便携存档：{portableSaveDir}");
            }
            else if (RbCustomSave.IsChecked == true && !string.IsNullOrEmpty(TxtCustomSavePath.Text))
            {
                string customSavePath = TxtCustomSavePath.Text.Trim();
                Directory.CreateDirectory(customSavePath);
                
                string localSaveConfig = Path.Combine(steamSettingsDir, "configs.user.ini");
                StringBuilder saveConfig = new StringBuilder();
                saveConfig.AppendLine("[user_settings]");
                saveConfig.AppendLine($"local_save_path={customSavePath}");
                File.WriteAllText(localSaveConfig, saveConfig.ToString(), Encoding.UTF8);
                AppendPatchLog($"✅ 已配置自定义存档：{customSavePath}");
            }

            if (ChkCustomSavesName.IsChecked == true && !string.IsNullOrEmpty(TxtSavesFolderName.Text))
            {
                savesFolderName = TxtSavesFolderName.Text.Trim();
                string localSaveConfig = Path.Combine(steamSettingsDir, "configs.user.ini");
                string existingConfig = File.Exists(localSaveConfig) ? File.ReadAllText(localSaveConfig, Encoding.UTF8) : "";
                if (!existingConfig.Contains("saves_folder_name"))
                {
                    File.AppendAllText(localSaveConfig, $"\nsaves_folder_name={savesFolderName}", Encoding.UTF8);
                }
                AppendPatchLog($"✅ 已配置存档文件夹名称：{savesFolderName}");
            }

            string networkConfigFile = Path.Combine(steamSettingsDir, "configs.main.ini");
            StringBuilder networkConfig = new StringBuilder();
            networkConfig.AppendLine("[network]");
            
            if (ChkForceLan.IsChecked == true)
            {
                networkConfig.AppendLine("force_lan=1");
                AppendPatchLog($"✅ 已启用强制局域网模式");
            }
            
            if (ChkLowLatency.IsChecked == true)
            {
                networkConfig.AppendLine("low_latency=1");
                AppendPatchLog($"✅ 已启用低延迟模式");
            }
            
            if (ChkAutoAcceptInvite.IsChecked == true)
            {
                string autoAcceptFile = Path.Combine(steamSettingsDir, "auto_accept_invite.txt");
                File.WriteAllText(autoAcceptFile, "", Encoding.UTF8);
                AppendPatchLog($"✅ 已启用自动接受邀请");
            }
            
            File.WriteAllText(networkConfigFile, networkConfig.ToString(), Encoding.UTF8);

            if (!string.IsNullOrEmpty(TxtBroadcastIps.Text))
            {
                string broadcastFile = Path.Combine(steamSettingsDir, "custom_broadcasts.txt");
                File.WriteAllText(broadcastFile, TxtBroadcastIps.Text, Encoding.UTF8);
                AppendPatchLog($"✅ 已配置自定义广播网段");
            }

            if (!string.IsNullOrEmpty(TxtSubscribedGroups.Text))
            {
                string groupsFile = Path.Combine(steamSettingsDir, "subscribed_groups.txt");
                File.WriteAllText(groupsFile, TxtSubscribedGroups.Text, Encoding.UTF8);
                AppendPatchLog($"✅ 已配置订阅组");
            }

            if (!string.IsNullOrEmpty(TxtGroupsClans.Text))
            {
                string clansFile = Path.Combine(steamSettingsDir, "subscribed_groups_clans.txt");
                File.WriteAllText(clansFile, TxtGroupsClans.Text, Encoding.UTF8);
                AppendPatchLog($"✅ 已配置Groups Clans");
            }

            if (!string.IsNullOrEmpty(TxtAppPaths.Text))
            {
                string appPathsFile = Path.Combine(steamSettingsDir, "app_paths.txt");
                File.WriteAllText(appPathsFile, TxtAppPaths.Text, Encoding.UTF8);
                AppendPatchLog($"✅ 已配置AppPaths");
            }

            if (ChkUnlockAllDlc.IsChecked == true)
            {
                string dlcfoldersFile = Path.Combine(steamSettingsDir, "dlcfolders.txt");
                if (!string.IsNullOrEmpty(TxtDlcFolder.Text))
                {
                    File.WriteAllText(dlcfoldersFile, TxtDlcFolder.Text, Encoding.UTF8);
                    AppendPatchLog($"✅ 已配置DLC文件夹：{TxtDlcFolder.Text}");
                }
            }

            if (ChkImportAchievements.IsChecked == true && !string.IsNullOrEmpty(TxtAchievementFile.Text))
            {
                if (File.Exists(TxtAchievementFile.Text))
                {
                    string destAchievementsFile = Path.Combine(steamSettingsDir, "achievements.json");
                    File.Copy(TxtAchievementFile.Text, destAchievementsFile, true);
                    AppendPatchLog($"✅ 已导入成就文件");
                }
            }

            if (ChkImportStats.IsChecked == true && !string.IsNullOrEmpty(TxtStatsFile.Text))
            {
                if (File.Exists(TxtStatsFile.Text))
                {
                    string destStatsFile = Path.Combine(steamSettingsDir, "stats.json");
                    File.Copy(TxtStatsFile.Text, destStatsFile, true);
                    AppendPatchLog($"✅ 已导入统计文件");
                }
            }

            if (!string.IsNullOrEmpty(TxtLeaderboards.Text))
            {
                string leaderboardsFile = Path.Combine(steamSettingsDir, "leaderboards.txt");
                File.WriteAllText(leaderboardsFile, TxtLeaderboards.Text, Encoding.UTF8);
                AppendPatchLog($"✅ 已配置排行榜");
            }

            if (ChkEnableOverlay.IsChecked == true)
            {
                string overlayConfigFile = Path.Combine(steamSettingsDir, "configs.overlay.ini");
                StringBuilder overlayConfig = new StringBuilder();
                overlayConfig.AppendLine("[overlay]");
                overlayConfig.AppendLine("enable_experimental_overlay=1");
                File.WriteAllText(overlayConfigFile, overlayConfig.ToString(), Encoding.UTF8);
                AppendPatchLog($"✅ 已启用Overlay覆盖层");
            }

            if (ChkEnableHttp.IsChecked == true && !string.IsNullOrEmpty(TxtHttpFolder.Text))
            {
                string httpSrcFolder = TxtHttpFolder.Text.Trim();
                if (Directory.Exists(httpSrcFolder))
                {
                    string httpDestFolder = Path.Combine(steamSettingsDir, "http");
                    CopyDirectory(httpSrcFolder, httpDestFolder);
                    AppendPatchLog($"✅ 已配置SteamHTTP模拟目录");
                }
            }

            string savesBaseDir = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData), savesFolderName);
            if (ChkEnableAvatar.IsChecked == true && !string.IsNullOrEmpty(TxtAvatarPath.Text))
            {
                if (File.Exists(TxtAvatarPath.Text))
                {
                    string avatarDestFile = Path.Combine(savesBaseDir, "settings", "account_avatar");
                    Directory.CreateDirectory(Path.GetDirectoryName(avatarDestFile)!);
                    File.Copy(TxtAvatarPath.Text, avatarDestFile, true);
                    AppendPatchLog($"✅ 已配置自定义头像");
                }
            }

            if (ChkEnableLanguage.IsChecked == true && !string.IsNullOrEmpty(TxtLanguages.Text))
            {
                string languagesFile = Path.Combine(steamSettingsDir, "supported_languages.txt");
                File.WriteAllText(languagesFile, TxtLanguages.Text, Encoding.UTF8);
                AppendPatchLog($"✅ 已配置支持语言列表");
            }

            if (ChkEnableMods.IsChecked == true && !string.IsNullOrEmpty(TxtModsFolder.Text))
            {
                string modsSrcFolder = TxtModsFolder.Text.Trim();
                if (Directory.Exists(modsSrcFolder))
                {
                    string modsDestFolder = Path.Combine(steamSettingsDir, "mods");
                    CopyDirectory(modsSrcFolder, modsDestFolder);
                    AppendPatchLog($"✅ 已配置Mods目录");
                }
            }

            if (ChkEnableController.IsChecked == true && !string.IsNullOrEmpty(TxtControllerConfig.Text))
            {
                string controllerConfigFile = Path.Combine(steamSettingsDir, "configs.controller.ini");
                File.WriteAllText(controllerConfigFile, TxtControllerConfig.Text, Encoding.UTF8);
                AppendPatchLog($"✅ 已配置手柄支持");
            }

            if (ChkEnableTicket.IsChecked == true && !string.IsNullOrEmpty(TxtTicket.Text))
            {
                string ticketFile = Path.Combine(steamSettingsDir, "configs.user.ini");
                string existingContent = File.Exists(ticketFile) ? File.ReadAllText(ticketFile, Encoding.UTF8) : "";
                if (!existingContent.Contains("ticket="))
                {
                    File.AppendAllText(ticketFile, $"\nticket={TxtTicket.Text}", Encoding.UTF8);
                    AppendPatchLog($"✅ 已配置授权Token");
                }
            }

            if (!string.IsNullOrEmpty(TxtAccountName.Text))
            {
                string settingsDir = Path.Combine(savesBaseDir, "settings");
                Directory.CreateDirectory(settingsDir);
                string settingsFile = Path.Combine(settingsDir, "settings.txt");
                StringBuilder settings = new StringBuilder();
                settings.AppendLine($"account_name={TxtAccountName.Text}");
                if (!string.IsNullOrEmpty(TxtSteamId.Text))
                {
                    settings.AppendLine($"steam_id={TxtSteamId.Text}");
                }
                File.WriteAllText(settingsFile, settings.ToString(), Encoding.UTF8);
                AppendPatchLog($"✅ 已配置账户信息");
            }

            AppendPatchLog("=" + new string('=', 40));
            AppendPatchLog("🎉 一键配置完成！");
            AppendPatchLog("=" + new string('=', 40));
            AppendPatchLog("💡 提示：现在可以直接运行游戏 exe 文件了");

            MessageBox.Show("✅ 一键配置完成！\n\n已生成 steam_interfaces.txt 文件\n请直接运行游戏 exe 文件进行测试。", "成功", MessageBoxButton.OK, MessageBoxImage.Information);
        }
        catch (Exception ex)
        {
            AppendPatchLog($"❌ 操作失败：{ex.Message}");
            MessageBox.Show($"操作失败：{ex.Message}", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
        }
    }

    /// <summary>
    /// 还原游戏按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private async void BtnRestoreGame_Click(object sender, RoutedEventArgs e)
    {
        if (CurrentSelectedGame == null || string.IsNullOrEmpty(TxtGamePath.Text))
        {
            MessageBox.Show("请先选择游戏安装路径！", "错误", MessageBoxButton.OK, MessageBoxImage.Warning);
            return;
        }
        
        string gamePath = TxtGamePath.Text;
        
        if (!Directory.Exists(gamePath))
        {
            MessageBox.Show("游戏路径不存在，请重新选择！", "错误", MessageBoxButton.OK, MessageBoxImage.Warning);
            return;
        }
        
        // 查找所有.bak 备份文件
        string[] bakFiles = Array.Empty<string>();
        
        // 首先检查统一的备份文件夹
        string backupFolder = Path.Combine(gamePath, "_patch_backup");
        if (Directory.Exists(backupFolder))
        {
            // 从统一备份文件夹中查找所有.bak 文件
            bakFiles = Directory.GetFiles(backupFolder, "*.bak", SearchOption.AllDirectories);
        }
        
        // 如果统一备份文件夹没有找到，再尝试查找原位置的.bak 文件（兼容旧版本）
        if (bakFiles.Length == 0)
        {
            bakFiles = Directory.GetFiles(gamePath, "*.bak", SearchOption.AllDirectories);
        }
        
        if (bakFiles.Length == 0)
        {
            MessageBox.Show($"未找到备份文件\n\n游戏目录中没有发现需要还原的文件。", "提示", MessageBoxButton.OK, MessageBoxImage.Information);
            return;
        }
        
        // 确认对话框
        var result = MessageBox.Show(
            $"确定要还原 {CurrentSelectedGame.GameName} 吗？\n\n" +
            $"将找到 {bakFiles.Length} 个备份文件\n" +
            $"还原后，注入的补丁文件将被删除，原始文件将恢复\n\n" +
            $"注意：此操作不可逆，请谨慎操作！",
            "确认还原",
            MessageBoxButton.YesNo,
            MessageBoxImage.Warning);
        
        if (result != MessageBoxResult.Yes)
            return;
        
        // 开始还原
        InjectProgressPanel.Visibility = Visibility.Visible;
        BtnRestoreGame.IsEnabled = false;
        TxtInjectLog.Text = "";
        
        try
        {
            await RestoreGameFiles(gamePath, bakFiles, backupFolder);
        }
        catch (Exception ex)
        {
            AppendPatchLog($"❌ 错误：{ex.Message}");
            MessageBox.Show($"还原失败：{ex.Message}", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
            BtnRestoreGame.IsEnabled = true;
        }
    }

    /// <summary>
    /// 执行游戏文件还原
    /// </summary>
    /// <param name="gamePath">游戏路径</param>
    /// <param name="bakFiles">备份文件列表</param>
    /// <param name="backupFolder">备份文件夹路径</param>
    private async Task RestoreGameFiles(string gamePath, string[] bakFiles, string backupFolder)
    {
        int totalFiles = bakFiles.Length;
        int processedFiles = 0;
        int restoredFiles = 0;
        int failedFiles = 0;
        
        TxtInjectStatus.Text = "开始还原游戏...";
        TxtInjectProgress.Text = $"0/{totalFiles}";
        InjectProgressBar.Maximum = totalFiles;
        InjectProgressBar.Value = 0;
        
        AppendPatchLog($"开始还原游戏：{gamePath}");
        AppendPatchLog($"备份文件夹：{(Directory.Exists(backupFolder) ? "_patch_backup" : "原位置备份 (兼容模式)")}");
        AppendPatchLog($"共 {totalFiles} 个备份文件需要还原");
        
        await Task.Run(() =>
        {
            foreach (string bakFile in bakFiles)
            {
                try
                {
                    // 原始文件路径（去掉.bak 后缀）
                    string originalFile = bakFile.Substring(0, bakFile.Length - 4); // 去掉".bak"
                    string relativePath = Path.GetRelativePath(gamePath, originalFile);
                    
                    AppendPatchLog($"正在还原：{relativePath}");
                    
                    // 如果原始文件存在，先删除（可能是注入的补丁文件）
                    if (File.Exists(originalFile))
                    {
                        try
                        {
                            // 移除只读属性
                            File.SetAttributes(originalFile, FileAttributes.Normal);
                            File.Delete(originalFile);
                            AppendPatchLog($"🗑️ 已删除补丁文件：{Path.GetFileName(originalFile)}");
                        }
                        catch (Exception ex)
                        {
                            AppendPatchLog($"⚠️ 删除补丁文件失败：{Path.GetFileName(originalFile)} - {ex.Message}");
                        }
                    }
                    
                    // 还原备份的原始文件
                    if (File.Exists(bakFile))
                    {
                        // 确保目标目录存在
                        string? targetDir = Path.GetDirectoryName(originalFile);
                        if (!string.IsNullOrEmpty(targetDir) && !Directory.Exists(targetDir))
                        {
                            Directory.CreateDirectory(targetDir);
                        }
                        
                        // 复制备份文件到原始位置（覆盖已存在的文件）
                        File.Copy(bakFile, originalFile, true);
                        
                        // 恢复文件属性（移除只读）
                        File.SetAttributes(originalFile, FileAttributes.Normal);
                        
                        // 删除备份文件
                        File.Delete(bakFile);
                        
                        restoredFiles++;
                        AppendPatchLog($"✅ 已还原：{Path.GetFileName(originalFile)}");
                    }
                    else
                    {
                        AppendPatchLog($"⚠️ 备份文件不存在：{Path.GetFileName(bakFile)}");
                        failedFiles++;
                    }
                }
                catch (Exception ex)
                {
                    AppendPatchLog($"❌ 还原失败：{Path.GetFileName(bakFile)} - {ex.Message}");
                    failedFiles++;
                }
                
                processedFiles++;
                
                // 更新进度（节流优化）
                int current = processedFiles;
                Dispatcher.Invoke(() =>
                {
                    TxtInjectProgress.Text = $"{current}/{totalFiles}";
                    InjectProgressBar.Value = current;
                }, System.Windows.Threading.DispatcherPriority.Background);
            }
        });
        
        // 清理空的补丁文件夹
        try
        {
            string steamSettingsDir = Path.Combine(gamePath, "steam_settings");
            if (Directory.Exists(steamSettingsDir))
            {
                Directory.Delete(steamSettingsDir, true);
                AppendPatchLog($"🗑️ 已删除 steam_settings 配置文件夹");
            }
            
            // 清理可能残留的配置文件
            string[] configFiles = {
                "steam_api64.dll",
                "steam_api.dll",
                "steamclient64.dll",
                "steamclient.dll",
                "steam_interfaces.txt"
            };
            
            foreach (string configFile in configFiles)
            {
                string configPath = Path.Combine(gamePath, configFile);
                if (File.Exists(configPath))
                {
                    File.SetAttributes(configPath, FileAttributes.Normal);
                    File.Delete(configPath);
                    AppendPatchLog($"🗑️ 已删除残留文件：{configFile}");
                }
            }
        }
        catch (Exception ex)
        {
            AppendPatchLog($"⚠️ 清理配置文件失败：{ex.Message}");
        }
        
        // 删除备份文件夹（如果存在）
        if (Directory.Exists(backupFolder))
        {
            try
            {
                Directory.Delete(backupFolder, true);
                AppendPatchLog($"🗑️ 已删除备份文件夹：_patch_backup");
            }
            catch (Exception ex)
            {
                AppendPatchLog($"⚠️ 删除备份文件夹失败：{ex.Message}");
            }
        }
        
        TxtInjectStatus.Text = "✅ 还原完成！";
        TxtInjectStatus.Foreground = new System.Windows.Media.SolidColorBrush(
            (System.Windows.Media.Color)System.Windows.Media.ColorConverter.ConvertFromString("#FF4CAF50"));
        
        AppendPatchLog($"========================================");
        AppendPatchLog($"🎉 游戏还原完成！");
        AppendPatchLog($"✅ 成功还原：{restoredFiles} 个文件");
        if (failedFiles > 0)
            AppendPatchLog($"⚠️ 还原失败：{failedFiles} 个文件");
        AppendPatchLog($"========================================");
        
        MessageBox.Show($"游戏还原完成！\n\n成功还原：{restoredFiles} 个文件" + 
            (failedFiles > 0 ? $"\n还原失败：{failedFiles} 个文件（请手动检查）" : ""), 
            "还原完成", MessageBoxButton.OK, MessageBoxImage.Information);
        
        InjectProgressPanel.Visibility = Visibility.Collapsed;
        BtnRestoreGame.IsEnabled = true;
    }

    /// <summary>
    /// 复制目录及其所有内容
    /// </summary>
    /// <param name="sourceDir">源目录</param>
    /// <param name="destDir">目标目录</param>
    private void CopyDirectory(string sourceDir, string destDir)
    {
        DirectoryInfo dir = new DirectoryInfo(sourceDir);
        DirectoryInfo[] dirs = dir.GetDirectories();

        Directory.CreateDirectory(destDir);

        foreach (FileInfo file in dir.GetFiles())
        {
            string targetFilePath = Path.Combine(destDir, file.Name);
            file.CopyTo(targetFilePath, true);
        }

        foreach (DirectoryInfo subDir in dirs)
        {
            string newDestinationDir = Path.Combine(destDir, subDir.Name);
            CopyDirectory(subDir.FullName, newDestinationDir);
        }
    }

    /// <summary>
    /// 全自动脱壳按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private async void RunSteamlessUnpack_Click(object sender, RoutedEventArgs e)
    {
        try
        {
            string gameDir = TxtGameDir.Text.Trim();
            string gameExePath = TxtGameExePath.Text.Trim();
            
            if (string.IsNullOrEmpty(gameDir))
            { MessageBox.Show("请先选择游戏目录！", "错误", MessageBoxButton.OK, MessageBoxImage.Error); return; }
            
            if (!Directory.Exists(gameDir))
            { MessageBox.Show("游戏目录不存在！", "错误", MessageBoxButton.OK, MessageBoxImage.Error); return; }
            
            // 获取 Steamless 路径（使用相对路径）
            string steamlessPath = Path.Combine(AppPath, "Resources", "gbe_fork", "tools", "steamless", "Steamless.CLI.exe");
            if (!File.Exists(steamlessPath))
            {
                AppendPatchLog($"❌ 未找到 Steamless 工具：{steamlessPath}");
                MessageBox.Show($"未找到 Steamless 脱壳工具！\n请确保程序根目录下有 Resources/gbe_fork/tools/steamless/Steamless.CLI.exe", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
                return;
            }
            
            List<string> targetFiles = new List<string>();
            
            // 如果用户指定了游戏主程序，只脱壳这个文件
            if (!string.IsNullOrEmpty(gameExePath) && File.Exists(gameExePath))
            {
                targetFiles.Add(gameExePath);
                AppendPatchLog($"🎯 指定脱壳文件：{Path.GetFileName(gameExePath)}");
            }
            else
            {
                // 否则扫描游戏目录中的所有.exe 文件
                AppendPatchLog("🔍 正在扫描游戏目录中的可执行文件...");
                
                // 查找游戏目录中的所有.exe 文件
                string[] exeFiles = Directory.GetFiles(gameDir, "*.exe", SearchOption.AllDirectories);
                
                if (exeFiles.Length == 0)
                {
                    AppendPatchLog($"❌ 未找到任何可执行文件 (.exe)");
                    MessageBox.Show($"游戏目录中未找到任何.exe 文件！", "提示", MessageBoxButton.OK, MessageBoxImage.Information);
                    return;
                }
                
                AppendPatchLog($"📊 找到 {exeFiles.Length} 个可执行文件");
                
                // 过滤掉已脱壳的文件（排除 steamclient_loader 等）
                foreach (string exeFile in exeFiles)
                {
                    string fileName = Path.GetFileName(exeFile);
                    // 排除一些已知的不需要脱壳的文件
                    if (!fileName.StartsWith("steamclient_loader", StringComparison.OrdinalIgnoreCase) &&
                        !fileName.StartsWith("steam_api", StringComparison.OrdinalIgnoreCase))
                    {
                        targetFiles.Add(exeFile);
                    }
                }
                
                if (targetFiles.Count == 0)
                {
                    AppendPatchLog("ℹ️ 没有找到需要脱壳的目标文件");
                    MessageBox.Show("没有找到需要脱壳的目标文件！", "提示", MessageBoxButton.OK, MessageBoxImage.Information);
                    return;
                }
            }
            
            AppendPatchLog($"🎯 准备对 {targetFiles.Count} 个文件进行脱壳处理...");
            
            int successCount = 0;
            int failedCount = 0;
            
            foreach (string exeFile in targetFiles)
            {
                string relativePath = Path.GetRelativePath(gameDir, exeFile);
                AppendPatchLog($"⏳ 正在处理：{relativePath}");
                
                // 创建备份目录
                string backupDir = Path.Combine(gameDir, "_steamless_backup");
                Directory.CreateDirectory(backupDir);
                
                // 生成备份文件路径
                string backupPath = Path.Combine(backupDir, relativePath + ".bak");
                Directory.CreateDirectory(Path.GetDirectoryName(backupPath)!);
                
                try
                {
                    // 备份原文件
                    File.Copy(exeFile, backupPath, true);
                    AppendPatchLog($"📦 已备份：{relativePath}");
                    
                    // 调用 Steamless.CLI.exe 进行脱壳
                    var startInfo = new ProcessStartInfo
                    {
                        FileName = steamlessPath,
                        Arguments = $"\"{exeFile}\" --output=\"{exeFile}\"",
                        UseShellExecute = false,
                        CreateNoWindow = true,
                        RedirectStandardOutput = true,
                        RedirectStandardError = true,
                        StandardOutputEncoding = Encoding.UTF8,
                        StandardErrorEncoding = Encoding.UTF8
                    };
                    
                    using (var process = new Process { StartInfo = startInfo })
                    {
                        process.Start();
                        string output = await process.StandardOutput.ReadToEndAsync();
                        string error = await process.StandardError.ReadToEndAsync();
                        await Task.Run(() => process.WaitForExit());
                        
                        if (process.ExitCode == 0)
                        {
                            AppendPatchLog($"✅ 脱壳成功：{relativePath}");
                            successCount++;
                        }
                        else
                        {
                            AppendPatchLog($"❌ 脱壳失败：{relativePath} - {error}");
                            // 恢复备份
                            File.Copy(backupPath, exeFile, true);
                            AppendPatchLog($"🔄 已恢复原始文件");
                            failedCount++;
                        }
                    }
                }
                catch (Exception ex)
                {
                    AppendPatchLog($"❌ 处理失败：{relativePath} - {ex.Message}");
                    failedCount++;
                }
            }
            
            AppendPatchLog($"========================================");
            AppendPatchLog($"🎉 脱壳完成！");
            AppendPatchLog($"✅ 成功：{successCount} 个");
            AppendPatchLog($"❌ 失败：{failedCount} 个");
            AppendPatchLog($"📁 备份位置：_steamless_backup/");
            AppendPatchLog($"========================================");
            
            MessageBox.Show($"脱壳完成！\n成功：{successCount} 个\n失败：{failedCount} 个\n\n备份文件已保存在 _steamless_backup 文件夹", 
                "脱壳完成", MessageBoxButton.OK, MessageBoxImage.Information);
        }
        catch (Exception ex)
        {
            AppendPatchLog($"❌ 操作失败：{ex.Message}");
            MessageBox.Show($"操作失败：{ex.Message}", "错误", MessageBoxButton.OK, MessageBoxImage.Error);
        }
    }

    #region 窗口控制方法

    /// <summary>
    /// 标题栏鼠标拖动事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void TitleBar_MouseLeftButtonDown(object sender, MouseButtonEventArgs e)
    {
        if (e.ClickCount == 2)
        {
            MaximizeBtn_Click(sender, e);
        }
        else
        {
            this.DragMove();
        }
    }

    /// <summary>
    /// 最小化按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void MinimizeBtn_Click(object sender, RoutedEventArgs e)
    {
        this.WindowState = WindowState.Minimized;
    }

    /// <summary>
    /// 最大化/还原按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void MaximizeBtn_Click(object sender, RoutedEventArgs e)
    {
        if (this.WindowState == WindowState.Maximized)
        {
            this.WindowState = WindowState.Normal;
        }
        else
        {
            this.WindowState = WindowState.Maximized;
        }
    }

    /// <summary>
    /// 窗口状态变更事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void MainWindow_StateChanged(object? sender, EventArgs e)
    {
        if (this.WindowState == WindowState.Maximized)
        {
            this.BorderThickness = new Thickness(0);
        }
        else
        {
            this.BorderThickness = new Thickness(1);
        }
    }

    /// <summary>
    /// 关闭按钮点击事件
    /// </summary>
    /// <param name="sender">发送者</param>
    /// <param name="e">事件参数</param>
    private void CloseBtn_Click(object sender, RoutedEventArgs e)
    {
        this.Close();
    }

    #endregion
}

/// <summary>
/// 游戏信息类
/// </summary>
public class GameInfo
{
    /// <summary>
    /// 游戏AppID
    /// </summary>
    public string AppId { get; set; }
    
    /// <summary>
    /// 游戏名称
    /// </summary>
    public string Name { get; set; }
    
    /// <summary>
    /// 游戏配置文件夹路径
    /// </summary>
    public string FolderPath { get; set; }
    
    /// <summary>
    /// Depot清单数量
    /// </summary>
    public int ManifestCount { get; set; }
}

/// <summary>
/// 成就项类
/// </summary>
public class AchievementItem
{
    /// <summary>
    /// 成就 ID
    /// </summary>
    public string Id { get; set; }
    
    /// <summary>
    /// 成就名称
    /// </summary>
    public string Name { get; set; }
    
    /// <summary>
    /// 成就描述
    /// </summary>
    public string Description { get; set; }
    
    /// <summary>
    /// 是否隐藏
    /// </summary>
    public bool Hidden { get; set; }
}

/// <summary>
/// 补丁类型枚举
/// </summary>
public enum PatchType
{
    /// <summary>
    /// 免 Steam 启动补丁
    /// </summary>
    NoSteam,
    
    /// <summary>
    /// 局域网联机补丁
    /// </summary>
    LAN,

    /// <summary>
    /// Steam 联机补丁
    /// </summary>
    SteamOnline,
    
    /// <summary>
    /// D 加密游戏
    /// </summary>
    DEncrypted
}

/// <summary>
/// 游戏补丁信息类
/// </summary>
public class PatchGameInfo
{
    /// <summary>
    /// 游戏名称
    /// </summary>
    public string GameName { get; set; } = "";
    
    /// <summary>
    /// 游戏 ID
    /// </summary>
    public string GameId { get; set; } = "";
    
    /// <summary>
    /// 封面图片路径（详情页使用 inside 目录）
    /// 如果不设置，将根据 GameId 自动生成
    /// </summary>
    public string CoverImage { get; set; } = "";
    
    /// <summary>
    /// 列表视图封面图片路径（使用 outside 目录）
    /// 如果不设置，将根据 GameId 自动生成
    /// </summary>
    public string ListCoverImage { get; set; } = "";
    
    /// <summary>
    /// 补丁源路径
    /// </summary>
    public string PatchSourcePath { get; set; } = "";
    
    /// <summary>
    /// 补丁类型
    /// </summary>
    public PatchType PatchType { get; set; } = PatchType.NoSteam;
    
    /// <summary>
    /// 是否为 D 加密游戏
    /// </summary>
    public bool IsDEncrypted { get; set; } = true;  // 默认都是 D 加密
    
    /// <summary>
    /// 获取详情页封面的 Pack URI（根据 GameId 动态生成）
    /// </summary>
    public string GetCoverImagePackUri()
    {
        // 如果自定义了封面路径，使用自定义路径
        if (!string.IsNullOrEmpty(CoverImage))
        {
            return $"pack://application:,,,/SteamToolPlus;component/{CoverImage}";
        }
        
        // 否则根据 GameId 生成默认路径
        string imageName = string.IsNullOrEmpty(GameId) ? "1245620.jpg" : $"{GameId}.jpg";
        return $"pack://application:,,,/SteamToolPlus;component/Resources/pic/inside/{imageName}";
    }
    
    /// <summary>
    /// 获取列表视图封面的 Pack URI（根据 GameId 动态生成）
    /// </summary>
    public string GetListCoverImagePackUri()
    {
        // 如果自定义了封面路径，使用自定义路径
        if (!string.IsNullOrEmpty(ListCoverImage))
        {
            return $"pack://application:,,,/SteamToolPlus;component/{ListCoverImage}";
        }
        
        // 否则根据 GameId 生成默认路径
        string imageName = string.IsNullOrEmpty(GameId) ? "1245620.jpg" : $"{GameId}.jpg";
        return $"pack://application:,,,/SteamToolPlus;component/Resources/pic/outside/{imageName}";
    }
}



/// <summary>
/// 游戏卡片视图模型（用于数据绑定）
/// </summary>
public class GameCardViewModel
{
    /// <summary>
    /// 游戏标签（唯一标识）
    /// </summary>
    public string Tag { get; set; } = "";

    /// <summary>
    /// 游戏英文名称
    /// </summary>
    public string GameName { get; set; } = "";

    /// <summary>
    /// 游戏中文名称
    /// </summary>
    public string? ChineseName { get; set; }

    /// <summary>
    /// 游戏中文名称显示（带前缀空格）
    /// </summary>
    public string ChineseNameDisplay
    {
        get
        {
            return string.IsNullOrEmpty(ChineseName) ? "" : $"  {ChineseName}";
        }
    }

    /// <summary>
    /// 游戏 ID 显示（带前缀）
    /// </summary>
    public string GameIdDisplay => $"Game ID: {GameId}";

    /// <summary>
    /// 游戏 Steam AppID
    /// </summary>
    public string GameId { get; set; } = "";

    /// <summary>
    /// 补丁类型
    /// </summary>
    public PatchType PatchType { get; set; }

    /// <summary>
    /// 补丁类型名称（带图标）
    /// </summary>
    public string PatchTypeName { get; set; } = "";

    /// <summary>
    /// 补丁类型背景颜色
    /// </summary>
    public SolidColorBrush PatchTypeColor { get; set; } = new SolidColorBrush(System.Windows.Media.Color.FromRgb(0x80, 0x80, 0x80));

    /// <summary>
    /// 网格视图封面图片路径（使用 inside 目录）
    /// </summary>
    public string CoverImage { get; set; } = "";

    /// <summary>
    /// 列表视图封面图片路径（使用 outside 目录）
    /// </summary>
    public string OutsideCoverImage { get; set; } = "";

    /// <summary>
    /// 初始化游戏卡片视图模型
    /// </summary>
    public GameCardViewModel()
    {
    }

    /// <summary>
    /// 从游戏配置创建游戏卡片视图模型
    /// </summary>
    public static GameCardViewModel FromConfig(PatchGameConfig config)
    {
        var viewModel = new GameCardViewModel
        {
            Tag = config.Tag,
            GameName = config.GameName,
            ChineseName = config.ChineseName,
            GameId = config.GameId,
            PatchType = (PatchType)config.PatchType
        };

        // 预计算所有属性值
        viewModel.PatchTypeName = viewModel.PatchType switch
        {
            PatchType.NoSteam => "🚫 免 Steam 启动",
            PatchType.LAN => "🌐 局域网联机",
            PatchType.SteamOnline => "🌍 Steam 联机",
            PatchType.DEncrypted => "🔒 D 加密虚拟机",
            _ => "未知"
        };

        viewModel.PatchTypeColor = viewModel.PatchType switch
        {
            PatchType.NoSteam => new SolidColorBrush(System.Windows.Media.Color.FromRgb(0x00, 0x99, 0xF2)),
            PatchType.LAN => new SolidColorBrush(System.Windows.Media.Color.FromRgb(0x4C, 0xAF, 0x50)),
            PatchType.SteamOnline => new SolidColorBrush(System.Windows.Media.Color.FromRgb(0x9C, 0x27, 0xB0)),
            PatchType.DEncrypted => new SolidColorBrush(System.Windows.Media.Color.FromRgb(0xFF, 0x98, 0x00)),
            _ => new SolidColorBrush(System.Windows.Media.Color.FromRgb(0x80, 0x80, 0x80))
        };

        string imageName = string.IsNullOrEmpty(viewModel.GameId) ? "1245620.jpg" : $"{viewModel.GameId}.jpg";
        // 网格视图和列表视图都使用 outside 目录的图片
        viewModel.CoverImage = $"pack://application:,,,/SteamToolPlus;component/Resources/pic/outside/{imageName}";
        viewModel.OutsideCoverImage = $"pack://application:,,,/SteamToolPlus;component/Resources/pic/outside/{imageName}";

        return viewModel;
    }
}