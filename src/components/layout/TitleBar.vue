<template>
  <!--
    TitleBar.vue - 顶部标题栏组件
    完全对标Steam官方客户端的标题栏设计
    包含：Logo、导航菜单（游戏、设置、帮助、更多功能）、窗口控制按钮
  -->
  <header
    class="title-bar"
    data-tauri-drag-region
  >
    <!-- 左侧区域：Logo、程序名称和导航菜单 -->
    <div class="title-bar-left">
      <div class="logo">
        <img class="logo-icon" src="/icon.png" alt="Steam Tool Plus" />
      </div>
      <!-- 导航菜单 - 使用 NavDropdown 组件复用重复结构 -->
      <nav class="title-bar-nav">
        <NavDropdown
          v-for="menu in navMenus"
          :key="menu.title"
          :title="menu.title"
          :items="menu.items"
        />
      </nav>
    </div>

    <!-- 右侧区域：窗口控制按钮 -->
    <div class="title-bar-controls">
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
 * 使用 NavDropdown 组件减少重复代码
 */

import { ref } from 'vue'
import { useWindowStore } from '../../store/window.store'
import NavDropdown from './NavDropdown.vue'
import type { NavMenuItem } from './NavDropdown.vue'

// 获取窗口store
const windowStore = useWindowStore()

// 是否最大化
const isMaximized = ref(false)

/**
 * 导航菜单配置
 * 使用动态数据驱动，避免硬编码重复模板
 */
const navMenus: { title: string; items: NavMenuItem[] }[] = [
  {
    title: '游戏',
    items: [
      { name: '浏览', path: '/' },
      { name: '库', path: '/library' }
    ]
  },
  {
    title: '工具',
    items: [
      { name: '本体下载', path: '/download' },
      { name: '免Steam补丁', path: '/patch' },
      { name: '清单入库', path: '/manifest-import' }
    ]
  },
  {
    title: '更多',
    items: [
      { name: 'Lua转VDF', path: '/lua-to-vdf' },
      { name: 'VDF转Lua', path: '/vdf-to-lua' },
      { name: '游戏封面图下载', path: '/cover-downloader' }
    ]
  },
  {
    title: '设置',
    items: [
      { name: '全局设置', path: '/settings' },
      { name: '个性化', path: '/personalization' }
    ]
  },
  {
    title: '帮助',
    items: [
      { name: '关于软件', path: '/about' },
      { name: '使用说明', path: '/help' },
      { name: '疑难解答', path: '/troubleshooting' },
      { name: '检查更新', path: '/update-check' }
    ]
  }
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
</script>

<style scoped>
.title-bar {
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: rgba(var(--steam-bg-primary-rgb), 0.85);
  backdrop-filter: blur(20px) saturate(180%);
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
  width: 18px;
  height: 18px;
  object-fit: contain;
}

/* 导航菜单 */
.title-bar-nav {
  display: flex;
  align-items: center;
  gap: 4px;
  -webkit-app-region: no-drag;
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
</style>
