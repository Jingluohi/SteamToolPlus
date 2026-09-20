<template>
  <!-- 下载列表页（仿 Steam 下载界面）：
       底部进度条点击进入；分「下载中 / 已完成」两个模块，Tab 切换 -->
  <div class="downloads-view">
    <!-- 页面标题 -->
    <div class="page-header">
      <h1 class="page-title">下载</h1>
      <p class="page-desc">管理所有游戏的下载任务与历史记录</p>
    </div>

    <!-- Tab 切换：下载中 / 已完成 -->
    <div class="dl-tabs">
      <button
        class="dl-tab"
        :class="{ active: activeTab === 'active' }"
        @click="activeTab = 'active'"
      >
        下载中
        <span class="tab-count" :class="tabType(activeTab)">{{ activeDownloads.length }}</span>
      </button>
      <button
        class="dl-tab"
        :class="{ active: activeTab === 'completed' }"
        @click="activeTab = 'completed'"
      >
        已完成
        <span class="tab-count">{{ completedDownloads.length }}</span>
      </button>
    </div>

    <!-- 操作栏：一键清除 / 多选（常驻；按当前 Tab 作用于下载中任务或已完成记录） -->
    <div class="dl-toolbar">
      <template v-if="!multiSelectMode">
        <button
          class="ui-btn ui-btn-sm ui-btn-outline"
          :disabled="(activeTab === 'active' ? activeDownloads.length : completedDownloads.length) === 0"
          @click="openClearAll"
        >
          一键清除
        </button>
        <button
          class="ui-btn ui-btn-sm ui-btn-outline"
          :disabled="(activeTab === 'active' ? activeDownloads.length : completedDownloads.length) === 0"
          @click="enterMultiSelect"
        >
          多选
        </button>
      </template>
      <template v-else>
        <span class="multi-hint">已选 {{ selectedCount }} 项</span>
        <button
          class="ui-btn ui-btn-sm ui-btn-danger"
          :disabled="selectedCount === 0"
          @click="openClearSelected"
        >
          清除选中
        </button>
        <button class="ui-btn ui-btn-sm ui-btn-ghost" @click="exitMultiSelect">
          取消
        </button>
      </template>
      <span class="toolbar-count">
        {{ activeTab === 'active'
          ? `下载中 ${activeDownloads.length} 条`
          : `共 ${completedDownloads.length} 条` }}
      </span>
    </div>

    <!-- 下载中列表 -->
    <div v-if="activeTab === 'active'" class="dl-list">
      <div
        v-for="d in activeDownloads"
        :key="d.gameId"
        class="dl-item"
        :class="{ 'is-selected': selectedIds.has(d.gameId) }"
        @click="rowClick(d.gameId)"
      >
        <!-- 多选模式勾选框 -->
        <input
          v-if="multiSelectMode"
          type="checkbox"
          class="dl-check"
          :checked="selectedIds.has(d.gameId)"
          @click.stop
          @change="toggleSelect(d.gameId)"
        />

        <!-- 封面 -->
        <img
          v-if="coverMap.get(d.gameId)"
          :src="coverMap.get(d.gameId)"
          class="dl-cover"
          alt=""
          draggable="false"
        />
        <div v-else class="dl-cover dl-cover-placeholder" />

        <!-- 信息主体 -->
        <div class="dl-body">
          <div class="dl-row">
            <span class="dl-name">{{ d.gameName || d.gameId }}</span>
            <span
              class="dl-percent"
              :class="{ 'dl-percent-paused': d.isPaused }"
            >
              {{ d.isPaused ? '已暂停' : Math.round(d.downloadProgress.overallPercentage) + '%' }}
            </span>
          </div>
          <div class="dl-progress-bg">
            <div
              class="dl-progress-fill"
              :style="{ width: Math.min(100, d.downloadProgress.overallPercentage) + '%' }"
            />
          </div>
          <div class="dl-row dl-sub">
            <span class="dl-meta">
              <template v-if="d.isPaused">
                已暂停，点击右侧「继续下载」即可断点续传
              </template>
              <template v-else>
                {{ d.speed > 0 ? formatSpeed(d.speed) : '获取进度中…' }}
                <template v-if="d.speed > 0">
                  · {{ remainingText(d) }}
                </template>
                <template v-if="hasCurrentFile(d)">
                  · {{ currentFileText(d) }}
                </template>
              </template>
            </span>
            <span class="dl-meta">{{ bytesText(d.doneBytes) }} / {{ bytesText(d.totalBytes) }}</span>
          </div>
        </div>

        <!-- 操作 -->
        <div class="dl-actions" @click.stop>
          <!-- 暂停态 → 继续下载（复用引擎断点续传，只补未完成的块） -->
          <button
            v-if="d.isPaused"
            class="ui-btn ui-btn-sm ui-btn-primary"
            :class="{ loading: resumingId === d.gameId }"
            :disabled="resumingId !== ''"
            title="继续下载"
            @click="continueDownload(d)"
          >
            <svg v-if="resumingId === d.gameId" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
            </svg>
            <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M5 20h14M12 4v12m0 0l-5-5m5 5l5-5"/>
            </svg>
            继续下载
          </button>
          <!-- 下载中 → 暂停 -->
          <button
            v-else
            class="ui-btn ui-btn-sm ui-btn-outline"
            title="暂停下载"
            @click="pauseDownload(d.gameId)"
          >
            <svg viewBox="0 0 24 24" fill="currentColor">
              <rect x="6" y="4" width="4" height="16" rx="1" />
              <rect x="14" y="4" width="4" height="16" rx="1" />
            </svg>
            暂停
          </button>
        </div>
      </div>

      <!-- 空态 -->
      <div v-if="activeDownloads.length === 0" class="dl-empty">
        <svg class="empty-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
          <polyline points="7 10 12 15 17 10" />
          <line x1="12" y1="15" x2="12" y2="3" />
        </svg>
        <p class="empty-text">暂无下载任务</p>
        <p class="empty-sub">从游戏详情页发起下载后，可在此查看进度</p>
      </div>
    </div>

    <!-- 已完成列表 -->
    <div v-else class="dl-list">
      <div
        v-for="item in completedDownloads"
        :key="item.gameId"
        class="dl-item"
        :class="{ 'is-selected': selectedIds.has(item.gameId) }"
        @click="rowClick(item.gameId)"
      >
        <!-- 多选模式勾选框 -->
        <input
          v-if="multiSelectMode"
          type="checkbox"
          class="dl-check"
          :checked="selectedIds.has(item.gameId)"
          @click.stop
          @change="toggleSelect(item.gameId)"
        />

        <!-- 封面 -->
        <img
          v-if="coverMap.get(item.gameId)"
          :src="coverMap.get(item.gameId)"
          class="dl-cover"
          alt=""
          draggable="false"
        />
        <div v-else class="dl-cover dl-cover-placeholder" />

        <!-- 信息主体 -->
        <div class="dl-body">
          <div class="dl-row">
            <span class="dl-name">{{ item.gameName || item.gameId }}</span>
            <span class="dl-percent dl-percent-done">已完成</span>
          </div>
          <div class="dl-progress-bg">
            <div class="dl-progress-fill dl-progress-done" style="width: 100%" />
          </div>
          <div class="dl-row dl-sub">
            <span class="dl-meta">
              <svg class="meta-icon" viewBox="0 0 24 24" fill="currentColor">
                <path d="M9 16.17 4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z" />
              </svg>
              下载完成
            </span>
            <span class="dl-meta">
              {{ item.totalBytes > 0 ? bytesText(item.totalBytes) : '—' }} ·
              {{ formatTime(item.completedAt) }}
            </span>
          </div>
        </div>
      </div>

      <!-- 空态 -->
      <div v-if="completedDownloads.length === 0" class="dl-empty">
        <svg class="empty-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <polyline points="20 6 9 17 4 12" />
        </svg>
        <p class="empty-text">暂无已完成下载</p>
        <p class="empty-sub">下载完成的游戏会记录在这里</p>
      </div>
    </div>

    <!-- 清除确认弹窗（下载中任务 / 已完成记录共用；
         下载中默认勾选"同时删除游戏文件"，已完成默认不勾选） -->
    <BaseModal
      v-model="showClearDialog"
      :title="clearMode === 'active' ? '清除下载任务' : '清除下载记录'"
      width="420px"
      :show-cancel="true"
      cancel-text="取消"
      :show-confirm="true"
      confirm-text="清除"
      confirm-variant="danger"
      @confirm="confirmClear"
      @close="showClearDialog = false"
    >
      <template #body>
        <div class="clear-dialog-body">
          <p class="clear-hint">
            {{ clearMode === 'active'
              ? `确定要清除 ${clearCount} 个下载任务吗？清除后下载进度将丢失。`
              : `确定要清除 ${clearCount} 条下载记录吗？清除后无法恢复。` }}
          </p>
          <label class="clear-delete-option">
            <input type="checkbox" v-model="deleteFilesChecked" />
            <span>同时删除游戏文件</span>
          </label>
        </div>
      </template>
    </BaseModal>
  </div>
