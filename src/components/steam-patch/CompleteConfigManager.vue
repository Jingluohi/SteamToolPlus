<template>
  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-content complete-config" @click.stop>
      <!-- 头部 -->
      <div class="modal-header">
        <h3>完整配置管理器</h3>
        <div class="header-status" v-if="configuredCount > 0">
          <span class="status-badge">已配置 {{ configuredCount }}/{{ totalCount }} 项</span>
        </div>
        <button class="close-btn" @click="$emit('close')" title="关闭">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <!-- 搜索栏 -->
      <div class="search-bar">
        <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8"/>
          <line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
        <input 
          v-model="searchQuery" 
          type="text" 
          placeholder="搜索配置项或内容..." 
          class="search-input"
        />
        <button v-if="searchQuery" class="clear-search" @click="searchQuery = ''">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <div class="modal-body">
        <!-- 左侧导航 -->
        <div class="config-nav">
          <div class="nav-section">
            <h4>核心配置</h4>
            <button 
              v-for="item in filteredCoreConfigs" 
              :key="item.id"
              class="nav-item"
              :class="{ 
                active: activeTab === item.id,
                configured: configStatus[item.id],
                disabled: item.id === 'overlay' && !useExperimental
              }"
              @click="handleNavClick(item.id)"
            >
              <span class="nav-label">{{ item.name }}</span>
              <span v-if="isContentOnlyMatch(item, searchQuery)" class="content-match">内容</span>
              <span v-if="configStatus[item.id]" class="nav-status configured">
                <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="13 4 6 11 3 8"/>
                </svg>
              </span>
            </button>
          </div>

          <div class="nav-section">
            <h4>游戏功能</h4>
            <button 
              v-for="item in filteredGameFeatures" 
              :key="item.id"
              class="nav-item"
              :class="{ 
                active: activeTab === item.id,
                configured: configStatus[item.id]
              }"
              @click="handleNavClick(item.id)"
            >
              <span class="nav-label">{{ item.name }}</span>
              <span v-if="isContentOnlyMatch(item, searchQuery)" class="content-match">内容</span>
              <span v-if="configStatus[item.id]" class="nav-status configured">
                <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="13 4 6 11 3 8"/>
                </svg>
              </span>
            </button>
          </div>

          <div class="nav-section">
            <h4>工具集成</h4>
            <button 
              v-for="item in filteredToolConfigs" 
              :key="item.id"
              class="nav-item"
              :class="{ 
                active: activeTab === item.id,
                configured: configStatus[item.id]
              }"
              @click="handleNavClick(item.id)"
            >
              <span class="nav-label">{{ item.name }}</span>
              <span v-if="isContentOnlyMatch(item, searchQuery)" class="content-match">内容</span>
              <span v-if="configStatus[item.id]" class="nav-status configured">
                <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="13 4 6 11 3 8"/>
                </svg>
              </span>
            </button>
          </div>

        </div>

        <!-- 右侧内容区 -->
        <div class="config-content">
          <!-- 主配置 -->
          <div v-if="activeTab === 'main'" class="config-panel">
            <MainConfigPanel
              ref="mainConfigPanelRef"
              :game-path="props.gamePath"
              @saved="handleMainConfigSaved"
            />
          </div>

          <!-- 用户配置 -->
          <div v-if="activeTab === 'user'" class="config-panel">
            <UserConfigPanel
              ref="userConfigPanelRef"
              :game-path="props.gamePath"
              @saved="handleUserConfigSaved"
            />
          </div>

          <!-- 应用配置 / DLC 与 Depot -->
          <div v-if="activeTab === 'app'" class="config-panel">
            <h3>应用配置 (configs.app.ini)</h3>
            <AppConfigPanel
              ref="appConfigPanelRef"
              :game-path="props.gamePath"
              @saved="handleAppConfigSaved"
            />
          </div>

          <!-- 覆盖层overlay配置 -->
          <div v-if="activeTab === 'overlay'" class="config-panel">
            <OverlayConfigPanel
              ref="overlayConfigPanelRef"
              :game-path="props.gamePath"
              @saved="handleOverlayConfigSaved"
            />
          </div>

          <!-- 成就配置 -->
          <div v-if="activeTab === 'achievements'" class="config-panel">
            <AchievementsConfigPanel
              ref="achievementsConfigPanelRef"
              :game-path="props.gamePath"
              :game-id="props.gameId"
              @saved="handleAchievementsSaved"
            />
          </div>

          <!-- 统计配置 -->
          <div v-if="activeTab === 'stats'" class="config-panel">
            <StatsConfigPanel
              ref="statsConfigPanelRef"
              :game-path="props.gamePath"
              @saved="handleStatsSaved"
            />
          </div>

          <!-- 物品配置 -->
          <div v-if="activeTab === 'items'" class="config-panel">
            <ItemsConfigPanel
              ref="itemsConfigPanelRef"
              :game-path="props.gamePath"
              @saved="handleItemsSaved"
            />
          </div>

          <!-- 模组配置 -->
          <div v-if="activeTab === 'mods'" class="config-panel">
            <ModsConfigPanel
              ref="modsConfigPanelRef"
              :game-path="props.gamePath"
              @saved="() => handleModsConfigSaved()"
            />
          </div>

          <!-- 排行榜配置 -->
          <div v-if="activeTab === 'leaderboards'" class="config-panel">
            <LeaderboardsConfigPanel
              ref="leaderboardsConfigPanelRef"
              :game-path="props.gamePath"
              @saved="handleLeaderboardsSaved"
            />
          </div>

          <!-- 控制器配置 -->
          <div v-if="activeTab === 'controller'" class="config-panel">
            <ControllerConfigPanel
              ref="controllerConfigPanelRef"
              :game-path="props.gamePath"
              @saved="handleControllerSaved"
            />
          </div>

          <!-- 局域网联机配置 -->
          <div v-if="activeTab === 'lan'" class="config-panel">
            <LanMultiplayerConfigPanel
              ref="lanMultiplayerConfigPanelRef"
              :game-path="props.gamePath"
              @saved="handleLanSaved"
            />
          </div>

          <!-- 其他配置 -->
          <div v-if="activeTab === 'other'" class="config-panel">
            <OtherConfigPanel
              ref="otherConfigPanelRef"
              :game-path="props.gamePath"
              @saved="handleOtherSaved"
            />
          </div>

          <!-- ColdClientLoader -->
          <div v-if="activeTab === 'coldclient'" class="config-panel">
            <ColdClientLoaderConfigPanel
              ref="coldClientLoaderConfigPanelRef"
              :game-path="props.gamePath"
              @saved="() => handleColdClientConfigSaved()"
            />
          </div>

          <!-- Lobby Connect -->
          <div v-if="activeTab === 'lobby'" class="config-panel">
            <LobbyConnectConfigPanel
              ref="lobbyConnectConfigPanelRef"
              :game-path="props.gamePath"
              @saved="() => handleLobbyConfigSaved()"
            />
          </div>
        </div>
      </div>

      <!-- 底部操作栏 -->
      <div class="modal-footer">
        <button class="btn-cancel" @click="$emit('close')">取消</button>
        <button
          class="btn-save"
          @click="saveCurrentTab"
          :class="{ saving: isSaving, saved: isSaved }"
          :disabled="isSaving"
        >
          <svg v-if="!isSaving && !isSaved" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
            <polyline points="17 21 17 13 7 13 7 21"/>
            <polyline points="7 3 7 8 15 8"/>
          </svg>
          <svg v-if="isSaving" class="spin-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
          </svg>
          <svg v-if="isSaved" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="20 6 9 17 4 12"/>
          </svg>
          <span>{{ isSaving ? '保存中...' : isSaved ? '已保存' : '保存当前配置' }}</span>
        </button>
      </div>

    </div>

    <!-- 保存成功提示 -->
    <transition name="toast">
      <div v-if="showToast" class="toast-success">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
          <polyline points="22 4 12 14.01 9 11.01"/>
        </svg>
        <span>所有配置已保存成功！</span>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { ref, shallowReactive, computed, watch, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { CONFIG_EVENTS } from '../../constants/config-events'
import * as SteamTypes from '../../types/steam-config.types'
import AppConfigPanel from './panels/AppConfigPanel.vue'
import MainConfigPanel from './panels/MainConfigPanel.vue'
import UserConfigPanel from './panels/UserConfigPanel.vue'
import OverlayConfigPanel from './panels/OverlayConfigPanel.vue'
import AchievementsConfigPanel from './panels/AchievementsConfigPanel.vue'
import StatsConfigPanel from './panels/StatsConfigPanel.vue'
import ItemsConfigPanel from './panels/ItemsConfigPanel.vue'
import LeaderboardsConfigPanel from './panels/LeaderboardsConfigPanel.vue'
import ControllerConfigPanel from './panels/ControllerConfigPanel.vue'
import LanMultiplayerConfigPanel from './panels/LanMultiplayerConfigPanel.vue'
import OtherConfigPanel from './panels/OtherConfigPanel.vue'
import ModsConfigPanel from './panels/ModsConfigPanel.vue'
import ColdClientLoaderConfigPanel from './panels/ColdClientLoaderConfigPanel.vue'
import LobbyConnectConfigPanel from './panels/LobbyConnectConfigPanel.vue'

const props = defineProps<{
  gamePath: string
  gameId: string
  useExperimental?: boolean
}>()

const emit = defineEmits<{
  close: []
  saved: []
}>()

const activeTab = ref('main')
const searchQuery = ref('')

/**
 * 处理导航项点击
 * 若点击的是 Overlay 且未启用功能版，则阻止切换并提示
 */
function handleNavClick(tabId: string) {
  if (tabId === 'overlay' && !props.useExperimental) {
    alert('Overlay 功能需要启用功能版才能使用')
    return
  }
  activeTab.value = tabId
}
const debouncedSearchQuery = ref('')
const isSaving = ref(false)
const isSaved = ref(false)
const showToast = ref(false)

/**
 * 简易防抖函数
 * 用于搜索输入，避免快速输入时频繁重算过滤结果
 */
function debounce<T extends (...args: any[]) => void>(fn: T, delay: number): (...args: Parameters<T>) => void {
  let timer: ReturnType<typeof setTimeout> | null = null
  return (...args: Parameters<T>) => {
    if (timer) clearTimeout(timer)
    timer = setTimeout(() => fn(...args), delay)
  }
}

// 搜索防抖：300ms 后更新实际用于过滤的值
const updateDebouncedSearch = debounce((value: string) => {
  debouncedSearchQuery.value = value
}, 300)

watch(searchQuery, (value) => {
  updateDebouncedSearch(value)
})

// 导航配置
const coreConfigs = [
  { id: 'main', name: '主配置' },
  { id: 'user', name: '用户配置' },
  { id: 'app', name: '应用配置' },
  { id: 'overlay', name: '覆盖层overlay' },
]

const gameFeatures = [
  { id: 'achievements', name: '成就系统' },
  { id: 'stats', name: '统计数据' },
  { id: 'items', name: '物品库存' },
  { id: 'mods', name: '创意工坊' },
  { id: 'leaderboards', name: '排行榜' },
  { id: 'controller', name: '控制器' },
  { id: 'lan', name: '局域网联机' },
  { id: 'other', name: '其他配置' },
]

const toolConfigs = [
  { id: 'coldclient', name: 'ColdClient' },
  { id: 'lobby', name: 'Lobby' },
]

/**
 * 各标签页的可搜索文本索引
 * 包含标签名、节标题、label、hint、placeholder 等所有可见文本，
 * 使搜索不仅能匹配导航名称，还能匹配配置项内部内容。
 */
const tabSearchIndex: Record<string, string> = {
  main: '主配置 configs.main.ini 通用设置 新版认证票据 new_app_ticket GC令牌 gc_token 阻止未知客户端 block_unknown_clients Steam Deck 头像 语音聊天 即时统计 匹配服务器列表 源查询 端口 listen_port 离线 网络 lobby 云 额外 dll extra',
  user: '用户配置 configs.user.ini 用户名 语言 国家 IP区域 存档文件夹 本地存档 加密票据 ticket Base64 alt_steamid',
  app: '应用配置 configs.app.ini 分支 beta 公开 public DLC 解锁 app路径 控制器 steam_input 云存档 windows linux 目录',
  overlay: '覆盖层overlay configs.overlay.ini 快捷键 shift tab hook 延迟 渲染器检测超时 fps 平均窗口 通知 成就 好友 进度 警告 上传gpu 用户信息 帧时间 游玩时间 时长 外观 主题 透明度 缩放 模糊 字体 字间距 图标 颜色 rgba 位置 统计 性能 硬件加速 低性能 功能 浏览器 聊天 设置',
  achievements: '成就系统 achievements.json 成就 通知 隐藏 名称 描述 图标 导入 导出',
  stats: '统计数据 stats.json int float avgrate 默认值 最小值 最大值 窗口大小',
  items: '物品库存 items.json default_items 初始库存 属性 堆叠 数量 物品ID',
  mods: '创意工坊 mods.json 模组 订阅 公开 私有 好友 预览图 文件ID',
  leaderboards: '排行榜 leaderboards.txt 排序 升序 降序 显示 数字 时间 秒 毫秒',
  controller: '控制器 controller xbox playstation nintendo 通用 死区 摇杆',
  lan: '局域网联机 LAN custom_broadcasts.txt auto_accept_invite.txt 广播 IP 域名 自动接受邀请 白名单 SteamID64 监听端口 UDP',
  other: '其他配置 installed_app_ids subscribed_groups purchased_keys supported_languages 已安装应用 订阅群组 购买密钥 支持语言',
  coldclient: 'ColdClient 注入模式 direct loader 额外DLL 启动参数',
  lobby: 'Lobby 大厅 自动加入 大厅ID 密码',
}

/**
 * 判断导航项是否匹配搜索词
 * 同时匹配导航名称和对应标签页的内容索引
 */
function configMatchesSearch(item: { id: string; name: string }, query: string): boolean {
  const q = query.toLowerCase()
  if (item.name.toLowerCase().includes(q)) return true
  const indexText = tabSearchIndex[item.id]
  return !!indexText && indexText.toLowerCase().includes(q)
}

/**
 * 判断导航项是否仅通过内容匹配（名称未命中但标签页内容命中）
 */
function isContentOnlyMatch(item: { id: string; name: string }, query: string): boolean {
  if (!query) return false
  const q = query.toLowerCase()
  if (item.name.toLowerCase().includes(q)) return false
  const indexText = tabSearchIndex[item.id]
  return !!indexText && indexText.toLowerCase().includes(q)
}

// 搜索过滤（同时匹配导航名称和配置项内容，使用防抖后的搜索词）
const filteredCoreConfigs = computed(() => {
  if (!debouncedSearchQuery.value) return coreConfigs
  return coreConfigs.filter(c => configMatchesSearch(c, debouncedSearchQuery.value))
})

const filteredGameFeatures = computed(() => {
  if (!debouncedSearchQuery.value) return gameFeatures
  return gameFeatures.filter(c => configMatchesSearch(c, debouncedSearchQuery.value))
})

const filteredToolConfigs = computed(() => {
  if (!debouncedSearchQuery.value) return toolConfigs
  return toolConfigs.filter(c => configMatchesSearch(c, debouncedSearchQuery.value))
})

// 所有匹配搜索的导航项（用于自动切换）
const allFilteredConfigs = computed(() => [
  ...filteredCoreConfigs.value,
  ...filteredGameFeatures.value,
  ...filteredToolConfigs.value,
])

/**
 * 当防抖搜索词变化时，如果当前标签不在搜索结果中，自动切换到首个匹配标签
 */
watch(debouncedSearchQuery, (query) => {
  if (!query) return
  const matched = allFilteredConfigs.value
  // 未启用功能版时，Overlay 标签不可选
  const availableTabs = matched.filter(c => c.id !== 'overlay' || props.useExperimental)
  if (availableTabs.length > 0 && !availableTabs.some(c => c.id === activeTab.value)) {
    activeTab.value = availableTabs[0].id
  }
})

// 配置状态
const configStatus = ref<Record<string, boolean>>({})

// Panel 组件引用
const appConfigPanelRef = ref<InstanceType<typeof AppConfigPanel> | null>(null)
const mainConfigPanelRef = ref<InstanceType<typeof MainConfigPanel> | null>(null)
const userConfigPanelRef = ref<InstanceType<typeof UserConfigPanel> | null>(null)
const overlayConfigPanelRef = ref<InstanceType<typeof OverlayConfigPanel> | null>(null)
const achievementsConfigPanelRef = ref<InstanceType<typeof AchievementsConfigPanel> | null>(null)
const statsConfigPanelRef = ref<InstanceType<typeof StatsConfigPanel> | null>(null)
const itemsConfigPanelRef = ref<InstanceType<typeof ItemsConfigPanel> | null>(null)
const leaderboardsConfigPanelRef = ref<InstanceType<typeof LeaderboardsConfigPanel> | null>(null)
const controllerConfigPanelRef = ref<InstanceType<typeof ControllerConfigPanel> | null>(null)
const lanMultiplayerConfigPanelRef = ref<InstanceType<typeof LanMultiplayerConfigPanel> | null>(null)
const otherConfigPanelRef = ref<InstanceType<typeof OtherConfigPanel> | null>(null)
const modsConfigPanelRef = ref<InstanceType<typeof ModsConfigPanel> | null>(null)
const coldClientLoaderConfigPanelRef = ref<InstanceType<typeof ColdClientLoaderConfigPanel> | null>(null)
const lobbyConnectConfigPanelRef = ref<InstanceType<typeof LobbyConnectConfigPanel> | null>(null)

/**
 * 应用配置保存后的处理
 * 更新配置状态并显示提示
 */
function handleAppConfigSaved() {
  configStatus.value.app = true
  showToast.value = true
  setTimeout(() => {
    showToast.value = false
  }, 3000)
}

/**
 * 用户配置保存后的处理
 * 更新配置状态，Toast 由 UserConfigPanel 自己显示
 */
function handleUserConfigSaved() {
  configStatus.value.user = true
}

/**
 * 主配置保存后的处理
 * 更新配置状态，Toast 由 MainConfigPanel 自己显示
 */
function handleMainConfigSaved() {
  configStatus.value.main = true
}

// 计算已配置数量
const totalCount = computed(() => {
  return coreConfigs.length + gameFeatures.length + toolConfigs.length
})

const configuredCount = computed(() => {
  return Object.values(configStatus.value).filter(Boolean).length
})

// 配置数据 - 父组件仅保留需要直接维护或传递给 Panel 的状态
// app / overlay 仍由父组件加载并传给对应 Panel（当前设计）
// 其它配置（items/mods/leaderboards/controller/lan/coldclient/lobby/other）
// 已完全抽为 Panel，由 Panel 自行维护状态，父组件只通过 configStatus 记录存在性
// 使用 shallowReactive 减少深层响应式代理开销
const configs = shallowReactive({
  app: {
    branch_name: 'public',
    is_beta_branch: false,
    app_paths: {},
    dlcs: {
      unlock_all: true,
      custom_list: '',
    },
    // Steam Input 控制器配置
    controller: {
      steam_input: false,
      type: '',
    },
    // 云存档配置
    cloud_saves: {
      enabled: false,
      create_default_dir: false,
      create_specific_dirs: false,
      windows_dirs: [] as string[],
      linux_dirs: [] as string[],
    },
  },
  overlay: {
    enableExperimentalOverlay: false,
    hookDelaySec: undefined,
    rendererDetectorTimeoutSec: undefined,
    overlayHotkey: 'shift + tab',
    fpsAveragingWindow: undefined,
    notifications: {
      disableAchievementNotification: false,
      disableFriendNotification: false,
      disableAchievementProgress: false,
      disableWarningAny: false,
      disableWarningBadAppid: false,
      disableWarningLocalSave: false,
      uploadAchievementsIconsToGpu: true,
      overlayAlwaysShowUserInfo: false,
      overlayAlwaysShowFps: false,
      overlayAlwaysShowFrametime: false,
      overlayAlwaysShowPlaytime: false,
      notificationDurationAchievement: undefined,
      notificationDurationInvitation: undefined,
      notificationDurationChat: undefined,
      notificationDurationProgress: undefined,
    },
    appearance: {
      fontOverride: undefined,
      fontSize: undefined,
      fontGlyphExtraSpacingX: undefined,
      fontGlyphExtraSpacingY: undefined,
      iconSize: undefined,
      theme: 'dark',
      opacity: 0.95,
      scale: 1.0,
      blur: true,
      notificationRounding: undefined,
      notificationMarginX: undefined,
      notificationMarginY: undefined,
      notificationR: undefined,
      notificationG: undefined,
      notificationB: undefined,
      notificationA: undefined,
      achievementUnlockDatetimeFormat: undefined,
      backgroundR: undefined,
      backgroundG: undefined,
      backgroundB: undefined,
      backgroundA: undefined,
      elementR: undefined,
      elementG: undefined,
      elementB: undefined,
      elementA: undefined,
      elementHoveredR: undefined,
      elementHoveredG: undefined,
      elementHoveredB: undefined,
      elementHoveredA: undefined,
      elementActiveR: undefined,
      elementActiveG: undefined,
      elementActiveB: undefined,
      elementActiveA: undefined,
      posAchievement: undefined,
      posInvitation: undefined,
      posChatMsg: undefined,
      statsBackgroundR: undefined,
      statsBackgroundG: undefined,
      statsBackgroundB: undefined,
      statsBackgroundA: undefined,
      statsTextR: undefined,
      statsTextG: undefined,
      statsTextB: undefined,
      statsTextA: undefined,
      statsPosX: undefined,
      statsPosY: undefined,
    },
    performance: {
      hardwareAcceleration: true,
      fpsLimit: 60,
      lowPerformanceMode: false,
    },
    features: {
      achievements: true,
      friends: true,
      chat: true,
      browser: true,
      settings: true,
    },
  } as SteamTypes.OverlayConfig,
})

// 保存当前选中的标签页配置
async function saveCurrentTab() {
  if (isSaving.value) return

  try {
    isSaving.value = true
    isSaved.value = false

    switch (activeTab.value) {
      case 'app':
        // app 配置已抽为 Panel，调用 Panel 的 save 方法
        // Panel 自己会显示保存成功提示并广播事件，这里直接返回避免重复提示
        await appConfigPanelRef.value?.save()
        isSaving.value = false
        return
      case 'main':
        // main 配置已抽为 Panel，调用 Panel 的 save 方法
        // Panel 自己会显示保存成功提示并广播事件，这里直接返回避免重复提示
        await mainConfigPanelRef.value?.save()
        isSaving.value = false
        return
      case 'user':
        // user 配置已抽为 Panel，调用 Panel 的 save 方法
        // Panel 自己会显示保存成功提示并广播事件，这里直接返回避免重复提示
        await userConfigPanelRef.value?.save()
        isSaving.value = false
        return
      case 'overlay':
        // overlay 配置已抽为 Panel，调用 Panel 的 save 方法
        // Panel 自己会显示保存成功提示并广播事件，这里直接返回避免重复提示
        await overlayConfigPanelRef.value?.save()
        isSaving.value = false
        return
      case 'achievements':
        // achievements 配置已抽为 Panel，调用 Panel 的 save 方法
        // Panel 自己会显示保存成功提示并广播事件，这里直接返回避免重复提示
        await achievementsConfigPanelRef.value?.save()
        isSaving.value = false
        return
      case 'stats':
        // stats 配置已抽为 Panel，调用 Panel 的 save 方法
        // Panel 自己会显示保存成功提示并广播事件，这里直接返回避免重复提示
        await statsConfigPanelRef.value?.save()
        isSaving.value = false
        return
      case 'items':
        // items 配置已抽为 Panel，调用 Panel 的 save 方法
        // Panel 自己会显示保存成功提示并广播事件，这里直接返回避免重复提示
        await itemsConfigPanelRef.value?.save()
        isSaving.value = false
        return
      case 'mods':
        // mods 配置已抽为 Panel，调用 Panel 的 save 方法
        // Panel 自己会显示保存成功提示并广播事件，这里直接返回避免重复提示
        await modsConfigPanelRef.value?.save()
        isSaving.value = false
        return
      case 'leaderboards':
        // leaderboards 配置已抽为 Panel，调用 Panel 的 save 方法
        // Panel 自己会显示保存成功提示并广播事件，这里直接返回避免重复提示
        await leaderboardsConfigPanelRef.value?.save()
        isSaving.value = false
        return
      case 'controller':
        // controller 配置已抽为 Panel，调用 Panel 的 save 方法
        // Panel 自己会显示保存成功提示并广播事件，这里直接返回避免重复提示
        await controllerConfigPanelRef.value?.save()
        isSaving.value = false
        return
      case 'lan':
        // lan 配置已抽为 Panel，调用 Panel 的 save 方法
        // Panel 自己会显示保存成功提示并广播事件，这里直接返回避免重复提示
        await lanMultiplayerConfigPanelRef.value?.save()
        isSaving.value = false
        return
      case 'coldclient':
        // coldclient 配置已抽为 Panel，调用 Panel 的 save 方法
        // Panel 自己会显示保存成功提示并广播事件，这里直接返回避免重复提示
        await coldClientLoaderConfigPanelRef.value?.save()
        isSaving.value = false
        return
      case 'lobby':
        // lobby 配置已抽为 Panel，调用 Panel 的 save 方法
        // Panel 自己会显示保存成功提示并广播事件，这里直接返回避免重复提示
        await lobbyConnectConfigPanelRef.value?.save()
        isSaving.value = false
        return
      case 'other':
        // other 配置已抽为 Panel，调用 Panel 的 save 方法
        // Panel 自己会显示保存成功提示并广播事件，这里直接返回避免重复提示
        await otherConfigPanelRef.value?.save()
        isSaving.value = false
        return
      default:
        break
    }

    isSaving.value = false
    isSaved.value = true

    // 显示成功提示
    showToast.value = true
    setTimeout(() => {
      showToast.value = false
      isSaved.value = false
    }, 3000)

    emit('saved')
  } catch (error) {
    isSaving.value = false
    alert(`保存失败: ${error}`)
  }
}

/**
 * 仅重新加载应用配置（包含 DLC、分支、控制器、云存档等）
 * 用于响应 app-config-saved 同步事件，避免整页刷新并保留其它未保存的修改
 */
async function reloadAppConfig() {
  try {
    const result = await invoke<{ exists: boolean; config?: SteamTypes.SteamAppConfig }>('load_app_config', { gamePath: props.gamePath })
    if (result.exists && result.config) {
      const app = result.config
      configs.app.branch_name = app.branch_name || 'public'
      configs.app.is_beta_branch = app.is_beta_branch || false
      configs.app.dlcs.unlock_all = app.dlcs?.unlock_all ?? true

      if (app.controller) {
        configs.app.controller = {
          steam_input: app.controller.steam_input ?? false,
          type: app.controller.type ?? '',
        }
      }

      if (app.cloud_saves) {
        configs.app.cloud_saves = {
          enabled: app.cloud_saves.enabled ?? false,
          create_default_dir: app.cloud_saves.create_default_dir ?? false,
          create_specific_dirs: app.cloud_saves.create_specific_dirs ?? false,
          windows_dirs: app.cloud_saves.windows_dirs || [],
          linux_dirs: app.cloud_saves.linux_dirs || [],
        }
      }

      if (app.dlcs?.individual_dlcs && app.dlcs.individual_dlcs.length > 0) {
        configs.app.dlcs.custom_list = app.dlcs.individual_dlcs
          .filter((d: any) => d.enabled)
          .map((d: any) => {
            if (d.name && d.name !== `DLC ${d.app_id}` && d.name !== d.app_id) {
              return `${d.app_id}=${d.name}`
            }
            return d.app_id
          })
          .join('\n')
      } else {
        configs.app.dlcs.custom_list = ''
      }
    }
    configStatus.value.app = result.exists
  } catch (error) {
    // 加载失败时保持当前状态
  }
}

/**
 * 仅重新加载覆盖层配置
 * 用于响应 overlay-config-saved 同步事件，避免整页刷新
 */
async function reloadOverlayConfig() {
  try {
    const result = await invoke<{ exists: boolean; config?: SteamTypes.OverlayConfig }>('load_overlay_config', { gamePath: props.gamePath })
    if (result.exists && result.config) {
      Object.assign(configs.overlay, result.config)
    }
    configStatus.value.overlay = result.exists
  } catch (error) {
    // 加载失败时保持当前状态
  }
}

/**
 * 处理主配置同步事件
 * 当 MainConfig.vue 等其它组件保存 main 配置时，若 gamePath 匹配则更新配置状态
 * 具体配置内容由 MainConfigPanel 自行监听并重新加载
 */
function handleMainConfigSyncEvent(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    configStatus.value.main = true
  }
}

