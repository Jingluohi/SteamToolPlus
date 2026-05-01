<template>
  <!-- 应用程序根组件 -->
  <!-- 实现无边框窗口布局、主题注入、路由视图 -->
  <div 
    id="app-root" 
    class="app-root"
    :class="{ 'fullscreen': isFullscreen }"
  >
    <!-- 标题栏 - 全屏模式下也保留，但可以通过CSS调整样式 -->
    <TitleBar 
      class="app-title-bar"
      @toggle-fullscreen="toggleFullscreen"
    />
    
    <!-- 主内容区域 -->
    <div class="main-container">
      <!-- 侧边栏 - 根据规则不使用侧边栏，但保留结构以备扩展 -->
      <!-- 主内容区 -->
      <main class="main-content">
        <RouterView />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * App.vue - 应用程序根组件
 * 实现无边框窗口布局、主题注入、路由
 */

import { computed, onMounted } from 'vue'
import { RouterView } from 'vue-router'
import { useWindowStore } from './store/window.store'
import { useThemeStore } from './store/theme.store'
import { useConfigStore } from './store/config.store'
import TitleBar from './components/layout/TitleBar.vue'

// 获取store
const windowStore = useWindowStore()
const themeStore = useThemeStore()
const configStore = useConfigStore()

// 计算属性：是否全屏
const isFullscreen = computed(() => windowStore.isFullscreen)

// 切换全屏
const toggleFullscreen = () => {
  windowStore.toggleFullscreen()
}

// 组件挂载时初始化
onMounted(async () => {
  // 先加载配置
  await configStore.loadConfig()

  // 如果配置加载成功，从配置加载主题设置
  if (configStore.config) {
    themeStore.loadFromConfig(configStore.config.theme)
  } else {
    // 如果配置加载失败，使用默认主题初始化
    themeStore.initTheme()
  }

  // 初始化窗口状态
  windowStore.initWindow()
})
</script>

<style scoped>
.app-root {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: var(--steam-bg-primary);
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
  background: var(--steam-bg-secondary);
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
</style>
