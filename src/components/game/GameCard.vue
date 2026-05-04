<template>
  <!-- 
    GameCard.vue - 游戏卡片组件
    仿照旧版实现，显示游戏封面、名称、ID等信息
    支持懒加载和内存管理
  -->
  <div
    ref="cardRef"
    class="game-card gpu-accelerated"
    :class="{ 'pressed': isPressed }"
    @mousedown="isPressed = true"
    @mouseup="isPressed = false"
    @mouseleave="isPressed = false"
    @click="handleClick"
  >
    <!-- 卡片内容容器 - 绝对定位填满卡片 -->
    <div class="card-content">
      <!-- 游戏封面图 -->
      <div class="card-image-wrapper">
        <img
          v-if="imageLoaded && coverUrl"
          :src="coverUrl"
          :alt="gameName"
          class="card-image"
          loading="lazy"
          @error="handleImageError"
          @load="onImageLoad"
        />
        <div v-else class="card-placeholder" :style="placeholderStyle">
          <span class="placeholder-text">{{ gameId.slice(0, 4) }}</span>
        </div>

        <!-- 底部渐变遮罩 -->
        <div class="card-overlay"></div>
        
        <!-- 可下载标记 -->
        <div v-if="downloadable" class="downloadable-badge">可下载</div>
      </div>

      <!-- 游戏信息 -->
      <div class="card-info">
        <h3 class="game-name">{{ gameName }}</h3>
        <span class="game-en-name">{{ gameEnName }}</span>
        <span class="game-id">ID: {{ gameId }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * GameCard.vue - 游戏卡片组件
 * 仿照旧版实现，展示单个游戏的信息
 */

import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import type { GameConfigData } from '../../types'
import { getCachedCoverImage } from '../../services/imageCache.service'

/**
 * 组件属性定义
 */
interface Props {
  /** 游戏数据 */
  game: GameConfigData
}

const props = defineProps<Props>()

/**
 * 组件事件定义
 */
const emit = defineEmits<{
  /** 点击卡片 */
  (e: 'click', game: GameConfigData): void
}>()

// 组件引用
const cardRef = ref<HTMLElement | null>(null)

// 本地状态
const coverUrl = ref('')
const imageLoaded = ref(false)
const isPressed = ref(false)
const isVisible = ref(false)

// Intersection Observer 实例
let observer: IntersectionObserver | null = null

// 释放图片的定时器
let releaseTimer: ReturnType<typeof setTimeout> | null = null

// 释放延迟时间（毫秒）- 滚动出视口2秒后释放
const RELEASE_DELAY = 2000

// 计算属性：游戏名称
const gameName = computed(() => props.game.chinese_name || props.game.game_name)

// 计算属性：游戏英文名
const gameEnName = computed(() => props.game.game_name)

// 计算属性：游戏ID
const gameId = computed(() => props.game.game_id)

// 计算属性：是否可下载
const downloadable = computed(() => props.game.downloadable)



// 生成占位符颜色（基于游戏ID的哈希值）
const placeholderStyle = computed(() => {
  const hash = props.game.game_id.split('').reduce((acc, char) => {
    return acc + char.charCodeAt(0)
  }, 0)

  const hue = hash % 360
  return {
    backgroundColor: `hsl(${hue}, 60%, 40%)`
  }
})



// 加载封面图片
// 使用全局缓存服务获取图片 URL，避免与详情页资源竞争
const loadCover = async () => {
  // 如果已经加载过且 URL 有效，直接返回
  if (coverUrl.value && imageLoaded.value) return

  try {
    // 使用缓存服务获取图片 URL
    // 如果图片已被其他组件加载过，会直接返回缓存的 URL
    const cachedUrl = await getCachedCoverImage(props.game.game_id)
    if (cachedUrl) {
      coverUrl.value = cachedUrl
      imageLoaded.value = true
    }
  } catch (error) {
    // 加载封面失败时静默处理
  }
}

// 释放图片内存
const releaseCover = () => {
  // 先标记为未加载，让 img 元素从 DOM 中移除
  imageLoaded.value = false

  // 使用 nextTick 确保 DOM 更新后再清除 URL
  nextTick(() => {
    // 清除 URL 引用
    if (coverUrl.value) {
      coverUrl.value = ''
    }
    isVisible.value = false
  })
}

// 处理图片加载错误
const handleImageError = () => {
  imageLoaded.value = false
}

// 处理图片加载完成
const onImageLoad = () => {
  imageLoaded.value = true
}

// 处理点击事件
const handleClick = () => {
  emit('click', props.game)
}

// 设置 Intersection Observer 监听可见性
const setupIntersectionObserver = () => {
  if (!cardRef.value) return

  observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          // 卡片进入视口
          // 清除释放定时器（如果存在）
          if (releaseTimer) {
            clearTimeout(releaseTimer)
            releaseTimer = null
          }
          
          // 如果还没加载过，加载封面
          if (!isVisible.value) {
            isVisible.value = true
            loadCover()
          }
        } else {
          // 卡片滚动出视口
          // 设置定时器，延迟释放图片内存
          if (isVisible.value && !releaseTimer) {
            releaseTimer = setTimeout(() => {
              releaseCover()
            }, RELEASE_DELAY)
          }
        }
      })
    },
    {
      root: null, // 使用视口
      rootMargin: '100px', // 提前100px开始加载
      threshold: 0.1 // 至少10%可见才触发
    }
  )

  observer.observe(cardRef.value)
}

