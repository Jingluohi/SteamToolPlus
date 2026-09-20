/**
 * downloadManager.store.ts - 全局多游戏下载状态管理
 * 按 game_id 统一管理所有游戏的下载进度、日志与事件桥，
 * 支持多游戏同时下载，页面切换后状态不丢失。
 *
 * 进度链路（方案 C：事件推送为主 + 快照兜底）：
 * - 主链路：订阅后端 Tauri 事件（depot-download-progress / depot-download-finished），
 *   实时更新进度与结束状态。引擎每次分块落盘都会推送一条进度事件，
 *   前端无需任何轮询（替代旧版「每 2 秒扫描 log 目录下 JSON 进度文件」）；
 * - 兜底链路：get_download_status 快照在页面刷新/进入时拉一次，
 *   恢复进行中下载的进度展示（事件在页面挂载前已发出也不丢数据）；
 * - 结束判定：以 depot-download-finished(success) 为唯一下载结束信号。
 *   成功时引擎已完成全部 depot 落盘与 SHA-1 校验，此时才置 100% 完成；
 *   取消/失败同样收到结束事件，前端据此收尾展示。
 *
 * 内存优化（沿用旧版约定）：
 * - 原地修改 downloadProgress 对象，不创建冗余副本
 * - 日志截断用 splice 而非 slice
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import type {
  DownloadLog,
  DownloadProgress,
  EngineDepotProgress,
  EngineStatus,
  DownloadProgressEvent,
  DownloadFinishedEvent
} from '../types/download.types'

/**
 * 单个游戏的下载状态
 */
export interface GameDownloadState {
  gameId: string
  gameName: string
  isMonitoring: boolean
  isDownloading: boolean
  /** 是否因用户暂停而中断（重置/续跑时清除） */
  isPaused: boolean
  downloadProgress: DownloadProgress
  downloadLogs: DownloadLog[]
  /** 本次下载总字节数（引擎全局字节模型） */
  totalBytes: number
  /** 本次下载已下载字节数（引擎累计值） */
  doneBytes: number
  /** 实时下载速度（字节/秒，由进度事件差分估算） */
  speed: number
  /** 完成时间戳（markComplete 时记录，供"已完成"列表排序） */
  completedAt?: number
}

/**
 * 已完成下载历史条目（localStorage 持久化，页面重启后仍可见）
 */
export interface DownloadHistoryItem {
  gameId: string
  gameName: string
  /** 完成时间戳 */
  completedAt: number
  /** 本次下载总字节数 */
  totalBytes: number
}

/** localStorage 中已完成历史记录的 key */
const HISTORY_KEY = 'stp_download_history'

/**
 * 归一化引擎消息：取消/暂停类（download-cancelled / 下载已取消 / 下载已暂停）
 * 统一显示"游戏下载已暂停"，其余原样返回；空消息用 fallback 兜底。
 */
