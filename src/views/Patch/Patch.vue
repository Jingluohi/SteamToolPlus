<template>
  <div class="steam-patch-inject-view">
    <!-- 页面标题 -->
    <div class="page-header">
      <h1 class="page-title">
        免 Steam 补丁注入
        <a
          href="https://steamdb.info/"
          target="_blank"
          class="steamdb-link"
          title="打开 SteamDB"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>
            <polyline points="15 3 21 3 21 9"/>
            <line x1="10" y1="14" x2="21" y2="3"/>
          </svg>
          SteamDB
        </a>
      </h1>
      <p class="page-desc">该功能可以实现游戏免Steam启动，还有局域网联机、成就系统、overlay显示等功能，将在基础配置之后显示</p>
    </div>

    <!-- 基础配置区域 -->
    <div class="basic-config-section">
      <div class="config-card">
        <div class="config-header">
          <h3 class="config-title">
            <svg class="config-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 2L2 7l10 5 10-5-10-5z"/>
              <path d="M2 17l10 5 10-5"/>
              <path d="M2 12l10 5 10-5"/>
            </svg>
            基础配置
          </h3>
          <!-- 右侧配置区域 -->
          <div class="config-options-wrapper">
            <!-- 模拟器模式选择 -->
            <div class="emulator-mode-section">
              <label class="mode-label">模拟器模式：</label>
              <div class="mode-options">
                <label 
                  class="mode-radio" 
                  :class="{ active: emulatorMode === 0 }"
                  @mouseenter="showTooltip('standard')"
                  @mouseleave="hideTooltip"
                >
                  <input type="radio" v-model="emulatorMode" :value="0" class="mode-input" />
                  <span class="mode-title">标准模式</span>
                  <!-- 悬浮提示 -->
                  <div class="mode-tooltip">
                    <div class="tooltip-content">
                      替换 steam_api.dll 或 steam_api64.dll<br>
                      适用于 90% 以上的 Steam 游戏<br>
                      <strong>使用场景：</strong><br>
                      • 游戏目录中有 steam_api.dll<br>
                      • 常规 Steam 游戏<br>
                      • 推荐首选此模式
                    </div>
                  </div>
                </label>
                <label 
                  class="mode-radio" 
                  :class="{ active: emulatorMode === 1 }"
                  @mouseenter="showTooltip('advanced')"
                  @mouseleave="hideTooltip"
                >
                  <input type="radio" v-model="emulatorMode" :value="1" class="mode-input" />
                  <span class="mode-title">高级模式</span>
                  <!-- 悬浮提示 -->
                  <div class="mode-tooltip">
                    <div class="tooltip-content">
                      替换 steamclient.dll 或 steamclient64.dll<br>
                      同时同步替换 steam_api.dll<br>
                      <strong>使用场景：</strong><br>
                      • 游戏目录中找不到 steam_api.dll<br>
                      • 只有 steamclient.dll 的游戏<br>
                      • 常规模式配置后仍无法启动<br>
                      • 已打 CPY 等破解补丁的游戏
                    </div>
                  </div>
                </label>
              </div>
            </div>

            <!-- 标准模式下的 DLL 版本选择 -->
            <div class="dll-version-section" v-if="emulatorMode === 0">
              <label class="dll-version-label">DLL 版本：</label>
              <div class="dll-version-options">
                <label 
                  class="dll-radio" 
                  :class="{ active: !useExperimental }"
                  @mouseenter="showTooltip('stable')"
                  @mouseleave="hideTooltip"
                >
                  <input type="radio" v-model="useExperimental" :value="false" class="dll-input" />
                  <span class="dll-name">稳定版</span>
                  <!-- 悬浮提示 -->
                  <div class="dll-tooltip">
                    <div class="tooltip-content">
                      推荐大多数用户使用<br>
                      <strong>特点：</strong><br>
                      • 兼容性好，稳定性高<br>
                      • 无 Overlay 等高级功能<br>
                      • 游戏启动速度快<br>
                      • 极少出现崩溃问题
                    </div>
                  </div>
                </label>
                <label 
                  class="dll-radio" 
                  :class="{ active: useExperimental }"
                  @mouseenter="showTooltip('feature')"
                  @mouseleave="hideTooltip"
                >
                  <input type="radio" v-model="useExperimental" :value="true" class="dll-input" />
                  <span class="dll-name">功能版</span>
                  <!-- 悬浮提示 -->
                  <div class="dll-tooltip">
                    <div class="tooltip-content">
                      包含更多高级功能<br>
                      <strong>特点：</strong><br>
                      • 支持游戏内 Overlay（Shift+Tab）<br>
                      • 可查看成就、好友、联机状态<br>
                      • 支持更完整的 Steam 功能模拟<br>
                      • 部分游戏可能出现兼容性问题
                    </div>
                  </div>
                </label>
              </div>
            </div>
          </div>
        </div>

        <!-- 实验版警告弹窗 -->
        <div v-if="showExperimentalWarning" class="modal-overlay" @click="showExperimentalWarning = false">
          <div class="modal-content warning-modal" @click.stop>
            <div class="modal-header">
              <svg class="warning-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
                <line x1="12" y1="9" x2="12" y2="13"/>
                <line x1="12" y1="17" x2="12.01" y2="17"/>
              </svg>
              <h4>实验版使用警告</h4>
            </div>
            <div class="modal-body">
              <p>实验版包含以下高级功能，但可能存在稳定性问题：</p>
              <ul>
                <li>Steam 客户端模拟（steamclient.dll）</li>
                <li>游戏内 Overlay（Shift+Tab）</li>
                <li>高级联机功能</li>
                <li>控制器支持增强</li>
              </ul>
              <p class="warning-text">⚠️ 某些游戏可能无法正常运行，建议先尝试常规版。</p>
            </div>
            <div class="modal-footer">
              <button class="btn-secondary" @click="showExperimentalWarning = false">我知道了</button>
            </div>
          </div>
        </div>

        <!-- 游戏文件夹路径配置 -->
        <div class="config-group">
          <label class="config-label">游戏文件夹 <span class="required">*</span></label>
          <div class="path-input-group">
            <input 
              type="text" 
              v-model="gamePath" 
              class="path-input" 
              :placeholder="emulatorMode === 0 ? '请选择包含 steam_api.dll 的文件夹...' : '请选择包含 steamclient.dll 的文件夹...'" 
              readonly 
            />
            <button class="browse-btn" @click="selectGameFolder">浏览</button>
          </div>
          <p class="config-hint">
            {{ emulatorMode === 0 ? '选择包含 steam_api.dll 或 steam_api64.dll 的游戏文件夹' : '选择包含 steamclient.dll 或 steamclient64.dll 的游戏文件夹' }}
          </p>
        </div>

        <!-- 游戏主程序路径配置 -->
        <div class="config-group">
          <label class="config-label">游戏主程序 <span class="optional">(可选)</span></label>
          <div class="path-input-group">
            <input type="text" v-model="gameExePath" class="path-input" placeholder="请选择游戏主程序 .exe 文件..." readonly />
            <button class="browse-btn" @click="selectGameExe">浏览</button>
            <button v-if="gameExePath" class="clear-btn" @click="gameExePath = ''">清除</button>
          </div>
          <p class="config-hint">选择游戏主程序 exe 文件，用于 Steam 脱壳（某些游戏需要先脱壳）</p>
        </div>

        <!-- 脱壳功能 -->
        <div class="config-group">
          <div class="unpack-header">
            <label class="config-label">Steam 脱壳</label>
            <button
              class="unpack-btn"
              :class="{ loading: isUnpacking, success: unpackSuccess, error: unpackError }"
              :disabled="isUnpacking || !gameExePath"
              @click="unpackGameExe"
            >
              <svg v-if="isUnpacking" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
              </svg>
              <svg v-else-if="unpackSuccess" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="20 6 9 17 4 12"/>
              </svg>
              <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                <polyline points="7 10 12 15 17 10"/>
                <line x1="12" y1="15" x2="12" y2="3"/>
              </svg>
              {{ isUnpacking ? '脱壳中...' : unpackSuccess ? '脱壳成功' : '开始脱壳' }}
            </button>
          </div>
          <p class="config-hint">使用 Steamless 移除 Steam 壳保护，自动备份原文件为 .bak，脱壳后自动替换</p>
          <div v-if="unpackMessage" class="unpack-message" :class="{ error: unpackError, success: unpackSuccess }">
            {{ unpackMessage }}
          </div>
        </div>

        <!-- Steam AppID 配置 -->
        <div class="config-group">
          <label class="config-label">Steam AppID <span class="required">*</span></label>
          <input type="text" v-model="steamAppId" class="config-input" placeholder="输入游戏 Steam AppID..." />
          <p class="config-hint">游戏的 Steam AppID，可从 SteamDB 查询</p>
        </div>

        <!-- 操作按钮 -->
        <div class="config-actions">
          <button class="btn-primary" :disabled="!canApplyBasicConfig" @click="applyBasicConfig">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
              <polyline points="17 21 17 13 7 13 7 21"/>
              <polyline points="7 3 7 8 15 8"/>
            </svg>
            应用基础配置
          </button>
          <button class="btn-secondary" @click="resetBasicConfig">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="23 4 23 10 17 10"/>
              <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"/>
            </svg>
            重置
          </button>
        </div>
      </div>
    </div>

    <!-- 高级功能子菜单 -->
    <div v-if="basicConfigApplied" class="advanced-features-section">
      <div class="section-card">
        <h3 class="section-title">
          <svg class="section-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/>
          </svg>
          高级功能配置
        </h3>
        <p class="section-desc">基础配置已应用，点击下方按钮配置更多功能</p>

        <div class="feature-grid">
          <!-- 局域网联机 -->
          <div class="feature-card" @click="openLanMultiplayerConfig">
            <div class="feature-icon lan">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
                <circle cx="9" cy="7" r="4"/>
                <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
                <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
              </svg>
            </div>
            <h4 class="feature-title">局域网联机</h4>
            <p class="feature-desc">配置局域网多人游戏，自定义广播IP</p>
            <span class="feature-status" :class="{ configured: configStatus.lanMultiplayer }">
              {{ configStatus.lanMultiplayer ? '已配置' : '未配置' }}
            </span>
          </div>

          <!-- Overlay -->
          <div class="feature-card" @click="openOverlayConfig">
            <div class="feature-icon overlay">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="2" y="3" width="20" height="14" rx="2" ry="2"/>
                <line x1="8" y1="21" x2="16" y2="21"/>
                <line x1="12" y1="17" x2="12" y2="21"/>
              </svg>
            </div>
            <h4 class="feature-title">游戏内 Overlay</h4>
            <p class="feature-desc">启用 Shift+Tab 游戏内覆盖界面（实验版功能）</p>
            <span class="feature-status" :class="{ configured: configStatus.overlay, disabled: !useExperimental }">
              {{ !useExperimental ? '需要实验版' : configStatus.overlay ? '已配置' : '未配置' }}
            </span>
          </div>

          <!-- 成就系统 -->
          <div class="feature-card" @click="openAchievementsConfig">
            <div class="feature-icon achievements">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="8" r="7"/>
                <polyline points="8.21 13.89 7 23 12 20 17 23 15.79 13.88"/>
              </svg>
            </div>
            <h4 class="feature-title">成就系统</h4>
            <p class="feature-desc">配置游戏成就和解锁通知</p>
            <span class="feature-status" :class="{ configured: configStatus.achievements }">
              {{ configStatus.achievements ? '已配置' : '未配置' }}
            </span>
          </div>

          <!-- 物品/库存 -->
          <div class="feature-card" @click="openItemsConfig">
            <div class="feature-icon items">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>
                <polyline points="3.27 6.96 12 12.01 20.73 6.96"/>
                <line x1="12" y1="22.08" x2="12" y2="12"/>
              </svg>
            </div>
            <h4 class="feature-title">物品与库存</h4>
            <p class="feature-desc">配置游戏物品和初始库存</p>
            <span class="feature-status" :class="{ configured: configStatus.items }">
              {{ configStatus.items ? '已配置' : '未配置' }}
            </span>
          </div>

          <!-- 控制器配置 -->
          <div class="feature-card" @click="openControllerConfig">
            <div class="feature-icon controller">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="2" y="8" width="20" height="12" rx="2"/>
                <path d="M6 12h4"/>
                <path d="M8 10v4"/>
                <circle cx="16" cy="13" r="1"/>
                <circle cx="18" cy="11" r="1"/>
              </svg>
            </div>
            <h4 class="feature-title">控制器支持</h4>
            <p class="feature-desc">配置 XInput 控制器映射和按键绑定</p>
            <span class="feature-status" :class="{ configured: configStatus.controller }">
              {{ configStatus.controller ? '已配置' : '未配置' }}
            </span>
          </div>

          <!-- 用户配置 -->
          <div class="feature-card" @click="openUserConfig">
            <div class="feature-icon user">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
                <circle cx="12" cy="7" r="4"/>
              </svg>
            </div>
            <h4 class="feature-title">用户配置</h4>
            <p class="feature-desc">设置用户名、头像、语言等个人信息</p>
            <span class="feature-status" :class="{ configured: configStatus.user }">
              {{ configStatus.user ? '已配置' : '未配置' }}
            </span>
          </div>

          <!-- 排行榜 -->
          <div class="feature-card" @click="openLeaderboardsConfig">
            <div class="feature-icon leaderboards">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M6 9H4.5a2.5 2.5 0 0 1 0-5H6"/>
                <path d="M18 9h1.5a2.5 2.5 0 0 0 0-5H18"/>
                <path d="M4 22h16"/>
                <path d="M10 14.66V17c0 .55-.47.98-.97 1.21C7.85 18.75 7 20.24 7 22"/>
                <path d="M14 14.66V17c0 .55.47.98.97 1.21C16.15 18.75 17 20.24 17 22"/>
                <path d="M18 2H6v7a6 6 0 0 0 12 0V2z"/>
              </svg>
            </div>
            <h4 class="feature-title">排行榜</h4>
            <p class="feature-desc">配置游戏排行榜和分数记录</p>
            <span class="feature-status" :class="{ configured: configStatus.leaderboards }">
              {{ configStatus.leaderboards ? '已配置' : '未配置' }}
            </span>
          </div>

          <!-- 统计 -->
          <div class="feature-card" @click="openStatsConfig">
            <div class="feature-icon stats">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="18" y1="20" x2="18" y2="10"/>
                <line x1="12" y1="20" x2="12" y2="4"/>
                <line x1="6" y1="20" x2="6" y2="14"/>
              </svg>
            </div>
            <h4 class="feature-title">游戏统计</h4>
            <p class="feature-desc">配置游戏统计数据和进度追踪</p>
            <span class="feature-status" :class="{ configured: configStatus.stats }">
              {{ configStatus.stats ? '已配置' : '未配置' }}
            </span>
          </div>

          <!-- DLC 和 depot -->
          <div class="feature-card" @click="openDlcConfig">
            <div class="feature-icon dlc">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
                <line x1="12" y1="8" x2="12" y2="16"/>
                <line x1="8" y1="12" x2="16" y2="12"/>
              </svg>
            </div>
            <h4 class="feature-title">DLC 与 Depot</h4>
            <p class="feature-desc">配置已安装的 DLC 和 depot 列表</p>
            <span class="feature-status" :class="{ configured: configStatus.dlc }">
              {{ configStatus.dlc ? '已配置' : '未配置' }}
            </span>
          </div>

          <!-- 主配置 -->
          <div class="feature-card" @click="openMainConfig">
            <div class="feature-icon main">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="3"/>
                <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
              </svg>
            </div>
            <h4 class="feature-title">主配置</h4>
            <p class="feature-desc">配置模拟器核心选项和高级设置</p>
            <span class="feature-status" :class="{ configured: configStatus.main }">
              {{ configStatus.main ? '已配置' : '未配置' }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- 配置弹窗组件 -->
    <LanMultiplayerConfig
      v-if="showLanMultiplayerModal"
      :game-path="gamePath"
      game-id=""
      @close="showLanMultiplayerModal = false"
      @saved="onConfigSaved('lanMultiplayer')"
    />

    <OverlayConfig
      v-if="showOverlayModal"
      :game-path="gamePath"
      game-id=""
      :is-experimental="useExperimental"
      @close="showOverlayModal = false"
      @saved="onConfigSaved('overlay')"
    />

    <AchievementsConfig
      v-if="showAchievementsModal"
      :game-path="gamePath"
      game-id=""
      @close="showAchievementsModal = false"
      @saved="onConfigSaved('achievements')"
    />

    <ItemsConfig
      v-if="showItemsModal"
      :game-path="gamePath"
      game-id=""
      @close="showItemsModal = false"
      @saved="onConfigSaved('items')"
    />

    <ControllerConfig
      v-if="showControllerModal"
      :game-path="gamePath"
      game-id=""
      @close="showControllerModal = false"
      @saved="onConfigSaved('controller')"
    />

    <UserConfig
      v-if="showUserModal"
      :game-path="gamePath"
      game-id=""
      @close="showUserModal = false"
      @saved="onConfigSaved('user')"
    />

    <LeaderboardsConfig
      v-if="showLeaderboardsModal"
      :game-path="gamePath"
      game-id=""
      @close="showLeaderboardsModal = false"
      @saved="onConfigSaved('leaderboards')"
    />

    <StatsConfig
      v-if="showStatsModal"
      :game-path="gamePath"
      game-id=""
      @close="showStatsModal = false"
      @saved="onConfigSaved('stats')"
    />

    <DlcConfig
      v-if="showDlcModal"
      :game-path="gamePath"
      game-id=""
      @close="showDlcModal = false"
      @saved="onConfigSaved('dlc')"
    />

    <MainConfig
      v-if="showMainModal"
      :game-path="gamePath"
      game-id=""
      @close="showMainModal = false"
      @saved="onConfigSaved('main')"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'

// 导入子配置组件
import LanMultiplayerConfig from '../../components/steam-patch/LanMultiplayerConfig.vue'
import OverlayConfig from '../../components/steam-patch/OverlayConfig.vue'
import AchievementsConfig from '../../components/steam-patch/AchievementsConfig.vue'
import ItemsConfig from '../../components/steam-patch/ItemsConfig.vue'
import ControllerConfig from '../../components/steam-patch/ControllerConfig.vue'
import UserConfig from '../../components/steam-patch/UserConfig.vue'
import LeaderboardsConfig from '../../components/steam-patch/LeaderboardsConfig.vue'
import StatsConfig from '../../components/steam-patch/StatsConfig.vue'
import DlcConfig from '../../components/steam-patch/DlcConfig.vue'
import MainConfig from '../../components/steam-patch/MainConfig.vue'

// ============================================
// 状态
// ============================================

/** 模拟器模式：0=常规模式(steam_api.dll), 1=实验版模式(steamclient.dll) */
const emulatorMode = ref(0)
/** 是否使用实验版DLL（仅在常规模式下有效） */
const useExperimental = ref(false)
/** 是否显示实验版警告 */
const showExperimentalWarning = ref(false)
/** 游戏文件夹路径（steam_api.dll 所在文件夹） */
const gamePath = ref('')
/** 游戏主程序 exe 路径 */
const gameExePath = ref('')
/** Steam AppID */
const steamAppId = ref('')
/** 是否正在脱壳 */
const isUnpacking = ref(false)
/** 脱壳是否成功 */
const unpackSuccess = ref(false)
/** 脱壳是否失败 */
const unpackError = ref(false)
/** 脱壳消息 */
const unpackMessage = ref('')
/** 基础配置是否已应用 */
const basicConfigApplied = ref(false)

/** 各功能配置状态 */
const configStatus = ref({
  lanMultiplayer: false,
  overlay: false,
  achievements: false,
  items: false,
  controller: false,
  user: false,
  leaderboards: false,
  stats: false,
  dlc: false,
  main: false,
})

/** 弹窗显示状态 */
const showLanMultiplayerModal = ref(false)
const showOverlayModal = ref(false)
const showAchievementsModal = ref(false)
const showItemsModal = ref(false)
const showControllerModal = ref(false)
const showUserModal = ref(false)
const showLeaderboardsModal = ref(false)
const showStatsModal = ref(false)
const showDlcModal = ref(false)
const showMainModal = ref(false)

// ============================================
// 计算属性
// ============================================

/** 是否可以应用基础配置 */
const canApplyBasicConfig = computed(() => {
  return gamePath.value !== '' && steamAppId.value !== ''
})

// ============================================
// 方法
// ============================================

/**
 * 选择游戏文件夹
 * 选择包含 steam_api.dll 的文件夹
 */
const selectGameFolder = async () => {
  try {
    const selected = await open({
      directory: true,
      title: '选择游戏文件夹（包含 steam_api.dll）'
    })
    if (selected) {
      gamePath.value = selected
    }
  } catch (error) {
    console.error('选择文件夹失败:', error)
  }
}

/**
 * 选择游戏主程序 exe 文件
 */
const selectGameExe = async () => {
  try {
    const result = await open({
      filters: [{
        name: '可执行文件',
        extensions: ['exe']
      }],
      title: '选择游戏主程序'
    })
    if (result) {
      gameExePath.value = result
    }
  } catch (error) {
    console.error('选择 exe 文件失败:', error)
  }
}

/**
 * 脱壳游戏主程序
 * 使用 Steamless.CLI.exe 移除 Steam 壳保护
 */
const unpackGameExe = async () => {
  if (!gameExePath.value) return

  isUnpacking.value = true
  unpackSuccess.value = false
  unpackError.value = false
  unpackMessage.value = ''

  try {
    // 使用选中的游戏主程序 exe 文件进行脱壳
    const result = await invoke<{
      success: boolean
      message: string
      unpackedPath?: string
    }>('unpack_game_exe', {
      gameExePath: gameExePath.value
    })

    unpackSuccess.value = result.success
    unpackError.value = !result.success
    unpackMessage.value = result.message

    if (result.success) {
      alert('脱壳成功！原文件已备份为 .bak，脱壳后的文件已替换原文件，可直接运行游戏')
    }
  } catch (error) {
    unpackError.value = true
    unpackMessage.value = `脱壳失败: ${error}`
  } finally {
    isUnpacking.value = false
  }
}

/**
 * 应用基础配置
 * 创建 steam_settings 文件夹和基础配置文件
 */
const applyBasicConfig = async () => {
  if (!canApplyBasicConfig.value) return

  try {
    const result = await invoke<{
      success: boolean
      message: string
    }>('apply_steam_patch_basic', {
      gamePath: gamePath.value,
      gameId: '',
      steamAppId: steamAppId.value,
      useExperimental: useExperimental.value,
      emulatorMode: emulatorMode.value
    })

    if (result.success) {
      basicConfigApplied.value = true
      // 基础配置已包含 custom_broadcasts.txt，自动标记局域网联机为已配置
      configStatus.value.lanMultiplayer = true
      alert('基础配置已应用成功！')
    } else {
      alert(`配置失败: ${result.message}`)
    }
  } catch (error) {
    alert(`配置失败: ${error}`)
  }
}

/**
 * 重置基础配置
 */
const resetBasicConfig = () => {
  gamePath.value = ''
  gameExePath.value = ''
  steamAppId.value = ''
  basicConfigApplied.value = false
  unpackSuccess.value = false
  unpackError.value = false
  unpackMessage.value = ''
  resetAllConfig()
}

/**
 * 重置所有配置状态
 */
const resetAllConfig = () => {
  configStatus.value = {
    lanMultiplayer: false,
    overlay: false,
    achievements: false,
    items: false,
    controller: false,
    user: false,
    leaderboards: false,
    stats: false,
    dlc: false,
    main: false,
  }
}

/**
 * 配置保存回调
 */
const onConfigSaved = (type: keyof typeof configStatus.value) => {
  configStatus.value[type] = true
}

// ============================================
// 打开配置弹窗
// ============================================

const openLanMultiplayerConfig = () => {
  showLanMultiplayerModal.value = true
}

const openOverlayConfig = () => {
  if (!useExperimental.value) {
    alert('Overlay 功能需要启用实验版才能使用')
    return
  }
  showOverlayModal.value = true
}

const openAchievementsConfig = () => {
  showAchievementsModal.value = true
}

const openItemsConfig = () => {
  showItemsModal.value = true
}

const openControllerConfig = () => {
  showControllerModal.value = true
}

const openUserConfig = () => {
  showUserModal.value = true
}

const openLeaderboardsConfig = () => {
  showLeaderboardsModal.value = true
}

const openStatsConfig = () => {
  showStatsModal.value = true
}

const openDlcConfig = () => {
  showDlcModal.value = true
}

const openMainConfig = () => {
  showMainModal.value = true
}

// ============================================
// Tooltip 控制
// ============================================

const activeTooltip = ref('')

const showTooltip = (type: string) => {
  activeTooltip.value = type
}

const hideTooltip = () => {
  activeTooltip.value = ''
}
</script>

<style scoped>
/* ============================================
   页面整体布局
   ============================================ */
.steam-patch-inject-view {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  padding: 24px 32px;
  background-color: var(--steam-bg-secondary);
}

/* ============================================
   页面头部
   ============================================ */
.page-header {
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--steam-border);
}

