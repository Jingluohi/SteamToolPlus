<template>
  <!-- 应用程序根组件 -->
  <!-- 实现无边框窗口布局、主题注入、路由视图、全局背景轮播 -->
  <div 
    id="app-root" 
    class="app-root"
    :class="{ 
      'fullscreen': isFullscreen,
      // 浏览页下载时不预留底部空间，让全宽进度条直接遮挡最下方内容（避免页码/最后一排被上抬）；
      // 其余页面保持预留底部空间，内容不被进度条遮挡
      'has-download-bar': hasActiveDownload && currentPageType !== 'browse'
    }"
  >
    <!-- 启动门禁遮罩：程序一渲染即显示，覆盖全窗口阻止任何点击，
         直到启动更新/公告检查得出结果（弹窗或放行）后才隐藏，
         确保更新/公告弹窗作为首个交互内容弹出 -->
    <div v-if="showStartupGate" class="startup-gate" aria-hidden="true">
      <div class="startup-gate-spinner"></div>
      <p class="startup-gate-text">正在初始化...</p>
    </div>

    <!-- 全局背景轮播组件 - 根据当前路由动态变化 -->
    <BackgroundSlideshow 
      v-if="showBackground"
      :key="currentPageType"
      ref="backgroundRef"
      class="app-background"
      :page-type="currentPageType"
    >
      <!-- 标题栏 - 全屏模式下也保留，但可以通过CSS调整样式 -->
      <TitleBar 
        class="app-title-bar"
        @toggle-fullscreen="toggleFullscreen"
      />
      
      <!-- 主内容区域 -->
      <div class="main-container">
        <main class="main-content">
          <RouterView />
        </main>
      </div>

      <!-- 底部下载进度条：有下载任务时显示，点击进入下载列表页 -->
      <DownloadBar />
    </BackgroundSlideshow>
    
    <!-- 当不显示背景时，直接渲染内容 -->
    <template v-else>
      <!-- 标题栏 - 全屏模式下也保留，但可以通过CSS调整样式 -->
      <TitleBar 
        class="app-title-bar"
        @toggle-fullscreen="toggleFullscreen"
      />
      
      <!-- 主内容区域 -->
      <div class="main-container">
        <main class="main-content">
          <RouterView />
        </main>
      </div>
    </template>

    <!-- 底部下载进度条：有下载任务时显示，点击进入下载列表页。
         放在背景组件之外，避免路由切换时随背景 key 变化被重建 -->
    <DownloadBar />

    <!-- 全局「下载完成 → 提示打补丁」弹窗：任意界面下载完成时弹出 -->
    <DownloadCompleteModal
      v-model="showDownloadComplete"
      :game-id="completedGameId"
    />

    <!-- 全局「未设置默认浏览器」提示弹窗：任意界面打开链接失败时弹出 -->
    <NoBrowserModal />

    <!-- 启动时自动更新提示弹窗：仅可通过弹窗按钮关闭，点击空白处不关闭 -->
    <div v-if="showUpdateModal" class="update-modal-overlay">
      <div class="update-modal">
        <h2 class="update-modal-title">发现新版本</h2>
        <div class="update-modal-body">
          <p class="update-modal-info">
            本地资源版本：<strong>{{ localResourceVersion }}</strong>
          </p>
          <p class="update-modal-info">
            远程资源版本：<strong>{{ remoteResourceVersion }}</strong>
          </p>
          <p v-if="remoteHasExeUpdate" class="update-modal-type type-exe">
            本次更新包含主程序，更新后会自动重启
          </p>
          <p v-else class="update-modal-type type-resource">
            本次仅更新资源文件，无需重启
          </p>
          <p v-if="remoteDescription" class="update-modal-desc">
            {{ remoteDescription }}
          </p>

          <div v-if="isUpdating" class="update-progress">
            <div class="progress-header">
              <span>{{ statusText }}</span>
              <span>{{ downloadedText }} / {{ totalText }}</span>
            </div>
            <div class="progress-bar-bg">
              <div class="progress-bar-fill" :style="{ width: `${downloadPercent}%` }" />
            </div>
            <div class="progress-percent">{{ downloadPercent }}%</div>
          </div>
        </div>
        <div class="update-modal-footer">
          <button
            class="modal-btn secondary"
            :disabled="isUpdating"
            @click="closeUpdateModal"
          >
            稍后更新
          </button>
          <button
            class="modal-btn primary"
            :disabled="isUpdating"
            @click="handleStartupUpdate"
          >
            {{ remoteHasExeUpdate ? '立即更新并重启' : '立即更新资源' }}
          </button>
        </div>
      </div>
    </div>

    <!-- 启动时「公告板」弹窗：无可用更新且存在 resources/announcement/announcement.md 时弹出，
         尺寸为程序窗口的 80%，正文为 markdown 渲染（支持文字、图片、链接），
         仅可通过弹窗按钮或右上角关闭，点击空白处不关闭 -->
    <AnnouncementModal
      v-if="showAnnouncementModal"
      :content="announcementContent"
      :base-path="announcementBasePath"
      @close="closeAnnouncementModal"
    />
  </div>
