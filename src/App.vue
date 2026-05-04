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
  </div>
</template>

<script setup lang="ts">
/**
 * App.vue - 应用程序根组件
 * 实现无边框窗口布局、主题注入、路由、全局背景轮播
 */

import { computed, onMounted, ref, watch } from 'vue'
import { RouterView, useRoute } from 'vue-router'
import { useWindowStore } from './store/window.store'
import { useThemeStore } from './store/theme.store'
import { useConfigStore } from './store/config.store'
import TitleBar from './components/layout/TitleBar.vue'
import BackgroundSlideshow from './components/background/BackgroundSlideshow.vue'
import type { PageType } from './types/background.types'

// 获取store
const windowStore = useWindowStore()
const themeStore = useThemeStore()
const configStore = useConfigStore()
const route = useRoute()

// 背景组件引用
const backgroundRef = ref<InstanceType<typeof BackgroundSlideshow> | null>(null)

// 计算属性：是否全屏
const isFullscreen = computed(() => windowStore.isFullscreen)

// 计算属性：当前页面类型
const currentPageType = computed<PageType>(() => {
  const path = route.path
  
  // 根据路由路径映射到页面类型
  if (path === '/' || path.includes('/browse')) {
    return 'browse'
  } else if (path.includes('/library')) {
    return 'library'
  } else if (path.includes('/download')) {
    return 'download'
  } else if (path.includes('/patch')) {
    return 'patch'
  } else if (path.includes('/settings')) {
    return 'settings'
  } else if (path.includes('/about') || path.includes('/update-check')) {
    return 'about'
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

  // 初始化窗口状态
  await windowStore.initWindow()
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
  background: transparent;
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
</style>
