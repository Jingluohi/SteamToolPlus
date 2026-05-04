/**
 * Steam 模拟器配置文件类型定义
 * 100% 对应 gbe_fork 的所有配置文件
 */

// ============================================
// 1. 核心配置文件类型
// ============================================

/**
 * configs.main.ini - 主配置文件
 * 100% 实现 gbe_fork 所有配置选项
 */
export interface MainConfig {
  // [main::general] 通用设置
  /** 生成新版认证票据 */
  newAppTicket: boolean
  /** 游戏协调器令牌 */
  gcToken: boolean
  /** 阻止未知客户端 */
  blockUnknownClients: boolean
  /** 模拟 Steam Deck */
  steamDeck: boolean
  /** 启用头像功能 */
  enableAccountAvatar: boolean
  /** 启用语音聊天 */
  enableVoiceChat: boolean
  /** 即时游戏服务器统计 */
  immediateGameserverStats: boolean
  /** 匹配服务器列表实际类型 */
  matchmakingServerListActualType: boolean
  /** 通过 Source 查询获取服务器详情 */
  matchmakingServerDetailsViaSourceQuery: boolean
  /** 崩溃日志位置 */
  crashPrinterLocation?: string

  // [main::stats] 统计设置
  /** 禁用未知排行榜创建 */
  disableLeaderboardsCreateUnknown: boolean
  /** 允许未知统计 */
  allowUnknownStats: boolean
  /** 统计成就进度功能 */
  statAchievementProgressFunctionality: boolean
  /** 只保存更高的统计成就进度 */
  saveOnlyHigherStatAchievementProgress: boolean
  /** 分页成就图标数量 */
  paginatedAchievementsIcons: number
  /** 记录游戏时间 */
  recordPlaytime: boolean

  // [main::connectivity] 连接设置
  /** 禁用仅局域网模式 */
  disableLanOnly: boolean
  /** 禁用网络功能 */
  disableNetworking: boolean
  /** 监听端口 */
  listenPort: number
  /** 离线模式 */
  offline: boolean
  /** 禁用与游戏服务器共享统计 */
  disableSharingStatsWithGameserver: boolean
  /** 禁用 Source 查询 */
  disableSourceQuery: boolean
  /** 网络共享排行榜 */
  shareLeaderboardsOverNetwork: boolean
  /** 禁用地堡创建 */
  disableLobbyCreation: boolean
  /** 下载 SteamHTTP 请求 */
  downloadSteamhttpRequests: boolean

  // [main::misc] 其他设置
  /** 成就绕过 */
  achievementsBypass: boolean
  /** 强制 SteamHTTP 成功 */
  forceSteamhttpSuccess: boolean
  /** 禁用 Steam 覆盖层游戏 ID 环境变量 */
  disableSteamoverlaygameidEnvVar: boolean
  /** 启用 Steam 预拥有 ID */
  enableSteamPreownedIds: boolean
  /** Steam 游戏统计报告目录 */
  steamGameStatsReportsDir?: string
  /** 免费周末 */
  freeWeekend: boolean
  /** 使用 32 位库存物品 ID */
  use32bitInventoryItemIds: boolean

  // 额外DLL列表
  extraDlls: string[]
}

/**
 * configs.user.ini - 用户配置文件
 * 100% 实现 gbe_fork 所有配置选项
 */
export interface UserConfig {
  /** 用户名 */
  username: string
  /** 语言 */
  language: string
  /** 存档路径 */
  savePath: string
  /** 头像路径 */
  avatarPath?: string
  /** 默认头像 */
  useDefaultAvatar: boolean
  /** 存档文件夹名称（覆盖默认的 "GSE Saves"） */
  savesFolderName?: string
  /** 本地存档路径（便携模式） */
  localSavePath?: string
  /** EncryptedAppTicket (Base64编码) */
  ticket?: string
}

/**
 * configs.app.ini - 应用配置文件
 */
export interface AppConfig {
  /** 分支名称 */
  branchName: string
  /** 应用路径 */
  appPaths: Record<string, string>
  /** DLC解锁配置 */
  dlcs: {
    unlockAll: boolean
    individualDlcs: Array<{
      appId: string
      name: string
      enabled: boolean
    }>
  }
}

/**
 * configs.overlay.ini - 覆盖层配置文件
 * 100% 实现 gbe_fork 所有配置选项
 */