/**
 * 处理用户配置同步事件
 * 当 UserConfig.vue 等其它组件保存 user 配置时，若 gamePath 匹配则更新配置状态
 * 具体配置内容由 UserConfigPanel 自行监听并重新加载
 */
function handleUserConfigSyncEvent(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    configStatus.value.user = true
  }
}

/**
 * 成就配置保存后的处理
 * 仅更新配置状态；具体配置内容由 AchievementsConfigPanel 自行维护
 */
function handleAchievementsSaved() {
  configStatus.value.achievements = true
}

/**
 * 处理成就配置同步事件
 * 当 AchievementsConfig.vue 等其它组件保存 achievements 配置时，若 gamePath 匹配则更新状态
 */
function handleAchievementsConfigSyncEvent(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    configStatus.value.achievements = true
  }
}

/**
 * 统计配置保存后的处理
 * 更新配置状态，Toast 由 StatsConfigPanel 自己显示
 */
function handleStatsSaved() {
  configStatus.value.stats = true
}

/**
 * 物品配置保存后的处理
 * 更新配置状态，Toast 由 ItemsConfigPanel 自己显示
 */
function handleItemsSaved() {
  configStatus.value.items = true
}

/**
 * 排行榜配置保存后的处理
 * 更新配置状态，Toast 由 LeaderboardsConfigPanel 自己显示
 */