</template>

<script setup lang="ts">
/**
 * App.vue - 应用程序根组件
 * 实现无边框窗口布局、主题注入、路由、全局背景轮播
 */

import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import { RouterView, useRoute } from 'vue-router'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import { useWindowStore } from './store/window.store'
import { useThemeStore } from './store/theme.store'
import { useConfigStore } from './store/config.store'
import { useDownloadManagerStore } from './store/downloadManager.store'
import { clearAllImageCaches, triggerImageRefresh, triggerImageRelease } from './services/imageCache.service'
import TitleBar from './components/layout/TitleBar.vue'
import BackgroundSlideshow from './components/background/BackgroundSlideshow.vue'
import DownloadBar from './components/download/DownloadBar.vue'
import DownloadCompleteModal from './components/download/DownloadCompleteModal.vue'
import NoBrowserModal from './components/common/NoBrowserModal.vue'
import AnnouncementModal from './components/common/AnnouncementModal.vue'
import type { PageType } from './types/background.types'

interface UpdateCheckResult {
  has_update: boolean
  local_resource_version: string
  remote_resource_version: string
  remote_description: string | null
  has_exe_update: boolean
  patch_url: string
  /** 本地 version.json 中的更新说明（上一次更新的说明），已不再作为公告内容使用，仅保留下发字段 */
  local_description: string | null
}

/** 公告板信息（后端 get_announcement 命令返回） */
interface AnnouncementInfo {
  /** 公告正文（markdown 原文） */
  content: string
  /** announcement 目录绝对路径，前端据此解析公告内的相对图片 */
  base_path: string
}

interface DownloadProgress {
  url: string
  downloaded: number
  total: number
  percent: number
}

// 获取store
const windowStore = useWindowStore()
const themeStore = useThemeStore()
const configStore = useConfigStore()
const downloadManagerStore = useDownloadManagerStore()
const route = useRoute()

// 背景组件引用
const backgroundRef = ref<InstanceType<typeof BackgroundSlideshow> | null>(null)

// 事件监听器句柄，用于卸载时清理
let unlistenFocused: (() => void) | null = null
let unlistenBlurred: (() => void) | null = null
let unlistenRestartRequest: (() => void) | null = null
let unlistenResourceUpdate: (() => void) | null = null
let unlistenDownloadProgress: UnlistenFn | null = null
let unlistenUpdateFinished: UnlistenFn | null = null
let unlistenUpdateError: UnlistenFn | null = null
/** 游戏下载完成事件监听（成功时弹「提示打补丁」全局弹窗） */
let unlistenDownloadFinished: UnlistenFn | null = null

// --- GPU 显存空闲回收 ---
// 用户长时间无操作时，结束 WebView2 GPU 进程释放显存（GPU 进程由 Chromium 自动重启）
// 避免翻页浏览大量封面后 GPU 显存持续累积
const IDLE_TIMEOUT_MS = 8 * 60 * 1000 // 空闲 8 分钟触发
let lastActivityTime = Date.now()
let idleCheckTimer: ReturnType<typeof setInterval> | null = null

// 重置空闲计时（用户有操作时调用）
function resetIdleTimer() {
  lastActivityTime = Date.now()
}

// 需要监听的用户操作事件
const idleEvents = ['pointermove', 'pointerdown', 'keydown', 'wheel', 'touchstart'] as const

// 启动时自动更新弹窗状态
const showUpdateModal = ref(false)
const localResourceVersion = ref('')
const remoteResourceVersion = ref('')
const remoteDescription = ref('')
const remoteHasExeUpdate = ref(false)
const patchUrl = ref('')
const isUpdating = ref(false)
const statusText = ref('')
const downloadPercent = ref(0)
const downloadedBytes = ref(0)
const totalBytes = ref(0)