export interface OverlayConfig {
  /** 启用实验性覆盖层 (gbe_fork 关键配置) */
  enableExperimentalOverlay: boolean
  /** 快捷键 */
  hotkey: string
  /** 通知设置 */
  notifications: {
    /** 成就通知 */
    achievement: boolean
    /** 好友通知 */
    friend: boolean
    /** 消息通知 */
    message: boolean
    /** 通知显示时间(秒) */
    duration: number
    /** 通知位置 */
    position: 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right'
  }
  /** 界面设置 */
  appearance: {
    /** 主题 */
    theme: 'dark' | 'light' | 'auto'
    /** 透明度 */
    opacity: number
    /** 缩放比例 */
    scale: number
    /** 背景模糊 */
    blur: boolean
  }
  /** 性能设置 */
  performance: {
    /** 硬件加速 */
    hardwareAcceleration: boolean
    /** 帧率限制 */
    fpsLimit: number
    /** 低性能模式 */
    lowPerformanceMode: boolean
  }
  /** 功能开关 */
  features: {
    /** 成就页面 */
    achievements: boolean
    /** 好友列表 */
    friends: boolean
    /** 聊天 */
    chat: boolean
    /** 浏览器 */
    browser: boolean
    /** 设置 */
    settings: boolean
  }
}

// ============================================
// 2. 成就系统配置
// ============================================

/**
 * 成就数据
 */
export interface Achievement {
  /** 成就唯一标识名 */
  name: string
  /** 显示名称 */
  displayName: string
  /** 成就描述 */
  description: string
  /** 是否隐藏成就 */
  hidden: boolean
  /** 图标路径(已解锁) */
  icon?: string
  /** 图标路径(未解锁) */
  iconGray?: string
}

/**
 * achievements.json - 成就配置
 */
export interface AchievementsConfig {
  /** 启用成就系统 */
  enabled: boolean
  /** 显示解锁通知 */
  showNotifications: boolean
  /** 成就列表 */
  achievements: Achievement[]
}

// ============================================
// 3. 统计数据配置
// ============================================

/**
 * 统计项类型
 */
export type StatType = 'int' | 'float' | 'avgrate' | 'achievements'

/**
 * 统计数据项
 */
export interface StatItem {
  /** 统计名称 */
  name: string
  /** 统计类型 */
  type: StatType
  /** 默认值 */
  defaultValue: number
  /** 最小值(仅float) */
  minValue?: number
  /** 最大值(仅float) */
  maxValue?: number
  /** 窗口大小(仅avgrate) */
  windowSize?: number
}

/**
 * stats.json - 统计配置
 */
export interface StatsConfig {
  /** 启用统计系统 */
  enabled: boolean
  /** 统计项列表 */
  stats: StatItem[]
}

// ============================================
// 4. 物品库存配置
// ============================================

/**
 * 物品属性
 */
export interface ItemProperty {
  /** 属性名 */
  name: string
  /** 属性值 */
  value: string | number | boolean
}

/**
 * 游戏物品
 */
export interface GameItem {
  /** 物品ID */
  itemId: string
  /** 数量 */
  quantity: number
  /** 属性列表 */
  properties?: ItemProperty[]
}

/**
 * 物品定义
 */
export interface ItemDefinition {
  /** 物品ID */
  id: string
  /** 物品名称 */
  name: string
  /** 物品描述 */
  description?: string
  /** 物品类型 */
  type?: string
  /** 图标路径 */
  icon?: string
  /** 可堆叠 */
  stackable: boolean
  /** 最大堆叠数量 */
  maxStackSize: number
}

/**
 * items.json - 物品配置
 */
export interface ItemsConfig {
  /** 启用物品系统 */
  enabled: boolean
  /** 物品定义列表 */
  itemDefinitions: ItemDefinition[]
  /** 初始库存物品 */
  initialItems: GameItem[]
}

// ============================================
// 5. 创意工坊模组配置
// ============================================

/**
 * 模组文件
 */
export interface ModFile {
  /** 文件名 */
  name: string
  /** 文件大小 */
  size: number
  /** 文件路径 */
  path: string
}

/**
 * 创意工坊模组
 */