function handleLeaderboardsSaved() {
  configStatus.value.leaderboards = true
}

/**
 * 控制器配置保存后的处理
 * 更新配置状态，Toast 由 ControllerConfigPanel 自己显示
 */
function handleControllerSaved() {
  configStatus.value.controller = true
}

/**
 * 局域网联机配置保存后的处理
 * 更新配置状态，Toast 由 LanMultiplayerConfigPanel 自己显示
 */
function handleLanSaved() {
  configStatus.value.lan = true
}

/**
 * 其他配置保存后的处理
 * 更新配置状态，Toast 由 OtherConfigPanel 自己显示
 */
function handleOtherSaved() {
  configStatus.value.other = true
}

/**
 * 处理统计配置同步事件
 * 当 StatsConfig.vue 等其它组件保存 stats 配置时，若 gamePath 匹配则更新状态
 */
function handleStatsConfigSyncEvent(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    configStatus.value.stats = true
  }
}

/**
 * 处理物品配置同步事件
 * 当 ItemsConfig.vue 等其它组件保存 items 配置时，若 gamePath 匹配则更新状态
 */
function handleItemsConfigSaved(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    configStatus.value.items = true
  }
}

/**
 * 处理排行榜配置同步事件
 * 当 LeaderboardsConfig.vue 等其它组件保存 leaderboards 配置时，若 gamePath 匹配则更新状态
 */