.page-title {
  font-size: 22px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 8px 0;
  display: flex;
  align-items: center;
  gap: 12px;
}

/* SteamDB 跳转链接按钮样式 */
.steamdb-link {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background-color: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border);
  border-radius: 6px;
  color: var(--steam-text-secondary);
  font-size: 13px;
  font-weight: 500;
  text-decoration: none;
  transition: all 0.15s ease;
  cursor: pointer;
}

.steamdb-link:hover {
  background-color: var(--steam-accent-blue);
  border-color: var(--steam-accent-blue);
  color: white;
  transform: scale(1.02);
}

.steamdb-link svg {
  width: 14px;
  height: 14px;
}

.page-desc {
  font-size: 14px;
  color: var(--steam-text-secondary);
  margin: 0;
}

/* ============================================
   区域卡片样式
   ============================================ */
.section-card,
.config-card {
  background-color: var(--steam-bg-primary);
  border-radius: 12px;
  border: 1px solid var(--steam-border);
  padding: 20px;
  margin-bottom: 20px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 16px 0;
  display: flex;
  align-items: center;
  gap: 10px;
}

.section-icon {
  width: 20px;
  height: 20px;
  color: var(--steam-accent-blue);
}

.section-desc {
  font-size: 13px;
  color: var(--steam-text-secondary);
  margin: -8px 0 16px 0;
}

