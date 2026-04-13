<template>
  <aside 
    class="sidebar" 
    :style="{ width: sidebarWidth + 'px' }"
    :class="{ collapsed: isCollapsed }"
  >
    <!-- 拖动调整宽度的手柄 -->
    <div 
      class="resize-handle"
      @mousedown="startResize"
      title="拖动调整宽度"
    ></div>

    <!-- 应用 Logo -->
    <div class="sidebar-header no-select">
      <div class="logo">
        <img src="/pic/icon.png" alt="Steam" class="logo-icon" />
        <span v-show="!isCollapsed" class="logo-text">SteamToolPlus</span>
      </div>
    </div>

    <!-- 导航菜单 -->
    <nav class="sidebar-nav">
      <!-- 游戏分类（可折叠） -->
      <div class="nav-section">
        <ul class="nav-list">
          <!-- 主菜单项：全部游戏（带折叠按钮） -->
          <li 
            class="nav-item main-menu-item"
            :class="{ active: isActive(mainMenuItem.path) }"
          >
            <div class="nav-link-with-toggle">
              <!-- 折叠按钮 -->
              <button 
                v-show="!isCollapsed"
                class="toggle-btn"
                :class="{ expanded: isSubmenuExpanded }"
                @click.prevent="toggleSubmenu"
                title="展开/折叠子菜单"
              >
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M9 18l6-6-6-6"/>
                </svg>
              </button>
              <RouterLink 
                :to="mainMenuItem.path" 
                class="nav-link main-link"
                :class="{ 'icon-only': isCollapsed }"
                @click="handleNavClick(mainMenuItem)"
              >
                <component :is="mainMenuItem.icon" class="nav-icon" />
                <span v-show="!isCollapsed" class="nav-text">{{ mainMenuItem.title }}</span>
              </RouterLink>
            </div>
          </li>
          
          <!-- 子菜单项：可折叠 -->
          <li 
            v-for="item in submenuItems" 
            :key="item.path"
            class="nav-item submenu-item"
            :class="{ 
              active: isActive(item.path), 
              collapsed: !isSubmenuExpanded || isCollapsed
            }"
          >
            <RouterLink 
              :to="item.path" 
              class="nav-link"
              :class="{ 'icon-only': isCollapsed }"
              @click="handleNavClick(item)"
            >
              <component :is="item.icon" class="nav-icon" />
              <span v-show="!isCollapsed" class="nav-text">{{ item.title }}</span>
            </RouterLink>
          </li>
        </ul>
      </div>

      <!-- 独立的父类菜单项（在全部游戏下面） -->
      <div class="nav-section standalone-section">
        <ul class="nav-list">
          <li 
            v-for="item in standaloneMenuItems" 
            :key="item.path"
            class="nav-item"
            :class="{ active: isActive(item.path), collapsed: isCollapsed }"
          >
            <RouterLink 
              :to="item.path" 
              class="nav-link"
              :class="{ 'icon-only': isCollapsed }"
              @click="handleNavClick(item)"
            >
              <component :is="item.icon" class="nav-icon" />
              <span v-show="!isCollapsed" class="nav-text">{{ item.title }}</span>
            </RouterLink>
          </li>
        </ul>
      </div>
    </nav>

    <!-- 底部信息 -->
    <div class="sidebar-footer no-select">
      <!-- B站链接 -->
      <a 
        href="#"
        class="footer-link"
        :class="{ 'icon-only': isCollapsed }"
        title="B 站主页（鲸落_hi）"
        @click.prevent="openExternalLink('https://space.bilibili.com/3546733333513943')"
      >
        <svg class="footer-icon" viewBox="0 0 24 24" fill="currentColor">
          <path d="M17.813 4.653h.854c1.51.054 2.769.578 3.773 1.574 1.004.995 1.524 2.249 1.56 3.76v7.36c-.036 1.51-.556 2.769-1.56 3.773s-2.262 1.524-3.773 1.56H5.333c-1.51-.036-2.769-.556-3.773-1.56S.036 18.858 0 17.347v-7.36c.036-1.511.556-2.765 1.56-3.76 1.004-.996 2.262-1.52 3.773-1.574h.774l-1.174-1.12a1.234 1.234 0 0 1-.373-.906c0-.356.124-.658.373-.907l.027-.027c.267-.249.573-.373.92-.373.347 0 .653.124.92.373L9.653 4.44c.071.071.134.142.187.213h4.267a.836.836 0 0 1 .16-.213l2.853-2.747c.267-.249.573-.373.92-.373.347 0 .662.151.929.4.267.249.391.551.391.907 0 .355-.124.657-.373.906zM5.333 7.24c-.746.018-1.373.276-1.88.773-.506.498-.769 1.13-.786 1.894v7.52c.017.764.28 1.395.786 1.893.507.498 1.134.756 1.88.773h13.334c.746-.017 1.373-.275 1.88-.773.506-.498.769-1.129.786-1.893v-7.52c-.017-.765-.28-1.396-.786-1.894-.507-.497-1.134-.755-1.88-.773zM8 11.107c.373 0 .684.124.933.373.25.249.383.569.4.96v1.173c-.017.391-.15.711-.4.96-.249.25-.56.374-.933.374s-.684-.125-.933-.374c-.25-.249-.383-.569-.4-.96V12.44c0-.373.129-.689.386-.947.258-.257.574-.386.947-.386zm8 0c.373 0 .684.124.933.373.25.249.383.569.4.96v1.173c-.017.391-.15.711-.4.96-.249.25-.56.374-.933.374s-.684-.125-.933-.374c-.25-.249-.383-.569-.4-.96V12.44c.017-.391.15-.711.4-.96.249-.249.56-.373.933-.373z"/>
        </svg>
        <span v-show="!isCollapsed">B 站主页（鲸落_hi）</span>
      </a>
      
      <!-- 版本和更新链接 -->
      <div class="footer-info" :class="{ 'collapsed': isCollapsed }">
        <div class="version-row">
          <span v-show="!isCollapsed" class="version">v1.05</span>
          <a 
            href="#"
            class="update-link"
            :class="{ 'icon-only': isCollapsed }"
            title="查看最新版"
            @click.prevent="openExternalLink('https://pan.baidu.com/s/1XbcZOLQcn4500z-SL1RDug?pwd=v1xm')"
          >
            <svg class="update-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
              <polyline points="7 10 12 15 17 10"/>
              <line x1="12" y1="15" x2="12" y2="3"/>
            </svg>
            <span v-show="!isCollapsed">查看最新版 v1xm</span>
          </a>
        </div>
        <span v-show="sidebarWidth > 60" class="copyright">© 2026 Steam Tool Plus</span>
      </div>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRoute } from 'vue-router'