function normalizeEngineMessage(raw: string, fallback: string): string {
  const msg = raw || ''
  if (/cancel|取消|暂停/i.test(msg)) return '游戏下载已暂停'
  return msg || fallback
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

/** 读取旧 localStorage 历史（仅用于一次性迁移到 game.json，迁移后清除） */
function loadHistory(): DownloadHistoryItem[] {
  try {
    const raw = localStorage.getItem(HISTORY_KEY)
    return raw ? (JSON.parse(raw) as DownloadHistoryItem[]) : []
  } catch {
    return []
  }
}

// ==================== 游戏名解析（模块级，供列表/进度条共用） ====================

/** 解析结果缓存（避免重复 IPC） */
const gameNameCache = new Map<string, string>()
/** 游戏库配置（games_config.json）的 id→名称 映射，懒加载一次 */
let gamesConfigCache: Map<string, string> | null = null

/** 轻量游戏信息形状（get_game_data / games_config 的公共字段） */
interface NameSource {
  game_name?: string
  chinese_name?: string
}

/**
 * 按 gameId 解析显示名：已入库 game.json → 游戏库配置 → 兜底 gameId。
 * 显示名 = 「中文名 + 英文名」（中文在前；二者相同或缺失时取其一）。
 * 结果缓存到模块级 Map（一次解析，全局复用）。
 */
export async function resolveGameName(gameId: string): Promise<string> {
  const cached = gameNameCache.get(gameId)
  if (cached) return cached

  // 组合显示名：中文名在前，英文名在后；相同或缺任一则只显示可用的一项
  const pick = (s?: NameSource) => {
    const cn = s?.chinese_name?.trim()
    const en = s?.game_name?.trim()
    if (cn && en && cn !== en) return `${cn} ${en}`
    return cn || en || gameId
  }

  // 1. 已入库游戏数据（game.json）
  try {
    const data = await invoke<NameSource | null>('get_game_data', { gameId })
    if (data) {
      const name = pick(data)
      gameNameCache.set(gameId, name)
      return name
    }
  } catch {
    // 查询失败走下一级
  }

  // 2. 游戏库配置（games_config.json，懒加载一次）
  try {
    if (!gamesConfigCache) {
      gamesConfigCache = new Map()
      const list = await invoke<Array<{ game_id?: string } & NameSource>>(
        'load_games_config_from_file'
      )
      for (const g of list) {
        if (g.game_id) gamesConfigCache.set(g.game_id, pick(g))
      }
    }
    const name = gamesConfigCache.get(gameId)
    if (name) {
      gameNameCache.set(gameId, name)
      return name
    }
  } catch {
    // 配置解析失败走兜底
  }

  // 3. 兜底 gameId（不缓存：允许查询数据就绪后再次解析出真实名称）
  return gameId
}

export const useDownloadManagerStore = defineStore('downloadManager', () => {
  // ==================== State ====================

  /** 所有游戏的下载状态 Map<gameId, GameDownloadState> */
  const downloads = ref<Map<string, GameDownloadState>>(new Map())

  /** 事件桥注册标记（惰性注册，全局只注册一次） */
  let eventBridged = false
  /** 已注册的事件退订函数（供页面/应用退出时清理） */
  const progressUnlistens: UnlistenFn[] = []
  const finishUnlistens: UnlistenFn[] = []

  /** 下载完成回调表 Map<gameId, Set<callback>>（非响应式） */
  const completeCallbacks = new Map<string, Set<() => void>>()

  /** 进度变化回调表 Map<gameId, Set<callback>>（非响应式） */
  const progressCallbacks = new Map<string, Set<() => void>>()

  /**
   * 速度滑动窗口（非响应式）：Map<gameId, {bytes,time}[]>。
   * 事件到达即把采样推入窗口，剔除过期样本（>1.5s）后，用窗口首尾
   * 「字节差 / 用时差」结算速度——事件密集时随每个事件平滑刷新，
   * 大文件下载中（单块耗时较长）速度也不再长时间冻结在旧值。
   */
  const speedWindows = new Map<string, { bytes: number; time: number }[]>()

  /**
   * 被「清除」的游戏集合（非响应式）：清除后忽略其后续引擎事件，
   * 避免 stop_download 引发的 finished 事件把条目重新建出来
   */
  const clearedGames = new Set<string>()

  /** 已完成下载历史（持久化于 game.json，前端内存镜像；清理系统缓存不影响） */
  const historyList = ref<DownloadHistoryItem[]>([])

  /**
   * 下载结束回调表 Map<gameId, Set<(success, message) => void>>（非响应式）
   * 成功/取消/失败统一触发；参数 success 区分成功与否，message 为结束原因
   */
  const finishCallbacks = new Map<string, Set<(success: boolean, message: string) => void>>()

  // ==================== 内部辅助 ====================

  /** 获取或创建游戏状态 */
  function ensureState(gameId: string): GameDownloadState {
    if (!downloads.value.has(gameId)) {
      downloads.value.set(gameId, {
        gameId,
        gameName: '',
        isMonitoring: false,
        isDownloading: false,
        isPaused: false,
        downloadProgress: createEmptyProgress(),
        downloadLogs: [],
        totalBytes: 0,
        doneBytes: 0,
        speed: 0
      })
    }
    return downloads.value.get(gameId)!
  }

  /**
   * 通知某个游戏的回调表（先快照，回调内部可能移除自身）。
   * 泛型 T 为回调参数列表，配合 finishCallbacks(带参数) 使用类型安全
   */
  function notifyCallbacks<T extends unknown[]>(
    table: Map<string, Set<(...args: T) => void>>,
    gameId: string,
    ...args: T
  ) {
    const callbacks = table.get(gameId)
    if (!callbacks || callbacks.size === 0) return
    for (const cb of Array.from(callbacks)) {
      cb(...args)
    }
  }

  // ==================== 事件桥（主链路） ====================

  /**
   * 惰性注册 Tauri 事件监听（首次监控任意游戏时建桥，全局唯一）。
   * 两个事件：
   * - depot-download-progress → 实时进度；
   * - depot-download-finished → 整场下载结束（成功/取消/失败）。
   */
  async function ensureEventBridge(): Promise<void> {
    if (eventBridged) return
    eventBridged = true

    try {
      progressUnlistens.push(
        await listen<DownloadProgressEvent>('depot-download-progress', (event) => {
          applyEngineProgress(event.payload.gameId, event.payload.progress)
        })
      )
      finishUnlistens.push(
        await listen<DownloadFinishedEvent>('depot-download-finished', (event) => {
          handleEngineFinished(
            event.payload.gameId,
            event.payload.success,
            event.payload.message
          )
        })
      )
    } catch {
      // 建桥失败为极端场景（IPC 初始化异常）：恢复标记允许重试，
      // 进度主链路由快照兜底补齐，不阻塞调用方
      eventBridged = false
    }
  }

  /**
   * 应用一条引擎进度事件：
   * - 整体进度 = 引擎全局字节百分比（doneBytes/totalBytes 为全部活跃 depot 汇总）；
   * - 按引擎下发的 depot 状态（downloading/completed/error）更新该 depot 条目，
   *   并记录说明文字（当前文件 / 完成提示 / 失败错误日志）供 UI 展示；
   * - 整体「完成态」不以任一进度事件为准，等 depot-download-finished(success)
   *   统一判定（引擎最后若干次分块写盘后还要做整文件 SHA-1 校验）。
   */
  function applyEngineProgress(gameId: string, p: EngineDepotProgress) {
    const state = ensureState(gameId)
    const progress = state.downloadProgress

    // 收到进度事件即代表引擎确实在运行：激活"下载中"标记，并清除暂停标记（已恢复）。
    // （GameDetail 只调 startMonitoring 置 isMonitoring，isDownloading 此前仅靠
    // fetchSnapshot 时引擎已 running 才会置位——若快照先于引擎启动执行，
    // isDownloading 恒为 false，底部进度条/下载列表将永不出现，故此处强制激活）
    state.isDownloading = true
    if (state.isPaused) state.isPaused = false

    // 方案 B：跟随引擎真实百分比（允许回退）。引擎在"文件 SHA-1 校验失败 → 回吐字节 → 重下"
    // 时 percent 会低于先前已显示值；若用 Math.max 会把历史高点焊死，正好表现为
    // "速度在走、进度却卡住"的假象。跟随真实值后，重下期间进度条如实回落，一眼可辨。
    progress.overallPercentage = Math.round(p.percent)

    // 字节记录 + 实时速度（滑动窗口 + 低通滤波，压制 CDN 瞬时抖动）：
    // - 滑动窗口保留最近 3 秒样本（比原 1.5s 更稳），用窗口首尾「字节差 / 用时差」
    //   结算瞬时速度；
    // - 再对瞬时值做指数滑动平均（EMA，α=0.3）：显示 = 0.3×瞬时 + 0.7×上次显示，
    //   曲线更平滑——10↔80MB/s 的 CDN 抖动不会直接画出来，整体观感稳定；
    // - 字节回退（SHA-1 校验失败重下）已把 state.speed 归零，重下首帧从 0 平滑爬升。
    const now = Date.now()
    state.doneBytes = p.doneBytes
    state.totalBytes = p.totalBytes
    // 字节回退（SHA-1 校验失败 → 引擎回吐后重下）：清空速度窗口并归零速度。
    // 否则窗口里旧的高字节样本会让速度残留在回吐前的值上，配合回落中的
    // 进度条形成"速度满格、进度倒退"的假象；重下开始后事件重新结算速度。
    const prev = speedWindows.get(gameId)
    if (prev && prev.length > 0 && prev[prev.length - 1].bytes > p.doneBytes) {
      speedWindows.delete(gameId)
      state.speed = 0
    }
    let window = speedWindows.get(gameId) || []
    window.push({ bytes: p.doneBytes, time: now })
    // 剔除过期样本（只保留最近 3 秒）
    window = window.filter((s) => now - s.time <= 3000)
    speedWindows.set(gameId, window)
    // 窗口至少需要 2 个样本才能结算速度（首帧无从差分，速度保持 0 由 UI 兜底占位）
    if (window.length >= 2) {
      const first = window[0]
      const last = window[window.length - 1]
      const deltaBytes = last.bytes - first.bytes
      const deltaMs = last.time - first.time
      if (deltaBytes > 0 && deltaMs > 0) {
        const instant = (deltaBytes * 1000) / deltaMs
        // 指数滑动平均：α=0.3 给瞬时值权重、0.7 保留历史趋势。
        // 旧值为 0（首帧/回退后）时直接用瞬时值起步，避免从 0 缓慢爬升显钝
        state.speed = Math.round(0.3 * instant + 0.7 * (state.speed || instant))
      }
    }

    const depotId = String(p.depotId)
    let depot = progress.depots.find((d) => d.depotId === depotId)
    if (!depot) {
      // 事件先于 initDepots 到达（极端时序）：按需补建条目
      depot = {
        depotId,
        percentage: 0,
        downloadedFiles: 0,
        totalFiles: 0,
        status: 'pending',
        message: ''
      }
      progress.depots.push(depot)
      progress.totalDepots = progress.depots.length
    }

    if (p.status === 'completed') {
      // 完成事件（含断点续传整包跳过补发的完成事件）
      depot.percentage = 100
      depot.status = 'completed'
      depot.message = p.message || 'depot 下载完成'
    } else if (p.status === 'error') {
      // 失败事件：保留当前百分比，记录错误日志（整场即将经 finished 事件中断）。
      // 取消类错误（download-cancelled 等）归一化为"游戏下载已暂停"
      depot.status = 'error'
      depot.message = normalizeEngineMessage(p.message, 'depot 下载失败')
    } else {
      // 进行中：百分比跟随全局真实值（引擎为全局字节模型，单 depot 无独立百分比；
      // 与整体进度同进退——重下导致整体回落时条目一并回落，保持 UI 一致性）
      depot.percentage = Math.round(progress.overallPercentage)
      depot.status = 'downloading'
      depot.message = p.message || p.currentFile || '下载中'
    }

    progress.completedDepots = progress.depots.filter((d) => d.status === 'completed').length
  }

  /**
 * 引擎结束事件处理（成功/暂停/失败统一收口）：
 * - 成功 → 通知完成/进度回调 → markComplete（置 100% 完成态，清监控与回调）；
 * - 暂停（取消类消息）→ 置 isPaused，任务保留在下载中列表显示"已暂停"；
 * - 失败 → 复位下载中/监控标记，任务从活跃列表移除。
 * 结束回调收到的 message 统一经归一化（取消类 → "游戏下载已暂停"）。
 */
function handleEngineFinished(gameId: string, success: boolean, message: string) {
  if (clearedGames.has(gameId)) return
  const state = ensureState(gameId)

  if (success) {
    // 完成回调先行（页面 handleDownloadComplete 内会调用 stopMonitoring 清理回调表）
    notifyCallbacks(completeCallbacks, gameId)
    notifyCallbacks(progressCallbacks, gameId)
    markComplete(gameId)
  } else {
    // RUST 端暂停消息为"下载已暂停"，故取消/暂停一起判定（否则暂停任务会被当作失败移除）
    const isCancelled = /cancel|取消|暂停/i.test(message)
    if (isCancelled) {
      // 暂停：引擎已停，保留条目展示"已暂停"；再次开始下载会清标记并续跑
      state.isPaused = true
    } else {
      state.isPaused = false
    }
    state.isDownloading = false
    state.isMonitoring = false
    // 暂停/结束时清空速度锚点，避免下次续跑后旧窗口干扰速度计算
    speedWindows.delete(gameId)
  }

  // 结束回调（页面可据此处理暂停/失败收尾；消息已归一化）
  notifyCallbacks(finishCallbacks, gameId, success, normalizeEngineMessage(message, '下载失败'))
}

  // ==================== Getters ====================

  function isGameMonitoring(gameId: string): boolean {
    return downloads.value.get(gameId)?.isMonitoring ?? false
  }

  function isGameDownloading(gameId: string): boolean {
    return downloads.value.get(gameId)?.isDownloading ?? false
  }

  /** 该游戏是否处于暂停态（详情页据此把按钮切换为「继续下载」） */
  function isGamePaused(gameId: string): boolean {
    return downloads.value.get(gameId)?.isPaused ?? false
  }

  /** 该游戏的实时下载速度（字节/秒；进度组件头部展示） */
  function getGameSpeed(gameId: string): number {
    return downloads.value.get(gameId)?.speed ?? 0
  }

  /** 该游戏本次下载的已下载/总字节数（供进度组件头部估算剩余时间） */
  function getGameBytes(gameId: string): { doneBytes: number; totalBytes: number } {
    const state = downloads.value.get(gameId)
    return { doneBytes: state?.doneBytes ?? 0, totalBytes: state?.totalBytes ?? 0 }
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

  /**
   * 活跃下载任务列表（供下载列表页"下载中"Tab 使用）：
   * 正在进行中的任务 + 已暂停的任务（显示"已暂停"）。
   * 保持 Map 插入顺序（开始监控的先后）。
   */
  const activeDownloads = computed<GameDownloadState[]>(() => {
    const result: GameDownloadState[] = []
    downloads.value.forEach((state) => {
      if ((state.isDownloading || state.isPaused) && !state.downloadProgress.isComplete) {
        result.push(state)
      }
    })
    return result
  })

  /** 真正正在下载中的任务数（暂停任务不计入，供底部条"等 N 个任务"文案） */
  const downloadingCount = computed(() =>
    activeDownloads.value.filter((s) => s.isDownloading).length
  )

  /**
   * 已完成下载列表（当前会话完成态 + localStorage 持久化历史，去重），
   * 按完成时间倒序，供下载列表页"已完成"Tab 使用。
   */
  const completedDownloads = computed(() => {
    const merged = new Map<
      string,
      { gameName: string; completedAt: number; totalBytes: number }
    >()
    // 1. 本会话已完成（store 内存态，含刚完成的最新记录）
    downloads.value.forEach((s, gameId) => {
      if (s.downloadProgress.isComplete && s.completedAt) {
        merged.set(gameId, {
          gameName: s.gameName || gameId,
          completedAt: s.completedAt,
          totalBytes: s.doneBytes || s.totalBytes
        })
      }
    })
    // 2. 跨会话历史（不去重覆盖内存态，保证取最新 gameName）
    for (const h of historyList.value) {
      merged.set(h.gameId, { gameName: h.gameName, completedAt: h.completedAt, totalBytes: h.totalBytes })
    }
    return Array.from(merged.entries())
      .map(([gameId, v]) => ({ gameId, ...v }))
      .sort((a, b) => b.completedAt - a.completedAt)
  })

  /**
   * 底部进度条主下载：正在下载（未暂停）列表中进度最小的那个（最需要用户关注），
   * 无正在下载的任务时返回 null（组件隐藏/显示下载记录入口）。
   * 暂停的任务不计入（不显示"正在下载中"）。
   */
  const primaryDownload = computed<GameDownloadState | null>(() => {
    const actives = activeDownloads.value.filter((s) => s.isDownloading)
    if (actives.length === 0) return null
    let min = actives[0]
    for (let i = 1; i < actives.length; i++) {
      if (actives[i].downloadProgress.overallPercentage < min.downloadProgress.overallPercentage) {
        min = actives[i]
      }
    }
    return min
  })

  /** 全部下载中游戏的加权平均进度（多个并发任务时底部条/列表头部展示）
 * 字节模型优先；ddv20 模式不提供字节（done/total 恒为 0），退化为各任务
 * 整体百分比的简单平均，保证底部条/列表仍有总进度可看 */
  const globalPercent = computed(() => {
    const actives = activeDownloads.value
    if (actives.length === 0) return 0
    const totalDone = actives.reduce((sum, s) => sum + s.doneBytes, 0)
    const totalAll = actives.reduce((sum, s) => sum + (s.totalBytes || 0), 0)
    if (totalAll > 0) {
      return Math.round((totalDone / totalAll) * 100)
    }
    const sumPct = actives.reduce(
      (sum, s) => sum + s.downloadProgress.overallPercentage,
      0
    )
    return Math.round(sumPct / actives.length)
  })

  // ==================== Actions ====================

  function setGameName(gameId: string, gameName: string) {
    ensureState(gameId).gameName = gameName
  }

  /**
   * 解析并回填游戏显示名（「中文名 + 英文名」，已入库 → 游戏库配置 → 兜底 ID）。
   * 始终以 resolveGameName 的组合结果为准——即使此前 setGameName 已写入（如详情页
   * 只传了英文名），此处也会被正确的中英文组合覆盖，保证底部下载条/列表展示统一。
   */
  async function ensureGameName(gameId: string): Promise<string> {
    const state = ensureState(gameId)
    const name = await resolveGameName(gameId)
    if (name) {
      state.gameName = name
    }
    return state.gameName
  }

  function setGameDownloading(gameId: string, downloading: boolean) {
    ensureState(gameId).isDownloading = downloading
  }

  /**
   * 初始化 depot 列表（原地修改，不创建新 downloadProgress 对象）。
   * 用 Set 对 depotIds 去重：暂停后续传时引擎可能重新上报同一批 depot，
   * 同一 depot 的多个清单文件也会解析出重复 id，前台每次只下载/展示一份
   * （用户确认后台仅实际下载一个，这里保证前端不出现重复词条）。
   */
  function initDepots(gameId: string, depotIds: string[]) {
    const state = ensureState(gameId)
    const progress = state.downloadProgress
    const uniqueIds = Array.from(new Set(depotIds))
    progress.totalDepots = uniqueIds.length
    progress.depots = uniqueIds.map((depotId) => ({
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
   * 保留供页面/外部直接设置特定 depot 状态使用
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
    const depotIndex = depots.findIndex((d) => d.depotId === depotId)

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
    progress.completedDepots = depots.filter((d) => d.status === 'completed').length
    progress.overallPercentage = depots.length > 0
      ? Math.round(depots.reduce((sum, d) => sum + d.percentage, 0) / depots.length)
      : 0
    progress.isComplete = depots.length > 0 && depots.every((d) => d.status === 'completed')
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
   * 快照兜底：拉取引擎当前运行状态，恢复进行中下载的进度展示。
   * 页面刷新/进入时在 startProgressMonitoring 中调用一次；
   * 事件主链路正常时不需重复调用。
   */
  async function fetchSnapshot(gameId: string): Promise<void> {
    try {
      const status = await invoke<EngineStatus>('get_download_status', { gameId })

      const state = ensureState(gameId)
      if (status.running) {
        // 引擎仍在跑：用快照进度覆盖展示（事件桥未注册期间错过的进度由此补齐）
        state.isDownloading = true
        state.isMonitoring = true
        for (const p of status.depots) {
          applyEngineProgress(gameId, p)
        }
        await ensureEventBridge()
      } else {
        // 引擎已结束：若此前已完整下载（如完成时正刷新页面）则保留完成态，
        // 否则检查 game.json 是否为暂停状态（刷新后仍保留"已暂停"展示）
        const progress = state.downloadProgress
        if (progress.depots.length > 0 && progress.depots.every((d) => d.status === 'completed')) {
          if (!progress.isComplete) progress.isComplete = true
        } else {
          state.isMonitoring = false
          state.isDownloading = false
          try {
            const data = await invoke<{ download_status?: string } | null>('get_game_data', {
              gameId
            })
            state.isPaused = data?.download_status === 'paused'
          } catch {
            state.isPaused = false
          }
        }
      }
    } catch {
      // 快照失败不打断页面：事件主链路仍可用，等待事件更新即可
    }
  }

  /**
   * 开始监控指定游戏：
   * 建桥订阅引擎事件（全局仅一次）并置监控标记，供页面据此渲染“下载中”态。
   */
  async function startMonitoring(gameId: string) {
    ensureState(gameId).isMonitoring = true
    await ensureEventBridge()
  }

  /** 停止指定游戏的监控并清理其回调表 */
  function stopMonitoring(gameId: string) {
    completeCallbacks.delete(gameId)
    progressCallbacks.delete(gameId)
    finishCallbacks.delete(gameId)
    const state = downloads.value.get(gameId)
    if (state) state.isMonitoring = false
  }

  /** 注册某个游戏的下载完成回调 */
  function onComplete(gameId: string, callback: () => void) {
    registerCallback(completeCallbacks, gameId, callback)
  }

  /** 注册某个游戏的进度变化回调（每次进度事件都会触发） */
  function onProgress(gameId: string, callback: () => void) {
    registerCallback(progressCallbacks, gameId, callback)
  }

  /** 注册某个游戏的下载结束回调（成功/取消/失败统一触发） */
  function onFinish(gameId: string, callback: (success: boolean, message: string) => void) {
    registerCallback(finishCallbacks, gameId, callback)
  }

  /** 向指定回调表注册回调 */
  function registerCallback<T extends (...args: never[]) => void>(
    table: Map<string, Set<T>>,
    gameId: string,
    callback: T
  ) {
    if (!table.has(gameId)) {
      table.set(gameId, new Set())
    }
    table.get(gameId)!.add(callback)
  }

  /** 标记下载完成（引擎结束事件成功时调用） */
  function markComplete(gameId: string) {
    const state = ensureState(gameId)
    state.isDownloading = false
    state.isMonitoring = false
    state.isPaused = false
    const progress = state.downloadProgress
    progress.isComplete = true
    progress.overallPercentage = 100
    progress.completedDepots = progress.totalDepots
    progress.depots.forEach((depot) => {
      depot.percentage = 100
      depot.status = 'completed'
    })
    // 记录完成时间，并持久化到已完成的本地历史（保留最近 50 条）
    state.completedAt = Date.now()
    state.speed = 0
    const item: DownloadHistoryItem = {
      gameId,
      gameName: state.gameName || gameId,
      completedAt: state.completedAt,
      totalBytes: state.doneBytes || state.totalBytes
    }
    // 本地镜像置顶（去重截断），并写入 game.json（后端按 game_id 去重，保留 50 条）
    const rest = historyList.value.filter((h) => h.gameId !== gameId)
    historyList.value = [item, ...rest].slice(0, 50)
    void invoke('append_download_history', { entries: [item] }).catch(() => {})
    // 清空速度锚点，避免同一 session 二次下载时首帧速度沿用旧值
    speedWindows.delete(gameId)
    stopMonitoring(gameId)
  }

  /** 重置指定游戏 */
  function resetGame(gameId: string) {
    stopMonitoring(gameId)
    downloads.value.delete(gameId)
  }

  /**
   * 清除下载中任务（下载记录页"下载中"Tab 一键/多选清除）。
   * - 把游戏加入 clearedGames，忽略其后续引擎事件（stop 引发的 finished 不会重建条目）；
   * - 从 store 移除条目并清理回调。
   * 不在此处理：引擎停止 / 游戏文件删除 / game.json 数据移除（由调用方页面执行）。
   */
  function clearActiveGames(gameIds: string[]): void {
    for (const id of gameIds) {
      clearedGames.add(id)
      resetGame(id)
    }
  }

  /**
   * 移除完成记录（下载列表"已完成"Tab 一键清除 / 多选清除）。
   * - 从本地镜像剔除对应条目；
   * - 同步移除 game.json 中的对应历史（持久化记录一并删除）。
   * 注意：仅清记录，不删除游戏文件（文件删除由调用方按需执行 delete_game_directory）。
   */
  function removeHistory(gameIds: string[]): void {
    const set = new Set(gameIds)
    historyList.value = historyList.value.filter((h) => !set.has(h.gameId))
    for (const id of set) {
      const st = downloads.value.get(id)
      if (st && st.downloadProgress.isComplete) {
        downloads.value.delete(id)
      }
    }
    void invoke('remove_download_history', { gameIds }).catch(() => {})
  }

  /**
   * 从 game.json 恢复所有「已暂停」任务（程序重启/页面刷新后仍可见）。
   * 进度百分比来自 game.json 持久化的 download_progress；depot 明细引擎不落库，
   * 因此恢复后仅展示总进度与"已暂停"标签（引擎续跑后由进度事件补齐明细）。
   * 幂等：本进程内只执行一次。
   */
  let pausedRestored = false
  async function restorePausedGames(): Promise<void> {
    if (pausedRestored) return
    pausedRestored = true
    try {
      const games = await invoke<
        Array<{
          game_id?: string
          download_status?: string
          download_progress?: number
          game_name?: string
          chinese_name?: string
        }>
      >('get_all_games_data')
      for (const g of games) {
        if (g.download_status !== 'paused' || !g.game_id) continue
        const state = ensureState(g.game_id)
        state.isPaused = true
        state.isDownloading = false
        state.isMonitoring = false
        state.gameName = g.chinese_name || g.game_name || g.game_id
        const progress = state.downloadProgress
        progress.isComplete = false
        progress.overallPercentage = Math.max(
          progress.overallPercentage,
          g.download_progress ?? 0
        )
      }
    } catch {
      // 恢复失败静默（首次进入相关页面时会再次尝试），不阻塞界面
    }
  }

  /**
   * 初始化已完成下载历史：从 game.json 读取（不随系统缓存清理）。
   * 若后端为空而旧 localStorage 有记录，则一次性迁移到 game.json 后清除旧缓存。
   * 幂等：本进程内只执行一次。
   */
  let historyLoaded = false
  async function initHistory(): Promise<void> {
    if (historyLoaded) return
    historyLoaded = true
    try {
      const rows = await invoke<DownloadHistoryItem[] | null>('get_download_history')
      let list = Array.isArray(rows) ? rows : []
      // 旧 localStorage 数据迁移（仅当后端尚无记录时）
      const legacy = loadHistory()
      if (list.length === 0 && legacy.length > 0) {
        await invoke('append_download_history', { entries: legacy }).catch(() => {})
        list = legacy
      }
      if (legacy.length > 0) {
        localStorage.removeItem(HISTORY_KEY)
      }
      historyList.value = list
    } catch {
      // 加载失败静默（下次进入下载记录页可重试），不阻塞界面
      historyLoaded = false
    }
  }

  return {
    downloads,
    isGameMonitoring,
    isGameDownloading,
    isGamePaused,
    getGameSpeed,
    getGameBytes,
    getGameProgress,
    getGameLogs,
    activeDownloadGameIds,
    activeDownloads,
    downloadingCount,
    completedDownloads,
    primaryDownload,
    globalPercent,
    setGameName,
    ensureGameName,
    setGameDownloading,
    initDepots,
    updateDepotProgress,
    fetchSnapshot,
    updateProgress,
    addLog,
    clearLogs,
    startMonitoring,
    stopMonitoring,
    onComplete,
    onProgress,
    onFinish,
    markComplete,
    resetGame,
    clearActiveGames,
    removeHistory,
    restorePausedGames,
    initHistory
  }
})