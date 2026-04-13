<template>
  <div class="game-list-view">
    <GameGrid />
  </div>
</template>

<script setup lang="ts">
import GameGrid from '../components/game/GameGrid.vue'
</script>

<style scoped>
.game-list-view {
  width: 100%;
  height: 100%;
  overflow: hidden;
  background-color: var(--bg-primary);
}
</style>