import { useGamesStore } from '../../stores/games'
import { invoke } from '@tauri-apps/api/core'

// 导入图标组件
import IconGrid from '../icons/IconGrid.vue'
import IconZap from '../icons/IconZap.vue'
import IconUsers from '../icons/IconUsers.vue'
import IconGlobe from '../icons/IconGlobe.vue'
import IconShield from '../icons/IconShield.vue'
import IconDownload from '../icons/IconDownload.vue'
import IconInjection from '../icons/IconInjection.vue'

const route = useRoute()
const gamesStore = useGamesStore()

/**
 * 在系统默认浏览器中打开链接
 * @param url 要打开的URL
 */
const openExternalLink = async (url: string) => {
  try {
    await invoke('open_external_link', { url })
  } catch (error) {
    console.error('打开链接失败:', error)
    // 如果Tauri命令失败，尝试使用普通方式打开
    window.open(url, '_blank')
  }
}

// 侧边栏宽度常量
const MIN_WIDTH = 60
const MAX_WIDTH = 250
const DEFAULT_WIDTH = 200
const sidebarWidth = ref(DEFAULT_WIDTH)

// 计算属性：是否折叠状态
const isCollapsed = computed(() => sidebarWidth.value <= MIN_WIDTH)

// 是否正在拖动
const isResizing = ref(false)