function handleLeaderboardsConfigSaved(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    configStatus.value.leaderboards = true
  }
}

/**
 * 处理模组配置同步事件
 * 当 ModsConfig.vue 等其它组件保存 mods 配置时，若 gamePath 匹配则更新状态
 */
function handleModsConfigSaved(event?: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }> | undefined
  if (customEvent?.detail?.gamePath === props.gamePath) {
    configStatus.value.mods = true
  }
}

/**
 * 处理 ColdClientLoader 配置同步事件
 * 当 ColdClientConfig.vue 等其它组件保存 coldclient 配置时，若 gamePath 匹配则更新状态
 */
function handleColdClientConfigSaved(event?: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }> | undefined
  if (customEvent?.detail?.gamePath === props.gamePath) {
    configStatus.value.coldclient = true
  }
}

/**
 * 处理 Lobby Connect 配置同步事件
 * 当 LobbyConnectConfig.vue 等其它组件保存 lobby 配置时，若 gamePath 匹配则更新状态
 */
function handleLobbyConfigSaved(event?: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }> | undefined
  if (customEvent?.detail?.gamePath === props.gamePath) {
    configStatus.value.lobby = true
  }
}

/**
 * 处理控制器配置同步事件
 * 当 ControllerConfig.vue 等其它组件保存 controller 配置时，若 gamePath 匹配则更新配置状态
 * 具体配置内容由 ControllerConfigPanel 自行监听并重新加载
 */
