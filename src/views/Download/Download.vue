<template>
  <div class="game-download-view">
    <!-- 页面标题 -->
    <div class="page-header">
      <div class="page-title-row">
        <h1 class="page-title">游戏本体下载</h1>
        <button
          class="manifest-link-btn"
          @click="openExternalLink('https://pan.baidu.com/s/1FTZyknIObyzMuLAJC-Uj9g?pwd=8uwx')"
        >
          <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="7 10 12 15 17 10"/>
            <line x1="12" y1="15" x2="12" y2="3"/>
          </svg>
          清单文件下载
        </button>
      </div>
      <p class="page-desc">选择游戏清单文件夹并配置下载路径</p>
    </div>

    <!-- 主要内容区域 -->
    <div class="content-wrapper">
      <!-- 下载模式选择 -->
      <div class="mode-selection-section">
        <div class="mode-tabs">
          <button
            class="mode-tab"
            :class="{ active: downloadMode === 'single' }"
            @click="downloadMode = 'single'"
          >
            <svg class="tab-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
              <line x1="12" y1="8" x2="12" y2="16"/>
              <line x1="8" y1="12" x2="16" y2="12"/>
            </svg>
            单游戏下载
          </button>
          <button
            class="mode-tab"
            :class="{ active: downloadMode === 'batch' }"
            @click="downloadMode = 'batch'"
          >
            <svg class="tab-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
              <polyline points="14 2 14 8 20 8"/>
              <line x1="16" y1="13" x2="8" y2="13"/>
              <line x1="16" y1="17" x2="8" y2="17"/>
              <polyline points="10 9 9 9 8 9"/>
            </svg>
            批量下载
          </button>
        </div>
      </div>

      <!-- 单游戏下载模式 -->
      <template v-if="downloadMode === 'single'">
        <!-- 路径选择区域 -->
        <div class="path-selection-section">
          <!-- 清单文件夹选择 -->
          <div class="path-item">
            <label class="path-label">
              <svg class="label-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
              </svg>
              选择清单文件夹
            </label>
            <div class="path-input-group">
              <input
                type="text"
                v-model="manifestPath"
                class="path-input"
                placeholder="请选择包含 .vdf 和 .manifest 文件的文件夹..."
                readonly
              />
              <button class="browse-btn" @click="selectManifestFolder">浏览</button>
            </div>
          </div>

          <!-- 游戏下载文件夹选择 -->
          <div class="path-item">
            <label class="path-label">
              <svg class="label-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                <polyline points="7 10 12 15 17 10"/>
                <line x1="12" y1="15" x2="12" y2="3"/>
              </svg>
              游戏下载文件夹
            </label>
            <div class="path-input-group">
              <input
                type="text"
                v-model="downloadPath"
                class="path-input"
                placeholder="请选择游戏下载保存路径..."
                readonly
              />
              <button class="browse-btn" @click="selectDownloadFolder">浏览</button>
            </div>
          </div>
        </div>

        <!-- 游戏信息显示区域 -->
        <div class="game-info-section" v-if="manifestPath">
          <div class="info-card">
            <h3 class="info-title">游戏信息</h3>
            <div class="info-grid">
              <div class="info-item">
                <span class="info-label">游戏名称:</span>
                <span class="info-value">{{ gameName || '-' }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">Game ID:</span>
                <span class="info-value game-id">{{ gameId || '-' }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- 清单文件验证状态 -->
        <div class="validation-section" v-if="manifestPath">
          <div class="validation-card" :class="{ valid: isManifestValid, invalid: !isManifestValid }">
            <div class="validation-header">
              <svg v-if="isManifestValid" class="validation-icon success" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                <polyline points="22 4 12 14.01 9 11.01"/>
              </svg>
              <svg v-else class="validation-icon error" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"/>
                <line x1="15" y1="9" x2="9" y2="15"/>
                <line x1="9" y1="9" x2="15" y2="15"/>
              </svg>
              <span class="validation-title">{{ isManifestValid ? '清单文件验证通过' : '清单文件验证失败' }}</span>
            </div>
            <div class="validation-details" v-if="!isManifestValid">
              <p class="validation-error">{{ validationError }}</p>
            </div>
            <div class="validation-details" v-else>
              <p class="validation-success">找到 {{ manifestFiles.length }} 个清单文件</p>
              <ul class="file-list">
                <li v-for="(file, index) in manifestFiles.slice(0, 3)" :key="index" class="file-item">
                  {{ file }}
                </li>
                <li v-if="manifestFiles.length > 3" class="file-item more">
                  ...还有 {{ manifestFiles.length - 3 }} 个文件
                </li>
              </ul>
            </div>
          </div>
        </div>
      </template>

      <!-- 批量下载模式 -->
      <template v-else>
        <!-- 父文件夹选择 -->
        <div class="path-selection-section">
          <div class="path-item">
            <label class="path-label">
              <svg class="label-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
              </svg>
              选择包含多个游戏清单的父文件夹
            </label>
            <div class="path-input-group">
              <input
                type="text"
                v-model="batchParentPath"
                class="path-input"
                placeholder="请选择包含多个游戏清单文件夹的父文件夹..."
                readonly
              />
              <button class="browse-btn" @click="selectBatchParentFolder">浏览</button>
            </div>
          </div>

          <!-- 批量下载基础路径 -->
          <div class="path-item">
            <label class="path-label">
              <svg class="label-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                <polyline points="7 10 12 15 17 10"/>
                <line x1="12" y1="15" x2="12" y2="3"/>
              </svg>
              批量下载基础路径
            </label>
            <div class="path-input-group">
              <input
                type="text"
                v-model="batchDownloadBasePath"
                class="path-input"
                placeholder="请选择批量下载的基础路径..."
                readonly
              />
              <button class="browse-btn" @click="selectBatchDownloadBasePath">浏览</button>
            </div>
          </div>
        </div>

        <!-- 已选择的游戏列表 -->
        <div class="batch-games-section" v-if="batchGames.length > 0">
          <div class="batch-games-card">
            <div class="batch-games-header">
              <h3 class="info-title">
                已选择 {{ selectedBatchGames.length }}/{{ batchGames.length }} 个游戏
              </h3>
              <div class="batch-actions">
                <button class="action-btn" @click="selectAllBatchGames">全选</button>
                <button class="action-btn" @click="deselectAllBatchGames">取消全选</button>
                <button class="action-btn primary" @click="showGameSelector = true">
                  选择游戏
                </button>
              </div>
            </div>
            <div class="batch-games-list">
              <div
                v-for="game in selectedBatchGames"
                :key="game.id"
                class="batch-game-item"
              >
                <span class="game-id">{{ game.id }}</span>
                <span class="game-name">{{ game.name || '未知游戏' }}</span>
                <span class="game-status" :class="game.status">
                  {{ getStatusText(game.status) }}
                </span>
              </div>
              <div v-if="selectedBatchGames.length === 0" class="no-games-hint">
                请点击"选择游戏"按钮选择要下载的游戏
              </div>
            </div>
          </div>
        </div>
      </template>

      <!-- 自动关机选项 -->
      <div class="auto-shutdown-section">
        <label class="shutdown-checkbox">
          <input
            type="checkbox"
            v-model="autoShutdown"
          />
          <span class="checkmark"></span>
          <span class="label-text">
            <svg class="label-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M18.36 6.64a9 9 0 1 1-12.73 0"/>
              <line x1="12" y1="2" x2="12" y2="12"/>
            </svg>
            下载完成后自动关机
          </span>
        </label>
        <p v-if="autoShutdown" class="shutdown-warning">
          ⚠️ 下载完成后将自动关闭计算机，请确保保存好所有工作
        </p>
      </div>

      <!-- 下载按钮区域 -->
      <div class="download-action-section">
        <button
          class="start-download-btn"
          :class="{ disabled: !canStartDownload, loading: isDownloading }"
          :disabled="!canStartDownload || isDownloading"
          @click="startDownload"
        >
          <svg v-if="isDownloading" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
          </svg>
          <svg v-else class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="7 10 12 15 17 10"/>
            <line x1="12" y1="15" x2="12" y2="3"/>
          </svg>
          {{ isDownloading ? '下载中...' : (downloadMode === 'batch' ? `开始批量下载 (${selectedBatchGames.length}个游戏)` : '开始下载') }}
        </button>
        <p class="download-hint" v-if="!canStartDownload">
          {{ getDownloadHint() }}
        </p>
      </div>

      <!-- 批量下载进度 -->
      <div v-if="downloadMode === 'batch' && isDownloading" class="batch-progress-section">
        <div class="batch-progress-card">
          <h3 class="progress-title">批量下载进度</h3>
          <div class="overall-progress">
            <div class="progress-bar">
              <div class="progress-fill" :style="{ width: batchOverallProgress + '%' }"></div>
            </div>
            <span class="progress-text">{{ batchCompletedGames }}/{{ batchTotalGames }} ({{ batchOverallProgress }}%)</span>
          </div>
          <div class="current-game-info" v-if="currentDownloadingGame">
            当前下载: {{ currentDownloadingGame.name }} ({{ currentDownloadingGame.id }})
          </div>
        </div>
      </div>

      <!-- 下载日志区域 -->
      <div class="download-log-section" v-if="downloadLogs.length > 0">
        <div class="log-card">
          <h3 class="log-title">下载日志</h3>
          <div class="log-content" ref="logContentRef">
            <div
              v-for="(log, index) in downloadLogs"
              :key="index"
              class="log-line"
              :class="log.type"
            >
              <span class="log-time">{{ log.time }}</span>
              <span class="log-message">{{ log.message }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- 下载进度组件 -->
      <DownloadProgress
        v-if="isMonitoring || downloadProgress.depots.length > 0"
        :progress="downloadProgress"
        :is-monitoring="isMonitoring"
        class="download-progress-section"
      />
    </div>

    <!-- 游戏选择弹窗 -->
    <div v-if="showGameSelector" class="game-selector-modal" @click.self="showGameSelector = false">
      <div class="modal-content">
        <div class="modal-header">
          <h3 class="modal-title">选择要下载的游戏</h3>
          <button class="close-btn" @click="showGameSelector = false">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"/>
              <line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
          </button>
        </div>
        <div class="modal-body">
          <div class="selector-actions">
            <button class="action-btn" @click="selectAllInModal">全选</button>
            <button class="action-btn" @click="deselectAllInModal">取消全选</button>
          </div>
          <div class="games-checklist">
            <label
              v-for="game in batchGames"
              :key="game.id"
              class="game-checkbox-item"
              :class="{ checked: game.selected }"
            >
              <input
                type="checkbox"
                v-model="game.selected"
              />
              <span class="checkmark"></span>
              <span class="game-info">
                <span class="game-id">{{ game.id }}</span>
                <span class="game-name">{{ game.name || '未知游戏' }}</span>
              </span>
            </label>
          </div>
        </div>
        <div class="modal-footer">
          <button class="confirm-btn" @click="showGameSelector = false">
            确定 (已选择 {{ batchGames.filter(g => g.selected).length }} 个)
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { open as openShell } from '@tauri-apps/plugin-shell'
import DownloadProgress from '../../components/download/DownloadProgress.vue'
import type { DownloadProgress as DownloadProgressType, DepotProgress } from '../../types/download.types'

/**
 * 在系统默认浏览器中打开外部链接
 * @param url 要打开的URL
 */
const openExternalLink = async (url: string) => {
  try {
    // 使用 Tauri shell 插件在默认浏览器中打开链接
    await openShell(url)
  } catch (error) {
    console.error('打开链接失败:', error)
  }
}

// ============================================
// 类型定义
// ============================================

/**
 * 清单文件 JSON 数据结构
 */
interface ManifestJson {
  appid?: string
  name?: string
  schinese_name?: string
  type?: string
  [key: string]: any
}

/**
 * 下载日志条目
 */
interface DownloadLog {
  time: string
  message: string
  type: 'info' | 'success' | 'error' | 'warning'
}

/**
 * 批量游戏信息
 */
interface BatchGame {
  id: string
  name: string
  path: string
  selected: boolean
  status: 'pending' | 'downloading' | 'completed' | 'error'
}

// ============================================
// 响应式状态
// ============================================

/** 下载模式: single-单游戏 batch-批量 */
const downloadMode = ref<'single' | 'batch'>('single')

/** 单游戏模式状态 */
const manifestPath = ref('')
const downloadPath = ref('')
const gameName = ref('')
const gameId = ref('')
const manifestFiles = ref<string[]>([])
const vdfFilePath = ref('')
const validationError = ref('')

/** 批量下载模式状态 */
const batchParentPath = ref('')
const batchDownloadBasePath = ref('')
const batchGames = ref<BatchGame[]>([])
const showGameSelector = ref(false)
const batchTotalGames = ref(0)
const batchCompletedGames = ref(0)
const currentDownloadingGame = ref<BatchGame | null>(null)

/** 自动关机选项 */
const autoShutdown = ref(false)

/** 下载状态 */
const isDownloading = ref(false)
const downloadLogs = ref<DownloadLog[]>([])
const logContentRef = ref<HTMLDivElement>()

/** 下载进度监控 */
const isMonitoring = ref(false)
const downloadProgress = ref<DownloadProgressType>({
  totalDepots: 0,
  completedDepots: 0,
  overallPercentage: 0,
  depots: [],
  isComplete: false
})
let monitorInterval: number | null = null

// ============================================
// 计算属性
// ============================================

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

// ============================================
// 方法
// ============================================

/**
 * 获取下载提示文本
 */
const getDownloadHint = () => {
  if (downloadMode.value === 'single') {
    if (!manifestPath.value) return '请先选择清单文件夹'
    if (!downloadPath.value) return '请配置下载路径'
    if (!isManifestValid.value) return '清单文件验证失败'
  } else {
    if (!batchParentPath.value) return '请先选择父文件夹'
    if (!batchDownloadBasePath.value) return '请配置批量下载基础路径'
    if (selectedBatchGames.value.length === 0) return '请选择要下载的游戏'
  }
  return ''
}

/**
 * 获取状态文本
 */
const getStatusText = (status: BatchGame['status']) => {
  const statusMap = {
    pending: '等待中',
    downloading: '下载中',
    completed: '已完成',
    error: '出错'
  }
  return statusMap[status]
}

/**
 * 添加下载日志
 */
const addLog = (message: string, type: DownloadLog['type'] = 'info') => {
  const now = new Date()
  const time = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:${now.getSeconds().toString().padStart(2, '0')}`
  downloadLogs.value.push({ time, message, type })

  if (downloadLogs.value.length > 500) {
    downloadLogs.value = downloadLogs.value.slice(-400)
  }

  nextTick(() => {
    if (logContentRef.value) {
      logContentRef.value.scrollTop = logContentRef.value.scrollHeight
    }
  })
}

/**
 * 选择清单文件夹（单游戏模式）
 */
const selectManifestFolder = async () => {
  try {
    const selected = await open({
      directory: true,
      title: '选择清单文件夹'
    })

    if (selected) {
      manifestPath.value = selected
      addLog(`已选择清单文件夹: ${selected}`, 'info')
      await parseManifestFolder(selected)
    }
  } catch (error) {
    console.error('选择文件夹失败:', error)
    addLog(`选择文件夹失败: ${error}`, 'error')
  }
}

/**
 * 解析清单文件夹
 */
const parseManifestFolder = async (folderPath: string) => {
  gameName.value = ''
  gameId.value = ''
  manifestFiles.value = []
  vdfFilePath.value = ''
  validationError.value = ''

  try {
    const result = await invoke<{
      jsonFiles: string[]
      vdfFiles: string[]
      manifestFiles: string[]
    }>('read_manifest_folder', { folderPath })

    if (result.vdfFiles.length === 0) {
      validationError.value = '未找到 .vdf 配置文件'
      addLog('验证失败: 未找到 .vdf 文件', 'error')
      return
    }

    vdfFilePath.value = result.vdfFiles[0]

    if (result.manifestFiles.length === 0) {
      validationError.value = '未找到 .manifest 清单文件'
      addLog('验证失败: 未找到 .manifest 文件', 'error')
      return
    }

    manifestFiles.value = result.manifestFiles
    addLog(`找到 ${result.manifestFiles.length} 个清单文件`, 'success')

    if (result.jsonFiles.length > 0) {
      try {
        const jsonContent = await invoke<string>('read_text_file', {
          filePath: result.jsonFiles[0]
        })
        const jsonData: ManifestJson = JSON.parse(jsonContent)
        gameName.value = jsonData.name || ''
        addLog(`读取到游戏名称: ${gameName.value || '未找到'}`, 'info')
      } catch (e) {
        addLog(`读取JSON文件失败: ${e}`, 'warning')
      }
    }

    try {
      const vdfContent = await invoke<string>('read_text_file', {
        filePath: vdfFilePath.value
      })
      const extractedId = extractGameIdFromVdf(vdfContent)

      if (extractedId) {
        const numericId = parseInt(extractedId, 10)
        if (!isNaN(numericId)) {
          gameId.value = (numericId - 1).toString()
          addLog(`提取到游戏ID: ${gameId.value}`, 'success')
        }
      } else {
        validationError.value = '无法从 .vdf 文件中提取游戏ID'
        addLog('验证失败: 无法从VDF提取游戏ID', 'error')
        return
      }
    } catch (e) {
      validationError.value = `读取 .vdf 文件失败: ${e}`
      addLog(`读取VDF文件失败: ${e}`, 'error')
      return
    }

    await autoSetDownloadPath()
    addLog('清单文件夹验证通过', 'success')

  } catch (error) {
    validationError.value = `读取文件夹失败: ${error}`
    addLog(`读取文件夹失败: ${error}`, 'error')
  }
}

/**
 * 从VDF内容中提取游戏ID
 */
const extractGameIdFromVdf = (vdfContent: string): string | null => {
  const match = vdfContent.match(/"(\d{6,7})"/)
  return match ? match[1] : null
}

/**
 * 清理文件夹名称
 */
const sanitizeFolderName = (name: string): string => {
  return name.replace(/[<>:"/\\|?*]/g, '_')
}

/**
 * 自动设置下载路径
 * 优先使用用户设置的自定义路径，否则使用默认路径（D盘优先）
 */
const autoSetDownloadPath = async () => {
  try {
    const rawFolderName = gameName.value || gameId.value
    const folderName = sanitizeFolderName(rawFolderName)
    
    // 检查是否有用户设置的自定义下载路径
    const customPath = localStorage.getItem('customDownloadPath')
    if (customPath) {
      // 使用用户设置的自定义路径 + 游戏文件夹名
      const path = `${customPath}\\${folderName}`
      downloadPath.value = path
      addLog(`已使用自定义下载路径: ${path}`, 'info')
    } else {
      // 使用默认路径（D盘优先，没有D盘则用C盘）
      const drive = await invoke<string>('get_available_drive')
      const path = `${drive}\\SteamGame\\${folderName}`
      downloadPath.value = path
      addLog(`已自动设置下载路径: ${path}`, 'info')
    }
  } catch (error) {
    addLog(`自动设置下载路径失败: ${error}`, 'warning')
  }
}

/**
 * 选择游戏下载文件夹
 */
const selectDownloadFolder = async () => {
  try {
    const selected = await open({
      directory: true,
      title: '选择游戏下载文件夹'
    })

    if (selected) {
      downloadPath.value = selected
      addLog(`已选择下载路径: ${selected}`, 'info')
    }
  } catch (error) {
    console.error('选择文件夹失败:', error)
    addLog(`选择文件夹失败: ${error}`, 'error')
  }
}

/**
 * 选择批量下载的父文件夹
 */
const selectBatchParentFolder = async () => {
  try {
    const selected = await open({
      directory: true,
      title: '选择包含多个游戏清单的父文件夹'
    })

    if (selected) {
      batchParentPath.value = selected
      addLog(`已选择父文件夹: ${selected}`, 'info')
      await scanBatchGames(selected)
    }
  } catch (error) {
    console.error('选择文件夹失败:', error)
    addLog(`选择文件夹失败: ${error}`, 'error')
  }
}

/**
 * 扫描批量游戏
 */
const scanBatchGames = async (parentPath: string) => {
  batchGames.value = []

  try {
    // 读取父文件夹下的所有子文件夹
    const entries = await invoke<Array<{ name: string; path: string; is_dir: boolean }>>('read_directory', {
      path: parentPath
    })

    const gameFolders = entries.filter(e => e.is_dir)

    for (const folder of gameFolders) {
      try {
        // 检查文件夹内是否有清单文件
        const result = await invoke<{
          jsonFiles: string[]
          vdfFiles: string[]
          manifestFiles: string[]
        }>('read_manifest_folder', { folderPath: folder.path })

        if (result.vdfFiles.length > 0 && result.manifestFiles.length > 0) {
          // 提取游戏ID
          const vdfContent = await invoke<string>('read_text_file', {
            filePath: result.vdfFiles[0]
          })
          const extractedId = extractGameIdFromVdf(vdfContent)

          if (extractedId) {
            const numericId = (parseInt(extractedId, 10) - 1).toString()

            // 读取游戏名称
            let gameName = ''
            if (result.jsonFiles.length > 0) {
              try {
                const jsonContent = await invoke<string>('read_text_file', {
                  filePath: result.jsonFiles[0]
                })
                const jsonData: ManifestJson = JSON.parse(jsonContent)
                gameName = jsonData.name || jsonData.schinese_name || ''
              } catch (e) {
                // 忽略JSON读取错误
              }
            }

            batchGames.value.push({
              id: numericId,
              name: gameName,
              path: folder.path,
              selected: false,
              status: 'pending'
            })
          }
        }
      } catch (e) {
        // 忽略单个文件夹的错误
      }
    }

    addLog(`扫描到 ${batchGames.value.length} 个游戏`, 'success')

    // 自动设置批量下载基础路径
    if (batchGames.value.length > 0) {
      // 检查是否有用户设置的自定义下载路径
      const customPath = localStorage.getItem('customDownloadPath')
      if (customPath) {
        batchDownloadBasePath.value = customPath
        addLog(`已使用自定义批量下载基础路径: ${batchDownloadBasePath.value}`, 'info')
      } else {
        const drive = await invoke<string>('get_available_drive')
        batchDownloadBasePath.value = `${drive}\\SteamGame`
        addLog(`已自动设置批量下载基础路径: ${batchDownloadBasePath.value}`, 'info')
      }
    }
  } catch (error) {
    addLog(`扫描游戏失败: ${error}`, 'error')
  }
}

/**
 * 选择批量下载基础路径
 */
const selectBatchDownloadBasePath = async () => {
  try {
    const selected = await open({
      directory: true,
      title: '选择批量下载基础路径'
    })

    if (selected) {
      batchDownloadBasePath.value = selected
      addLog(`已选择批量下载基础路径: ${selected}`, 'info')
    }
  } catch (error) {
    console.error('选择文件夹失败:', error)
    addLog(`选择文件夹失败: ${error}`, 'error')
  }
}

/**
 * 全选批量游戏
 */
const selectAllBatchGames = () => {
  batchGames.value.forEach(game => game.selected = true)
}

/**
 * 取消全选批量游戏
 */
const deselectAllBatchGames = () => {
  batchGames.value.forEach(game => game.selected = false)
}

/**
 * 弹窗中全选
 */
const selectAllInModal = () => {
  batchGames.value.forEach(game => game.selected = true)
}

/**
 * 弹窗中取消全选
 */
const deselectAllInModal = () => {
  batchGames.value.forEach(game => game.selected = false)
}

/**
 * 开始下载
 */
const startDownload = async () => {
  if (!canStartDownload.value) return

  isDownloading.value = true
  downloadLogs.value = []

  if (downloadMode.value === 'single') {
    await startSingleDownload()
  } else {
    await startBatchDownload()
  }
}

/**
 * 开始单游戏下载
 */
const startSingleDownload = async () => {
  addLog('开始下载游戏...', 'info')
  addLog(`清单路径: ${manifestPath.value}`, 'info')
  addLog(`下载路径: ${downloadPath.value}`, 'info')

  try {
    const result = await invoke<{
      success: boolean
      message: string
    }>('start_game_download', {
      manifestPath: manifestPath.value,
      downloadPath: downloadPath.value
    })

    if (result.success) {
      addLog('下载命令已启动', 'success')
      startProgressMonitoring()
    } else {
      addLog(`下载启动失败: ${result.message}`, 'error')
      isDownloading.value = false
    }
  } catch (error) {
    addLog(`下载出错: ${error}`, 'error')
    isDownloading.value = false
  }
}

/**
 * 开始批量下载
 */
const startBatchDownload = async () => {
  const games = selectedBatchGames.value
  batchTotalGames.value = games.length
  batchCompletedGames.value = 0

  addLog(`开始批量下载，共 ${games.length} 个游戏`, 'info')

  for (let i = 0; i < games.length; i++) {
    const game = games[i]
    currentDownloadingGame.value = game
    game.status = 'downloading'

    addLog(`[${i + 1}/${games.length}] 开始下载: ${game.name} (${game.id})`, 'info')

    try {
      // 构建该游戏的下载路径
      const gameDownloadPath = `${batchDownloadBasePath.value}\\${sanitizeFolderName(game.name || game.id)}`

      // 启动下载
      const result = await invoke<{
        success: boolean
        message: string
      }>('start_game_download', {
        manifestPath: game.path,
        downloadPath: gameDownloadPath
      })

      if (result.success) {
        addLog(`[${i + 1}/${games.length}] ${game.name} 下载命令已启动`, 'success')
        // 等待当前游戏下载完成
        await waitForGameDownload(game)
        game.status = 'completed'
        batchCompletedGames.value++
        addLog(`[${i + 1}/${games.length}] ${game.name} 下载完成`, 'success')
      } else {
        game.status = 'error'
        addLog(`[${i + 1}/${games.length}] ${game.name} 下载启动失败: ${result.message}`, 'error')
      }
    } catch (error) {
      game.status = 'error'
      addLog(`[${i + 1}/${games.length}] ${game.name} 下载出错: ${error}`, 'error')
    }
  }

  currentDownloadingGame.value = null
  addLog('批量下载完成！', 'success')

  // 如果开启了自动关机，执行关机
  if (autoShutdown.value) {
    addLog('5秒后自动关机...', 'warning')
    setTimeout(() => {
      invoke('shutdown_system').catch((e: any) => {
        addLog(`关机失败: ${e}`, 'error')
      })
    }, 5000)
  }

  isDownloading.value = false
}

/**
 * 等待游戏下载完成
 */
const waitForGameDownload = async (game: BatchGame): Promise<void> => {
  return new Promise((resolve) => {
    let timeout: number | null = null

    // 设置超时（30分钟）
    timeout = window.setTimeout(() => {
      resolve()
    }, 30 * 60 * 1000)

    // 实际应该监控进度文件，这里简化处理
    // 启动进度监控
    startProgressMonitoring()

    // 监听下载完成
    const unwatch = watch(() => downloadProgress.value.isComplete, (isComplete) => {
      if (isComplete) {
        if (timeout) clearTimeout(timeout)
        unwatch()
        stopProgressMonitoring()
        resolve()
      }
    })
  })
}

/**
 * 解析进度文件名
 */
const parseProgressFileName = (fileName: string): { depotId: string; percentage: number } | null => {
  const match = fileName.match(/^(\d+)%\s*-\s*(\d+)\.json$/)
  if (match) {
    return {
      percentage: parseInt(match[1], 10),
      depotId: match[2]
    }
  }
  return null
}

/**
 * 扫描进度文件
 */
const scanProgressFiles = async () => {
  try {
    const progressFiles = await invoke<Array<{ name: string; path: string }>>('get_download_progress_files')
    const updatedDepots = [...downloadProgress.value.depots]

    for (const file of progressFiles) {
      const parsed = parseProgressFileName(file.name)
      if (parsed) {
        const fileContent = await invoke<Record<string, string[]>>('read_json_file', {
          filePath: file.path
        }).catch(() => ({}))

        const downloadedFiles = Object.keys(fileContent).length
        const depotIndex = updatedDepots.findIndex(d => d.depotId === parsed.depotId)

        if (depotIndex !== -1) {
          updatedDepots[depotIndex] = {
            depotId: parsed.depotId,
            percentage: parsed.percentage,
            downloadedFiles,
            totalFiles: downloadedFiles,
            status: parsed.percentage >= 100 ? 'completed' : 'downloading'
          }
        } else {
          updatedDepots.push({
            depotId: parsed.depotId,
            percentage: parsed.percentage,
            downloadedFiles,
            totalFiles: downloadedFiles,
            status: parsed.percentage >= 100 ? 'completed' : 'downloading'
          })
        }
      }
    }

    const completedDepots = updatedDepots.filter(d => d.status === 'completed').length
    const overallPercentage = updatedDepots.length > 0
      ? Math.round(updatedDepots.reduce((sum, d) => sum + d.percentage, 0) / updatedDepots.length)
      : 0

    const isComplete = updatedDepots.length > 0 && updatedDepots.every(d => d.status === 'completed')

    downloadProgress.value = {
      totalDepots: downloadProgress.value.totalDepots || updatedDepots.length,
      completedDepots,
      overallPercentage,
      depots: updatedDepots,
      isComplete
    }

    if (isComplete) {
      stopProgressMonitoring()
      addLog('所有depot下载完成！', 'success')
      // 删除所有进度 JSON 文件
      await deleteProgressFiles(progressFiles)
    }
  } catch (error) {
    console.error('扫描进度文件失败:', error)
  }
}

/**
 * 删除进度文件
 */
const deleteProgressFiles = async (progressFiles: Array<{ name: string; path: string }>) => {
  addLog('正在清理进度文件...', 'info')
  let deletedCount = 0

  for (const file of progressFiles) {
    try {
      await invoke('delete_file', { filePath: file.path })
      deletedCount++
      addLog(`已删除进度文件: ${file.name}`, 'info')
    } catch (error) {
      addLog(`删除进度文件失败 ${file.name}: ${error}`, 'warning')
    }
  }

  addLog(`进度文件清理完成，共删除 ${deletedCount} 个文件`, 'success')
}

/**
 * 开始监控下载进度
 */
const startProgressMonitoring = async () => {
  if (isMonitoring.value) return

  isMonitoring.value = true

  const depotIds = manifestFiles.value.map(filePath => {
    const match = filePath.match(/[\\/](\d+)_\d+\.manifest$/)
    return match ? match[1] : null
  }).filter(id => id !== null) as string[]

  downloadProgress.value.totalDepots = depotIds.length
  downloadProgress.value.depots = depotIds.map(depotId => ({
    depotId,
    percentage: 0,
    downloadedFiles: 0,
    totalFiles: 0,
    status: 'pending' as const
  }))

  addLog(`检测到 ${depotIds.length} 个depot`, 'info')

  await scanProgressFiles()

  monitorInterval = window.setInterval(() => {
    scanProgressFiles()
  }, 1000)
}

/**
 * 停止监控下载进度
 */
const stopProgressMonitoring = () => {
  isMonitoring.value = false
  if (monitorInterval) {
    clearInterval(monitorInterval)
    monitorInterval = null
  }
}

// 组件卸载时清理
onUnmounted(() => {
  stopProgressMonitoring()
})

// 监听清单路径变化
watch(manifestPath, () => {
  downloadLogs.value = []
  downloadProgress.value = {
    totalDepots: 0,
    completedDepots: 0,
    overallPercentage: 0,
    depots: [],
    isComplete: false
  }
  stopProgressMonitoring()
})
</script>

<style scoped>
/* ============================================
   页面整体布局
   ============================================ */
.game-download-view {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  padding: 24px 32px;
  background-color: var(--steam-bg-secondary);
}

/* ============================================
   页面头部
   ============================================ */
.page-header {
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--steam-border);
}

.page-title-row {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 8px;
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0;
}

.manifest-link-btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background-color: var(--steam-accent-blue);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.manifest-link-btn:hover {
  background-color: var(--steam-accent-hover);
  transform: scale(1.02);
}

.manifest-link-btn .btn-icon {
  width: 14px;
  height: 14px;
}

.page-desc {
  font-size: 14px;
  color: var(--steam-text-secondary);
  margin: 0;
}

/* ============================================
   下载模式选择
   ============================================ */
.mode-selection-section {
  margin-bottom: 20px;
}

.mode-tabs {
  display: flex;
  gap: 8px;
}

.mode-tab {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  background-color: var(--steam-bg-primary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  color: var(--steam-text-secondary);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.mode-tab:hover {
  background-color: var(--steam-bg-hover);
}

.mode-tab.active {
  background-color: var(--steam-accent-blue);
  color: white;
  border-color: var(--steam-accent-blue);
}

.tab-icon {
  width: 18px;
  height: 18px;
}

/* ============================================
   内容包装器
   ============================================ */
.content-wrapper {
  display: flex;
  flex-direction: column;
  gap: 20px;
  width: 100%;
  max-width: 1600px;
}

/* ============================================
   路径选择区域
   ============================================ */
.path-selection-section {
  display: flex;
  flex-direction: column;
  gap: 16px;
  background-color: var(--steam-bg-primary);
  padding: 20px;
  border-radius: 12px;
  border: 1px solid var(--steam-border);
}

.path-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.path-label {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

.label-icon {
  width: 16px;
  height: 16px;
  color: var(--steam-accent-blue);
}

.path-input-group {
  display: flex;
  gap: 8px;
}

.path-input {
  flex: 1;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 14px;
}

.browse-btn {
  padding: 10px 20px;
  background-color: var(--steam-accent-blue);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.browse-btn:hover {
  background-color: var(--steam-accent-hover);
}

/* ============================================
   游戏信息区域
   ============================================ */
.game-info-section {
  margin-top: 4px;
}

.info-card {
  background-color: var(--steam-bg-primary);
  padding: 20px;
  border-radius: 12px;
  border: 1px solid var(--steam-border);
}

.info-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 16px 0;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.info-label {
  font-size: 12px;
  color: var(--steam-text-secondary);
}

.info-value {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

.info-value.game-id {
  color: var(--steam-accent-blue);
  font-family: monospace;
}

/* ============================================
   验证状态区域
   ============================================ */
.validation-section {
  margin-top: 4px;
}

.validation-card {
  background-color: var(--steam-bg-primary);
  padding: 16px 20px;
  border-radius: 12px;
  border: 1px solid var(--steam-border);
}

.validation-card.valid {
  border-color: var(--steam-accent-green);
  background-color: rgba(102, 192, 244, 0.1);
}

.validation-card.invalid {
  border-color: #ff4d4f;
  background-color: rgba(255, 77, 79, 0.1);
}

.validation-header {
  display: flex;
  align-items: center;
  gap: 8px;
}

.validation-icon {
  width: 20px;
  height: 20px;
}

.validation-icon.success {
  color: var(--steam-accent-green);
}

.validation-icon.error {
  color: #ff4d4f;
}

.validation-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

.validation-details {
  margin-top: 8px;
  padding-left: 28px;
}

.validation-error {
  font-size: 13px;
  color: #ff4d4f;
  margin: 0;
}

.validation-success {
  font-size: 13px;
  color: var(--steam-accent-green);
  margin: 0;
}

.file-list {
  list-style: none;
  padding: 0;
  margin: 8px 0 0 0;
}

.file-item {
  font-size: 12px;
  color: var(--steam-text-secondary);
  padding: 2px 0;
}

.file-item.more {
  color: var(--steam-text-muted);
  font-style: italic;
}

/* ============================================
   批量游戏区域
   ============================================ */
.batch-games-section {
  margin-top: 4px;
}

.batch-games-card {
  background-color: var(--steam-bg-primary);
  padding: 20px;
  border-radius: 12px;
  border: 1px solid var(--steam-border);
}

.batch-games-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.batch-actions {
  display: flex;
  gap: 8px;
}

.action-btn {
  padding: 6px 12px;
  background-color: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 6px;
  color: var(--steam-text-primary);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.action-btn:hover {
  background-color: var(--steam-bg-hover);
}

.action-btn.primary {
  background-color: var(--steam-accent-blue);
  color: white;
  border-color: var(--steam-accent-blue);
}

.action-btn.primary:hover {
  background-color: var(--steam-accent-hover);
}

.batch-games-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 300px;
  overflow-y: auto;
}

.batch-game-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  font-size: 13px;
}

.batch-game-item .game-id {
  font-family: monospace;
  color: var(--steam-accent-blue);
  font-weight: 500;
  min-width: 80px;
}

.batch-game-item .game-name {
  flex: 1;
  color: var(--steam-text-primary);
}

.batch-game-item .game-status {
  font-size: 12px;
  padding: 2px 8px;
  border-radius: 4px;
}

.batch-game-item .game-status.pending {
  background-color: var(--steam-bg-hover);
  color: var(--steam-text-secondary);
}

.batch-game-item .game-status.downloading {
  background-color: rgba(59, 130, 246, 0.2);
  color: #3b82f6;
}

.batch-game-item .game-status.completed {
  background-color: rgba(34, 197, 94, 0.2);
  color: #22c55e;
}

.batch-game-item .game-status.error {
  background-color: rgba(239, 68, 68, 0.2);
  color: #ef4444;
}

.no-games-hint {
  text-align: center;
  padding: 20px;
  color: var(--steam-text-secondary);
  font-size: 13px;
}

/* ============================================
   自动关机区域
   ============================================ */
.auto-shutdown-section {
  background-color: var(--steam-bg-primary);
  padding: 16px 20px;
  border-radius: 12px;
  border: 1px solid var(--steam-border);
}

.shutdown-checkbox {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
}

.shutdown-checkbox input {
  display: none;
}

.checkmark {
  width: 18px;
  height: 18px;
  border: 2px solid var(--steam-border);
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.shutdown-checkbox input:checked + .checkmark {
  background-color: var(--steam-accent-blue);
  border-color: var(--steam-accent-blue);
}

.shutdown-checkbox input:checked + .checkmark::after {
  content: '✓';
  color: white;
  font-size: 12px;
}

.label-text {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 14px;
  color: var(--steam-text-primary);
}

.label-text .label-icon {
  width: 16px;
  height: 16px;
}

.shutdown-warning {
  margin: 8px 0 0 28px;
  font-size: 12px;
  color: #f59e0b;
}

/* ============================================
   下载按钮区域
   ============================================ */
.download-action-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 20px 0;
}

.start-download-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 14px 32px;
  background-color: var(--steam-accent-blue);
  color: white;
  border: none;
  border-radius: 10px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.15s ease;
  min-width: 200px;
}

.start-download-btn:hover:not(:disabled) {
  background-color: var(--steam-accent-hover);
  transform: scale(1.02);
}

.start-download-btn:disabled,
.start-download-btn.disabled {
  background-color: var(--steam-bg-hover);
  color: var(--steam-text-muted);
  cursor: not-allowed;
  transform: none;
}

.start-download-btn.loading {
  cursor: wait;
}

.start-download-btn .btn-icon,
.start-download-btn .loading-icon {
  width: 20px;
  height: 20px;
}

.start-download-btn .loading-icon {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.download-hint {
  font-size: 13px;
  color: var(--steam-text-secondary);
  margin: 0;
}

/* ============================================
   批量下载进度
   ============================================ */
.batch-progress-section {
  margin-top: 4px;
}

.batch-progress-card {
  background-color: var(--steam-bg-primary);
  padding: 20px;
  border-radius: 12px;
  border: 1px solid var(--steam-border);
}

.progress-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 16px 0;
}

.overall-progress {
  display: flex;
  align-items: center;
  gap: 12px;
}

.progress-bar {
  flex: 1;
  height: 8px;
  background-color: var(--steam-bg-secondary);
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background-color: var(--steam-accent-blue);
  border-radius: 4px;
  transition: width 0.3s ease;
}

.progress-text {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
  min-width: 80px;
  text-align: right;
}

.current-game-info {
  margin-top: 12px;
  font-size: 13px;
  color: var(--steam-text-secondary);
}

/* ============================================
   下载日志区域
   ============================================ */
.download-log-section {
  margin-top: 4px;
}

.log-card {
  background-color: var(--steam-bg-primary);
  border-radius: 12px;
  border: 1px solid var(--steam-border);
  overflow: hidden;
}

.log-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  padding: 12px 16px;
  margin: 0;
  border-bottom: 1px solid var(--steam-border);
}

.log-content {
  max-height: 300px;
  overflow-y: auto;
  padding: 12px 16px;
}

.log-line {
  display: flex;
  gap: 12px;
  font-size: 13px;
  padding: 4px 0;
  font-family: monospace;
}

.log-time {
  color: var(--steam-text-muted);
  min-width: 70px;
}

.log-message {
  color: var(--steam-text-primary);
}

.log-line.success .log-message {
  color: var(--steam-accent-green);
}

.log-line.error .log-message {
  color: #ff4d4f;
}

.log-line.warning .log-message {
  color: #f59e0b;
}

/* ============================================
   游戏选择弹窗
   ============================================ */
.game-selector-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background-color: var(--steam-bg-primary);
  border-radius: 12px;
  width: 90%;
  max-width: 600px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--steam-border);
}

.modal-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0;
}

.close-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: none;
  border: none;
  color: var(--steam-text-secondary);
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.15s ease;
}

.close-btn:hover {
  background-color: var(--steam-bg-hover);
  color: var(--steam-text-primary);
}

.close-btn svg {
  width: 20px;
  height: 20px;
}

.modal-body {
  padding: 16px 20px;
  overflow-y: auto;
  flex: 1;
}

.selector-actions {
  display: flex;
  gap: 8px;
  margin-bottom: 16px;
}

.games-checklist {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.game-checkbox-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.game-checkbox-item:hover {
  background-color: var(--steam-bg-hover);
}

.game-checkbox-item input {
  display: none;
}

.game-checkbox-item .checkmark {
  width: 18px;
  height: 18px;
  border: 2px solid var(--steam-border);
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.game-checkbox-item input:checked + .checkmark {
  background-color: var(--steam-accent-blue);
  border-color: var(--steam-accent-blue);
}

.game-checkbox-item input:checked + .checkmark::after {
  content: '✓';
  color: white;
  font-size: 12px;
}

.game-checkbox-item .game-info {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
}

.game-checkbox-item .game-id {
  font-family: monospace;
  color: var(--steam-accent-blue);
  font-weight: 500;
  min-width: 80px;
}

.game-checkbox-item .game-name {
  color: var(--steam-text-primary);
  font-size: 14px;
}

.modal-footer {
  padding: 16px 20px;
  border-top: 1px solid var(--steam-border);
  display: flex;
  justify-content: flex-end;
}

.confirm-btn {
  padding: 10px 24px;
  background-color: var(--steam-accent-blue);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.confirm-btn:hover {
  background-color: var(--steam-accent-hover);
}

/* ============================================
   下载进度组件
   ============================================ */
.download-progress-section {
  margin-top: 4px;
}
</style>