// 开始拖动调整宽度
const startResize = (e: MouseEvent) => {
  isResizing.value = true
  document.body.style.cursor = 'col-resize'
  document.body.style.userSelect = 'none'
  
  const startX = e.clientX
  const startWidth = sidebarWidth.value
  
  const handleMouseMove = (e: MouseEvent) => {
    if (!isResizing.value) return
    
    const deltaX = e.clientX - startX
    let newWidth = startWidth + deltaX
    
    // 限制最小和最大宽度
    newWidth = Math.max(MIN_WIDTH, Math.min(MAX_WIDTH, newWidth))
    
    sidebarWidth.value = newWidth
  }
  
  const handleMouseUp = () => {
    isResizing.value = false
    document.body.style.cursor = ''
    document.body.style.userSelect = ''
    
    document.removeEventListener('mousemove', handleMouseMove)
    document.removeEventListener('mouseup', handleMouseUp)
  }
  
  document.addEventListener('mousemove', handleMouseMove)
  document.addEventListener('mouseup', handleMouseUp)
}

// 子菜单折叠状态
const isSubmenuExpanded = ref(true)

// 切换子菜单折叠状态
const toggleSubmenu = () => {
  isSubmenuExpanded.value = !isSubmenuExpanded.value
}

// 独立的父类菜单项（不可折叠）
const standaloneMenuItems = [
  { path: '/game-download', title: '游戏本体下载', icon: IconDownload, category: 'game-download' },
  { path: '/steam-patch-inject', title: '免 Steam 补丁注入', icon: IconInjection, category: 'steam-patch-inject' },
]

// 主菜单项
const mainMenuItem = { path: '/', title: '全部游戏', icon: IconGrid, category: 'all' }

// 子菜单项
const submenuItems = [
  { path: '/no-steam', title: '免 Steam 启动', icon: IconZap, category: 'no-steam' },
  { path: '/lan-multiplayer', title: '局域网联机', icon: IconUsers, category: 'lan-multiplayer' },
  { path: '/steam-multiplayer', title: 'Steam 联机', icon: IconGlobe, category: 'steam-multiplayer' },
  { path: '/denuvo-vm', title: 'D 加密虚拟机', icon: IconShield, category: 'denuvo-vm' },
]

// 检查当前路由是否激活
const isActive = (path: string) => {
  return route.path === path
}

// 处理导航点击
const handleNavClick = (item: typeof mainMenuItem) => {
  gamesStore.setCategory(item.category)
}
</script>

<style scoped>
.sidebar {
  height: 100%;
  display: flex;
  flex-direction: column;
  background-color: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
  transition: width 0.1s ease;
  position: relative;
  min-width: 60px;
  max-width: 250px;
}

/* 拖动调整宽度的手柄 */
.resize-handle {
  position: absolute;
  right: 0;
  top: 0;
  bottom: 0;
  width: 4px;
  cursor: col-resize;
  background: transparent;
  transition: background-color 0.15s ease;
  z-index: 10;
}

.resize-handle:hover {
  background-color: var(--accent-color);
}

/* 折叠状态 */
.sidebar.collapsed {
  overflow: hidden;
}

.sidebar.collapsed .nav-link {
  justify-content: center;
  padding: 10px 0;
}

.sidebar.collapsed .nav-icon {
  margin: 0;
}

/* 侧边栏头部 - Logo */
.sidebar-header {
  padding: 20px 16px;
  border-bottom: 1px solid var(--border-color);
}

.logo {
  display: flex;
  align-items: center;
  gap: 10px;
}

.logo-icon {
  width: 28px;
  height: 28px;
  border-radius: 6px;
  object-fit: contain;
  flex-shrink: 0;
}