export interface WorkshopMod {
  /** 模组ID */
  publishedFileId: string
  /** 模组标题 */
  title: string
  /** 模组描述 */
  description?: string
  /** 作者SteamID */
  authorId?: string
  /** 作者名称 */
  authorName?: string
  /** 创建时间 */
  timeCreated?: string
  /** 更新时间 */
  timeUpdated?: string
  /** 预览图路径 */
  previewImage?: string
  /** 模组文件列表 */
  files: ModFile[]
  /** 标签 */
  tags?: string[]
  /** 可见性 */
  visibility: 'public' | 'friends' | 'private'
}

/**
 * mods.json - 模组配置
 */
export interface ModsConfig {
  /** 启用模组系统 */
  enabled: boolean
  /** 已订阅模组列表 */
  subscribedMods: WorkshopMod[]
  /** 自动更新模组 */
  autoUpdate: boolean
}

// ============================================
// 6. 排行榜配置
// ============================================

/**
 * 排行榜条目
 */
export interface LeaderboardEntry {
  /** 排名 */
  rank: number
  /** SteamID */
  steamId: string
  /** 用户名 */
  username: string
  /** 分数 */
  score: number
  /** 详情数据 */
  details?: string
}

/**
 * 排行榜定义
 */
export interface Leaderboard {
  /** 排行榜名称 */
  name: string
  /** 显示名称 */
  displayName: string
  /** 排序方式 */
  sortMethod: 'asc' | 'desc'
  /** 显示类型 */
  displayType: 'numeric' | 'seconds' | 'milliseconds'
  /** 条目列表 */
  entries: LeaderboardEntry[]
}

/**
 * leaderboards.txt - 排行榜配置
 */
export interface LeaderboardsConfig {
  /** 启用排行榜 */
  enabled: boolean
  /** 排行榜列表 */
  leaderboards: Leaderboard[]
}

// ============================================
// 7. 控制器配置
// ============================================

/**
 * 控制器按键绑定
 */
export interface ControllerBinding {
  /** 动作名称 */
  action: string
  /** 按键 */
  button: string
  /** 描述 */
  description?: string
}

/**
 * 控制器配置
 */
export interface ControllerConfig {
  /** 启用控制器支持 */
  enabled: boolean
  /** 控制器类型 */
  controllerType: 'xbox' | 'playstation' | 'nintendo' | 'generic'
  /** 按键绑定 */
  bindings: ControllerBinding[]
  /** 死区设置 */
  deadzone: {
    leftStick: number
    rightStick: number
    leftTrigger: number
    rightTrigger: number
  }
  /** 震动设置 */
  rumble: {
    enabled: boolean
    intensity: number
  }
  /** 自定义图标 */
  customGlyphs?: {
    enabled: boolean
    path?: string
  }
}

// ============================================
// 8. 音效配置
// ============================================

/**
 * 音效配置项
 */
export interface SoundConfig {
  /** 启用自定义音效 */
  enabled: boolean
  /** 成就通知音效 */
  achievementNotification?: string
  /** 好友通知音效 */
  friendNotification?: string
  /** 消息通知音效 */
  messageNotification?: string
  /** 主音量 */
  masterVolume: number
}

// ============================================
// 9. 字体配置
// ============================================

/**
 * 字体配置
 */
export interface FontConfig {
  /** 启用自定义字体 */
  enabled: boolean
  /** 字体文件路径 */
  fontPath?: string
  /** 字体大小 */
  fontSize: number
  /** 字体粗细 */
  fontWeight: 'normal' | 'bold' | 'light'
}

// ============================================
// 10. 其他配置文件
// ============================================

/**
 * installed_app_ids.txt
 */
export interface InstalledAppIdsConfig {
  /** 已安装的应用ID列表 */
  appIds: string[]
}

/**
 * subscribed_groups.txt
 */
export interface SubscribedGroupsConfig {
  /** 订阅的群组ID列表 */
  groupIds: string[]
}

/**
 * 订阅群组（公会）项
 */
export interface SubscribedGroupClan {
  /** 群组ID */
  groupId: string
  /** 群组名称 */
  name: string
  /** 公会标签 */
  tag: string
}

/**
 * subscribed_groups_clans.txt
 */
export interface SubscribedGroupsClansConfig {
  /** 订阅的群组（公会）列表 */
  groups: SubscribedGroupClan[]
}

/**
 * supported_languages.txt
 */
export interface SupportedLanguagesConfig {
  /** 支持的语言列表 */
  languages: Array<{
    code: string
    name: string
    nativeName: string
  }>
}

/**
 * 购买密钥项
 */
