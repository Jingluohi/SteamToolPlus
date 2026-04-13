<template>
  <div class="game-grid-container">
    <!-- 加载状态 -->
    <div v-if="gamesStore.loading" class="loading-state">
      <div class="loading-spinner"></div>
      <span>加载游戏中...</span>
    </div>

    <!-- 错误状态 -->
    <div v-else-if="gamesStore.error" class="empty-state">
      <svg class="empty-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
        <line x1="9" y1="9" x2="15" y2="15"/>
        <line x1="15" y1="9" x2="9" y2="15"/>
      </svg>
      <span class="empty-text">加载失败</span>
      <span class="empty-hint">{{ gamesStore.error }}</span>
    </div>

    <!-- 空状态 -->
    <div v-else-if="gamesStore.filteredGames.length === 0" class="empty-state">
      <svg class="empty-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
        <line x1="9" y1="9" x2="15" y2="15"/>
        <line x1="15" y1="9" x2="9" y2="15"/>
      </svg>
      <span class="empty-text">没有找到游戏</span>
      <span class="empty-hint">尝试调整搜索条件或分类</span>
    </div>

    <!-- 游戏网格 -->
    <div
      v-else
      class="game-grid"
      :class="columnsClass"
    >
      <GameCard
        v-for="game in gamesStore.filteredGames"
        :key="game.game_id"
        :game="game"
        @click="handleGameClick"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useGamesStore, type Game } from '../../stores/games'
import GameCard from './GameCard.vue'

const gamesStore = useGamesStore()
const router = useRouter()

// 固定间距为16px
const GAP = 16

// 列数（4或5列）
const columns = ref(4)

// 计算当前列数的class
const columnsClass = computed(() => ({
  'four-columns': columns.value === 4,
  'five-columns': columns.value === 5
}))

// 更新列数
const updateColumns = () => {
  const width = window.innerWidth
  // 侧边栏约200px
  const contentWidth = width - 200
  // 内容区域≥1400px显示5列，否则4列
  columns.value = contentWidth >= 1400 ? 5 : 4
}

// 处理游戏点击 - 跳转到详情页
const handleGameClick = (game: Game) => {
  router.push(`/game/${game.game_id}`)
}

// 监听筛选条件变化时，滚动到顶部
watch(() => gamesStore.filteredGames, () => {
  // 筛选变化不需要滚动到顶部
}, { deep: true })

// 监听窗口大小变化
const handleResize = () => {
  updateColumns()
}

onMounted(() => {
  updateColumns()
  gamesStore.loadGames()
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
})
</script>

<style scoped>
.game-grid-container {
  width: 100%;
  height: 100%;
  overflow: hidden;
}

/* 加载状态 */
.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 16px;
  color: var(--text-secondary);
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--border-color);
  border-top-color: var(--accent-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* 空状态 */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 12px;
  color: var(--text-secondary);
}

.empty-icon {
  width: 64px;
  height: 64px;
  opacity: 0.5;
}

.empty-text {
  font-size: 16px;
  font-weight: 500;
}

.empty-hint {
  font-size: 13px;
  opacity: 0.7;
}

/* 游戏网格 - 使用CSS Grid布局 */
.game-grid {
  height: 100%;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 16px;
  display: grid;
  gap: 16px;
  /* 使用 auto 让行高由内容决定，配合卡片的 aspect-ratio */
  grid-auto-rows: auto;
  align-content: start;
}

/* 4列布局 - 每列使用1fr自动分配宽度 */
.game-grid.four-columns {
  grid-template-columns: repeat(4, 1fr);
}

/* 5列布局 - 每列使用1fr自动分配宽度 */
.game-grid.five-columns {
  grid-template-columns: repeat(5, 1fr);
}

/* 自定义滚动条 */
.game-grid::-webkit-scrollbar {
  width: 6px;
}

.game-grid::-webkit-scrollbar-track {
  background: transparent;
}

.game-grid::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 3px;
}

.game-grid::-webkit-scrollbar-thumb:hover {
  background: var(--text-secondary);
}
</style>
