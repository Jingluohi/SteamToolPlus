/**
 * downloadManager.store.ts - 全局多游戏下载状态管理
 * 按 game_id 统一管理所有游戏的下载进度、日志和监控定时器，
 * 支持多游戏同时下载，页面切换后状态不丢失。
 *
 * 内存优化：
 * - 原地修改 downloadProgress 对象，避免每 2 秒创建新对象
 * - 日志截断用 splice 而非 slice，避免创建新数组
 * - 集中管理扫描逻辑，消除页面级重复代码
 *
 * 轮询优化（问题 12）：
 * - 全局限定单个定时器，取代过去「每个游戏各自一个 setInterval」的方案
 * - 每次轮询只发起一次批量 IPC（扫描全部游戏的进度文件），按 gameId 分发结果
 * - 下载完成事件通过回调表通知页面，页面不再各自维护完成检测代码
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { DownloadProgress, DownloadLog, ProgressFileInfo } from '../types/download.types'

/** 轮询间隔（毫秒） */
const POLL_INTERVAL_MS = 2000

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

export const useDownloadManagerStore = defineStore('downloadManager', () => {
  // ==================== State ====================

  /** 所有游戏的下载状态 Map<gameId, GameDownloadState> */
  const downloads = ref<Map<string, GameDownloadState>>(new Map())

  /** 统一轮询的全局定时器（非响应式，全局唯一） */
  let pollTimer: number | null = null

  /** 正在被轮询监控的游戏 ID 集合（非响应式） */
  const polledGameIds = new Set<string>()

  /** 下载完成回调表 Map<gameId, Set<callback>>（非响应式） */
  const completeCallbacks = new Map<string, Set<() => void>>()

  /** 进度变化回调表 Map<gameId, Set<callback>>（非响应式） */
  const progressCallbacks = new Map<string, Set<() => void>>()

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

  /** 启动全局定时器（幂等） */
  function ensureGlobalTimer() {
    if (pollTimer !== null) return
    pollTimer = window.setInterval(() => {
      void pollAllProgress()
    }, POLL_INTERVAL_MS)
  }

  /** 停止全局定时器 */
  function stopGlobalTimer() {
    if (pollTimer !== null) {
      clearInterval(pollTimer)
      pollTimer = null
    }
  }

  /**
   * 根据一批进度文件更新某个游戏的 depot 进度
   * 返回是否发生了进度变化
   */
  function applyProgressFiles(gameId: string, files: ProgressFileInfo[]): boolean {
    let hasChanges = false

    for (const file of files) {
      const newStatus: 'downloading' | 'completed' =
        file.progress >= 100 ? 'completed' : 'downloading'
      const current = getGameProgress(gameId).depots.find(d => d.depotId === file.depotId)

      if (!current || current.percentage !== file.progress || current.status !== newStatus) {
        updateDepotProgress(gameId, file.depotId, file.progress, newStatus)
        hasChanges = true
      }
    }

    return hasChanges
  }

  /** 进度变化或完成时，触发该游戏对应的回调 */
  function applyAndNotify(gameId: string, files: ProgressFileInfo[]): boolean {
    const hasChanges = applyProgressFiles(gameId, files)
    if (hasChanges) {
      notifyCallbacks(progressCallbacks, gameId)
      if (getGameProgress(gameId).isComplete) {
        notifyCallbacks(completeCallbacks, gameId)
      }
    }
    return hasChanges
  }

  /** 通知某个游戏的回调表（先快照，回调内部可能移除自身） */
  function notifyCallbacks(
    table: Map<string, Set<() => void>>,
    gameId: string
  ) {
    const callbacks = table.get(gameId)
    if (!callbacks || callbacks.size === 0) return
    for (const cb of Array.from(callbacks)) {
      cb()
    }
  }

  /**
   * 批量轮询：一次 IPC 扫描所有被监控游戏，按 gameId 分发更新
   * 取代过去「每个游戏各自一个 setInterval + 各发一次 IPC」的方案
   */
  async function pollAllProgress(): Promise<void> {
    if (polledGameIds.size === 0) return

    try {
      // 不传 gameId，后端扫描 log/*/ 全部子目录
      const files = await invoke<ProgressFileInfo[]>('get_download_progress_files')

      // 按 gameId 分组
      const byGame = new Map<string, ProgressFileInfo[]>()
      for (const file of files) {
        if (!polledGameIds.has(file.gameId)) continue
        if (!byGame.has(file.gameId)) byGame.set(file.gameId, [])
        byGame.get(file.gameId)!.push(file)
      }

      for (const [gameId, gameFiles] of byGame) {
        applyAndNotify(gameId, gameFiles)
      }
    } catch {
      // 忽略单次扫描错误，等待下一轮重试
    }
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
   * 立即扫描指定游戏的进度文件（单次，用于开始监控时的首屏反馈）
   * 若检测到完成会触发完成回调
   */
  async function scanProgressFiles(gameId: string): Promise<{ hasChanges: boolean; isComplete: boolean }> {
    try {
      const files = await invoke<ProgressFileInfo[]>('get_download_progress_files', { gameId })
      const hasChanges = applyAndNotify(gameId, files)
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

  /**
   * 开始监控指定游戏（加入统一轮询）
   * 不再需要页面传入定时器 ID，全局定时器由 store 统一管理
   */
  function startMonitoring(gameId: string) {
    ensureState(gameId).isMonitoring = true
    polledGameIds.add(gameId)
    ensureGlobalTimer()
  }

  /** 停止指定游戏的监控（从统一轮询中移除，并清理回调） */
  function stopMonitoring(gameId: string) {
    polledGameIds.delete(gameId)
    completeCallbacks.delete(gameId)
    progressCallbacks.delete(gameId)
    const state = downloads.value.get(gameId)
    if (state) state.isMonitoring = false
    if (polledGameIds.size === 0) {
      stopGlobalTimer()
    }
  }

  /** 注册某个游戏的下载完成回调 */
  function onComplete(gameId: string, callback: () => void) {
    registerCallback(completeCallbacks, gameId, callback)
  }

  /** 注册某个游戏的进度变化回调（每个轮询周期的进度更新都会触发） */
  function onProgress(gameId: string, callback: () => void) {
    registerCallback(progressCallbacks, gameId, callback)
  }

  /** 向指定回调表注册回调 */
  function registerCallback(
    table: Map<string, Set<() => void>>,
    gameId: string,
    callback: () => void
  ) {
    if (!table.has(gameId)) {
      table.set(gameId, new Set())
    }
    table.get(gameId)!.add(callback)
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
    startMonitoring,
    stopMonitoring,
    onComplete,
    onProgress,
    markComplete,
    resetGame
  }
})