function handleControllerConfigSaved(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    configStatus.value.controller = true
  }
}

/**
 * 处理局域网联机配置同步事件
 * 当 LanMultiplayerConfig.vue 保存 lan 配置时，若 gamePath 匹配则更新配置状态
 * 具体配置内容由 LanMultiplayerConfigPanel 自行监听并重新加载
 */
function handleLanConfigSaved(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    configStatus.value.lan = true
  }
}

/**
 * 处理应用配置同步事件
 * 当 DlcConfig.vue 等其它组件保存 app 配置时，若 gamePath 匹配则自动重载应用配置
 */
function handleAppConfigSyncEvent(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    reloadAppConfig()
  }
}

/**
 * 覆盖层配置保存后的处理
 * 更新配置状态，Toast 由 OverlayConfigPanel 自己显示
 */
function handleOverlayConfigSaved() {
  configStatus.value.overlay = true
}

/**
 * 处理覆盖层配置同步事件
 * 当 OverlayConfig.vue 等其它组件保存 overlay 配置时，若 gamePath 匹配则自动重载覆盖层配置
 */
function handleOverlayConfigSyncEvent(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    reloadOverlayConfig()
  }
}

// 加载所有配置
// 父组件只保留 app / overlay 的完整数据（当前设计需要传给 Panel）
// 其余配置的具体数据由各 Panel 自行加载；父组件仅读取 exists 用于导航显示，减少内存与 IPC 开销
async function loadAllConfigs() {
  const results = await Promise.all([
    invoke<{ exists: boolean }>('load_main_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean }>('load_user_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: SteamTypes.SteamAppConfig }>('load_app_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: SteamTypes.OverlayConfig }>('load_overlay_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean }>('load_achievements_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean }>('load_stats_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean }>('load_items_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean }>('load_mods_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean }>('load_leaderboards_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean }>('load_controller_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean }>('load_lan_multiplayer_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean }>('load_coldclient_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean }>('load_lobby_connect_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean }>('load_other_config', { gamePath: props.gamePath }),
  ])

  const [main, user, app, overlay, achievements, stats, items, mods, leaderboards, controller, lan, coldclient, lobby, other] = results

  // 加载应用配置（仍需父组件维护并传给 AppConfigPanel）
  if (app.exists && app.config) {
    configs.app.branch_name = app.config.branch_name || 'public'
    configs.app.is_beta_branch = app.config.is_beta_branch || false
    configs.app.dlcs.unlock_all = app.config.dlcs?.unlock_all ?? true

    if (app.config.controller) {
      configs.app.controller = {
        steam_input: app.config.controller.steam_input ?? false,
        type: app.config.controller.type ?? '',
      }
    }

    if (app.config.cloud_saves) {
      configs.app.cloud_saves = {
        enabled: app.config.cloud_saves.enabled ?? false,
        create_default_dir: app.config.cloud_saves.create_default_dir ?? false,
        create_specific_dirs: app.config.cloud_saves.create_specific_dirs ?? false,
        windows_dirs: app.config.cloud_saves.windows_dirs || [],
        linux_dirs: app.config.cloud_saves.linux_dirs || [],
      }
    }

    if (app.config.dlcs?.individual_dlcs && app.config.dlcs.individual_dlcs.length > 0) {
      configs.app.dlcs.custom_list = app.config.dlcs.individual_dlcs
        .filter((d: any) => d.enabled)
        .map((d: any) => {
          if (d.name && d.name !== `DLC ${d.app_id}` && d.name !== d.app_id) {
            return `${d.app_id}=${d.name}`
          }
          return d.app_id
        })
        .join('\n')
    } else {
      configs.app.dlcs.custom_list = ''
    }
  }

  // 加载覆盖层配置（仍需父组件维护并传给 OverlayConfigPanel）
  if (overlay.exists && overlay.config) {
    Object.assign(configs.overlay, overlay.config)
  }

  // 更新配置存在状态（Panel 内部自行加载具体数据）
  configStatus.value = {
    main: main.exists,
    user: user.exists,
    app: app.exists,
    overlay: overlay.exists,
    achievements: achievements.exists,
    stats: stats.exists,
    items: items.exists,
    mods: mods.exists,
    leaderboards: leaderboards.exists,
    controller: controller.exists,
    lan: lan.exists,
    coldclient: coldclient.exists,
    lobby: lobby.exists,
    other: other.exists,
  }
}

onMounted(() => {
  loadAllConfigs()
  window.addEventListener(CONFIG_EVENTS.MAIN_SAVED, handleMainConfigSyncEvent)
  window.addEventListener(CONFIG_EVENTS.USER_SAVED, handleUserConfigSyncEvent)
  window.addEventListener(CONFIG_EVENTS.ACHIEVEMENTS_SAVED, handleAchievementsConfigSyncEvent)
  window.addEventListener(CONFIG_EVENTS.STATS_SAVED, handleStatsConfigSyncEvent)
  window.addEventListener(CONFIG_EVENTS.ITEMS_SAVED, handleItemsConfigSaved)
  window.addEventListener(CONFIG_EVENTS.MODS_SAVED, handleModsConfigSaved)
  window.addEventListener(CONFIG_EVENTS.LEADERBOARDS_SAVED, handleLeaderboardsConfigSaved)
  window.addEventListener(CONFIG_EVENTS.CONTROLLER_SAVED, handleControllerConfigSaved)
  window.addEventListener(CONFIG_EVENTS.APP_SAVED, handleAppConfigSyncEvent)
  window.addEventListener(CONFIG_EVENTS.OVERLAY_SAVED, handleOverlayConfigSyncEvent)
  window.addEventListener(CONFIG_EVENTS.LAN_SAVED, handleLanConfigSaved)
  window.addEventListener(CONFIG_EVENTS.COLDCLIENT_SAVED, handleColdClientConfigSaved)
  window.addEventListener(CONFIG_EVENTS.LOBBY_SAVED, handleLobbyConfigSaved)
  window.addEventListener(CONFIG_EVENTS.OTHER_SAVED, handleOtherSaved)
})

onUnmounted(() => {
  window.removeEventListener(CONFIG_EVENTS.MAIN_SAVED, handleMainConfigSyncEvent)
  window.removeEventListener(CONFIG_EVENTS.USER_SAVED, handleUserConfigSyncEvent)
  window.removeEventListener(CONFIG_EVENTS.ACHIEVEMENTS_SAVED, handleAchievementsConfigSyncEvent)
  window.removeEventListener(CONFIG_EVENTS.STATS_SAVED, handleStatsConfigSyncEvent)
  window.removeEventListener(CONFIG_EVENTS.ITEMS_SAVED, handleItemsConfigSaved)
  window.removeEventListener(CONFIG_EVENTS.MODS_SAVED, handleModsConfigSaved)
  window.removeEventListener(CONFIG_EVENTS.LEADERBOARDS_SAVED, handleLeaderboardsConfigSaved)
  window.removeEventListener(CONFIG_EVENTS.CONTROLLER_SAVED, handleControllerConfigSaved)
  window.removeEventListener(CONFIG_EVENTS.APP_SAVED, handleAppConfigSyncEvent)
  window.removeEventListener(CONFIG_EVENTS.OVERLAY_SAVED, handleOverlayConfigSyncEvent)
  window.removeEventListener(CONFIG_EVENTS.LAN_SAVED, handleLanConfigSaved)
  window.removeEventListener(CONFIG_EVENTS.COLDCLIENT_SAVED, handleColdClientConfigSaved)
  window.removeEventListener(CONFIG_EVENTS.LOBBY_SAVED, handleLobbyConfigSaved)
  window.removeEventListener(CONFIG_EVENTS.OTHER_SAVED, handleOtherSaved)
})
</script>

<style scoped>
/* 遮罩层 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

/* 模态框主体 */
.modal-content {
  background-color: var(--steam-bg-primary);
  border-radius: 12px;
  border: 1px solid var(--steam-border);
  width: 90%;
  max-width: 1200px;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 24px 48px rgba(0, 0, 0, 0.3);
}

/* 头部 */
.modal-header {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px 24px;
  border-bottom: 1px solid var(--steam-border);
  flex-shrink: 0;
}

.modal-header h3 {
  flex: 1;
  font-size: 18px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0;
}

.header-status {
  display: flex;
  align-items: center;
}

.status-badge {
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-accent-blue);
  background-color: rgba(var(--steam-accent-blue-rgb, 107, 170, 255), 0.1);
  padding: 4px 12px;
  border-radius: 20px;
  border: 1px solid rgba(var(--steam-accent-blue-rgb, 107, 170, 255), 0.2);
}