export interface PurchasedKey {
  /** 应用ID */
  appId: string
  /** CD密钥 */
  key: string
}

/**
 * purchased_keys.txt
 */
export interface PurchasedKeysConfig {
  /** CD密钥列表 */
  keys: PurchasedKey[]
}

/**
 * depots.txt
 */
export interface DepotsConfig {
  /** Depot列表 */
  depots: Array<{
    depotId: string
    manifestId: string
  }>
}

/**
 * branches.json
 */
export interface BranchesConfig {
  /** 分支列表 */
  branches: Array<{
    name: string
    description?: string
    protected: boolean
    password?: string
  }>
}

/**
 * default_items.json
 */
export interface DefaultItemsConfig {
  /** 默认物品列表 */
  items: GameItem[]
}

/**
 * gc.json - 游戏协调器配置
 */
export interface GameCoordinatorConfig {
  /** GC配置文件 */
  gcProfile?: string
  /** GC版本 */
  gcVersion?: number
  /** 额外配置 */
  [key: string]: string | number | boolean | undefined
}

// ============================================
// 11. 头像配置
// ============================================

/**
 * 头像配置
 */
export interface AvatarConfig {
  /** 头像类型 */
  type: 'account' | 'default'
  /** 文件扩展名 */
  extension: 'png' | 'jpg' | 'jpeg'
  /** 文件数据 (Base64) */
  data?: string
}

// ============================================
// 12. SteamHTTP 配置
// ============================================

/**
 * SteamHTTP 响应配置
 */
export interface SteamHttpConfig {
  /** 域名 */
  domain: string
  /** 路径 */
  path: string
  /** 响应内容 */
  content: string
}

/**
 * SteamHTTP 配置列表项
 */
export interface SteamHttpConfigItem {
  /** 域名 */
  domain: string
  /** 路径列表 */
  paths: string[]
}

// ============================================
// 13. ColdClientLoader 配置
// ============================================

/**
 * ColdClientLoader 配置
 */
export interface ColdClientLoaderConfig {
  /** 启用ColdClientLoader */
  enabled: boolean
  /** 注入模式 */
  injectionMode: 'direct' | 'loader'
  /** 额外DLL列表 */
  extraDlls: string[]
  /** 启动参数 */
  launchArgs: string
  /** 工作目录 */
  workingDirectory?: string
}

// ============================================
// 14. lobby_connect 配置
// ============================================

/**
 * 大厅信息
 */
export interface LobbyInfo {
  /** 大厅ID */
  lobbyId: string
  /** 大厅名称 */
  name: string
  /** 游戏模式 */
  gameMode?: string
  /** 当前玩家数 */
  memberCount: number
  /** 最大玩家数 */
  maxMembers: number
  /** 主机SteamID */
  ownerId: string
  /** 主机名称 */
  ownerName: string
  /** 延迟 */
  ping?: number
}

/**
 * lobby_connect 配置
 */
export interface LobbyConnectConfig {
  /** 启用lobby_connect */
  enabled: boolean
  /** 自动加入大厅 */
  autoJoin: boolean
  /** 大厅ID */
  targetLobbyId?: string
  /** 连接密码 */
  password?: string
}

// ============================================
// 15. 完整配置集合
// ============================================

/**
 * 完整的Steam设置配置
 */
export interface CompleteSteamSettings {
  // 核心配置
  main: MainConfig
  user: UserConfig
  app: AppConfig
  overlay: OverlayConfig

  // 游戏功能
  achievements: AchievementsConfig
  stats: StatsConfig
  items: ItemsConfig
  mods: ModsConfig
  leaderboards: LeaderboardsConfig

  // 硬件支持
  controller: ControllerConfig
  sounds: SoundConfig
  fonts: FontConfig

  // 其他配置
  installedAppIds: InstalledAppIdsConfig
  subscribedGroups: SubscribedGroupsConfig
  subscribedGroupsClans: SubscribedGroupsClansConfig
  supportedLanguages: SupportedLanguagesConfig
  purchasedKeys: PurchasedKeysConfig
  depots: DepotsConfig
  branches: BranchesConfig
  defaultItems: DefaultItemsConfig
  gameCoordinator: GameCoordinatorConfig

  // 工具配置
  coldClientLoader: ColdClientLoaderConfig
  lobbyConnect: LobbyConnectConfig

