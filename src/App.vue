<template>
  <!-- 应用程序根组件 -->
  <!-- 实现无边框窗口布局、主题注入、路由视图、全局背景轮播 -->
  <div 
    id="app-root" 
    class="app-root"
    :class="{ 'fullscreen': isFullscreen }"
  >
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

    <!-- 启动时自动更新提示弹窗 -->
    <div v-if="showUpdateModal" class="update-modal-overlay" @click.self="closeUpdateModal">
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
import { clearAllImageCaches, triggerImageRefresh } from './services/imageCache.service'
import TitleBar from './components/layout/TitleBar.vue'
import BackgroundSlideshow from './components/background/BackgroundSlideshow.vue'
import type { PageType } from './types/background.types'

interface UpdateCheckResult {
  has_update: boolean
  local_resource_version: string
  remote_resource_version: string
  remote_description: string | null
  has_exe_update: boolean
  patch_url: string
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
 * 启动时自动检查更新
 */
async function checkUpdateOnStartup() {
  try {
    const result = await invoke<UpdateCheckResult>('check_for_update')
    if (!result.has_update) return

    localResourceVersion.value = result.local_resource_version
    remoteResourceVersion.value = result.remote_resource_version
    remoteDescription.value = result.remote_description ?? ''
    remoteHasExeUpdate.value = result.has_exe_update
    patchUrl.value = result.patch_url
    showUpdateModal.value = true
  } catch (error) {
    console.log('[自动更新] 检查失败:', error)
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

// 路由路径到页面类型的映射表
const PAGE_TYPE_MAP: { keyword: string; pageType: PageType }[] = [
  { keyword: '/library', pageType: 'library' },
  { keyword: '/download', pageType: 'download' },
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
    backgroundRef.value?.refreshItems()
    // 触发所有游戏封面和库背景图片重新加载
    triggerImageRefresh()
  })

  // 监听窗口隐藏事件（仅在隐藏到托盘时由 Rust 端触发）
  // 清空图片缓存，确保窗口显示时重新加载
  unlistenBlurred = await listen('window-blurred', () => {
    // 清空图片缓存，释放内存
    clearAllImageCaches()
    // 清空游戏封面的 coverUrl，强制重新获取 asset:// URL
    triggerImageRefresh()
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

  // 启动 2 秒后自动检查更新，避免影响启动速度并确保事件监听已注册
  setTimeout(() => {
    checkUpdateOnStartup()
  }, 2000)
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