</template>

<script setup lang="ts">
/**
 * Downloads.vue - 下载列表页（仿 Steam 下载界面）
 *
 * 两个模块：
 * - 下载中：活跃下载任务列表（进度 / 速度 / 字节 / 当前文件 / 暂停）；
 * - 已完成：完成历史（本会话 + localStorage 持久化），点击跳游戏详情。
 *
 * 数据全部来自 downloadManager store（事件驱动），无需轮询。
 */

import { computed, onMounted, onBeforeUnmount, ref, reactive, watch } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import {
  useDownloadManagerStore,
  type GameDownloadState
} from '../../store/downloadManager.store'
import { useConfigStore } from '../../store/config.store'
import { getCachedCoverImage } from '../../services/imageCache.service'
import { getGameData, updateGameDownloadStatus } from '../../api/gameData.api'
import { formatSpeed, bytesText, formatRemaining } from '../../utils/downloadFormat'
import BaseModal from '../../components/common/BaseModal.vue'

const router = useRouter()
const downloadStore = useDownloadManagerStore()
const configStore = useConfigStore()

/** 当前 Tab：active=下载中 / completed=已完成 */
const activeTab = ref<'active' | 'completed'>('active')

/** 正在发起"继续下载"的游戏 ID（按钮 loading 态） */
const resumingId = ref('')

