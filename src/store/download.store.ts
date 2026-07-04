/**
 * download.store.ts - 游戏本体下载界面状态管理
 * 使用 Pinia 持久化下载界面的表单状态、下载进度与日志，
 * 避免用户在下载过程中切换到其他页面后返回时状态丢失。
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { DownloadProgress } from '../types/download.types'

/**
 * 批量游戏信息
 */
export interface StoreBatchGame {
  id: string
  name: string
  path: string
  selected: boolean
  status: 'pending' | 'downloading' | 'completed' | 'error'
}

/**
 * 下载日志条目
 */
export interface StoreDownloadLog {
  time: string
  message: string
  type: 'info' | 'success' | 'error' | 'warning'
}

/**
 * 下载界面 Store
 */
export const useDownloadStore = defineStore('download', () => {
  // ==================== State ====================

  /** 下载模式: single-单游戏 batch-批量 */
  const downloadMode = ref<'single' | 'batch'>('single')

  /** 单游戏模式状态 */
  const manifestPath = ref('')
  const downloadPath = ref('')
  const gameName = ref('')
  const customGameName = ref('')
  const gameId = ref('')
  const manifestFiles = ref<string[]>([])
  const vdfFilePath = ref('')
  const validationError = ref('')

  /** 批量下载模式状态 */
  const batchParentPath = ref('')
  const batchDownloadBasePath = ref('')
  const batchGames = ref<StoreBatchGame[]>([])
  const batchTotalGames = ref(0)
  const batchCompletedGames = ref(0)
  const currentDownloadingGame = ref<StoreBatchGame | null>(null)

  /** 自动关机选项 */
  const autoShutdown = ref(false)

  /** 下载状态 */
  const isDownloading = ref(false)
  const downloadLogs = ref<StoreDownloadLog[]>([])

  /** 下载进度监控 */
  const isMonitoring = ref(false)
  const downloadProgress = ref<DownloadProgress>({
    totalDepots: 0,
    completedDepots: 0,
    overallPercentage: 0,
    depots: [],
    isComplete: false
  })

  /**
   * 返回下载页面时是否需要重置状态
   * 当下载完成（100%）后置为 true，用户切走再回来时重置为初始状态
   */
  const shouldResetOnReturn = ref(false)

  // ==================== Getters ====================

  /**
   * 清单文件是否有效
   */
  const isManifestValid = computed(() => {
    return manifestFiles.value.length > 0 &&
           vdfFilePath.value !== '' &&
           gameId.value !== '' &&
           validationError.value === ''
  })

  /**
   * 已选择的批量游戏
   */
  const selectedBatchGames = computed(() => {
    return batchGames.value.filter(g => g.selected)
  })

  /**
   * 批量下载总体进度
   */
  const batchOverallProgress = computed(() => {
    if (batchTotalGames.value === 0) return 0
    return Math.round((batchCompletedGames.value / batchTotalGames.value) * 100)
  })

  /**
   * 当前实际使用的游戏名称（用户自定义优先）
   */
  const displayGameName = computed(() => {
    return customGameName.value || gameName.value
  })

  /**
   * 是否可以开始下载
   */
  const canStartDownload = computed(() => {
    if (isDownloading.value) return false

    if (downloadMode.value === 'single') {
      return isManifestValid.value && downloadPath.value !== ''
    } else {
      return batchParentPath.value !== '' &&
             batchDownloadBasePath.value !== '' &&
             selectedBatchGames.value.length > 0
    }
  })

  // ==================== Actions ====================

  /**
   * 设置单游戏模式路径与游戏信息
   */
  function setSingleGameState(state: {
    manifestPath?: string
    downloadPath?: string
    gameName?: string
    customGameName?: string
    gameId?: string
    manifestFiles?: string[]
    vdfFilePath?: string
    validationError?: string
  }) {
    if (state.manifestPath !== undefined) manifestPath.value = state.manifestPath
    if (state.downloadPath !== undefined) downloadPath.value = state.downloadPath
    if (state.gameName !== undefined) gameName.value = state.gameName
    if (state.customGameName !== undefined) customGameName.value = state.customGameName
    if (state.gameId !== undefined) gameId.value = state.gameId
    if (state.manifestFiles !== undefined) manifestFiles.value = state.manifestFiles
    if (state.vdfFilePath !== undefined) vdfFilePath.value = state.vdfFilePath
    if (state.validationError !== undefined) validationError.value = state.validationError
  }

  /**
   * 设置批量下载模式状态
   */
  function setBatchState(state: {
    batchParentPath?: string
    batchDownloadBasePath?: string
    batchGames?: StoreBatchGame[]
  }) {
    if (state.batchParentPath !== undefined) batchParentPath.value = state.batchParentPath
    if (state.batchDownloadBasePath !== undefined) batchDownloadBasePath.value = state.batchDownloadBasePath
    if (state.batchGames !== undefined) batchGames.value = state.batchGames
  }

  /**
   * 设置下载模式
   */
  function setDownloadMode(mode: 'single' | 'batch') {
    downloadMode.value = mode
  }

  /**
   * 设置下载路径
   */
  function setDownloadPath(path: string) {
    downloadPath.value = path
  }

  /**
   * 设置用户自定义游戏名称
   */
  function setCustomGameName(name: string) {
    customGameName.value = name
  }

  /**
   * 添加下载日志
   */
  function addLog(log: StoreDownloadLog) {
    downloadLogs.value.push(log)
    if (downloadLogs.value.length > 500) {
      downloadLogs.value = downloadLogs.value.slice(-400)
    }
  }

  /**
   * 清空下载日志
   */
  function clearLogs() {
    downloadLogs.value = []
  }

  /**
   * 设置下载中状态
   */
  function setDownloading(downloading: boolean) {
    isDownloading.value = downloading
  }

  /**
   * 设置自动关机选项
   */
  function setAutoShutdown(value: boolean) {
    autoShutdown.value = value
  }

  /**
   * 设置下载进度
   */
  function setDownloadProgress(progress: DownloadProgress) {
    downloadProgress.value = progress
  }

  /**
   * 设置监控状态
   */
  function setMonitoring(monitoring: boolean) {
    isMonitoring.value = monitoring
  }

  /**
   * 设置返回时是否需要重置状态
   */
  function setShouldResetOnReturn(value: boolean) {
    shouldResetOnReturn.value = value
  }

  /**
   * 设置批量下载进度
   */
  function setBatchProgress(total: number, completed: number, current: StoreBatchGame | null) {
    batchTotalGames.value = total
    batchCompletedGames.value = completed
    currentDownloadingGame.value = current
  }

  /**
   * 更新批量游戏中某个游戏的状态
   */
  function updateBatchGameStatus(id: string, status: StoreBatchGame['status']) {
    const game = batchGames.value.find(g => g.id === id)
    if (game) {
      game.status = status
    }
  }

  /**
   * 重置为初始状态（下载完成后调用）
   */
  function resetState() {
    downloadMode.value = 'single'
    manifestPath.value = ''
    downloadPath.value = ''
    gameName.value = ''
    customGameName.value = ''
    gameId.value = ''
    manifestFiles.value = []
    vdfFilePath.value = ''
    validationError.value = ''
    batchParentPath.value = ''
    batchDownloadBasePath.value = ''
    batchGames.value = []
    batchTotalGames.value = 0
    batchCompletedGames.value = 0
    currentDownloadingGame.value = null
    autoShutdown.value = false
    isDownloading.value = false
    downloadLogs.value = []
    isMonitoring.value = false
    downloadProgress.value = {
      totalDepots: 0,
      completedDepots: 0,
      overallPercentage: 0,
      depots: [],
      isComplete: false
    }
    shouldResetOnReturn.value = false
  }

  // 返回所有状态和方法
  return {
    // State
    downloadMode,
    manifestPath,
    downloadPath,
    gameName,
    customGameName,
    gameId,
    manifestFiles,
    vdfFilePath,
    validationError,
    batchParentPath,
    batchDownloadBasePath,
    batchGames,
    batchTotalGames,
    batchCompletedGames,
    currentDownloadingGame,
    autoShutdown,
    isDownloading,
    downloadLogs,
    isMonitoring,
    downloadProgress,
    shouldResetOnReturn,
    // Getters
    isManifestValid,
    selectedBatchGames,
    batchOverallProgress,
    displayGameName,
    canStartDownload,
    // Actions
    setSingleGameState,
    setBatchState,
    setDownloadMode,
    setDownloadPath,
    setCustomGameName,
    addLog,
    clearLogs,
    setDownloading,
    setAutoShutdown,
    setDownloadProgress,
    setMonitoring,
    setShouldResetOnReturn,
    setBatchProgress,
    updateBatchGameStatus,
    resetState
  }
})