// 启动时「公告板」弹窗状态（无可用更新时，读取 resources/announcement/announcement.md 展示）
const showAnnouncementModal = ref(false)
const announcementContent = ref('')
const announcementBasePath = ref('')

// 启动门禁遮罩：程序一渲染即显示，防止在更新/公告检查完成前用户误点到底下页面；
// 检查有结果（弹出更新/公告弹窗或确认无需弹窗）后置为 false
const showStartupGate = ref(true)

// 兜底看门狗：无论更新/公告检查是否顺利，最多等待一段时间后强制解除门禁，
// 避免因任何异常导致程序被永久锁住不可交互
setTimeout(() => {
  showStartupGate.value = false
}, 8000)

// 全局「下载完成 → 提示打补丁」弹窗状态
const showDownloadComplete = ref(false)
/** 最近一次成功下载完成的游戏 ID */
const completedGameId = ref('')

/**
 * 格式化字节数为可读文本
 */
function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(1024))
  return `${(bytes / Math.pow(1024, i)).toFixed(2)} ${units[i]}`
}

const downloadedText = computed(() => formatBytes(downloadedBytes.value))
const totalText = computed(() => formatBytes(totalBytes.value))

/**
 * 关闭自动更新弹窗
 */
function closeUpdateModal() {
  if (isUpdating.value) return
  showUpdateModal.value = false
}

/**
 * 展示启动时「公告板」弹窗
 * 无可用更新时，读取 resources/announcement/announcement.md 渲染展示（支持文字/图片/链接）；
 * 文件不存在时静默跳过，不弹窗
 */
async function showAnnouncement() {
  try {
    const info = await invoke<AnnouncementInfo | null>('get_announcement')
    if (info && info.content.trim()) {
      announcementContent.value = info.content
      announcementBasePath.value = info.base_path
      showAnnouncementModal.value = true
    }
  } catch {
    // 读取公告失败不影响程序启动，静默忽略（保持原有"无公告不弹窗"行为）
  }
}

/**
 * 关闭公告板弹窗
 */
function closeAnnouncementModal() {
  showAnnouncementModal.value = false
}

/**
 * 启动时自动检查更新
 * 轮询后端预检查结果，不阻塞窗口显示，更新弹窗第一时间弹出
 */
async function checkUpdateOnStartup() {
  // 轮询后端预检查结果（后端在 setup 中已异步触发更新检查）
  const maxAttempts = 30 // 30 * 200ms = 6 秒超时
  for (let i = 0; i < maxAttempts; i++) {
    try {
      const pendingResult = await invoke<UpdateCheckResult | null>('get_pending_startup_update')
      if (pendingResult !== null) {
        // 后端检查已完成
        if (pendingResult.has_update) {
          // 显示更新弹窗
          localResourceVersion.value = pendingResult.local_resource_version
          remoteResourceVersion.value = pendingResult.remote_resource_version
          remoteDescription.value = pendingResult.remote_description ?? ''
          remoteHasExeUpdate.value = pendingResult.has_exe_update
          patchUrl.value = pendingResult.patch_url
          showUpdateModal.value = true
        } else {
          // 没有更新 → 弹出公告板（读取 resources/announcement/announcement.md）
          await showAnnouncement()
        }
        // 检查完成：有弹窗时弹窗遮罩接管阻挡，无弹窗则放行，两种情况都解除启动门禁
        showStartupGate.value = false
        return // 检查完成，无论是否有更新都结束
      }
    } catch {
      // 忽略轮询中的错误
    }
    // 等待 200ms 后再次轮询
    await new Promise(resolve => setTimeout(resolve, 200))
  }

  // 兜底：轮询超时后直接调用实时检查
  try {
    const result = await invoke<UpdateCheckResult>('check_for_update')
    if (result.has_update) {
      localResourceVersion.value = result.local_resource_version
      remoteResourceVersion.value = result.remote_resource_version
      remoteDescription.value = result.remote_description ?? ''
      remoteHasExeUpdate.value = result.has_exe_update
      patchUrl.value = result.patch_url
      showUpdateModal.value = true
    } else {
      // 没有更新 → 弹出公告板（读取 resources/announcement/announcement.md）
      await showAnnouncement()
    }
  } catch (error) {
    console.log('[自动更新] 检查失败:', error)
  } finally {
    // 无论实时检查成功与否都解除启动门禁（弹窗遮罩会接管阻挡）
    showStartupGate.value = false
  }
}

/**
 * 启动时弹窗中的立即更新按钮
 */
