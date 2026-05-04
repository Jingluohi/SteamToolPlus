/**
 * game.store.ts - 游戏库状态管理
 * 使用Pinia实现游戏相关的全局状态
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Game } from '../types/game.types'
import * as gameApi from '../api/game.api'

/**
 * 游戏Store
 */
export const useGameStore = defineStore('game', () => {
  // ==================== State ====================
  const games = ref<Game[]>([])
  const selectedGame = ref<Game | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  // ==================== Getters ====================
  const gameCount = computed(() => games.value.length)
  const installedGames = computed(() => games.value.filter(g => g.isInstalled))
  const favoriteGames = computed(() => games.value.filter(g => g.isFavorite))

  // ==================== Actions ====================

  /**
   * 加载游戏列表
   */
  async function loadGames() {
    loading.value = true
    error.value = null
    try {
      games.value = await gameApi.getGames()
    } catch (err) {
      error.value = err instanceof Error ? err.message : '加载游戏列表失败'
    } finally {
      loading.value = false
    }
  }

  /**
   * 选择游戏
   */
  function selectGame(game: Game | null) {
    selectedGame.value = game
  }

  /**
   * 根据ID获取游戏
   */
  function getGameById(id: string): Game | undefined {
    return games.value.find(g => g.id === id)
  }

  // 返回所有状态和方法
  return {
    // State
    games,
    selectedGame,
    loading,
    error,
    // Getters
    gameCount,
    installedGames,
    favoriteGames,
    // Actions
    loadGames,
    selectGame,
    getGameById
  }
})
