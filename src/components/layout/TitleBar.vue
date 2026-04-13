<template>
  <!-- 整个标题栏设置为可拖动区域 -->
  <header class="title-bar no-select" data-tauri-drag-region="true">
    <!-- 左侧：页面标题 -->
    <div class="title-left">
      <h1 class="page-title">{{ pageTitle }}</h1>
    </div>

    <!-- 中间：搜索框 -->
    <!-- 阻止 mousedown 事件冒泡，使搜索框区域可以正常交互 -->
    <div class="title-center" @mousedown.stop>
      <div class="search-box">
        <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
          <circle cx="11" cy="11" r="8" stroke-width="2"/>
          <path d="M21 21l-4.35-4.35" stroke-width="2" stroke-linecap="round"/>
        </svg>
        <input
          v-model="searchText"
          type="text"
          class="search-input"
          placeholder="搜索游戏..."
          @input="handleSearch"
        />
        <button
          v-if="searchText"
          class="search-clear"
          @click="clearSearch"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path d="M18 6L6 18M6 6l12 12" stroke-width="2" stroke-linecap="round"/>
          </svg>
        </button>
      </div>
    </div>

    <!-- 右侧：设置 + 主题切换 + 窗口控制 -->
    <!-- 阻止 mousedown 事件冒泡，使按钮可以正常交互 -->
    <div class="title-right" @mousedown.stop>
      <!-- 设置按钮 -->
      <button class="window-btn settings-btn" @click="openSettings" title="设置">
        <svg class="settings-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="3"/>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
        </svg>
      </button>

      <!-- 主题切换按钮 -->
      <button class="window-btn theme-btn" @click="themeStore.toggleTheme" title="切换主题">
        <IconSun v-if="themeStore.isDark" class="theme-icon" />
        <IconMoon v-else class="theme-icon" />
      </button>

      <!-- 窗口控制按钮 -->
      <button class="window-btn minimize" @click="minimizeWindow" title="最小化">
        <svg viewBox="0 0 24 24" fill="currentColor">
          <rect x="4" y="11" width="16" height="2" rx="1"/>
        </svg>
      </button>
      <button class="window-btn maximize" @click="toggleMaximize" title="最大化/还原">
        <svg v-if="isMaximized" viewBox="0 0 24 24" fill="currentColor">
          <path d="M4 8v10h10V8H4zm8 8H6v-6h6v6zm4-12v10h-2V6h-6V4h10v2h-2z"/>
        </svg>
        <svg v-else viewBox="0 0 24 24" fill="currentColor">
          <rect x="4" y="4" width="16" height="16" rx="2"/>
        </svg>
      </button>
      <button class="window-btn close" @click="closeWindow" title="关闭">
        <svg viewBox="0 0 24 24" fill="currentColor">
          <path d="M18.3 5.7a1 1 0 0 0-1.4 0L12 10.6 7.1 5.7a1 1 0 0 0-1.4 1.4L10.6 12l-4.9 4.9a1 1 0 0 0 1.4 1.4L12 13.4l4.9 4.9a1 1 0 0 0 1.4-1.4L13.4 12l4.9-4.9a1 1 0 0 0 0-1.4z"/>
        </svg>
      </button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRoute } from 'vue-router'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { useThemeStore } from '../../stores/theme'
import { useGamesStore } from '../../stores/games'
import IconSun from '../icons/IconSun.vue'
import IconMoon from '../icons/IconMoon.vue'

const route = useRoute()
const themeStore = useThemeStore()
const gamesStore = useGamesStore()

// 搜索文本
const searchText = ref('')

// 窗口最大化状态
const isMaximized = ref(false)

// 窗口大小变化监听器清理函数
let unlistenResize: (() => void) | null = null

// 当前页面标题
const pageTitle = computed(() => {
  return (route.meta.title as string) || '全部游戏'
})

// 处理搜索输入
const handleSearch = () => {
  gamesStore.setSearchQuery(searchText.value)
}

// 清空搜索
const clearSearch = () => {
  searchText.value = ''
  gamesStore.setSearchQuery('')
}

// 窗口控制函数
const minimizeWindow = async () => {
  const window = getCurrentWindow()
  await window.minimize()
}

const toggleMaximize = async () => {
  const window = getCurrentWindow()
  const maximized = await window.isMaximized()
  if (maximized) {
    await window.unmaximize()
  } else {
    await window.maximize()
  }
  isMaximized.value = await window.isMaximized()
}

const closeWindow = async () => {
  const window = getCurrentWindow()
  await window.close()
}