// ==================== 清除记录（一键清除 / 多选） ====================

/** 清除确认弹窗显隐 */
const showClearDialog = ref(false)
/** 待清除的游戏 ID 列表 */
const clearIds = ref<string[]>([])
/** 待清除记录数量（弹窗文案） */
const clearCount = ref(0)
/** 清除来源：active=下载中任务 / completed=已完成记录 */
const clearMode = ref<'active' | 'completed'>('completed')
/** 是否同时删除游戏文件（下载中清除默认勾选） */
const deleteFilesChecked = ref(false)

/** 多选模式 */
const multiSelectMode = ref(false)
/** 已勾选的游戏 ID 集合 */
const selectedIds = ref<Set<string>>(new Set())
/** 已选数量 */
const selectedCount = computed(() => selectedIds.value.size)

/** 打开清除确认弹窗（一键清除 / 多选清除共用）。mode 决定文案与"删除文件"默认勾选 */
function openClearDialog(ids: string[], count: number, mode: 'active' | 'completed') {
  clearMode.value = mode
  clearIds.value = ids
  clearCount.value = count
  // 下载中清除默认勾选删除文件；已完成清除默认不勾选
  deleteFilesChecked.value = mode === 'active'
  showClearDialog.value = true
}

/** 一键清除：按当前 Tab 选择清除目标（自动切到对应 Tab） */
function openClearAll() {
  if (activeTab.value === 'active') {
    const ids = downloadStore.activeDownloads.map((d) => d.gameId)
    openClearDialog(ids, ids.length, 'active')
  } else {
    const ids = completedDownloads.value.map((c) => c.gameId)
    openClearDialog(ids, ids.length, 'completed')
  }
}