  // 资源文件
  avatars: AvatarConfig[]
  steamHttp: SteamHttpConfigItem[]
}

// ============================================
// 14. 默认配置值
// ============================================

export const DEFAULT_MAIN_CONFIG: MainConfig = {
  // [main::general] 默认值
  newAppTicket: true,
  gcToken: true,
  blockUnknownClients: false,
  steamDeck: false,
  enableAccountAvatar: false,
  enableVoiceChat: false,
  immediateGameserverStats: false,
  matchmakingServerListActualType: false,
  matchmakingServerDetailsViaSourceQuery: false,

  // [main::stats] 默认值
  disableLeaderboardsCreateUnknown: false,
  allowUnknownStats: true,
  statAchievementProgressFunctionality: true,
  saveOnlyHigherStatAchievementProgress: true,
  paginatedAchievementsIcons: 10,
  recordPlaytime: false,

  // [main::connectivity] 默认值
  disableLanOnly: false,
  disableNetworking: false,
  listenPort: 47584,
  offline: false,
  disableSharingStatsWithGameserver: false,
  disableSourceQuery: false,
  shareLeaderboardsOverNetwork: false,
  disableLobbyCreation: false,
  downloadSteamhttpRequests: false,

  // [main::misc] 默认值
  achievementsBypass: false,
  forceSteamhttpSuccess: false,
  disableSteamoverlaygameidEnvVar: false,
  enableSteamPreownedIds: false,
  freeWeekend: false,
  use32bitInventoryItemIds: false,

  // 额外DLL
  extraDlls: []
}

export const DEFAULT_USER_CONFIG: UserConfig = {
  username: 'Player',
  language: 'schinese',
  savePath: '%appdata%/GSE Saves',
  useDefaultAvatar: true
}

export const DEFAULT_APP_CONFIG: AppConfig = {
  branchName: 'public',
  appPaths: {},
  dlcs: {
    unlockAll: true,
    individualDlcs: []
  }
}

export const DEFAULT_OVERLAY_CONFIG: OverlayConfig = {
  enableExperimentalOverlay: false,
  hotkey: 'Shift+Tab',
  notifications: {
    achievement: true,
    friend: true,
    message: true,
    duration: 5,
    position: 'bottom-right'
  },
  appearance: {
    theme: 'dark',
    opacity: 0.95,
    scale: 1.0,
    blur: true
  },
  performance: {
    hardwareAcceleration: true,
    fpsLimit: 60,
    lowPerformanceMode: false
  },
  features: {
    achievements: true,
    friends: true,
    chat: true,
    browser: true,
    settings: true
  }
}

export const DEFAULT_ACHIEVEMENTS_CONFIG: AchievementsConfig = {
  enabled: true,
  showNotifications: true,
  achievements: []
}

export const DEFAULT_STATS_CONFIG: StatsConfig = {
  enabled: true,
  stats: []
}

export const DEFAULT_ITEMS_CONFIG: ItemsConfig = {
  enabled: true,
  itemDefinitions: [],
  initialItems: []
}

export const DEFAULT_MODS_CONFIG: ModsConfig = {
  enabled: true,
  subscribedMods: [],
  autoUpdate: false
}

export const DEFAULT_LEADERBOARDS_CONFIG: LeaderboardsConfig = {
  enabled: true,
  leaderboards: []
}

export const DEFAULT_CONTROLLER_CONFIG: ControllerConfig = {
  enabled: true,
  controllerType: 'xbox',
  bindings: [],
  deadzone: {
    leftStick: 0.1,
    rightStick: 0.1,
    leftTrigger: 0.1,
    rightTrigger: 0.1
  },
  rumble: {
    enabled: true,
    intensity: 0.8
  },
  customGlyphs: {
    enabled: false
  }
}

export const DEFAULT_SOUNDS_CONFIG: SoundConfig = {
  enabled: false,
  masterVolume: 1.0
}

export const DEFAULT_FONTS_CONFIG: FontConfig = {
  enabled: false,
  fontSize: 14,
  fontWeight: 'normal'
}

export const DEFAULT_COLD_CLIENT_LOADER_CONFIG: ColdClientLoaderConfig = {
  enabled: false,
  injectionMode: 'direct',
  extraDlls: [],
  launchArgs: ''
}

export const DEFAULT_LOBBY_CONNECT_CONFIG: LobbyConnectConfig = {
  enabled: false,
  autoJoin: false
}