async function handleStartupUpdate() {
  if (isUpdating.value) return

  isUpdating.value = true
  downloadPercent.value = 0
  downloadedBytes.value = 0
  totalBytes.value = 0
  statusText.value = '正在下载更新...'

  try {
    const willRestart = await invoke<boolean>('apply_update', {
      patchUrl: patchUrl.value,
      hasExeUpdate: remoteHasExeUpdate.value,
    })

    if (!willRestart) {
      statusText.value = '资源更新完成，正在刷新...'
      // 资源更新完成后刷新页面以重新加载配置
      window.location.reload()
    }
  } catch (error) {
    statusText.value = `更新失败: ${error}`
    isUpdating.value = false
  }
}

// 计算属性：是否全屏
const isFullscreen = computed(() => windowStore.isFullscreen)

// 是否有正在下载的任务（下载中才显示全宽底部进度条）：为真时主内容区底部
// 预留 58px 给进度条，避免内容被遮挡 —— 见 .has-download-bar 规则。
// 说明：浏览页例外，下载时不预留空间，底部进度条直接遮挡最下方内容，
// 避免把页码/最后一排游戏上抬。
// 用 downloadingCount 而非 activeDownloads：暂停任务只显示右下角小入口，
// 不占全宽，无需预留下方空间。
const hasActiveDownload = computed(() => downloadManagerStore.downloadingCount > 0)

// 路由路径到页面类型的映射表
const PAGE_TYPE_MAP: { keyword: string; pageType: PageType }[] = [
  { keyword: '/library', pageType: 'library' },
  { keyword: '/download', pageType: 'download' },
  { keyword: '/downloads', pageType: 'download' },
  { keyword: '/patch', pageType: 'patch' },
  { keyword: '/settings', pageType: 'settings' },
  { keyword: '/about', pageType: 'about' },
  { keyword: '/update-check', pageType: 'about' },
]

// 计算属性：当前页面类型
const currentPageType = computed<PageType>(() => {
  const path = route.path

  // 首页或包含 browse 的路由都映射到 browse
  if (path === '/' || path.includes('/browse')) {
    return 'browse'
  }

  // 使用映射表进行 O(1) 查找，替代多层 if-else
  for (const { keyword, pageType } of PAGE_TYPE_MAP) {
    if (path.includes(keyword)) {
      return pageType
    }
  }

  // 默认返回 browse（因为首页是浏览页面）
  return 'browse'
})

// 计算属性：是否显示背景
const showBackground = computed(() => {
  // 所有页面都显示背景，由 BackgroundSlideshow 内部根据配置决定是否显示
  return true
})

// 切换全屏
const toggleFullscreen = () => {
  windowStore.toggleFullscreen()
}

// 监听路由变化，刷新背景
watch(() => route.path, () => {
  // 路由变化时，背景组件会根据新的 pageType 重新加载
})