// 打开设置窗口
const openSettings = async () => {
  console.log('开始打开设置窗口...')
  try {
    // 检查设置窗口是否已存在
    const existingWindow = await WebviewWindow.getByLabel('settings')
    console.log('检查已存在窗口:', existingWindow)
    if (existingWindow) {
      // 如果窗口已存在，将其置于前台
      await existingWindow.setFocus()
      console.log('已有窗口，设置为焦点')
      return
    }

    // 获取当前窗口的 URL 基础路径
    const baseUrl = window.location.origin
    const settingsUrl = `${baseUrl}/settings.html`
    console.log('创建窗口，URL:', settingsUrl)

    // 创建新的设置窗口
    const settingsWindow = new WebviewWindow('settings', {
      url: settingsUrl,
      title: '设置',
      width: 720,
      height: 480,
      minWidth: 720,
      minHeight: 480,
      maxWidth: 720,
      maxHeight: 480,
      resizable: false,
      center: true,
      decorations: false,
      transparent: false,
      shadow: true,
      visible: true
    })

    // 监听窗口创建成功事件
    settingsWindow.once('tauri://created', () => {
      console.log('设置窗口创建成功')
    })

    // 监听窗口创建失败事件
    settingsWindow.once('tauri://error', (e) => {
      console.error('设置窗口创建失败:', e)
    })

    console.log('设置窗口已创建')
  } catch (error) {
    console.error('打开设置窗口失败:', error)
  }
}

// 初始化窗口状态
onMounted(async () => {
  const window = getCurrentWindow()
  isMaximized.value = await window.isMaximized()
  
  // 监听窗口大小变化
  unlistenResize = await window.onResized(async () => {
    isMaximized.value = await window.isMaximized()
  })
})

// 组件卸载时清理监听器
onUnmounted(() => {
  unlistenResize?.()
})
</script>

<style scoped>
/* 标题栏 - 整个区域可拖动 */
.title-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 48px;
  padding: 0 16px;
  background-color: var(--bg-primary);
  border-bottom: 1px solid var(--border-color);
  transition: background-color 0.2s ease;
  /* 整个标题栏都是拖动区域 */
  app-region: drag;
  -webkit-app-region: drag;
}

/* 左侧标题 */
.title-left {
  width: 200px;
  flex-shrink: 0;
}

.page-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

/* 中间搜索框 */
.title-center {
  flex: 1;
  display: flex;
  justify-content: center;
  /* 允许在此区域进行鼠标交互 */
  app-region: no-drag;
  -webkit-app-region: no-drag;
}

.search-box {
  position: relative;
  width: 100%;
  max-width: 400px;
  display: flex;
  align-items: center;
}

.search-icon {
  position: absolute;
  left: 12px;
  width: 16px;
  height: 16px;
  color: var(--text-secondary);
  pointer-events: none;
}

.search-input {
  width: 100%;
  height: 32px;
  padding: 0 36px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
  transition: border-color 0.15s ease, background-color 0.15s ease;
  /* 输入框不参与拖动 */
  app-region: no-drag;
  -webkit-app-region: no-drag;
}

.search-input::placeholder {
  color: var(--text-secondary);
}

.search-input:focus {
  border-color: var(--accent-color);
  background-color: var(--bg-primary);
}

.search-clear {
  position: absolute;
  right: 8px;
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: background-color 0.15s ease, color 0.15s ease;
  /* 按钮不参与拖动 */
  app-region: no-drag;
  -webkit-app-region: no-drag;
}

.search-clear:hover {
  background-color: var(--bg-surface);
  color: var(--text-primary);
}

.search-clear svg {
  width: 14px;
  height: 14px;
}

/* 右侧控制按钮 */
.title-right {
  display: flex;
  align-items: center;
  gap: 4px;
  /* 允许在此区域进行鼠标交互 */
  app-region: no-drag;
  -webkit-app-region: no-drag;
}

.window-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: background-color 0.15s ease, color 0.15s ease;
  /* 按钮不参与拖动 */
  app-region: no-drag;
  -webkit-app-region: no-drag;
}

.window-btn:hover {
  background-color: var(--bg-surface);
  color: var(--text-primary);
}

.window-btn.close:hover {
  background-color: #e11d48;
  color: #ffffff;
}

.window-btn svg {
  width: 14px;
  height: 14px;
}

.theme-btn svg {
  width: 16px;
  height: 16px;
}

.theme-icon {
  width: 16px;
  height: 16px;
}

.settings-btn svg {
  width: 16px;
  height: 16px;
}

.settings-icon {
  width: 16px;
  height: 16px;
}
</style>