/** 清除选中：按当前 Tab 的模式清除多选勾选项 */
function openClearSelected() {
  const ids = [...selectedIds.value]
  if (ids.length === 0) return
  openClearDialog(ids, ids.length, activeTab.value)
}

/** 进入多选模式（保持当前 Tab，勾选当前 Tab 的列表） */
function enterMultiSelect() {
  multiSelectMode.value = true
  selectedIds.value = new Set()
}

/** 退出多选模式 */
function exitMultiSelect() {
  multiSelectMode.value = false
  selectedIds.value = new Set()
}

/** 勾选/取消勾选某个记录 */
function toggleSelect(id: string) {
  const next = new Set(selectedIds.value)
  if (next.has(id)) {
    next.delete(id)
  } else {
    next.add(id)
  }
  selectedIds.value = next
}

/** 记录条目点击：多选模式切换勾选，否则跳转游戏详情 */
function rowClick(id: string) {
  if (multiSelectMode.value) {
    toggleSelect(id)
  } else {
    goToGame(id)
  }
}

/**
 * 确认清除：
 * - active（下载中任务）：先停引擎 →（勾选时）删文件 → 清除 store 任务 + 移除
 *   game.json 游戏数据与历史，进度不再保留；
 * - completed（已完成记录）：（勾选时）删文件 → 移除历史记录。
 */
async function confirmClear() {
  const ids = [...clearIds.value]
  const mode = clearMode.value
  showClearDialog.value = false

  if (mode === 'active') {
    await clearActiveTasks(ids, deleteFilesChecked.value)
  } else {
    if (deleteFilesChecked.value) {
      await deleteGameFiles(ids)
    }
    downloadStore.removeHistory(ids)
  }

  if (multiSelectMode.value) {
    exitMultiSelect()
  }
}

/**
 * 清除下载中任务：
 * 1. 停止仍在下行的引擎（stop 事件会被 store 的 clearedGames 忽略，不会重建条目）；
 * 2. （勾选时）删除已下载的游戏文件；
 * 3. 从 store 移除任务条目，并移除 game.json 中的游戏数据与完成历史。
 */
async function clearActiveTasks(ids: string[], deleteFiles: boolean) {
  // 1. 停止仍在下载的任务（批量并行）
  const running = ids.filter((id) => {
    const st = downloadStore.downloads.get(id)
    return st?.isDownloading ?? false
  })
  await Promise.all(
    running.map((id) =>
      invoke('stop_download', { gameId: id }).catch(() => {})
    )
  )

  // 2. 可选删除游戏文件（在移除 game.json 数据前读取路径）
  let paths: Record<string, string> = {}
  if (deleteFiles) {
    for (const id of ids) {
      try {
        const data = await getGameData(id)
        const p = data?.download_path || data?.install_path || ''
        if (p) paths[id] = p
      } catch {
        // 路径读取失败则跳过该游戏的删文件
      }
    }
    await Promise.all(
      Object.values(paths).map((p) =>
        invoke('delete_game_directory', { path: p }).catch(() => {})
      )
    )
  }

  // 3. 移除 store 任务条目（clearedGames 防事件重建）
  downloadStore.clearActiveGames(ids)
  // 4. 移除 game.json 游戏数据与历史（防止重启后 restorePausedGames 复活）
  await Promise.all(
    ids.map((id) => invoke('remove_game_data', { gameId: id }).catch(() => {}))
  )
  downloadStore.removeHistory(ids)
}