// 组件挂载时初始化
onMounted(async () => {
  // 第一步就触发启动更新/公告检查（不 await，独立异步执行）。
  // 门禁遮罩默认已显示，即使后续初始化步骤出错，检查路径也会自行解除门禁。
  checkUpdateOnStartup()

  // 先加载配置
  await configStore.loadConfig()

  // 如果配置加载成功，从配置加载主题设置
  if (configStore.config) {
    themeStore.loadFromConfig(configStore.config.theme)
  } else {
    // 如果配置加载失败（第一次打开），默认使用深色主题
    themeStore.setThemeMode('dark')
  }

  // 初始化主题监听（跟随系统主题变化）
  themeStore.initTheme()

  // 初始化窗口状态
  await windowStore.initWindow()

  // 监听窗口获得焦点事件：从托盘恢复时刷新背景图片
  unlistenFocused = await listen('window-focused', () => {
    // 窗口仍处于隐藏状态时跳过刷新，避免无谓重载
    if (document.hidden) return
    backgroundRef.value?.refreshItems()
    // 触发所有游戏封面和库背景图片重新加载
    triggerImageRefresh()
  })

  // 监听窗口隐藏事件（仅在隐藏到托盘时由 Rust 端触发）
  // 隐藏时彻底清空图片资源以释放显存，等待窗口恢复时再重新加载
  unlistenBlurred = await listen('window-blurred', () => {
    // 清空图片缓存，释放内存
    clearAllImageCaches()
    // 停止背景轮播并释放背景图片资源
    backgroundRef.value?.releaseImages()
    // 触发释放信号，清空所有游戏封面的 coverUrl（不重新加载）
    triggerImageRelease()
  })

  // 监听游戏下载完成事件（成功时无论当前在哪个界面都弹「提示打补丁」弹窗；
  // 弹窗组件内部会判断该游戏是否有补丁配置，无补丁则仅提示不纠错）
  unlistenDownloadFinished = await listen<{
    gameId: string
    success: boolean
    message: string
  }>('depot-download-finished', (event) => {
    if (event.payload.success) {
      completedGameId.value = event.payload.gameId
      showDownloadComplete.value = true
    }
  })

  // 监听来自新实例的重启请求
  // 当用户从新的 exe 路径启动程序时，旧实例会收到此事件并提示用户，随后自动退出
  const handleRestartRequest = (newPath: string) => {
    const message = `检测到程序从新的位置启动：\n${newPath}\n\n当前实例将关闭，新实例会继续运行。`
    window.alert(message)
    invoke('exit_app', { exitCode: 0 })
  }

  unlistenRestartRequest = await listen<{ new_path: string }>('instance-restart-request', (event) => {
    handleRestartRequest(event.payload.new_path)
  })

  // 启动时主动检查是否已有重启请求（避免事件在页面挂载前已发出）
  try {
    const pendingRestartPath = await invoke<string | null>('check_instance_restart_request')
    if (pendingRestartPath) {
      handleRestartRequest(pendingRestartPath)
    }
  } catch {
    // 忽略检查失败
  }

  // 监听资源更新事件，仅记录日志
  unlistenResourceUpdate = await listen('resource-update-start', (event) => {
    console.log('[资源更新] 开始更新:', event.payload)
  })

  // 注册下载进度与更新完成/错误事件监听
  unlistenDownloadProgress = await listen<DownloadProgress>('download-progress', (event) => {
    downloadedBytes.value = event.payload.downloaded
    totalBytes.value = event.payload.total
    downloadPercent.value = event.payload.percent
    statusText.value = '正在下载更新...'
  })

  unlistenUpdateFinished = await listen<{ version: string; has_exe_update: boolean }>('resource-update-finished', (event) => {
    if (event.payload.has_exe_update) {
      statusText.value = 'exe 更新完成，正在重启...'
    } else {
      statusText.value = '资源更新完成，正在刷新...'
      window.location.reload()
    }
  })

  unlistenUpdateError = await listen<{ error: string }>('resource-update-error', (event) => {
    statusText.value = `更新失败: ${event.payload.error}`
    isUpdating.value = false
  })

  // --- 启动 GPU 显存空闲回收 ---
  // 监听用户操作，空闲超时后结束 GPU 进程释放显存
  idleEvents.forEach((evt) => window.addEventListener(evt, resetIdleTimer))
  idleCheckTimer = setInterval(() => {
    if (Date.now() - lastActivityTime >= IDLE_TIMEOUT_MS) {
      // 重置空闲计时，避免每个检查周期重复触发
      lastActivityTime = Date.now()
      invoke('kill_gpu_process')
    }
  }, 30000)
})

// 组件卸载时清理事件监听器
onUnmounted(() => {
  unlistenFocused?.()
  unlistenBlurred?.()
  unlistenRestartRequest?.()
  unlistenResourceUpdate?.()
  unlistenDownloadProgress?.()
  unlistenUpdateFinished?.()
  unlistenUpdateError?.()
  unlistenDownloadFinished?.()

  // 清理 GPU 空闲回收监听
  if (idleCheckTimer !== null) {
    clearInterval(idleCheckTimer)
    idleCheckTimer = null
  }
  idleEvents.forEach((evt) => window.removeEventListener(evt, resetIdleTimer))
})
</script>

<style scoped>
.app-root {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  position: relative;
  /* 使用主题对应的兜底背景色，避免图片缺失或透明窗口时透出异常 */
  background: var(--app-bg-color);
}

/* 全局背景层 */
.app-background {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 0;
}

/* 标题栏样式 - 确保始终显示在最上层 */
.app-title-bar {
  position: relative;
  z-index: 1000;
  flex-shrink: 0;
}

.main-container {
  flex: 1;
  display: flex;
  overflow: hidden;
  position: relative;
  z-index: 1;
}

.main-content {
  flex: 1;
  overflow: hidden;
  position: relative;
  z-index: 1;
}

/* 全屏模式样式 */
.app-root.fullscreen {
  background: transparent;
}

/* 有下载任务时，主内容区底部预留进度条高度（50px + 间距），避免遮挡页面内容 */
.app-root.has-download-bar .main-content {
  padding-bottom: 58px;
}