/* ============================================
   基础配置
   ============================================ */
.basic-config-section {
  margin-bottom: 20px;
}

.config-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

/* 右侧配置选项容器 */
.config-options-wrapper {
  display: flex;
  align-items: center;
  gap: 24px;
}

.config-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0;
  display: flex;
  align-items: center;
  gap: 10px;
}

.config-icon {
  width: 20px;
  height: 20px;
  color: var(--steam-accent-blue);
}

/* 模拟器模式选择 */
.emulator-mode-section {
  display: flex;
  align-items: center;
  gap: 12px;
}

.mode-label {
  font-size: 13px;
  color: var(--steam-text-secondary);
  font-weight: 500;
  white-space: nowrap;
}

.mode-options {
  display: flex;
  gap: 12px;
  flex: 1;
}

.mode-radio {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 14px;
  border-radius: 8px;
  border: 1px solid var(--steam-border);
  cursor: pointer;
  transition: all 0.15s ease;
  flex: 1;
  background-color: var(--steam-bg-secondary);
  position: relative;
}

.mode-radio:hover {
  border-color: var(--steam-accent-blue);
}

.mode-radio.active {
  border-color: var(--steam-accent-blue);
  background-color: rgba(59, 130, 246, 0.1);
}

.mode-input {
  width: 16px;
  height: 16px;
  accent-color: var(--steam-accent-blue);
  margin-top: 2px;
}