.close-btn {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 8px;
  background-color: transparent;
  color: var(--steam-text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.close-btn:hover {
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
}

/* 搜索栏 */
.search-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  border-bottom: 1px solid var(--steam-border);
  flex-shrink: 0;
}

.search-icon {
  width: 16px;
  height: 16px;
  color: var(--steam-text-secondary);
  flex-shrink: 0;
}

.search-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 13px;
  outline: none;
  transition: border-color 0.15s ease;
}

.search-input:focus {
  border-color: var(--steam-accent-blue);
}

.search-input::placeholder {
  color: var(--steam-text-secondary);
}

.clear-search {
  width: 24px;
  height: 24px;
  border: none;
  border-radius: 6px;
  background-color: transparent;
  color: var(--steam-text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.clear-search:hover {
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
}

/* 主体布局 */
.modal-body {
  flex: 1;
  overflow: hidden;
  display: flex;
}

/* 底部操作栏 */
.modal-footer {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  padding: 12px 24px;
  border-top: 1px solid var(--steam-border);
  flex-shrink: 0;
}

/* 左侧导航 */
.config-nav {
  width: 200px;
  border-right: 1px solid var(--steam-border);
  overflow-y: auto;
  padding: 16px;
  flex-shrink: 0;
}

.nav-section {
  margin-bottom: 20px;
}

.nav-section h4 {
  font-size: 11px;
  font-weight: 600;
  color: var(--steam-text-secondary);
  text-transform: uppercase;
  margin: 0 0 8px 0;
  padding-left: 8px;
  letter-spacing: 0.5px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 8px 12px;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s ease;
  background-color: transparent;
  color: var(--steam-text-primary);
  text-align: left;
  position: relative;
}

.nav-item:hover {
  background-color: var(--steam-bg-tertiary);
}

.nav-item.active {
  background-color: var(--steam-accent-blue);
  color: white;
}

.nav-item.configured:not(.active) {
  color: #10b981;
}

.nav-item.configured:not(.active)::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 60%;
  background-color: #10b981;
  border-radius: 2px;
}

.nav-item.disabled {
  opacity: 0.5;
  cursor: not-allowed;
  color: var(--steam-text-secondary);
}

.nav-item.disabled:hover {
  background-color: transparent;
}

.nav-label {
  flex: 1;
}

.nav-status {
  font-size: 12px;
  color: #10b981;
}

.nav-status svg {
  width: 14px;
  height: 14px;
}

/* 右侧内容 */
.config-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

.config-panel {
  animation: fadeIn 0.2s ease;
}

.config-panel h3 {
  font-size: 16px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 20px 0;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--steam-border);
}

