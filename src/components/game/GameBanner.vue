<template>
  <!-- 
    GameBanner.vue - 游戏轮播Banner组件
    展示精选游戏的宣传图
    支持自动轮播和手动切换
  -->
  <div class="game-banner">
    <!-- 轮播容器 -->
    <div class="banner-container">
      <TransitionGroup name="banner">
        <div 
          v-for="(game, index) in games"
          v-show="index === currentIndex"
          :key="game.id"
          class="banner-slide"
        >
          <!-- 背景图 -->
          <div 
            class="banner-bg"
            :style="{ backgroundImage: `url(${game.coverPath})` }"
          />
          
          <!-- 内容覆盖层 -->
          <div class="banner-content">
            <div class="banner-info">
              <h2 class="banner-title">{{ game.name }}</h2>
              <p v-if="game.publisher" class="banner-publisher">{{ game.publisher }}</p>
              <div class="banner-tags">
                <span 
                  v-for="tag in game.tags.slice(0, 3)" 
                  :key="tag"
                  class="banner-tag"
                >
                  {{ tag }}
                </span>
              </div>
              <button 
                class="banner-play-btn"
                @click="handlePlay(game)"
              >
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M8 5v14l11-7z"/>
                </svg>
                开始游戏
              </button>
            </div>
          </div>
        </div>
      </TransitionGroup>
      
      <!-- 左右切换按钮 -->
      <button 
        class="nav-btn nav-prev"
        @click="prevSlide"
      >
        <svg viewBox="0 0 24 24" fill="currentColor">
          <path d="M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z"/>
        </svg>
      </button>
      
      <button 
        class="nav-btn nav-next"
        @click="nextSlide"
      >
        <svg viewBox="0 0 24 24" fill="currentColor">
          <path d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z"/>
        </svg>
      </button>
    </div>
    
    <!-- 指示器 -->
    <div class="banner-indicators">
      <button
        v-for="(_, index) in games"
        :key="index"
        class="indicator"
        :class="{ active: index === currentIndex }"
        @click="goToSlide(index)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * GameBanner.vue - 游戏轮播Banner组件
 * 展示精选游戏，支持自动轮播
 */

import { ref, onMounted, onUnmounted } from 'vue'
import type { Game } from '../../types'

/**
 * 组件属性定义
 */
interface Props {
  /** 游戏列表 */
  games: Game[]
  /** 自动轮播间隔（毫秒） */
  interval?: number
}

const props = withDefaults(defineProps<Props>(), {
  interval: 5000
})

/**
 * 组件事件定义
 */
const emit = defineEmits<{
  (e: 'play', game: Game): void
}>()

// 当前索引
const currentIndex = ref(0)
// 自动播放定时器
let autoPlayTimer: ReturnType<typeof setInterval> | null = null

/**
 * 下一张
 */
function nextSlide() {
  if (props.games.length === 0) return
  currentIndex.value = (currentIndex.value + 1) % props.games.length
}

/**
 * 上一张
 */
function prevSlide() {
  if (props.games.length === 0) return
  currentIndex.value = (currentIndex.value - 1 + props.games.length) % props.games.length
}

/**
 * 跳转到指定幻灯片
 */
function goToSlide(index: number) {
  currentIndex.value = index
  resetAutoPlay()
}

/**
 * 处理开始游戏
 */
function handlePlay(game: Game) {
  emit('play', game)
}

/**
 * 开始自动播放
 */
function startAutoPlay() {
  if (props.games.length <= 1) return
  autoPlayTimer = setInterval(nextSlide, props.interval)
}

/**
 * 停止自动播放
 */
function stopAutoPlay() {
  if (autoPlayTimer) {
    clearInterval(autoPlayTimer)
    autoPlayTimer = null
  }
}

/**
 * 重置自动播放
 */
function resetAutoPlay() {
  stopAutoPlay()
  startAutoPlay()
}

// 生命周期钩子
onMounted(() => {
  startAutoPlay()
})

onUnmounted(() => {
  stopAutoPlay()
})
</script>

<style scoped>
.game-banner {
  position: relative;
  border-radius: 12px;
  overflow: hidden;
}

.banner-container {
  position: relative;
  height: 360px;
  background: var(--steam-bg-secondary);
}

.banner-slide {
  position: absolute;
  inset: 0;
}

.banner-bg {
  position: absolute;
  inset: 0;
  background-size: cover;
  background-position: center;
  filter: brightness(0.6);
}

.banner-content {
  position: relative;
  height: 100%;
  display: flex;
  align-items: flex-end;
  padding: 32px;
  background: linear-gradient(to top, rgba(0, 0, 0, 0.8) 0%, transparent 100%);
}

.banner-info {
  max-width: 500px;
}

.banner-title {
  font-size: 32px;
  font-weight: 600;
  color: white;
  margin-bottom: 8px;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
}

.banner-publisher {
  font-size: 16px;
  color: var(--steam-text-secondary);
  margin-bottom: 12px;
}

.banner-tags {
  display: flex;
  gap: 8px;
  margin-bottom: 20px;
}

.banner-tag {
  padding: 4px 12px;
  background: rgba(255, 255, 255, 0.15);
  color: white;
  font-size: 12px;
  border-radius: 4px;
  backdrop-filter: blur(4px);
}

.banner-play-btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  background: var(--steam-accent-blue);
  color: white;
  font-size: 14px;
  font-weight: 500;
  border-radius: 4px;
  transition: background 0.15s ease-out;
}

.banner-play-btn:hover {
  background: var(--steam-accent-green);
}

.banner-play-btn svg {
  width: 20px;
  height: 20px;
}

/* 导航按钮 */
.nav-btn {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.5);
  color: white;
  border-radius: 50%;
  opacity: 0;
  transition: opacity 0.2s ease-out, background 0.15s ease-out;
}

.game-banner:hover .nav-btn {
  opacity: 1;
}

.nav-btn:hover {
  background: rgba(0, 0, 0, 0.7);
}

.nav-btn svg {
  width: 24px;
  height: 24px;
}

.nav-prev {
  left: 16px;
}

.nav-next {
  right: 16px;
}

/* 指示器 */
.banner-indicators {
  position: absolute;
  bottom: 16px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  gap: 8px;
}

.indicator {
  width: 8px;
  height: 8px;
  background: rgba(255, 255, 255, 0.4);
  border-radius: 50%;
  transition: background 0.2s ease-out, transform 0.2s ease-out;
}

.indicator.active {
  background: white;
  transform: scale(1.2);
}

.indicator:hover {
  background: rgba(255, 255, 255, 0.7);
}

/* 轮播动画 */
.banner-enter-active,
.banner-leave-active {
  transition: opacity 0.3s ease-in-out;
}

.banner-enter-from,
.banner-leave-to {
  opacity: 0;
}
</style>
