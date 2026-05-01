<template>
  <!--
    TitleBar.vue - 顶部标题栏组件
    完全对标Steam官方客户端的标题栏设计
    包含：Logo、导航菜单（游戏、设置、帮助）、窗口控制按钮
  -->
  <header
    class="title-bar"
    data-tauri-drag-region
  >
    <!-- 左侧区域：Logo、程序名称和导航菜单 -->
    <div class="title-bar-left">
      <div class="logo">
        <svg class="logo-icon" viewBox="0 0 24 24" fill="currentColor">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
        </svg>
        <span class="logo-text">Steam Tool Plus v1.11</span>
      </div>
      <!-- 导航菜单 -->
      <nav class="title-bar-nav">
      <!-- 游戏菜单 -->
      <div
        class="nav-menu"
        @mouseenter="showGameMenu = true"
        @mouseleave="showGameMenu = false"
      >
        <button class="nav-menu-btn">
          <span>游戏</span>
          <svg class="dropdown-icon" viewBox="0 0 24 24" fill="currentColor">
            <path d="M7 10l5 5 5-5z"/>
          </svg>
        </button>
        <!-- 游戏下拉菜单 -->
        <Transition name="dropdown">
          <div v-show="showGameMenu" class="dropdown-menu">
            <RouterLink
              v-for="item in gameMenuItems"
              :key="item.path"
              :to="item.path"
              class="dropdown-item"
              @click="showGameMenu = false"
            >
              {{ item.name }}
            </RouterLink>
          </div>
        </Transition>
      </div>

      <!-- 设置菜单 -->
      <div
        class="nav-menu"
        @mouseenter="showSettingsMenu = true"
        @mouseleave="showSettingsMenu = false"
      >
        <button class="nav-menu-btn">
          <span>设置</span>
          <svg class="dropdown-icon" viewBox="0 0 24 24" fill="currentColor">
            <path d="M7 10l5 5 5-5z"/>
          </svg>
        </button>
        <!-- 设置下拉菜单 -->
        <Transition name="dropdown">
          <div v-show="showSettingsMenu" class="dropdown-menu">
            <RouterLink
              v-for="item in settingsMenuItems"
              :key="item.path"
              :to="item.path"
              class="dropdown-item"
              @click="showSettingsMenu = false"
            >
              {{ item.name }}
            </RouterLink>
          </div>
        </Transition>
      </div>

      <!-- 帮助菜单 -->
      <div
        class="nav-menu"
        @mouseenter="showHelpMenu = true"
        @mouseleave="showHelpMenu = false"
      >
        <button class="nav-menu-btn">
          <span>帮助</span>
          <svg class="dropdown-icon" viewBox="0 0 24 24" fill="currentColor">
            <path d="M7 10l5 5 5-5z"/>
          </svg>
        </button>
        <!-- 帮助下拉菜单 -->
        <Transition name="dropdown">
          <div v-show="showHelpMenu" class="dropdown-menu">
            <RouterLink
              v-for="item in helpMenuItems"
              :key="item.name"
              :to="item.path"
              class="dropdown-item"
              @click="handleMenuClick(item.path)"
            >
              {{ item.name }}
            </RouterLink>
          </div>
        </Transition>
      </div>
      </nav>
    </div>

    <!-- 右侧区域：窗口控制按钮 -->
    <div class="title-bar-controls">
      <!-- 全屏按钮 -->
      <button
        class="control-btn"
        title="全屏"
        @click="handleFullscreen"
      >
        <svg viewBox="0 0 24 24" fill="currentColor">
          <path d="M7 14H5v5h5v-2H7v-3zm-2-4h2V7h3V5H5v5zm12 7h-3v2h5v-5h-2v3zM14 5v2h3v3h2V5h-5z"/>
        </svg>
      </button>

      <!-- 最小化按钮 -->
      <button
        class="control-btn"
        title="最小化"
        @click="handleMinimize"
      >
        <svg viewBox="0 0 24 24" fill="currentColor">
          <path d="M19 13H5v-2h14v2z"/>
        </svg>
      </button>

      <!-- 最大化/还原按钮 -->
      <button
        class="control-btn"
        :title="isMaximized ? '还原' : '最大化'"
        @click="handleMaximize"
      >
        <svg v-if="!isMaximized" viewBox="0 0 24 24" fill="currentColor">
          <path d="M7 14H5v5h5v-2H7v-3zm-2-4h2V7h3V5H5v5zm12 7h-3v2h5v-5h-2v3zM14 5v2h3v3h2V5h-5z"/>
        </svg>
        <svg v-else viewBox="0 0 24 24" fill="currentColor">
          <path d="M5 16h3v3h2v-5H5v2zm3-8H5v2h5V5H8v3zm6 11h2v-3h3v-2h-5v5zm2-11V5h-2v5h5V8h-3z"/>
        </svg>
      </button>

      <!-- 关闭按钮 -->
      <button
        class="control-btn close-btn"
        title="关闭"
        @click="handleClose"
      >
        <svg viewBox="0 0 24 24" fill="currentColor">
          <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
        </svg>
      </button>
    </div>

  </header>
