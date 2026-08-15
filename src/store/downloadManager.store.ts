/**
 * downloadManager.store.ts - 全局多游戏下载状态管理
 * 按 game_id 统一管理所有游戏的下载进度、日志和监控定时器，
 * 支持多游戏同时下载，页面切换后状态不丢失。
 *
 * 内存优化：
 * - 原地修改 downloadProgress 对象，避免每 2 秒创建新对象
 * - 日志截断用 splice 而非 slice，避免创建新数组
 * - 集中管理扫描逻辑，消除页面级重复代码
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { DownloadProgress, DownloadLog } from '../types/download.types'

/**
 * 单个游戏的下载状态
 */
export interface GameDownloadState {
  gameId: string
  gameName: string
  isMonitoring: boolean
  isDownloading: boolean
  downloadProgress: DownloadProgress
  downloadLogs: DownloadLog[]
}

/** 创建空的下载进度 */
function createEmptyProgress(): DownloadProgress {
  return {
    totalDepots: 0,
    completedDepots: 0,
    overallPercentage: 0,
    depots: [],
    isComplete: false
  }
}

/** 解析进度文件名: "{百分比}% - {depotId}.json" */
function parseProgressFileName(fileName: string): { depotId: string; percentage: number } | null {
  const match = fileName.match(/^(\d+)%\s*-\s*(\d+)\.json$/)
  if (match) {
    return {
      percentage: parseInt(match[1], 10),
      depotId: match[2]
    }
  }
  return null
}