.mode-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

/* 悬浮提示样式 - 显示在按钮下方 */
.mode-tooltip,
.dll-tooltip {
  position: absolute;
  top: calc(100% + 8px);
  left: 50%;
  transform: translateX(-50%) scale(0.95);
  width: 280px;
  padding: 12px 16px;
  background-color: var(--steam-bg-primary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  opacity: 0;
  visibility: hidden;
  transition: all 0.2s ease;
  z-index: 100;
}

.mode-radio:hover .mode-tooltip,
.dll-radio:hover .dll-tooltip {
  opacity: 1;
  visibility: visible;
  transform: translateX(-50%) scale(1);
}

/* 提示框箭头 - 指向上方 */
.mode-tooltip::after,
.dll-tooltip::after {
  content: '';
  position: absolute;
  bottom: 100%;
  left: 50%;
  transform: translateX(-50%);
  border: 6px solid transparent;
  border-bottom-color: var(--steam-border);
}

.mode-tooltip::before,
.dll-tooltip::before {
  content: '';
  position: absolute;
  bottom: 100%;
  left: 50%;
  transform: translateX(-50%);
  border: 5px solid transparent;
  border-bottom-color: var(--steam-bg-primary);
  z-index: 1;
}

.tooltip-content {
  font-size: 12px;
  color: var(--steam-text-secondary);
  line-height: 1.6;
}

.tooltip-content strong {
  color: var(--steam-text-primary);
}

/* DLL 版本选择 */
.dll-version-section {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 14px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--steam-border);
}