.config-panel h4 {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 20px 0 12px 0;
}

/* 格式说明 */
.format-info {
  background-color: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  padding: 14px 16px;
  margin-bottom: 16px;
}

.format-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 10px;
  font-size: 13px;
  font-weight: 600;
  color: var(--steam-accent-blue);
}

.format-header svg {
  width: 16px;
  height: 16px;
}

.format-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
  margin-bottom: 10px;
}

.format-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
}

.format-label {
  color: var(--steam-text-secondary);
  white-space: nowrap;
}

.format-value {
  color: var(--steam-text-primary);
  font-family: 'Courier New', monospace;
}

.format-example {
  background-color: var(--steam-bg-primary);
  border-radius: 6px;
  padding: 10px 12px;
}

.format-example-title {
  font-size: 12px;
  font-weight: 500;
  color: var(--steam-text-secondary);
  margin-bottom: 6px;
  display: block;
}

.format-code {
  font-size: 12px;
  color: #e2e8f0;
  background-color: #1e293b;
  padding: 8px 12px;
  border-radius: 4px;
  overflow-x: auto;
  line-height: 1.5;
  margin: 0;
  white-space: pre;
}

/* 配置分组 */
.config-section {
  background-color: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 16px;
}

.config-section h4 {
  margin-top: 0;
  margin-bottom: 16px;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--steam-border);
  color: var(--steam-accent-blue);
}