/* 全屏模式下标题栏自动隐藏，鼠标悬停时显示 */
.app-root.fullscreen .app-title-bar {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
  transform: translateY(-100%);
  transition: transform 0.3s ease-out;
  opacity: 0;
}

/* 鼠标悬停在顶部区域时显示标题栏 */
.app-root.fullscreen:hover .app-title-bar,
.app-root.fullscreen .app-title-bar:hover {
  transform: translateY(0);
  opacity: 1;
}

/* 添加一个触发区域，让鼠标更容易触发标题栏显示 */
.app-root.fullscreen::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 8px;
  z-index: 999;
}

.app-root.fullscreen .main-content {
  padding: 0;
}

/* 启动门禁遮罩：覆盖全窗口阻止交互，直到启动更新/公告检查得出结果 */
.startup-gate {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 10000;
  background: var(--app-bg-color, #1b2838);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  cursor: wait;
}

/* 启动门禁加载指示器 */
.startup-gate-spinner {
  width: 36px;
  height: 36px;
  border: 3px solid var(--steam-border-color, rgba(255, 255, 255, 0.15));
  border-top-color: var(--steam-accent, #66c0f4);
  border-radius: 50%;
  animation: startup-gate-spin 0.9s linear infinite;
}

@keyframes startup-gate-spin {
  to {
    transform: rotate(360deg);
  }
}

.startup-gate-text {
  margin: 0;
  font-size: 13px;
  color: var(--steam-text-secondary, #8f98a0);
}

/* 启动时自动更新弹窗 */
.update-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
  backdrop-filter: blur(4px);
}

.update-modal {
  width: 90%;
  max-width: 480px;
  background: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border-color);
  border-radius: 4px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
  overflow: hidden;
}

.update-modal-title {
  margin: 0;
  padding: 18px 24px;
  font-size: 18px;
  font-weight: 600;
  color: var(--steam-text-primary);
  background: var(--steam-bg-tertiary);
  border-bottom: 1px solid var(--steam-border-color);
}

.update-modal-body {
  padding: 24px;
}

.update-modal-info {
  margin: 0 0 10px;
  font-size: 14px;
  color: var(--steam-text-secondary);
}

.update-modal-info strong {
  color: var(--steam-text-primary);
  font-weight: 600;
}

.update-modal-type {
  margin: 14px 0 0;
  padding: 10px 14px;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
}

.update-modal-type.type-exe {
  background: rgba(255, 159, 67, 0.15);
  color: #ff9f43;
}

.update-modal-type.type-resource {
  background: rgba(59, 130, 246, 0.15);
  color: var(--steam-accent-blue);
}

.update-modal-desc {
  margin: 14px 0 0;
  font-size: 13px;
  color: var(--steam-text-secondary);
  line-height: 1.5;
  padding: 12px;
  background: var(--steam-bg-tertiary);
  border-radius: 4px;
}

/* 公告板正文样式已迁移至 AnnouncementModal.vue（markdown 渲染） */

.update-progress {
  margin-top: 18px;
}

.update-progress .progress-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
  font-size: 13px;
  color: var(--steam-text-secondary);
}

.update-progress .progress-bar-bg {
  height: 8px;
  background: var(--steam-bg-primary);
  border-radius: 4px;
  overflow: hidden;
}

.update-progress .progress-bar-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--steam-accent-blue), var(--steam-accent-green));
  transition: width 0.2s ease-out;
}

.update-progress .progress-percent {
  margin-top: 6px;
  text-align: right;
  font-size: 12px;
  color: var(--steam-text-secondary);
  font-family: monospace;
}

.update-modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px;
  background: var(--steam-bg-tertiary);
  border-top: 1px solid var(--steam-border-color);
}

.modal-btn {
  padding: 8px 18px;
  font-size: 14px;
  border-radius: 4px;
  border: 1px solid var(--steam-border-color);
  cursor: pointer;
  transition: all 0.15s ease-out;
}

.modal-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.modal-btn.secondary {
  background: transparent;
  color: var(--steam-text-primary);
}

.modal-btn.secondary:hover:not(:disabled) {
  background: var(--steam-bg-tertiary);
}

.modal-btn.primary {
  background: var(--steam-accent-blue);
  color: white;
  border-color: var(--steam-accent-blue);
}

.modal-btn.primary:hover:not(:disabled) {
  background: var(--steam-accent-green);
  border-color: var(--steam-accent-green);
}
</style>
