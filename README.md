# Steam Tool Plus 项目报告

> **💬 工具、游戏交流群**：1095428733
>
> **📥 工具下载地址**：https://pan.baidu.com/s/1EMDn8l2d198Q6Lp3pVTzMQ?pwd=vtbe
>
> 提取码：**vtbe**
>
> **📥 迅雷网盘**：https://pan.xunlei.com/s/VOrkq4Tq0c0Sootmhpp4433yA1?pwd=2tmn#
>
> 提取码：**2tmn**
>
> **📥 夸克网盘**：https://pan.quark.cn/s/bc38612b683f

---

## 目录

1. [项目概述](#一项目概述)
2. [技术架构](#二技术架构)
3. [前端架构详解](#三前端架构详解)
4. [后端架构详解](#四后端架构详解)
5. [核心功能模块](#五核心功能模块)
6. [数据模型设计](#六数据模型设计)
7. [配置系统](#七配置系统)
8. [图片与资源管理](#八图片与资源管理)
9. [主题与个性化](#九主题与个性化)
10. [构建与部署](#十构建与部署)
11. [使用教程](#十一使用教程)
12. [常见问题与故障排除](#十二常见问题与故障排除)
13. [版本历史](#十三版本历史)
14. [版权声明](#十四版权声明)

---

## 一、项目概述

### 1.1 项目简介

**Steam Tool Plus** 是一款基于 **Tauri 2.0 + Vue 3 + TypeScript** 技术栈开发的纯本地 Steam 游戏管理工具。本程序完全绿色便携，无需安装，无需联网（除下载游戏外），所有数据保存在程序运行目录下，实现了真正意义上的即插即用。

本项目的核心定位是：**纯本地、绿色便携、无网络依赖、无付费功能、无服务器连接**。UI 设计 99% 对标 Steam 官方 Windows 客户端，同时完成了视觉与交互的全面升级。程序体积与运行内存经过极致精简，扩展性对标 VS Code / Chrome 插件系统，能够适配老旧核显 Windows 电脑并保证全程 60fps 丝滑运行。

### 1.2 核心特性

| 特性 | 说明 |
|------|------|
| **纯本地运行** | 所有功能 100% 本地执行，除用户手动触发的浏览器跳转外，禁止任何网络请求 |
| **绿色便携** | 单文件 exe，所有数据、配置、扩展保存在程序运行目录，不写入注册表、不写入系统目录 |
| **极致体积** | 基于 Tauri 2.0 不打包 Chromium 内核，打包后 exe 体积 ≤ 10MB |
| **极低内存** | Rust 后端 + 系统 WebView 组合，运行内存稳定 ≤ 50MB，CPU 占用 ≤ 2% |
| **Steam UI 复刻** | 基于 Vue 3 + Tailwind CSS 1:1 复刻 Steam 客户端 UI，支持深色/浅色双主题 |
| **插件化架构** | 核心代码与扩展功能完全隔离，对标 VS Code 的扩展能力 |
| **内存优化** | 图片懒加载 + IntersectionObserver 自动释放，全局缓存服务避免资源竞争 |
| **GPU 加速** | CSS transform 和 will-change 优化，确保老旧核显电脑 60fps 流畅运行 |

### 1.3 系统要求

- **操作系统**：Windows 10/11 64位系统
- **运行环境**：无需 Steam 客户端，无需联网（除下载游戏外）
- **硬件要求**：支持老旧核显电脑，最低 4GB 内存即可流畅运行
- **存储空间**：程序本体 ≤ 10MB，游戏数据根据用户需求另行计算

### 1.4 项目版本

- **当前版本**：v1.24.1
- **更新日期**：2026-05-09
- **开发时间**：2026年
- **版权所有**：© 2026 Steam Tool Plus

---

## 二、技术架构

### 2.1 技术栈选型

本项目严格遵循前后端完全分离、MVVM 架构、依赖注入 DI、接口抽象、插件化架构的设计原则。

#### 前端技术栈

| 技术 | 版本 | 用途 |
|------|------|------|
| **Vue 3** | 3.5.x | 前端框架，Composition API 开发模式 |
| **TypeScript** | 5.6.x | 类型安全，提升代码可维护性 |
| **Vite** | 6.0.x | 构建工具，提供极速的开发体验 |
| **Vue Router** | 4.5.x | 前端路由管理 |
| **Pinia** | 2.3.x | 状态管理，替代 Vuex |
| **Tailwind CSS** | 3.4.x | 原子化 CSS 框架，快速构建 Steam 风格 UI |
| **Tauri API** | 2.2.x | 与 Rust 后端通信的桥接层 |

#### 后端技术栈

| 技术 | 版本 | 用途 |
|------|------|------|
| **Rust** | 1.83+ | 后端核心语言，提供内存安全和极致性能 |
| **Tauri 2.0** | 2.2.x | 跨平台应用框架，提供原生级性能 |
| **Serde** | 1.0.x | JSON 序列化/反序列化 |
| **Tokio** | 1.42.x | 异步运行时 |
| **UUID** | 1.12.x | 全局唯一标识符生成 |
| **Chrono** | 0.4.x | 日期时间处理 |

### 2.2 架构设计原则

本项目严格遵循以下核心设计原则：

1. **单一职责原则**：每个模块、组件、函数只负责一项功能
2. **依赖倒置原则**：所有业务逻辑依赖抽象接口，不依赖具体实现
3. **接口隔离原则**：每个接口只提供必要的方法，无冗余、无多余功能
4. **无硬编码原则**：所有配置、文本、路径、样式参数通过配置文件、常量、主题变量定义
5. **纯本地原则**：所有功能 100% 本地运行，禁止任何网络请求
6. **绿色便携原则**：所有数据、配置、扩展保存在程序运行目录，使用相对路径

### 2.3 项目目录结构

```
SteamToolPlus/
├── src/                          # 前端源代码
│   ├── api/                      # API 层：与 Rust 后端通信
│   │   ├── game.api.ts           # 游戏数据获取 API
│   │   ├── gameData.api.ts       # game.json 数据操作 API
│   │   ├── config.api.ts         # 配置读写 API
│   │   └── background.api.ts     # 背景图片管理 API
│   ├── components/               # 公共组件
│   │   ├── common/               # 通用 UI 组件
│   │   │   ├── Button.vue        # 按钮组件
│   │   │   ├── Dropdown.vue      # 下拉选择器
│   │   │   └── Toggle.vue        # 开关组件
│   │   ├── game/                 # 游戏相关组件
│   │   │   ├── GameCard.vue      # 游戏卡片（懒加载）
│   │   │   ├── GameDetailModal.vue # 游戏详情弹窗
│   │   │   └── GameFormDialog.vue  # 游戏表单对话框
│   │   ├── layout/               # 布局组件
│   │   │   └── TitleBar.vue      # 自定义标题栏
│   │   └── background/           # 背景相关组件
│   │       ├── BackgroundLayer.vue   # 背景图层
│   │       └── BackgroundSettings.vue # 背景设置面板
│   ├── store/                    # Pinia 状态管理
│   │   ├── game.store.ts         # 游戏状态管理
│   │   ├── config.store.ts       # 配置状态管理
│   │   └── theme.store.ts        # 主题状态管理
│   ├── types/                    # TypeScript 类型定义
│   │   ├── index.ts              # 类型导出
│   │   ├── game.types.ts         # 游戏相关类型
│   │   ├── config.types.ts       # 配置相关类型
│   │   └── background.types.ts   # 背景相关类型
│   ├── utils/                    # 工具函数
│   ├── views/                    # 页面视图
│   │   ├── Browse/               # 浏览页面
│   │   ├── Library/              # 游戏库页面
│   │   ├── GameDetail/           # 游戏详情页面
│   │   ├── Download/             # 下载页面
│   │   ├── Patch/                # 补丁注入页面
│   │   ├── GlobalSettings/       # 全局设置页面
│   │   └── Personalization/      # 个性化设置页面
│   ├── App.vue                   # 根组件
│   ├── main.ts                   # 入口文件
│   └── router.ts                 # 路由配置
├── src-tauri/                    # Tauri 后端代码
│   ├── src/                      # Rust 源代码
│   │   ├── commands/             # Tauri 命令（IPC 接口）
│   │   │   ├── game_commands.rs  # 游戏相关命令
│   │   │   ├── config_commands.rs # 配置相关命令
│   │   │   └── background_commands.rs # 背景相关命令
│   │   ├── services/             # 业务逻辑服务层
│   │   │   ├── game_service.rs   # 游戏业务逻辑
│   │   │   ├── config_service.rs # 配置管理服务
│   │   │   └── background_service.rs # 背景图片服务
│   │   ├── models/               # 数据模型
│   │   │   ├── mod.rs            # 模型模块入口
│   │   │   ├── game.rs           # 游戏数据模型
│   │   │   └── config.rs         # 配置数据模型
│   │   ├── utils/                # 工具模块
│   │   │   ├── mod.rs            # 工具模块入口
│   │   │   ├── config_path_utils.rs # 配置路径工具
│   │   │   ├── file_utils.rs     # 文件操作工具
│   │   │   ├── resource_utils.rs # 资源目录查找工具
│   │   │   └── log_utils.rs      # 日志记录工具
│   │   └── main.rs               # Rust 入口文件
│   ├── Cargo.toml                # Rust 依赖配置
│   └── tauri.conf.json           # Tauri 应用配置
├── resources/                    # 资源文件
│   ├── config/                   # 配置文件
│   │   └── games_config.json     # 游戏配置数据
│   ├── pic/                      # 图片资源
│   │   ├── Game_Cover/           # 游戏封面图 (460x215)
│   │   └── 库/                   # 游戏库背景图 (1920x620)
│   └── crack/                    # 补丁资源
│       ├── 免_steam/             # 免 Steam 补丁
│       ├── 局域网联机/           # 局域网联机补丁
│       ├── Steam联机/            # Steam 联机补丁
│       ├── D_加密虚拟机/         # D 加密虚拟机补丁
│       └── epic_联机/            # Epic 联机补丁
├── package.json                  # Node.js 依赖配置
├── vite.config.ts                # Vite 构建配置
├── tailwind.config.js            # Tailwind CSS 配置
└── tsconfig.json                 # TypeScript 配置
```

---

## 三、前端架构详解

### 3.1 入口与路由系统

#### main.ts - 应用入口

应用入口文件负责初始化 Vue 应用实例，注册 Pinia 状态管理、Vue Router 路由，并挂载应用到 DOM。程序启动时会自动加载用户配置，应用保存的主题模式，确保用户每次打开程序都能看到熟悉的界面风格。

#### router.ts - 路由配置

前端采用 Vue Router 4 进行页面导航管理，定义了以下路由：

| 路由路径 | 对应页面 | 功能说明 |
|----------|----------|----------|
| `/` | Browse.vue | 浏览所有游戏 |
| `/library` | Library.vue | 游戏库管理 |
| `/game/:id` | GameDetail.vue | 游戏详情页 |
| `/download` | Download.vue | 游戏本体下载 |
| `/patch` | Patch.vue | 免 Steam 补丁注入 |
| `/settings` | GlobalSettings.vue | 全局设置 |
| `/personalization` | Personalization.vue | 个性化设置 |

### 3.2 状态管理（Pinia Store）

#### game.store.ts - 游戏状态管理

游戏状态管理器负责维护游戏列表、加载状态、当前选中游戏等核心数据。它通过调用 `game.api.ts` 中的 API 函数与 Rust 后端通信，获取 `games_config.json` 中的游戏配置数据。Store 采用响应式设计，当游戏数据发生变化时，所有订阅该状态的组件会自动更新。

#### config.store.ts - 配置状态管理

配置状态管理器负责维护应用程序的全局配置，包括 Steam 路径、启动设置、主题模式等。配置数据采用双存储机制：运行时存储在 `%appdata%/SteamToolPlus/config/` 目录下，同时会在程序根目录的 `config/` 目录保留备份，确保数据安全。

#### theme.store.ts - 主题状态管理

主题状态管理器负责管理应用程序的主题模式（深色/浅色/跟随系统）。它会监听系统主题变化，当设置为"跟随系统"模式时，自动切换深色或浅色主题。主题切换会实时更新 CSS 变量，所有使用主题变量的组件会立即响应变化。

### 3.3 API 层设计

前端 API 层采用分层设计，所有与 Rust 后端的通信都通过 `@tauri-apps/api/core` 的 `invoke` 函数实现。

#### game.api.ts

负责游戏数据的获取，包括：
- 从 `resources/config/games_config.json` 加载游戏配置
- 获取游戏封面图片路径（`resources/pic/Game_Cover/{game_id}.jpg`）
- 获取游戏库背景图片路径（`resources/pic/库/{game_id}.jpg`）
- 图片路径自动转换（使用 `convertFileSrc` 转换为可访问的 URL）

#### gameData.api.ts

负责 `game.json` 数据的读写操作，包括：
- 获取单个游戏数据
- 获取所有游戏数据
- 导入自定义游戏
- 更新游戏信息
- 启动游戏并追踪游玩时长
- 关闭游戏进程
- 切换游戏收藏状态
- 更新游戏下载状态

#### config.api.ts

负责配置文件的读写：
- 加载应用程序配置
- 更新配置项
- 重置为默认配置

#### background.api.ts

负责背景图片的管理：
- 获取背景配置
- 保存背景配置
- 导入背景图片到运行时目录
- 获取背景图片列表
- 删除背景图片

### 3.4 组件系统

#### GameCard.vue - 游戏卡片组件

游戏卡片是浏览页面的核心组件，采用以下优化技术：
- **懒加载**：使用 IntersectionObserver 监听卡片是否进入视口，仅在可见时加载图片
- **内存管理**：当卡片离开视口一定距离后，自动释放图片资源，避免内存泄漏
- **骨架屏**：图片加载前显示骨架屏动画，提升用户体验
- **响应式布局**：使用 CSS Grid 自适应不同屏幕尺寸

#### GameDetailModal.vue - 游戏详情弹窗

游戏详情弹窗使用 Vue 3 的 `<Teleport>` 组件将内容渲染到 body 层级，避免被父元素的 z-index 或 overflow 限制。弹窗包含：
- 游戏封面大图展示
- 补丁类型标签（带颜色区分）
- 网盘下载链接（百度网盘、迅雷网盘、蓝奏云）
- 补丁应用功能（自动备份原文件）
- 游戏启动功能

#### TitleBar.vue - 自定义标题栏

由于采用 Windows 11 原生无边框窗口设计，标题栏完全由前端自定义实现：
- 左侧：应用图标和菜单栏（游戏、工具、更多、设置、帮助）
- 右侧：窗口控制按钮（最小化、最大化/还原、关闭）
- 支持拖拽移动窗口、双击最大化/还原
- 菜单采用下拉式设计，与 Steam 客户端风格一致

### 3.5 图片缓存服务

#### imageCache.service.ts

全局图片缓存服务是解决多组件资源竞争的关键设计。在浏览页面和游戏库页面同时加载大量图片时，如果不加以管理，会导致：
- 同一张图片被多次转换 `convertFileSrc`
- 内存占用持续攀升
- 页面切换时资源释放不及时

图片缓存服务通过维护一个全局的 `Map<string, string>` 缓存表，确保同一张图片只转换一次，所有组件共享缓存。当程序关闭到托盘时，2 秒后自动释放缓存，降低内存占用。

---

## 四、后端架构详解

### 4.1 Rust 入口与命令注册

#### main.rs - Rust 入口

Rust 后端入口文件负责初始化 Tauri 应用，注册所有 IPC 命令，配置窗口属性。程序启动时会执行以下操作：
1. 初始化配置目录（`%appdata%/SteamToolPlus`）
2. 确保配置文件存在
3. 注册所有前端可调用的命令
4. 创建主窗口（1500x900，最小尺寸 1500x900，圆角 12px）

### 4.2 命令层（Commands）

命令层是前端与后端通信的接口，每个命令对应一个前端可调用的函数。

#### game_commands.rs

游戏相关命令包括：
- `get_games_config`：获取游戏配置列表
- `get_game_cover_path`：获取游戏封面路径
- `get_game_library_image_path`：获取游戏库背景图路径
- `apply_patch`：应用补丁到游戏目录
- `check_patch_file_exists`：检查补丁文件是否存在
- `get_patch_readme`：获取补丁说明文档
- `start_game_download`：启动游戏下载
- `stop_download`：停止下载
- `import_game_manifest_to_steam`：导入游戏清单到 Steam
- `restart_steam`：重启 Steam 客户端

#### config_commands.rs

配置相关命令包括：
- `load_config`：加载应用程序配置
- `save_config`：保存配置到文件
- `get_config_path`：获取配置文件路径

#### background_commands.rs

背景相关命令包括：
- `get_background_config`：获取背景配置
- `save_background_config`：保存背景配置
- `import_background_image`：导入背景图片
- `get_background_images`：获取背景图片列表
- `delete_background_image`：删除背景图片

### 4.3 服务层（Services）

服务层包含核心业务逻辑，负责处理数据的读取、写入、转换等操作。

#### game_service.rs

游戏业务逻辑服务：
- 读取 `resources/config/games_config.json` 文件
- 解析游戏配置数据（支持中文名、英文名、游戏 ID、补丁类型等）
- 图片路径解析和验证
- 补丁路径自动计算（根据 `patch_type` 和 `game_id`）

补丁路径计算规则：
| patch_type | 路径 |
|------------|------|
| 0 (免Steam) | `Resources/crack/免_steam/{game_id}` |
| 1 (局域网联机) | `Resources/crack/局域网联机/{game_id}` |
| 2 (Steam联机) | `Resources/crack/Steam联机/{game_id}` |
| 3 (D加密虚拟机) | `Resources/crack/D_加密虚拟机/{game_id}` |
| 4 (epic联机) | `Resources/crack/epic_联机/{game_id}` |

#### config_service.rs

配置管理服务：
- 运行时目录管理（`%appdata%/SteamToolPlus`）
- 备份目录管理（程序根目录 `config/`）
- 配置读写（JSON 格式）
- 配置验证和默认值处理

#### background_service.rs

背景图片服务：
- 背景配置文件管理（`background.json`）
- 图片文件复制到运行时目录
- 图片列表维护
- 配置验证

### 4.4 工具模块（Utils）

#### config_path_utils.rs

配置路径工具提供统一的配置目录管理：
- `get_config_dir()`：获取运行时配置目录（`%appdata%/SteamToolPlus/config/`）
- `get_backup_config_dir()`：获取备份配置目录（程序根目录 `config/`）
- `ensure_config_dir()`：确保配置目录存在
- `get_runtime_dir()`：获取运行时数据目录

#### file_utils.rs

文件操作工具：
- `read_text_file()`：读取文本文件
- `read_json_file()`：读取 JSON 文件
- `write_json_file()`：写入 JSON 文件
- `ensure_dir_exists()`：确保目录存在
- `copy_file()`：复制文件

#### resource_utils.rs

资源目录查找工具：
- `get_resource_dir()`：获取程序资源目录
- `get_app_dir()`：获取程序根目录
- 支持开发模式和生产模式的路径切换

#### log_utils.rs

日志记录工具：
- 支持 JSON 格式和文本格式双模式
- 自动按日期分割日志文件
- 日志级别控制（Info、Warn、Error）

### 4.5 数据模型（Models）

#### game.rs

游戏数据模型定义了所有与游戏相关的数据结构：

```rust
// 游戏数据结构
pub struct Game {
    pub id: String,              // 唯一ID，UUID生成
    pub name: String,            // 游戏名称
    pub exe_path: String,        // 游戏exe文件路径
    pub cover_path: Option<String>, // 封面图路径
    pub launch_params: String,   // 启动参数
    pub publisher: String,       // 发行商
    pub release_date: String,    // 发行日期
    pub tags: Vec<String>,       // 标签/分类
    pub is_installed: bool,      // 是否安装
    pub is_favorite: bool,       // 是否收藏
    pub total_play_time: u64,    // 总游玩时长（秒）
    pub last_play_time: Option<String>, // 最近游玩时间
    pub create_time: String,     // 添加时间
}

// 游戏标签配置（来自games_config.json）
pub struct GameTagConfig {
    pub patch_type: u8,          // 补丁类型
    pub patch_source_path: Option<String>, // 补丁源路径
    pub download_url: Option<String>, // 下载链接
    pub download_urls: Option<Vec<DownloadUrlConfig>>, // 多网盘链接
}

// 游戏配置数据
pub struct GameConfigData {
    pub game_id: String,         // 游戏ID（Steam App ID）
    pub game_name: String,       // 英文游戏名称
    pub chinese_name: String,    // 中文游戏名称
    pub downloadable: bool,      // 是否可下载
    pub tags: Vec<GameTagConfig>, // 补丁标签列表
}
```

#### config.rs

配置数据模型定义了应用程序的配置结构：

```rust
pub struct AppConfig {
    pub version: String,         // 配置版本
    pub theme: ThemeConfig,      // 主题配置
    pub gameDirs: GameDirsConfig, // 游戏目录配置
    pub launch: LaunchConfig,    // 启动配置
    pub window: WindowConfig,    // 窗口配置
}

pub struct ThemeConfig {
    pub mode: String,            // dark/light/auto
    pub followSystem: bool,      // 是否跟随系统
    pub customVars: HashMap<String, String>, // 自定义CSS变量
}

pub struct GameDirsConfig {
    pub steamPath: String,       // Steam安装路径
    pub coversPath: String,      // 封面图路径
}

pub struct LaunchConfig {
    pub startMinimizedToTray: bool, // 启动后最小化到托盘
    pub hideToTrayOnClose: bool,    // 关闭后隐藏到托盘
    pub verifyBeforeLaunch: bool,   // 启动前验证
}
```

---

## 五、核心功能模块

### 5.1 浏览页面（Browse）

浏览页面是程序的主界面，展示所有可下载的游戏。

#### 功能特性

- **游戏卡片网格**：使用 CSS Grid 布局，自适应不同屏幕尺寸
- **搜索功能**：支持按中文名、英文名、游戏 ID 搜索
- **分页加载**：每页 16 张卡片，避免一次性加载过多资源
- **状态保持**：记住当前页码和搜索关键词，返回时恢复
- **懒加载**：图片使用 IntersectionObserver 懒加载，进入视口才加载
- **内存管理**：离开视口的卡片自动释放图片资源

#### 技术实现

游戏卡片组件（GameCard.vue）采用以下优化策略：
1. 使用 `IntersectionObserver` 监听卡片是否进入视口
2. 进入视口时加载图片，离开视口一定距离后释放图片
3. 图片加载前显示骨架屏动画
4. 使用 `transform` 和 `will-change` 实现 GPU 加速的悬浮效果

### 5.2 游戏库页面（Library）

游戏库页面管理已下载或导入的游戏，采用左右分栏布局。

#### 功能特性

- **左侧游戏列表**：紧凑卡片设计，显示游戏缩略图和名称
- **右侧详情面板**：选中游戏后显示大图背景和游戏信息
- **收藏功能**：支持收藏游戏，收藏的游戏显示在列表顶部
- **右键菜单**：支持收藏、编辑、浏览本地文件、卸载等操作
- **游戏启动**：点击"开始游戏"按钮启动游戏，自动记录游玩时长
- **进程监控**：使用 `requestAnimationFrame` 实时更新当前游玩时长
- **智能检测**：每 10 秒检查一次游戏进程状态，自动保存游玩时长

#### 技术实现

游戏进程监控采用双线程设计：
1. **UI 更新线程**：使用 `requestAnimationFrame` 每 500ms 更新一次显示
2. **状态检查线程**：使用 `setInterval` 每 10 秒检查一次进程是否仍在运行
3. 游戏结束时自动计算并保存游玩时长到 `game.json`

### 5.3 游戏详情页面（GameDetail）

游戏详情页面展示单个游戏的详细信息和操作选项。

#### 功能特性

- **游戏封面展示**：左侧显示游戏封面大图
- **路径选择**：选择游戏安装路径和下载路径
- **多标签页**：根据游戏支持的补丁类型显示不同标签页
- **补丁应用**：自动备份原文件，应用补丁，显示操作结果
- **下载功能**：支持从网盘下载补丁文件
- **入库 Steam**：将游戏清单导入 Steam 客户端

#### 补丁应用流程

1. 检查本地补丁文件是否存在
2. 如果不存在，提供网盘下载链接
3. 用户选择游戏路径
4. 点击"应用补丁"按钮
5. 后端自动备份原文件到 `backup/` 目录
6. 复制补丁文件到游戏目录
7. 显示操作结果（成功/失败的文件列表）

### 5.4 下载页面（Download）

下载页面支持单游戏下载和批量下载两种模式。

#### 功能特性

- **单游戏下载**：选择清单文件夹，自动解析游戏信息，设置下载路径
- **批量下载**：选择包含多个游戏清单的父文件夹，批量下载
- **自动路径**：优先使用 D 盘，自动创建 `SteamGame/游戏名` 目录
- **进度监控**：实时监控下载进度，显示每个 depot 的完成百分比
- **日志输出**：实时显示下载日志，支持 info/success/error/warning 级别
- **自动关机**：支持下载完成后自动关机

#### 下载流程

1. 选择包含 `.vdf` 和 `.manifest` 文件的清单文件夹
2. 程序自动验证清单文件，提取游戏 ID 和名称
3. 自动设置下载路径（优先 D 盘）
4. 点击"开始下载"启动 `ddv20.exe` 进程
5. 监控进度文件（`{百分比}% - {depotId}.json`）
6. 所有 depot 下载完成后自动清理进度文件

### 5.5 补丁注入页面（Patch）

补丁注入页面为游戏注入模拟 Steam 运行环境。

#### 功能特性

- **模拟器模式选择**：标准模式（steam_api.dll）/ 高级模式（steamclient.dll）
- **DLL 版本选择**：稳定版 / 功能版（含 Overlay）
- **Steam 脱壳**：使用 Steamless 移除 Steam 壳保护
- **高级功能配置**：局域网联机、Overlay、成就、物品、控制器等
- **配置状态追踪**：显示各功能是否已配置

#### 基础配置流程

1. 选择游戏文件夹（包含 steam_api.dll）
2. 选择模拟器模式（标准/高级）
3. 选择 DLL 版本（稳定版/功能版）
4. 配置 Steam App ID（从 SteamDB 查询）
5. 点击"应用配置"
6. 程序自动创建 `steam_settings` 文件夹和配置文件

### 5.6 全局设置页面（GlobalSettings）

全局设置页面配置应用程序的基本参数。

#### 功能特性

- **Steam 路径设置**：配置 Steam 安装目录，用于清单入库
- **启动设置**：
  - 启动后最小化到托盘
  - 关闭程序后隐藏在托盘（2 秒后自动释放图片缓存）
- **恢复默认**：一键恢复所有设置为默认值

### 5.7 个性化设置页面（Personalization）

个性化设置页面配置应用程序的外观和背景。

#### 功能特性

- **主题模式**：深色 / 浅色 / 跟随系统
- **背景图片设置**：
  - 为不同页面设置背景（浏览、库、下载、补丁、设置）
  - 支持浅色/深色模式分别设置
  - 显示模式：单张 / 轮播 / 随机
  - 视觉效果：模糊程度、暗化程度
  - 切换动画：淡入淡出 / 滑动 / 缩放
- **图片管理**：添加、删除、预览背景图片

#### 背景系统技术实现

背景系统采用分层设计：
1. **BackgroundLayer.vue**：背景图层组件，负责渲染背景图片
2. **BackgroundSettings.vue**：背景设置面板，管理配置
3. **配置存储**：背景配置保存在 `%appdata%/SteamToolPlus/config/background.json`
4. **图片存储**：背景图片复制到 `%appdata%/SteamToolPlus/resources/pic/background/`

---

## 六、数据模型设计

### 6.1 游戏配置数据（games_config.json）

游戏配置数据存储在 `resources/config/games_config.json`，采用 JSON 格式：

```json
{
  "version": "1.0",
  "games": [
    {
      "game_id": "1234560",
      "game_name": "Game Name",
      "chinese_name": "游戏中文名",
      "downloadable": true,
      "tags": [
        {
          "patch_type": 0,
          "download_urls": [
            {
              "source": "baidu",
              "url": "https://pan.baidu.com/s/xxxxx",
              "pwd": "abcd"
            },
            {
              "source": "thunder",
              "url": "https://pan.xunlei.com/s/xxxxx",
              "pwd": "1234"
            }
          ]
        }
      ]
    }
  ]
}
```

### 6.2 用户游戏数据（game.json）

用户游戏数据存储在 `%appdata%/SteamToolPlus/config/game.json`，记录用户导入或下载的游戏：

```json
{
  "games": [
    {
      "game_id": "1234560",
      "game_name": "Game Name",
      "chinese_name": "游戏中文名",
      "game_type": "downloaded",
      "install_path": "D:\\SteamGame\\Game Name",
      "exe_path": "D:\\SteamGame\\Game Name\\game.exe",
      "save_path": "C:\\Users\\User\\Documents\\Game Saves",
      "cover_path": "",
      "steam_game_id": "123456",
      "play_time": 120,
      "is_installed": true,
      "is_favorite": false,
      "added_date": "2026-01-01T00:00:00Z",
      "last_played": "2026-01-15T10:30:00Z",
      "download_status": "completed",
      "download_progress": 100,
      "download_path": "D:\\SteamGame\\Game Name"
    }
  ]
}
```

### 6.3 应用程序配置（config.json）

应用程序配置存储在 `%appdata%/SteamToolPlus/config/config.json`：

```json
{
  "version": "1.0",
  "theme": {
    "mode": "auto",
    "followSystem": true,
    "customVars": {}
  },
  "gameDirs": {
    "steamPath": "C:\\Program Files (x86)\\Steam",
    "coversPath": "data/covers"
  },
  "launch": {
    "startMinimizedToTray": false,
    "hideToTrayOnClose": true,
    "verifyBeforeLaunch": false
  },
  "window": {
    "width": 1500,
    "height": 900,
    "maximized": false
  }
}
```

### 6.4 背景配置（background.json）

背景配置存储在 `%appdata%/SteamToolPlus/config/background.json`：

```json
{
  "version": "1.0",
  "images": [
    {
      "id": "uuid",
      "filename": "bg1.jpg",
      "originalName": "background.jpg",
      "importedAt": "2026-01-01T00:00:00Z"
    }
  ],
  "configs": {
    "browse": {
      "light": {
        "imageIds": ["uuid1"],
        "mode": "single",
        "blur": 0,
        "darken": 0,
        "animation": "fade"
      },
      "dark": {
        "imageIds": ["uuid2"],
        "mode": "carousel",
        "blur": 5,
        "darken": 30,
        "animation": "slide"
      }
    }
  }
}
```

---

## 七、配置系统

### 7.1 双存储机制

本项目采用双存储机制确保数据安全：

1. **运行时目录**：`%appdata%/SteamToolPlus/`
   - 程序运行时读写的主目录
   - 包含 config、resources/pic/background 等子目录
   - 适合存储用户数据、缓存、运行时配置

2. **备份目录**：程序根目录 `config/`
   - 作为运行时目录的备份
   - 程序启动时自动同步
   - 确保程序移动到另一台电脑时配置不丢失

### 7.2 配置加载流程

1. 程序启动时检查运行时配置目录是否存在
2. 如果不存在，从备份目录复制配置
3. 如果备份目录也不存在，创建默认配置
4. 配置加载后存入 Pinia Store，供全局使用
5. 配置变更时同时写入运行时目录和备份目录

### 7.3 主题配置

主题配置支持三种模式：

| 模式 | 说明 |
|------|------|
| **dark** | 强制使用深色主题 |
| **light** | 强制使用浅色主题 |
| **auto** | 自动跟随系统主题变化 |

主题切换通过更新 CSS 变量实现，所有使用主题变量的组件会立即响应变化，无需刷新页面。

---

## 八、图片与资源管理

### 8.1 图片目录结构

```
resources/pic/
├── Game_Cover/           # 游戏封面图 (460x215)
│   ├── 1234560.jpg
│   └── 1234561.jpg
└── 库/                   # 游戏库背景图 (1920x620)
    ├── 1234560.jpg
    └── 1234561.jpg
```

### 8.2 图片加载策略

1. **浏览页面**：使用 460x215 封面图，懒加载
2. **游戏库**：优先使用 1920x620 库背景图，其次使用封面图
3. **游戏详情**：显示大尺寸封面图
4. **背景图片**：存储在运行时目录，支持动态切换

### 8.3 全局缓存服务

图片缓存服务（`imageCache.service.ts`）解决以下问题：

- **资源竞争**：浏览页面和游戏库页面同时加载同一张图片时，避免重复转换 `convertFileSrc`
- **内存管理**：程序关闭到托盘时，2 秒后自动释放缓存
- **路径转换**：将文件系统路径转换为前端可访问的 URL

---

## 九、主题与个性化

### 9.1 CSS 变量系统

本项目使用 CSS 变量实现主题切换，所有颜色值通过变量定义：

```css
:root {
  /* 背景色 */
  --steam-bg-primary: #1b2838;
  --steam-bg-secondary: #171a21;
  --steam-bg-tertiary: #2a475e;
  
  /* 文字色 */
  --steam-text-primary: #c7d5e0;
  --steam-text-secondary: #66c0f4;
  --steam-text-muted: #8f98a0;
  
  /* 强调色 */
  --steam-accent-blue: #66c0f4;
  --steam-accent-green: #5c7e10;
  --steam-accent-red: #ff4d4f;
  
  /* 边框 */
  --steam-border: #2a475e;
  --steam-border-light: #3d6a8e;
}
```

### 9.2 深色/浅色主题

深色主题和浅色主题通过切换 CSS 变量的值实现。当用户选择"跟随系统"模式时，程序会监听系统的 `prefers-color-scheme` 媒体查询，自动切换主题。

### 9.3 背景系统

背景系统支持以下配置：

| 配置项 | 说明 |
|--------|------|
| **页面** | 浏览、库、下载、补丁、设置 |
| **模式** | 浅色、深色 |
| **显示模式** | 单张、轮播、随机 |
| **模糊程度** | 0-20px |
| **暗化程度** | 0%-100% |
| **切换动画** | 淡入淡出、滑动、缩放 |

---

## 十、构建与部署

### 10.1 开发环境

```bash
# 安装依赖
npm install

# 启动开发服务器
npm run tauri dev

# 构建生产版本
npm run tauri build
```

### 10.2 生产构建

生产构建使用 Vite 打包前端资源，Tauri 打包 Rust 后端和 WebView2 运行时：

1. Vite 将 Vue/TypeScript 代码打包为静态资源
2. Tauri 将 Rust 代码编译为可执行文件
3. 前端资源嵌入到可执行文件中
4. 最终输出单个 exe 文件（≤ 10MB）

### 10.3 部署方式

本程序为绿色便携软件，部署方式极其简单：

1. 将打包后的 exe 文件复制到任意目录
2. 确保同级目录存在 `resources/` 文件夹（包含配置和图片）
3. 双击 exe 即可运行
4. 所有数据自动保存在程序运行目录下

---

## 十一、使用教程

### 11.1 首次使用

1. **下载程序**：从网盘下载最新版本
2. **解压文件**：将程序解压到任意目录（建议纯英文路径）
3. **运行程序**：双击 `SteamToolPlus.exe`
4. **配置 Steam 路径**（可选）：进入【设置】→【全局设置】配置 Steam 安装目录

### 11.2 浏览和下载游戏

1. 点击顶部菜单 **游戏** → **浏览**
2. 在搜索框输入游戏名称查找游戏
3. 点击游戏卡片进入详情页
4. 查看游戏信息和下载链接
5. 从网盘下载清单文件（manifest）
6. 进入【工具】→【本体下载】，选择清单文件夹
7. 点击"开始下载"下载游戏本体

### 11.3 应用补丁

1. 下载游戏完成后，进入【工具】→【免Steam补丁】
2. 选择游戏文件夹（包含 steam_api.dll）
3. 选择模拟器模式（标准/高级）
4. 输入游戏的 Steam App ID
5. 点击"应用配置"
6. 从【游戏】→【库】启动游戏

### 11.4 导入已有游戏

1. 进入【游戏】→【库】
2. 点击左侧搜索框旁的"+"按钮
3. 选择游戏安装目录和主程序 exe
4. 填写游戏信息（名称、Steam ID 等）
5. 点击"导入"

### 11.5 清单入库

1. 进入【工具】→【清单入库】
2. 选择 Steam 安装目录
3. 选择包含清单文件的文件夹或压缩包
4. 点击"开始清单入库"
5. 入库成功后重启 Steam

---

## 十二、常见问题与故障排除

### Q1: 程序无法启动？

- 确保系统为 Windows 10/11 64位
- 检查是否安装了 WebView2 运行时
- 尝试以管理员身份运行
- 检查杀毒软件是否拦截

### Q2: 游戏下载进度卡在 0%？

- 检查网络连接是否正常
- 确认清单文件夹路径正确，包含有效的 manifest 文件
- 检查磁盘空间是否充足
- 尝试更换下载路径到其他磁盘

### Q3: 补丁应用后游戏无法启动？

- 检查是否选择了正确的游戏文件夹
- 检查游戏是否放到纯英文路径
- 确保系统杀毒软件没有隔离删除补丁文件的 dll
- 尝试切换标准模式/高级模式
- 尝试切换稳定版/功能版

### Q4: 如何恢复原始游戏文件？

- 补丁应用时会自动创建 backup 文件夹
- 手动从 backup 文件夹复制原文件覆盖即可

### Q5: 游戏库不显示游戏？

- 检查游戏数据文件 `%appdata%/SteamToolPlus/config/game.json` 是否存在
- 尝试重新导入游戏
- 检查游戏 exe 路径是否正确

### Q6: 背景图片无法显示？

- 确保图片格式为 JPG、PNG 或 WebP
- 检查图片文件是否损坏
- 尝试重新导入背景图片
- 删除 `%appdata%/SteamToolPlus/config/background.json` 后重启程序

### Q7: 程序打开后显示空白/透明窗口？

- 确保程序根目录下存在 `resources/pic/background/` 背景图片
- 检查是否有杀毒软件拦截了程序
- 尝试删除 `%appdata%/SteamToolPlus/config/` 目录后重新启动程序

### Q8: 如何获取游戏的 Steam App ID？

- 访问 [SteamDB](https://steamdb.info/)
- 搜索游戏名称
- 查看页面上的 App ID
- 或在 Steam 商店页面 URL 中查看数字部分

### Q9: 局域网联机无法找到其他玩家？

- 确保所有玩家在同一局域网内
- 检查防火墙设置，允许程序通过
- 配置正确的广播地址（如 `192.168.1.0/24`）
- 确保所有玩家使用相同的补丁版本

---

## 十三、版本历史

### v1.24.1 (2026-05-09)

- **新增功能**：
  - 首次使用配置弹窗，引导新用户快速上手
  - 二维码弹窗功能，方便手机扫码访问网盘链接
  - 批量下载模式，支持同时下载多个游戏
  - 游戏详情页支持多种补丁类型标签页切换
- **功能优化**：
  - 优化下载页面UI，新增网盘快捷入口
  - 改进游戏详情页路径选择交互
  - 优化补丁配置管理器，支持更多高级选项
- **问题修复**：
  - 修复部分游戏补丁应用失败的问题
  - 修复下载进度显示异常的问题
  - 修复背景图片切换时的闪烁问题

### v1.23.02 (2026-01-22)

- 添加二维码弹窗组件
- 优化首次使用引导流程
- 修复已知Bug

### v1.22 (2026-01-20)

- 优化游戏库管理功能
- 改进补丁注入流程
- 修复已知问题

### v1.21 (2026-01-18)

- 添加游戏编辑功能
- 优化下载页面
- 改进清单入库功能

### v1.20 (2026-01-15)

- 初始版本发布
- 实现游戏浏览、下载、补丁注入核心功能
- 支持深色/浅色双主题
- 支持背景图片个性化设置
- 支持游戏库管理和游玩时长统计

---

## 十四、版权声明

**Steam Tool Plus** 是一款开源的学习交流工具。

- **版权所有**：© 2026 Steam Tool Plus
- **许可协议**：MIT License
- **免责声明**：本工具仅供学习交流使用，请支持正版游戏。使用本工具下载的游戏请确保您已拥有该游戏的正版授权。

---

**技术支持**：如有问题或建议，请通过以下方式联系：
- **💬 QQ群**：1095428733
- 百度网盘：https://pan.baidu.com/s/1EMDn8l2d198Q6Lp3pVTzMQ?pwd=vtbe
- 迅雷网盘：https://pan.xunlei.com/s/VOrkq4Tq0c0Sootmhpp4433yA1?pwd=2tmn#

---

*本文档最后更新于 2026-05-09*