// 组件挂载
onMounted(() => {
  // 延迟设置 observer，优先渲染占位符
  setTimeout(() => {
    setupIntersectionObserver()
  }, 50)
})

// 组件卸载
onUnmounted(() => {
  // 清除定时器
  if (releaseTimer) {
    clearTimeout(releaseTimer)
    releaseTimer = null
  }
  
  // 清除 observer
  if (observer && cardRef.value) {
    observer.unobserve(cardRef.value)
    observer = null
  }
  
  // 释放图片内存
  releaseCover()
})
</script>

<style scoped>
/* 游戏卡片 - 填满Grid单元格，保持约2.13:1比例 */
.game-card {
  position: relative;
  width: 100%;
  /* 使用 padding-bottom 技巧保持比例，确保Grid能正确计算行高 */
  padding-bottom: 46.8%; /* 约 2.13:1 比例 */
  border-radius: 10px;
  border: none;
  overflow: hidden;
  cursor: pointer;
  background-color: transparent;
  transition: transform 0.15s ease-out, box-shadow 0.15s ease;
  will-change: transform;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

/* 悬浮效果 - 使用 transform 和阴影 */
.game-card:hover {
  transform: scale(1.02);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.25);
}

/* 按压效果 */
.game-card.pressed {
  transform: scale(0.98);
}

/* 卡片内容容器 - 绝对定位填满卡片 */
.card-content {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}

/* 图片容器 - 填满整个卡片 */
.card-image-wrapper {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
}

/* 图片保持原比例填充 */
.card-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  object-position: center;
  display: block;
}

/* 占位符 */
.card-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.placeholder-text {
  font-size: 24px;
  font-weight: 700;
  color: rgba(255, 255, 255, 0.5);
  text-transform: uppercase;
}

/* 底部渐变遮罩 - 用于文字显示 */
.card-overlay {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 50%;
  background: linear-gradient(
    to top,
    rgba(0, 0, 0, 0.7) 0%,
    rgba(0, 0, 0, 0.4) 50%,
    transparent 100%
  );
  pointer-events: none;
}

/* 可下载标记 */
.downloadable-badge {
  position: absolute;
  top: 8px;
  right: 8px;
  padding: 4px 8px;
  background-color: var(--steam-accent-blue);
  color: white;
  font-size: 12px;
  font-weight: 600;
  border-radius: 4px;
  z-index: 2;
}

/* 游戏信息 - 固定在底部 */
.card-info {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 8px 12px;
  z-index: 1;
}

.game-name {
  font-size: 15px;
  font-weight: 600;
  color: #ffffff;
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  line-height: 1.3;
}

.game-en-name {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.9);
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  display: block;
  line-height: 1.3;
}

.game-id {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.7);
  font-family: 'Courier New', monospace;
  line-height: 1.3;
}

/* GPU加速 */
.gpu-accelerated {
  transform: translateZ(0);
  backface-visibility: hidden;
}
</style>