.dll-version-label {
  font-size: 13px;
  color: var(--steam-text-secondary);
  font-weight: 500;
  white-space: nowrap;
}

.dll-version-options {
  display: flex;
  gap: 12px;
  flex: 1;
}

.dll-radio {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 10px 16px;
  border-radius: 6px;
  border: 1px solid var(--steam-border);
  cursor: pointer;
  transition: all 0.15s ease;
  flex: 1;
  background-color: var(--steam-bg-primary);
  position: relative;
}

.dll-radio:hover {
  border-color: var(--steam-accent-blue);
}

.dll-radio.active {
  border-color: var(--steam-accent-blue);
  background-color: rgba(59, 130, 246, 0.1);
}

.dll-input {
  display: none;
}

.dll-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

/* 配置组 */
.config-group {
  margin-bottom: 20px;
}

.config-label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
  margin-bottom: 8px;
}

.config-label .required {
  color: #ef4444;
  margin-left: 4px;
}

.config-label .optional {
  color: var(--steam-text-secondary);
  font-size: 12px;
  font-weight: 400;
  margin-left: 4px;
}

.config-input {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color 0.15s ease;
}

.config-input:focus {
  border-color: var(--steam-accent-blue);
}

.path-input-group {
  display: flex;
  gap: 8px;
}

