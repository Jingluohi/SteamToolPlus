<template>
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
          :alt="game.chinese_name"
          class="card-image"
          loading="lazy"
          @error="handleImageError"
          @load="onImageLoad"
        />
        <div v-else class="card-placeholder" :style="placeholderStyle"></div>

        <!-- 底部渐变遮罩 -->
        <div class="card-overlay"></div>
      </div>

      <!-- 游戏信息 -->
      <div class="card-info">
        <h3 class="game-name">{{ game.chinese_name }}</h3>
        <span class="game-en-name">{{ game.game_name }}</span>
        <span class="game-id">ID: {{ game.game_id }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useGamesStore, type Game } from '../../stores/games'

// 组件属性
interface Props {
  game: Game
}

const props = defineProps<Props>()
const gamesStore = useGamesStore()

// 组件事件
const emit = defineEmits<{
  click: [game: Game]
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

// 图片释放延迟时间（毫秒）- 滚动出视口5秒后释放
const RELEASE_DELAY = 5000

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
const loadCover = async () => {
  if (coverUrl.value) return // 已经加载过

  try {
    // 优先使用 store 中已加载的封面
    if (props.game.coverUrl) {
      coverUrl.value = props.game.coverUrl
      imageLoaded.value = true
      return
    }

    // 否则按需加载
    const url = await gamesStore.loadGameCoverOnDemand(props.game.game_id)
    if (url) {
      coverUrl.value = url
      imageLoaded.value = true
    }
  } catch (error) {
    console.error(`加载封面失败 (${props.game.game_id}):`, error)
  }
}

// 释放图片内存
const releaseCover = () => {
  // 清除本地引用
  coverUrl.value = ''
  imageLoaded.value = false
  isVisible.value = false
  console.log(`释放图片内存: ${props.game.game_id}`)
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
  padding-bottom: 47%; /* 约 2.13:1 比例 */
  border-radius: 12px;
  border: 2px solid var(--card-border);
  overflow: hidden;
  cursor: pointer;
  background-color: var(--bg-secondary);
  transition: transform 0.15s ease-out, border-color 0.15s ease;
  will-change: transform;
}

/* 悬浮效果 - 仅使用 transform */
.game-card:hover {
  transform: scale(1.02);
  border-color: var(--accent-color);
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
  font-size: 14px;
  font-weight: 600;
  color: #ffffff;
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
  line-height: 1.3;
}

.game-en-name {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.9);
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  display: block;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
  line-height: 1.3;
}

.game-id {
  font-size: 10px;
  color: rgba(255, 255, 255, 0.7);
  font-family: 'Courier New', monospace;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
  line-height: 1.3;
}
</style>