</template>

<script setup lang="ts">
/**
 * TitleBar.vue - 顶部标题栏组件
 * 实现Steam风格的无边框窗口标题栏
 */

import { ref } from 'vue'
import { useWindowStore } from '../../store/window.store'
import { openHelpWindow } from '../../api/window.api'

// 获取窗口store
const windowStore = useWindowStore()

// 菜单显示状态
const showGameMenu = ref(false)
const showSettingsMenu = ref(false)
const showHelpMenu = ref(false)

// 是否最大化
const isMaximized = ref(false)

// 游戏菜单项
const gameMenuItems = [
  { name: '浏览', path: '/' },
  { name: '本体下载', path: '/download' },
  { name: '免Steam补丁', path: '/patch' },
  { name: '库', path: '/library' }
]

// 设置菜单项
const settingsMenuItems = [
  { name: '全局设置', path: '/settings' },
  { name: '管理扩展', path: '/extensions' }
]

// 帮助菜单项
const helpMenuItems = [
  { name: '关于软件', path: '/about' },
  { name: '使用说明', path: '/help' },
  { name: '检查更新', path: '/update-check' }
]

// 处理最小化
const handleMinimize = () => {
  windowStore.minimize()
}

// 处理最大化/还原
const handleMaximize = () => {
  windowStore.maximize()
  isMaximized.value = !isMaximized.value
}

// 处理关闭
const handleClose = () => {
  windowStore.close()
}

// 处理全屏
const handleFullscreen = () => {
  windowStore.toggleFullscreen()
}

// 打开帮助窗口（独立窗口）
const handleOpenHelpWindow = async () => {
  showHelpMenu.value = false
  try {
    await openHelpWindow()
  } catch (error) {
    console.error('打开帮助窗口失败:', error)
  }
}

// 处理菜单点击
const handleMenuClick = (path: string) => {
  showHelpMenu.value = false
}
</script>

<style scoped>
.title-bar {
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: var(--steam-bg-primary);
  border-bottom: 1px solid var(--steam-border);
  padding: 0 16px;
  user-select: none;
  -webkit-app-region: drag;
  position: relative;
  z-index: 100;
}

.title-bar-left {
  display: flex;
  align-items: center;
  gap: 10px;
}

.logo {
  display: flex;
  align-items: center;
  gap: 8px;
}

.logo-icon {
  width: 16px;
  height: 16px;
  color: var(--steam-accent-blue);
}

.logo-text {
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

/* 导航菜单 */
.title-bar-nav {
  display: flex;
  align-items: center;
  gap: 4px;
  -webkit-app-region: no-drag;
}

.nav-menu {
  position: relative;
}

.nav-menu-btn {
  display: flex;
  align-items: center;
  gap: 2px;
  padding: 0 6px;
  height: 32px;
  font-size: 16px;
  font-weight: 400;
  color: var(--steam-text-primary);
  background: transparent;
  border-radius: 4px;
  transition: background var(--transition-fast);
}

.nav-menu-btn:hover {
  background: var(--steam-accent-hover);
}

.dropdown-icon {
  width: 16px;
  height: 16px;
  opacity: 0.7;
}

/* 下拉菜单 */
.dropdown-menu {
  position: absolute;
  top: 100%;
  left: 0;
  width: 120px;
  min-width: 120px;
  background: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  box-shadow: var(--shadow-steam);
  padding: 8px 0;
  z-index: 1000;
  margin-top: 4px;
}

.dropdown-item {
  display: block;
  width: 100%;
  padding: 10px 16px;
  font-size: 14px;
  color: var(--steam-text-primary);
  background: transparent;
  border: none;
  text-align: left;
  cursor: pointer;
  transition: background var(--transition-fast);
  text-decoration: none;
}

.dropdown-item:hover {
  background: var(--steam-accent-hover);
}

/* 窗口控制按钮 */
.title-bar-controls {
  display: flex;
  align-items: center;
  gap: 2px;
  -webkit-app-region: no-drag;
}

.control-btn {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--steam-text-secondary);
  border-radius: 4px;
  transition: all var(--transition-fast);
}

.control-btn:hover {
  background: var(--steam-accent-hover);
  color: var(--steam-text-primary);
}

.control-btn svg {
  width: 16px;
  height: 16px;
}

.control-btn.close-btn:hover {
  background: #e81123;
  color: white;
}

/* 下拉动画 */
.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.2s ease-out;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}
</style>