.path-input {
  flex: 1;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 13px;
  outline: none;
}

.browse-btn {
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  background-color: var(--steam-accent-blue);
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.15s ease, transform 0.1s ease;
}

.browse-btn:hover {
  background-color: var(--steam-accent-hover);
}

.browse-btn:active {
  transform: scale(0.98);
}

.clear-btn {
  padding: 10px 16px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.clear-btn:hover {
  background-color: var(--steam-border);
}

.config-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 6px 0 0 0;
}

/* 脱壳区域 */
.unpack-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.unpack-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  background-color: var(--steam-accent-blue);
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.unpack-btn:hover:not(:disabled) {
  background-color: var(--steam-accent-hover);
  transform: scale(1.02);
}

.unpack-btn:disabled {
  background-color: var(--steam-text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

.unpack-btn.success {
  background-color: #10b981;
}

.unpack-btn.error {
  background-color: #ef4444;
}

.unpack-btn svg {
  width: 16px;
  height: 16px;
}

.loading-icon {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.unpack-message {
  margin-top: 8px;
  padding: 10px 12px;
  border-radius: 6px;
  font-size: 13px;
}

.unpack-message.success {
  background-color: rgba(16, 185, 129, 0.1);
  color: #10b981;
}

.unpack-message.error {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

/* 操作按钮 */
.config-actions {
  display: flex;
  gap: 12px;
  margin-top: 24px;
  padding-top: 20px;
  border-top: 1px solid var(--steam-border);
}

.btn-primary,
.btn-secondary {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.btn-primary {
  background-color: var(--steam-accent-blue);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background-color: var(--steam-accent-hover);
  transform: scale(1.02);
}

.btn-primary:disabled {
  background-color: var(--steam-text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

.btn-secondary {
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
  border: 1px solid var(--steam-border);
}

.btn-secondary:hover {
  background-color: var(--steam-border);
}

.btn-primary svg,
.btn-secondary svg {
  width: 16px;
  height: 16px;
}

/* ============================================
   高级功能区域
   ============================================ */
.advanced-features-section {
  margin-bottom: 20px;
}

.feature-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 16px;
}

.feature-card {
  background-color: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 10px;
  padding: 16px;
  cursor: pointer;
  transition: all 0.15s ease;
  position: relative;
}

.feature-card:hover {
  border-color: var(--steam-accent-blue);
  transform: translateY(-2px);
}

.feature-icon {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 12px;
}

.feature-icon svg {
  width: 24px;
  height: 24px;
}

.feature-icon.lan { background-color: rgba(59, 130, 246, 0.1); color: #3b82f6; }
.feature-icon.overlay { background-color: rgba(139, 92, 246, 0.1); color: #8b5cf6; }
.feature-icon.achievements { background-color: rgba(245, 158, 11, 0.1); color: #f59e0b; }
.feature-icon.items { background-color: rgba(16, 185, 129, 0.1); color: #10b981; }
.feature-icon.controller { background-color: rgba(239, 68, 68, 0.1); color: #ef4444; }
.feature-icon.user { background-color: rgba(99, 102, 241, 0.1); color: #6366f1; }
.feature-icon.leaderboards { background-color: rgba(236, 72, 153, 0.1); color: #ec4899; }
.feature-icon.stats { background-color: rgba(14, 165, 233, 0.1); color: #0ea5e9; }
.feature-icon.dlc { background-color: rgba(168, 85, 247, 0.1); color: #a855f7; }
.feature-icon.main { background-color: rgba(100, 116, 139, 0.1); color: #64748b; }

.feature-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 6px 0;
}

.feature-desc {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 0 0 12px 0;
  line-height: 1.4;
}

.feature-status {
  display: inline-block;
  padding: 3px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 500;
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-secondary);
}

.feature-status.configured {
  background-color: rgba(16, 185, 129, 0.1);
  color: #10b981;
}

.feature-status.disabled {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

/* ============================================
   弹窗样式
   ============================================ */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background-color: var(--steam-bg-primary);
  border-radius: 12px;
  border: 1px solid var(--steam-border);
  max-width: 500px;
  width: 90%;
  max-height: 80vh;
  overflow-y: auto;
}

.modal-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 20px;
  border-bottom: 1px solid var(--steam-border);
}

.modal-header h4 {
  font-size: 16px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0;
}

.warning-icon {
  width: 24px;
  height: 24px;
  color: #f59e0b;
}

.modal-body {
  padding: 20px;
}

.modal-body p {
  font-size: 14px;
  color: var(--steam-text-primary);
  margin: 0 0 12px 0;
}

.modal-body ul {
  margin: 0 0 16px 0;
  padding-left: 20px;
}

.modal-body li {
  font-size: 13px;
  color: var(--steam-text-secondary);
  margin-bottom: 6px;
}

.warning-text {
  color: #f59e0b !important;
  font-weight: 500;
}

.modal-footer {
  padding: 16px 20px;
  border-top: 1px solid var(--steam-border);
  display: flex;
  justify-content: flex-end;
}

/* ============================================
   响应式适配
   ============================================ */
@media (max-width: 768px) {
  .steam-patch-inject-view {
    padding: 16px;
  }

  .feature-grid {
    grid-template-columns: 1fr;
  }

  .config-header {
    flex-direction: column;
    gap: 12px;
    align-items: flex-start;
  }

  .config-actions {
    flex-direction: column;
  }

  .path-input-group {
    flex-direction: column;
  }
}
</style>