/* 表单 */
.form-group {
  margin-bottom: 16px;
}

.form-group > label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-text-primary);
  margin-bottom: 6px;
}

.form-group input,
.form-group select,
.form-group textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color 0.15s ease;
  box-sizing: border-box;
}

.form-group input:focus,
.form-group select:focus,
.form-group textarea:focus {
  border-color: var(--steam-accent-blue);
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 14px;
  color: var(--steam-text-primary);
  margin-bottom: 0;
}

.checkbox-label input[type="checkbox"] {
  width: 18px;
  height: 18px;
  accent-color: var(--steam-accent-blue);
  flex-shrink: 0;
  margin: 0;
}

.checkbox-label span {
  line-height: 1.4;
}

.field-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 4px 0 0 0;
}

.dlc-textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 13px;
  font-family: 'Consolas', 'Courier New', monospace;
  resize: vertical;
  outline: none;
  margin-top: 8px;
}

.dlc-textarea:focus {
  border-color: var(--steam-accent-blue);
}

/* DLC 模式选择器（单选互斥） */
.dlc-mode-selector {
  display: flex;
  gap: 24px;
  margin: 16px 0;
  padding: 12px 16px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--steam-border);
}

.dlc-mode-option {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 14px;
  color: var(--steam-text-primary);
}

.dlc-mode-option input[type="radio"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
}

.dlc-mode-label {
  font-weight: 500;
}

.dlc-manual-section {
  margin-top: 16px;
}

/* 高级选项 */
.advanced-toggle {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  color: var(--steam-text-secondary);
  font-size: 13px;
  padding: 10px 0;
  border-top: 1px solid var(--steam-border);
  margin-top: 12px;
}

.advanced-toggle:hover {
  color: var(--steam-accent-blue);
}

.chevron {
  width: 16px;
  height: 16px;
  transition: transform 0.2s ease;
  flex-shrink: 0;
}

.chevron.rotated {
  transform: rotate(180deg);
}

.advanced-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
}

.advanced-content {
  margin-top: 12px;
}

/* 面板操作 */
.panel-actions {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
}

/* 列表 */
.list-container {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.list-item {
  background-color: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  padding: 12px;
  display: flex;
  gap: 8px;
  align-items: center;
}

.list-item.expandable {
  flex-direction: column;
  align-items: stretch;
}

.item-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  cursor: pointer;
  padding: 4px 0;
}

.item-title {
  font-weight: 500;
  color: var(--steam-text-primary);
}

.item-badge {
  font-size: 11px;
  font-weight: 500;
  color: var(--steam-text-secondary);
  background-color: var(--steam-bg-tertiary);
  padding: 2px 8px;
  border-radius: 4px;
  text-transform: capitalize;
}

.item-body {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--steam-border);
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.item-body .form-group {
  margin-bottom: 0;
}

.list-item input,
.list-item select,
.list-item textarea {
  flex: 1;
  padding: 8px 10px;
  border: 1px solid var(--steam-border);
  border-radius: 6px;
  background-color: var(--steam-bg-primary);
  color: var(--steam-text-primary);
  font-size: 13px;
}

/* 空状态 */
.empty-state {
  text-align: center;
  padding: 32px 16px;
  color: var(--steam-text-secondary);
  font-size: 13px;
}

/* 按钮 */
.btn-primary {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  background-color: var(--steam-accent-blue);
  color: white;
  position: relative;
  overflow: hidden;
}

.btn-primary:hover:not(:disabled) {
  background-color: var(--steam-accent-hover);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(107, 170, 255, 0.3);
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-primary.saved {
  background-color: #10b981;
}

.btn-secondary {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 10px 16px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
}

.btn-secondary:hover {
  background-color: var(--steam-border);
}

.btn-add {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  background-color: var(--steam-accent-blue);
  color: white;
}

.btn-add:hover {
  background-color: var(--steam-accent-hover);
}

.btn-icon {
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 6px;
  background-color: transparent;
  color: var(--steam-text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
  font-size: 18px;
  flex-shrink: 0;
}

.btn-icon:hover {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

/* 旋转图标 */
.spin-icon {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* Toast 提示 */
.toast-success {
  position: absolute;
  bottom: 80px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 20px;
  background-color: #10b981;
  color: white;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  z-index: 9999;
}

.toast-success svg {
  width: 20px;
  height: 20px;
}

/* Toast 动画 */
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(-50%) translateY(20px);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(-20px);
}

/* 淡入动画 */
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

/* 取消按钮 */
.btn-cancel {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 6px 16px;
  border: 1px solid var(--steam-border);
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-secondary);
  white-space: nowrap;
}

.btn-cancel:hover {
  background-color: var(--steam-border);
  color: var(--steam-text-primary);
}

/* 保存按钮 */
.btn-save {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 6px 20px;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  background-color: var(--steam-accent-blue);
  color: white;
  white-space: nowrap;
  position: relative;
  overflow: hidden;
}

.btn-save:hover:not(:disabled) {
  background-color: var(--steam-accent-hover);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(107, 170, 255, 0.3);
}

.btn-save:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-save.saving {
  background-color: var(--steam-accent-blue);
  cursor: wait;
}

.btn-save.saved {
  background-color: #10b981;
}

.btn-save svg {
  width: 16px;
  height: 16px;
}

.color-input .color-row {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
}

.color-input .color-row input {
  text-align: center;
  padding: 8px 4px;
}

.form-row.three-col {
  grid-template-columns: repeat(3, 1fr);
}

/* 搜索内容匹配提示 */
.nav-item .content-match {
  font-size: 10px;
  color: var(--steam-accent-blue);
  margin-left: auto;
  padding: 1px 4px;
  background-color: rgba(var(--steam-accent-blue-rgb, 107, 170, 255), 0.1);
  border-radius: 4px;
}

/* 响应式 */
@media (max-width: 768px) {
  .modal-body {
    flex-direction: column;
  }
  
  .config-nav {
    width: 100%;
    border-right: none;
    border-bottom: 1px solid var(--steam-border);
    display: flex;
    overflow-x: auto;
  }
  
  .nav-section {
    display: flex;
    gap: 4px;
  }
  
  .nav-section h4 {
    display: none;
  }
  
  .form-row {
    grid-template-columns: 1fr;
  }
  
  .modal-header h3 {
    font-size: 16px;
  }
  
  .status-badge {
    font-size: 12px;
  }
}
</style>