.logo-text {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* 导航菜单 */
.sidebar-nav {
  flex: 1;
  padding: 16px 12px;
  overflow-y: auto;
  overflow-x: hidden;
}

.nav-section {
  margin-bottom: 16px;
}

/* 独立的父类菜单项区域 */
.standalone-section {
  padding-top: 16px;
  border-top: 1px solid var(--border-color);
}

.nav-list {
  list-style: none;
}

.nav-item {
  position: relative;
  margin-bottom: 2px;
}

/* 独立的父类菜单项在折叠时的样式 */
.nav-item.collapsed {
  max-height: 40px;
  opacity: 1;
}

/* 主菜单项样式 */
.main-menu-item {
  margin-bottom: 4px;
}

.nav-link-with-toggle {
  display: flex;
  align-items: center;
  gap: 4px;
}

/* 折叠按钮 */
.toggle-btn {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: background-color 0.15s ease, transform 0.2s ease;
  flex-shrink: 0;
}

.toggle-btn:hover {
  background-color: var(--bg-surface);
  color: var(--text-primary);
}

.toggle-btn svg {
  width: 14px;
  height: 14px;
  transition: transform 0.2s ease;
}

.toggle-btn.expanded svg {
  transform: rotate(90deg);
}

.main-link {
  flex: 1;
}

.nav-link.icon-only {
  justify-content: center;
  padding: 10px 0;
}

.nav-link.icon-only .nav-icon {
  margin: 0;
}

/* 子菜单项折叠动画 */
.submenu-item {
  max-height: 40px;
  opacity: 1;
  overflow: hidden;
  transition: max-height 0.2s ease, opacity 0.2s ease, margin 0.2s ease;
}

.submenu-item.collapsed {
  max-height: 0;
  opacity: 0;
  margin-bottom: 0;
}

.nav-item::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 2px;
  height: 0;
  background-color: var(--accent-color);
  border-radius: 0 2px 2px 0;
  transition: height 0.15s ease;
}

.nav-item.active::before {
  height: 60%;
}

.nav-link {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border-radius: 6px;
  text-decoration: none;
  color: var(--text-secondary);
  transition: background-color 0.15s ease, color 0.15s ease;
  white-space: nowrap;
  overflow: hidden;
}

.nav-link:hover {
  background-color: var(--bg-surface);
  color: var(--text-primary);
}

.nav-item.active .nav-link {
  background-color: var(--bg-surface);
  color: var(--accent-color);
}

.nav-icon {
  width: 18px;
  height: 18px;
  flex-shrink: 0;
}

.nav-text {
  font-size: 13px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* 底部信息 */
.sidebar-footer {
  padding: 16px;
  border-top: 1px solid var(--border-color);
}

.footer-link {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-radius: 6px;
  text-decoration: none;
  color: var(--text-secondary);
  font-size: 12px;
  transition: background-color 0.15s ease, color 0.15s ease;
  margin-bottom: 12px;
  white-space: nowrap;
  overflow: hidden;
}

.footer-link:hover {
  background-color: var(--bg-surface);
  color: var(--accent-color);
}

/* 折叠状态下的底部链接样式 */
.footer-link.icon-only {
  justify-content: center;
  padding: 8px 0;
  margin-bottom: 8px;
}

.footer-link.icon-only .footer-icon {
  margin: 0;
  width: 18px;
  height: 18px;
}

.footer-icon {
  width: 16px;
  height: 16px;
  flex-shrink: 0;
}

.footer-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding-left: 12px;
}

/* 折叠状态下的footer-info */
.footer-info.collapsed {
  padding-left: 0;
  align-items: center;
}

.version-row {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.version {
  font-size: 11px;
  font-weight: 600;
  color: var(--accent-color);
}

.update-link {
  font-size: 10px;
  color: var(--text-secondary);
  text-decoration: none;
  transition: color 0.15s ease;
  white-space: nowrap;
  display: flex;
  align-items: center;
  gap: 4px;
}

.update-link:hover {
  color: var(--accent-color);
}

/* 折叠状态下的更新链接样式 */
.update-link.icon-only {
  justify-content: center;
  padding: 6px 0;
}

.update-link.icon-only .update-icon {
  width: 18px;
  height: 18px;
  margin: 0;
}

.update-icon {
  width: 12px;
  height: 12px;
}

.copyright {
  font-size: 10px;
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
