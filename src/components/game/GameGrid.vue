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
      :key="game.game_id"
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

import type { GameConfigData } from '../../types'
import GameCard from './GameCard.vue'

/**
 * 组件属性定义
 */
interface Props {
  /** 游戏列表 */
  games: GameConfigData[]
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
  (e: 'click', game: GameConfigData): void
  (e: 'play', game: GameConfigData): void
}>()

/**
 * 处理游戏卡片点击
 */
function handleGameClick(game: GameConfigData) {
  emit('click', game)
}

/**
 * 处理开始游戏
 */
function handleGamePlay(game: GameConfigData) {
  emit('play', game)
}
</script>

<style scoped>
.game-grid {
  display: grid;
}

/* 网格视图 - 固定5列，间距与卡片宽度保持16:220比例 */
.game-grid.view-grid {
  grid-template-columns: repeat(5, 1fr);
  /* 行间距1.110%(增大2倍)，列间距0.666%(增大20%) */
  gap: 1.20% 0.750%;
}

/* 列表视图 */
.game-grid.view-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
</style>
