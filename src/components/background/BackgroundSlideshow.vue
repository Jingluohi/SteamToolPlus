<template>
  <div class="background-slideshow">
    <!-- 背景层（图片） -->
    <div class="background-layers">
      <transition
        v-for="(item, index) in displayItems"
        :key="index"
        :name="transitionName"
      >
        <div
          v-if="currentIndex === index && item.url"
          class="background-layer"
          :style="{ backgroundImage: `url('${item.url}')` }"
        ></div>
      </transition>
    </div>

    <!-- 效果层：模糊 + 暗化 -->
    <div
      class="background-effects"
      :style="effectsStyle"
    ></div>

    <!-- 内容插槽 -->
    <div class="background-content">
      <slot></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import type { BackgroundConfig, PageType } from '../../types/background.types'
import {
  getBackgroundConfig,
  getPageBackgroundFiles,
  getNextBackgroundIndex
} from '../../api/background.api'

// Props
const props = defineProps<{
  pageType: PageType
}>()

// 背景项类型
interface BackgroundItem {
  url: string
}

// 状态
const config = ref<BackgroundConfig | null>(null)
const displayItems = ref<BackgroundItem[]>([])
const currentIndex = ref(0)
const isLoading = ref(true)
let intervalId: number | null = null

// 计算属性：是否有背景
const hasBackground = computed(() => {
  return displayItems.value.length > 0
})

// 计算属性：过渡动画名称
const transitionName = computed(() => {
  const pageConfig = config.value?.pageConfigs.find(p => p.pageType === props.pageType)
  if (!pageConfig) return 'fade'
  return `bg-${pageConfig.transitionEffect}`
})

// 计算属性：效果层样式
const effectsStyle = computed(() => {
  const pageConfig = config.value?.pageConfigs.find(p => p.pageType === props.pageType)
  if (!pageConfig) return {}

  const styles: Record<string, string> = {}

  // 模糊效果
  if (pageConfig.blurStrength > 0) {
    styles.backdropFilter = `blur(${pageConfig.blurStrength}px)`
  }

  // 暗化效果
  if (pageConfig.darkness > 0) {
    styles.backgroundColor = `rgba(0, 0, 0, ${pageConfig.darkness})`
  }

  return styles
})

// 加载配置和背景
async function loadBackgroundConfig() {
  try {
    isLoading.value = true

    config.value = await getBackgroundConfig()

    if (config.value) {
      await loadBackgroundItems()

      // 启动轮播
      startSlideshow()
    }
  } catch (err) {
    // 加载失败时静默处理
  } finally {
    isLoading.value = false
  }
}

// 加载背景图片列表
async function loadBackgroundItems() {
  if (!config.value) return

  // 获取程序当前实际使用的主题（从DOM读取，与用户设置一致）
  const currentTheme = document.documentElement.getAttribute('data-theme') as 'light' | 'dark' | null
  const themeMode: 'light' | 'dark' = currentTheme === 'light' ? 'light' : 'dark'

  // 获取当前页面的背景文件（根据程序实际主题模式）
  const pageFiles = getPageBackgroundFiles(config.value, props.pageType, themeMode)

  // 构建背景项列表
  const items: BackgroundItem[] = []

  for (const file of pageFiles) {
    try {
      const { getBackgroundFileUrl } = await import('../../api/background.api')
      const url = await getBackgroundFileUrl(file.path)
      items.push({ url })
    } catch (err) {
      // 忽略加载失败的文件
    }
  }

  displayItems.value = items

  // 初始化当前索引
  const pageConfig = config.value.pageConfigs.find(p => p.pageType === props.pageType)
  if (pageConfig?.mode === 'random' && items.length > 0) {
    currentIndex.value = Math.floor(Math.random() * items.length)
  } else {
    currentIndex.value = 0
  }
}

// 启动轮播
function startSlideshow() {
  // 清除旧的定时器
  if (intervalId) {
    clearInterval(intervalId)
    intervalId = null
  }

  if (!config.value || displayItems.value.length <= 1) {
    return
  }

  const pageConfig = config.value.pageConfigs.find(p => p.pageType === props.pageType)
  if (!pageConfig || pageConfig.mode === 'single') {
    return
  }

  // 设置新的定时器
  intervalId = window.setInterval(() => {
    switchToNext()
  }, pageConfig.interval)
}

// 切换到下一张
function switchToNext() {
  if (!config.value || displayItems.value.length <= 1) {
    return
  }

  const pageConfig = config.value.pageConfigs.find(p => p.pageType === props.pageType)
  if (!pageConfig) return

  currentIndex.value = getNextBackgroundIndex(
    config.value,
    props.pageType,
    currentIndex.value
  )
}

// 生命周期
onMounted(() => {
  loadBackgroundConfig()

  // 监听程序主题变化（通过观察data-theme属性）
  const observer = new MutationObserver((mutations) => {
    mutations.forEach((mutation) => {
      if (mutation.attributeName === 'data-theme') {
        // 主题切换时重新加载背景图片
        loadBackgroundItems()
      }
    })
  })

  observer.observe(document.documentElement, {
    attributes: true,
    attributeFilter: ['data-theme']
  })
})

// 监听 pageType 变化，重新加载背景
watch(() => props.pageType, () => {
  // 清除旧的定时器
  if (intervalId) {
    clearInterval(intervalId)
    intervalId = null
  }
  // 重新加载背景配置
  loadBackgroundConfig()
})

onUnmounted(() => {
  if (intervalId) {
    clearInterval(intervalId)
  }
})

// 暴露方法供外部调用
defineExpose({
  refresh: loadBackgroundConfig,
  refreshItems: loadBackgroundItems,
  next: switchToNext
})
</script>

<style scoped>
.background-slideshow {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 0;
  overflow: hidden;
}

/* 背景层 */
.background-layers {
  position: absolute;
  inset: 0;
  z-index: 1;
}

/* 图片背景层 */
.background-layer {
  position: absolute;
  inset: 0;
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
}

/* 效果层 */
.background-effects {
  position: absolute;
  inset: 0;
  z-index: 2;
  pointer-events: none;
}

/* 内容层 */
.background-content {
  position: relative;
  z-index: 3;
  width: 100%;
  height: 100%;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

/* 过渡动画 - 淡入淡出 */
.bg-fade-enter-active,
.bg-fade-leave-active {
  transition: opacity 1s ease;
}

.bg-fade-enter-from,
.bg-fade-leave-to {
  opacity: 0;
}

/* 过渡动画 - 滑动 */
.bg-slide-enter-active,
.bg-slide-leave-active {
  transition: transform 0.8s ease, opacity 0.8s ease;
}

.bg-slide-enter-from {
  transform: translateX(100%);
  opacity: 0;
}

.bg-slide-leave-to {
  transform: translateX(-100%);
  opacity: 0;
}

/* 过渡动画 - 缩放 */
.bg-zoom-enter-active,
.bg-zoom-leave-active {
  transition: transform 1s ease, opacity 1s ease;
}

.bg-zoom-enter-from {
  transform: scale(1.1);
  opacity: 0;
}

.bg-zoom-leave-to {
  transform: scale(0.9);
  opacity: 0;
}

/* 无动画 */
.bg-none-enter-active,
.bg-none-leave-active {
  transition: none;
}
</style>