export const useDownloadManagerStore = defineStore('downloadManager', () => {
  // ==================== State ====================

  /** 所有游戏的下载状态 Map<gameId, GameDownloadState> */
  const downloads = ref<Map<string, GameDownloadState>>(new Map())

  /** 监控定时器表（非响应式） Map<gameId, timerId> */
  const monitorTimers = new Map<string, number>()

  // ==================== 内部辅助 ====================

  /** 获取或创建游戏状态 */
  function ensureState(gameId: string): GameDownloadState {
    if (!downloads.value.has(gameId)) {
      downloads.value.set(gameId, {
        gameId,
        gameName: '',
        isMonitoring: false,
        isDownloading: false,
        downloadProgress: createEmptyProgress(),
        downloadLogs: []
      })
    }
    return downloads.value.get(gameId)!
  }

  // ==================== Getters ====================

  function isGameMonitoring(gameId: string): boolean {
    return downloads.value.get(gameId)?.isMonitoring ?? false
  }

  function isGameDownloading(gameId: string): boolean {
    return downloads.value.get(gameId)?.isDownloading ?? false
  }

  function getGameProgress(gameId: string): DownloadProgress {
    return ensureState(gameId).downloadProgress
  }

  function getGameLogs(gameId: string): DownloadLog[] {
    return ensureState(gameId).downloadLogs
  }

  const activeDownloadGameIds = computed(() => {
    const result: string[] = []
    downloads.value.forEach((state, gameId) => {
      if (state.isDownloading || state.isMonitoring) {
        result.push(gameId)
      }
    })
    return result
  })

  // ==================== Actions ====================

  function setGameName(gameId: string, gameName: string) {
    ensureState(gameId).gameName = gameName
  }

  function setGameDownloading(gameId: string, downloading: boolean) {
    ensureState(gameId).isDownloading = downloading
  }

  /**
   * 初始化 depot 列表（原地修改，不创建新 downloadProgress 对象）
   */
  function initDepots(gameId: string, depotIds: string[]) {
    const state = ensureState(gameId)
    const progress = state.downloadProgress
    progress.totalDepots = depotIds.length
    progress.depots = depotIds.map(depotId => ({
      depotId,
      percentage: 0,
      downloadedFiles: 0,
      totalFiles: 0,
      status: 'pending' as const
    }))
    progress.completedDepots = 0
    progress.overallPercentage = 0
    progress.isComplete = false
  }

  /**
   * 增量更新单个 depot 进度（原地修改，不创建新对象）
   */
  function updateDepotProgress(
    gameId: string,
    depotId: string,
    percentage: number,
    status: 'pending' | 'downloading' | 'completed' | 'error'
  ) {
    const state = ensureState(gameId)
    const progress = state.downloadProgress
    const depots = progress.depots
    const depotIndex = depots.findIndex(d => d.depotId === depotId)

    if (depotIndex !== -1) {
      const depot = depots[depotIndex]
      if (depot.percentage !== percentage || depot.status !== status) {
        depot.percentage = percentage
        depot.status = status
      }
    } else {
      depots.push({ depotId, percentage, downloadedFiles: 0, totalFiles: 0, status })
    }

    // 原地修改总体进度字段
    progress.completedDepots = depots.filter(d => d.status === 'completed').length
    progress.overallPercentage = depots.length > 0
      ? Math.round(depots.reduce((sum, d) => sum + d.percentage, 0) / depots.length)
      : 0
    progress.isComplete = depots.length > 0 && depots.every(d => d.status === 'completed')
  }

  /**
   * 扫描指定游戏的进度文件，更新 depot 进度
   * 返回结果供页面处理完成逻辑
   */
  async function scanProgressFiles(gameId: string): Promise<{ hasChanges: boolean; isComplete: boolean }> {
    try {
      const progressFiles = await invoke<Array<{ name: string; path: string }>>(
        'get_download_progress_files',
        { gameId }
      )

      let hasChanges = false

      for (const file of progressFiles) {
        const parsed = parseProgressFileName(file.name)
        if (!parsed) continue

        const newStatus = parsed.percentage >= 100 ? 'completed' : 'downloading'
        const currentDepot = getGameProgress(gameId).depots
          .find(d => d.depotId === parsed.depotId)

        if (!currentDepot || currentDepot.percentage !== parsed.percentage || currentDepot.status !== newStatus) {
          updateDepotProgress(gameId, parsed.depotId, parsed.percentage, newStatus)
          hasChanges = true
        }
      }

      return { hasChanges, isComplete: getGameProgress(gameId).isComplete }
    } catch {
      return { hasChanges: false, isComplete: false }
    }
  }

  /** 添加下载日志 */
  function addLog(gameId: string, log: DownloadLog) {
    const state = ensureState(gameId)
    const logs = state.downloadLogs
    logs.push(log)
    // 用 splice 原地截断，避免 slice 创建新数组
    if (logs.length > 500) {
      logs.splice(0, logs.length - 400)
    }
  }

  /** 清空日志 */
  function clearLogs(gameId: string) {
    const state = ensureState(gameId)
    state.downloadLogs.length = 0
  }

  /** 原地更新整个进度对象 */
  function updateProgress(gameId: string, progress: DownloadProgress) {
    const state = ensureState(gameId)
    state.downloadProgress = progress
  }

  /** 注册监控定时器 */
  function registerMonitorTimer(gameId: string, timerId: number) {
    stopMonitoring(gameId)
    monitorTimers.set(gameId, timerId)
    ensureState(gameId).isMonitoring = true
  }

  /** 停止指定游戏的监控 */
  function stopMonitoring(gameId: string) {
    const timerId = monitorTimers.get(gameId)
    if (timerId !== undefined) {
      clearInterval(timerId)
      monitorTimers.delete(gameId)
    }
    const state = downloads.value.get(gameId)
    if (state) state.isMonitoring = false
  }

  /** 标记下载完成 */
  function markComplete(gameId: string) {
    const state = ensureState(gameId)
    state.isDownloading = false
    state.isMonitoring = false
    const progress = state.downloadProgress
    progress.isComplete = true
    progress.overallPercentage = 100
    progress.completedDepots = progress.totalDepots
    progress.depots.forEach(depot => {
      depot.percentage = 100
      depot.status = 'completed'
    })
    stopMonitoring(gameId)
  }

  /** 重置指定游戏 */
  function resetGame(gameId: string) {
    stopMonitoring(gameId)
    downloads.value.delete(gameId)
  }

  return {
    downloads,
    isGameMonitoring,
    isGameDownloading,
    getGameProgress,
    getGameLogs,
    activeDownloadGameIds,
    setGameName,
    setGameDownloading,
    initDepots,
    updateDepotProgress,
    scanProgressFiles,
    updateProgress,
    addLog,
    clearLogs,
    registerMonitorTimer,
    stopMonitoring,
    markComplete,
    resetGame
  }
})