/** 逐条删除游戏文件目录（路径取自 game.json，单条失败不阻断其余） */
async function deleteGameFiles(ids: string[]) {
  for (const id of ids) {
    try {
      const data = await getGameData(id)
      const path = data?.download_path || data?.install_path || ''
      if (path) {
        await invoke('delete_game_directory', { path, gameId: id })
      }
    } catch {
      // 单个删除失败静默，不影响其余记录的清除
    }
  }
}

// ==================== 数据 ====================

/** 下载中列表 */
const activeDownloads = computed(() => downloadStore.activeDownloads)

/** 已完成列表 */
const completedDownloads = computed(() => downloadStore.completedDownloads)

/** 封面缓存 Map<gameId, url>（列表项共用，避免重复拉取） */
const coverMap = reactive(new Map<string, string>())

/**
 * Tab 数量徽章类型由当前 Tab 状态决定：
 * 下载中 Tab 激活时徽章用强调色高亮；已完成 Tab 用普通灰
 */
function tabType(_current: string) {
  return activeTab.value === 'active' ? 'hot' : ''
}

// ==================== 封面加载 ====================

async function loadCover(gameId: string) {
  if (coverMap.has(gameId)) return
  try {
    const url = await getCachedCoverImage(gameId)
    if (url) {
      coverMap.set(gameId, url)
      return
    }
  } catch {
    // 封面缺失静默，显示占位
  }
}

function loadAllCovers() {
  const pendingIds = new Set<string>()
  for (const d of downloadStore.activeDownloads) {
    if (!coverMap.has(d.gameId)) pendingIds.add(d.gameId)
  }
  for (const item of downloadStore.completedDownloads) {
    if (!coverMap.has(item.gameId)) pendingIds.add(item.gameId)
  }
  pendingIds.forEach((id) => void loadCover(id))
}

// 列表变化时补加载新出现的封面（coverMap 已有则跳过）
function watchList() {
  loadAllCovers()
  // 解析缺失的游戏名
  for (const d of downloadStore.activeDownloads) {
    if (!d.gameName) void downloadStore.ensureGameName(d.gameId)
  }
}

// ==================== 交互 ====================

/** 跳转到游戏详情页 */
function goToGame(gameId: string) {
  router.push(`/game/${gameId}`)
}

/** 暂停指定下载任务 */
function pauseDownload(gameId: string) {
  invoke('stop_download', { gameId }).catch(() => {
    // 暂停失败静默（引擎可能已结束，列表会自动移除）
  })
}

/**
 * 继续下载（断点续传）：
 * 读取该游戏的清单路径（resources/manifest/{gameId}）与下载路径（game.json），
 * 重新发起引擎下载——引擎会跳过已完整落盘的块（SHA-1 校验），只补未完成部分。
 * 成功后在 store 中恢复运行态；失败回退为暂停态。
 */
async function continueDownload(d: GameDownloadState) {
  const gameId = d.gameId
  if (resumingId.value) return // 已有任务在恢复中，避免并发
  resumingId.value = gameId

  // 读取清单路径与下载路径
  let manifestPath = ''
  let downloadPath = ''
  try {
    manifestPath = await invoke<string>('get_manifest_path', { gameId })
    const data = await getGameData(gameId)
    downloadPath = data?.download_path || ''
  } catch {
    manifestPath = ''
  }

  if (!manifestPath || !downloadPath) {
    resumingId.value = ''
    alert(manifestPath
      ? '未找到该游戏的下载路径（game.json），请前往游戏详情页重新开始下载'
      : '未找到该游戏的清单文件，请前往游戏详情页导入清单后再下载')
    return
  }

  // 立即切回"下载中"展示，并把 game.json 状态置为 downloading
  //（避免后续 fetchSnapshot 读仍为 paused 而把任务打回暂停态）
  d.isPaused = false
  d.isDownloading = true
  d.isMonitoring = true
  const resumeProgress = Math.round(d.downloadProgress.overallPercentage)
  await updateGameDownloadStatus(gameId, 'downloading', resumeProgress)

  try {
    const result = await invoke<{ success: boolean; message: string }>(
      'start_game_download',
      {
        manifestPath,
        downloadPath,
        gameId,
        // 继续下载沿用启动时的引擎模式（默认 ddv20.exe）
        useEngine: configStore.config?.launch?.useNewEngine ?? false
      }
    )
    if (result.success) {
      // 恢复事件桥 + 拉取引擎当前进度（引擎刚启动可能尚未 running，事件会随后补齐）
      await downloadStore.fetchSnapshot(gameId)
      await downloadStore.startMonitoring(gameId)
    } else {
      // 启动失败：回退到暂停态展示，保留用户可见的进度
      d.isPaused = true
      d.isDownloading = false
      d.isMonitoring = false
      alert(`继续下载启动失败: ${result.message}`)
    }
  } catch (error) {
    d.isPaused = true
    d.isDownloading = false
    d.isMonitoring = false
    alert(`继续下载启动失败: ${error}`)
  } finally {
    resumingId.value = ''
  }
}

