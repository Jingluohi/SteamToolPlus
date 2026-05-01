<template>
  <!-- 
    GameGrid.vue - 游戏网格组件
    以网格或列表形式展示游戏卡片
    支持响应式布局
  -->
  <div 
    class="game-grid"
    :class="`view-${viewType}`"
  >
    <GameCard
      v-for="game in games"
      :key="game.id"
      :game="game"
      :view-type="viewType"
      @click="handleGameClick"
      @play="handleGamePlay"
    />
  </div>
</template>

<script setup lang="ts">
/**
 * GameGrid.vue - 游戏网格组件
 * 响应式网格布局展示游戏列表
 */

import type { Game } from '../../types'
import GameCard from './GameCard.vue'

/**
 * 组件属性定义
 */
interface Props {
  /** 游戏列表 */
  games: Game[]
  /** 视图类型 */
  viewType?: 'grid' | 'list'
}

withDefaults(defineProps<Props>(), {
  viewType: 'grid'
})

/**
 * 组件事件定义
 */
const emit = defineEmits<{
  (e: 'click', game: Game): void
  (e: 'play', game: Game): void
}>()

/**
 * 处理游戏卡片点击
 */
function handleGameClick(game: Game) {
  emit('click', game)
}

/**
 * 处理开始游戏
 */
function handleGamePlay(game: Game) {
  emit('play', game)
}
</script>

<style scoped>
.game-grid {
  display: grid;
  gap: 16px;
}

/* 网格视图 */
.game-grid.view-grid {
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
}

/* 列表视图 */
.game-grid.view-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

/* 响应式布局 */
@media (min-width: 1280px) {
  .game-grid.view-grid {
    grid-template-columns: repeat(4, 1fr);
  }
}

@media (min-width: 1600px) {
  .game-grid.view-grid {
    grid-template-columns: repeat(5, 1fr);
  }
}

@media (min-width: 1920px) {
  .game-grid.view-grid {
    grid-template-columns: repeat(6, 1fr);
  }
}

@media (max-width: 1024px) {
  .game-grid.view-grid {
    grid-template-columns: repeat(3, 1fr);
  }
}

@media (max-width: 768px) {
  .game-grid.view-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}
</style>