// ==================== 格式化 ====================
// 速度 / 字节 / 剩余时间格式化统一使用公共工具 utils/downloadFormat
// （与游戏详情页、下载进度组件共用，避免重复实现）

/**
 * 剩余下载时间（封装公共工具，入参为任务状态）
 */
function remainingText(d: GameDownloadState): string {
  return formatRemaining(d.speed, d.doneBytes, d.totalBytes)
}

/** 时间戳 → 可读时间文本 */
function formatTime(ts: number): string {
  const d = new Date(ts)
  const pad = (n: number) => String(n).padStart(2, '0')
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}`
}

/** 是否有当前文件信息（取 depots 中状态为 downloading 的最新 message） */
function hasCurrentFile(d: GameDownloadState): boolean {
  return currentFileText(d).length > 0
}

/** 当前正在下载文件的说明文字（取进行中 depot 的 message / currentFile 语义） */
function currentFileText(d: GameDownloadState): string {
  const depot = d.downloadProgress.depots.find((x) => x.status === 'downloading')
  return depot?.message || ''
}

// ==================== 生命周期 ====================

let refreshTimer: ReturnType<typeof setInterval> | null = null

onMounted(async () => {
  // 恢复上次会话遗留的「已暂停」任务（程序重启/页面刷新后仍可见）
  await downloadStore.restorePausedGames()
  // 加载已完成历史（持久化于 game.json，不随系统缓存清理）
  await downloadStore.initHistory()
  watchList()
  // 封面/名称兜底定时刷新：下载过程中新条目出现时自动补加载
  refreshTimer = setInterval(watchList, 2000)
})

// 切换 Tab 时退出多选模式（勾选集合仅对当前 Tab 有效）
watch(activeTab, () => {
  if (multiSelectMode.value) exitMultiSelect()
})

onBeforeUnmount(() => {
  if (refreshTimer) clearInterval(refreshTimer)
  coverMap.clear()
})
</script>

<style scoped>
/* 页面布局（与 Browse 等页面保持一致：header + 内容区） */
.downloads-view {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 20px 24px 16px;
  box-sizing: border-box;
}

.page-header {
  flex-shrink: 0;
  margin-bottom: 16px;
}

.page-title {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.page-desc {
  margin: 4px 0 0;
  font-size: 13px;
  color: var(--steam-text-muted);
}

/* Tab 切换 */
.dl-tabs {
  display: flex;
  gap: 8px;
  flex-shrink: 0;
  margin-bottom: 14px;
  border-bottom: 1px solid var(--steam-border);
  padding-bottom: 10px;
}

.dl-tab {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 16px;
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-secondary);
  background: transparent;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: background-color 0.15s ease, color 0.15s ease;
}

.dl-tab:hover {
  background: var(--steam-accent-hover);
}

.dl-tab.active {
  color: var(--steam-text-primary);
  background: var(--steam-bg-tertiary);
}

/* 数量徽章 */
.tab-count {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 20px;
  height: 18px;
  padding: 0 6px;
  font-size: 12px;
  line-height: 18px;
  border-radius: 9px;
  background: var(--steam-bg-quaternary);
  color: var(--steam-text-secondary);
  font-variant-numeric: tabular-nums;
}

.tab-count.hot {
  background: var(--steam-accent-blue);
  color: #fff;
}

/* 列表容器 */
.dl-list {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding-right: 6px;
}

/* 已完成操作栏 */
.dl-toolbar {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
  margin: 0 0 10px;
}

.multi-hint {
  font-size: 13px;
  color: var(--steam-text-secondary);
}

.toolbar-count {
  margin-left: auto;
  font-size: 12px;
  color: var(--steam-text-subtle);
}

/* 多选勾选框 */
.dl-check {
  width: 16px;
  height: 16px;
  flex-shrink: 0;
  accent-color: var(--steam-accent-blue);
  cursor: pointer;
}

/* 多选选中态 */
.dl-item.is-selected {
  border-color: var(--steam-accent-blue);
  background: var(--steam-bg-hover);
}

/* 清除确认弹窗 */
.clear-dialog-body {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.clear-hint {
  margin: 0;
  font-size: 14px;
  color: var(--steam-text-primary);
}

.clear-delete-option {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: var(--steam-text-secondary);
  cursor: pointer;
  user-select: none;
}

.clear-delete-option input {
  width: 15px;
  height: 15px;
  accent-color: var(--steam-accent-blue);
  cursor: pointer;
}

/* 单个条目 */
.dl-item {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 12px 14px;
  background: var(--steam-card-bg);
  border: 1px solid var(--steam-card-border);
  border-radius: 10px;
  cursor: pointer;
  transition: background-color 0.15s ease, border-color 0.15s ease;
}

.dl-item:hover {
  background: var(--steam-bg-hover);
  border-color: var(--steam-border-light);
}

/* 封面 */
.dl-cover {
  width: 64px;
  height: 32px;
  border-radius: 4px;
  object-fit: cover;
  flex-shrink: 0;
  background: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border-subtle);
}

.dl-cover-placeholder {
  background: var(--steam-bg-secondary);
}

/* 信息主体 */
.dl-body {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.dl-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}

.dl-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.dl-percent {
  font-size: 13px;
  font-weight: 700;
  color: var(--steam-accent-blue);
  font-variant-numeric: tabular-nums;
  flex-shrink: 0;
}

/* 已暂停：警示色 */
.dl-percent-paused {
  color: var(--steam-warning);
}

.dl-percent-done {
  color: var(--steam-success);
}

/* 进度条 */
.dl-progress-bg {
  height: 6px;
  background: var(--steam-bg-primary);
  border-radius: 3px;
  overflow: hidden;
}

.dl-progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--steam-accent-blue), var(--steam-accent-green));
  border-radius: 3px;
  transition: width 0.25s ease;
}

.dl-progress-done {
  background: var(--steam-success);
}

/* 副信息行 */
.dl-sub {
  justify-content: flex-start;
}

.dl-meta {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: var(--steam-text-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.meta-icon {
  width: 12px;
  height: 12px;
  color: var(--steam-success);
  flex-shrink: 0;
}

/* 操作区 */
.dl-actions {
  flex-shrink: 0;
  display: flex;
  align-items: center;
}

.dl-actions button {
  display: inline-flex;
  align-items: center;
  gap: 4px;
}

.dl-actions svg {
  width: 11px;
  height: 11px;
}

/* 空态 */
.dl-empty {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 40px 0;
  color: var(--steam-text-subtle);
}

.empty-icon {
  width: 40px;
  height: 40px;
  color: var(--steam-text-subtle);
  margin-bottom: 4px;
}

.empty-text {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-secondary);
}

.empty-sub {
  margin: 0;
  font-size: 12px;
  color: var(--steam-text-subtle);
}
</style>