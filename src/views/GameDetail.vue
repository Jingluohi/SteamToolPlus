<template>
  <div class="game-detail">
    <!-- 返回按钮和标题栏 -->
    <div class="detail-header">
      <button class="back-btn" @click="goBack">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M19 12H5M12 19l-7-7 7-7"/>
        </svg>
        <span>返回</span>
      </button>
      
      <div class="header-info">
        <h1 class="game-title">{{ game?.game_name }} {{ game?.chinese_name }}</h1>
        <span class="game-id">Game ID: {{ game?.game_id }}</span>
      </div>
      
      <!-- 分类标签 -->
      <div class="category-tags">
        <span 
          v-for="(tag, index) in game?.tags" 
          :key="index"
          class="category-tag"
          :style="{ backgroundColor: getCategoryColor(tag.patch_type) }"
        >
          {{ getCategoryName(tag.patch_type) }}
        </span>
      </div>
    </div>

    <!-- 主要内容区 -->
    <div class="detail-content">
      <!-- 左侧：游戏封面 -->
      <div class="cover-section">
        <img 
          :src="game?.coverUrl" 
          :alt="game?.chinese_name"
          class="game-cover"
          @error="handleImageError"
        />
      </div>

      <!-- 右侧：路径选择 -->
      <div class="paths-section">
        <!-- 游戏下载路径 - 仅当 downloadable 为 true 时显示 -->
        <div v-if="game?.downloadable" class="path-item">
          <label class="path-label">游戏下载路径</label>
          <div class="path-input-group">
            <input 
              type="text" 
              v-model="downloadPath"
              class="path-input"
              placeholder="请选择下载路径..."
              readonly
            />
            <button class="browse-btn" @click="selectDownloadPath">浏览</button>
          </div>
        </div>

        <div class="path-item">
          <label class="path-label">游戏路径选择</label>
          <div class="path-input-group">
            <input 
              type="text" 
              v-model="gamePath"
              class="path-input"
              placeholder="请选择游戏路径..."
              readonly
            />
            <button class="browse-btn" @click="selectGamePath">浏览</button>
          </div>
        </div>
      </div>
    </div>

    <!-- 底部：标签页 -->
    <div class="tabs-section">
      <div class="tabs-header">
        <button 
          v-for="tab in availableTabs" 
          :key="tab.id"
          class="tab-btn"
          :class="{ active: currentTab === tab.id }"
          @click="currentTab = tab.id"
        >
          {{ tab.name }}
        </button>
      </div>

      <div class="tabs-content">
        <!-- 游戏下载标签页 -->
        <div v-if="currentTab === 'download'" class="tab-panel">
          <h3 class="panel-title">游戏下载</h3>
          <div class="download-info">
            <!-- 清单文件夹检测状态 -->
            <div class="info-item" :class="{
              'success': manifestCheckStatus === 'found',
              'warning': manifestCheckStatus === 'not_found',
              'checking': manifestCheckStatus === 'checking'
            }">
              <span class="status-icon">
                <svg v-if="manifestCheckStatus === 'checking'" class="spin-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
                </svg>
                <svg v-else-if="manifestCheckStatus === 'found'" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                  <polyline points="22 4 12 14.01 9 11.01"/>
                </svg>
                <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="10"/>
                  <line x1="15" y1="9" x2="9" y2="15"/>
                  <line x1="9" y1="9" x2="15" y2="15"/>
                </svg>
              </span>
              <span>
                {{ manifestCheckStatus === 'checking' ? '正在检测清单文件夹...' :
                   manifestCheckStatus === 'found' ? `已找到清单文件夹: ${manifestFolderPath}` :
                   '未找到清单文件夹，请先下载清单文件' }}
              </span>
            </div>

            <!-- 下载路径显示 -->
            <div class="download-path-section">
              <label class="path-label">下载路径</label>
              <div class="path-display">{{ downloadPath || '未选择' }}</div>
              <p v-if="downloadPath" class="path-hint">自动设置为: {{ downloadPath }}</p>
            </div>
          </div>

          <!-- 开始下载按钮 -->
          <button
            class="start-download-btn"
            :class="{ disabled: !canDownload, loading: isDownloading }"
            :disabled="!canDownload || isDownloading"
            @click="startDownload"
          >
            <svg v-if="isDownloading" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
            </svg>
            <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
              <polyline points="7 10 12 15 17 10"/>
              <line x1="12" y1="15" x2="12" y2="3"/>
            </svg>
            {{ isDownloading ? '下载中...' : '开始下载' }}
          </button>

          <!-- 下载日志区域 -->
          <div v-if="downloadLogs.length > 0" class="download-logs">
            <h4 class="logs-title">下载日志</h4>
            <div class="logs-content">
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

          <!-- 下载进度组件 -->
          <DownloadProgress
            v-if="isMonitoring || downloadProgress.depots.length > 0"
            :progress="downloadProgress"
            :is-monitoring="isMonitoring"
            class="download-progress-section"
          />
        </div>

        <!-- 其他分类标签页 -->
        <div
          v-for="tab in patchTabs"
          :key="tab.id"
          v-show="currentTab === tab.id"
          class="tab-panel"
        >
          <h3 class="panel-title">{{ tab.name }}</h3>
          <div class="patch-info">
            <!-- 本地补丁状态 -->
            <div class="patch-status" :class="{ 'local-exists': isPatchLocalExists(tab.patchType) }">
              <span class="status-icon">
                <svg v-if="isPatchLocalExists(tab.patchType)" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                  <polyline points="22 4 12 14.01 9 11.01"/>
                </svg>
                <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="10"/>
                  <line x1="12" y1="8" x2="12" y2="16"/>
                  <line x1="8" y1="12" x2="16" y2="12"/>
                </svg>
              </span>
              <span class="status-text">
                {{ isPatchLocalExists(tab.patchType) ? '本地补丁已存在' : '本地补丁未下载' }}
              </span>
            </div>

            <!-- 下载补丁按钮（当本地不存在且有下载链接时显示） -->
            <button
              v-if="!isPatchLocalExists(tab.patchType) && tab.downloadUrl"
              class="download-patch-btn"
              @click="openDownloadUrl(tab.downloadUrl)"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                <polyline points="7 10 12 15 17 10"/>
                <line x1="12" y1="15" x2="12" y2="3"/>
              </svg>
              <span>下载补丁</span>
            </button>

            <!-- 选择补丁文件并应用按钮（当本地不存在时显示） -->
            <button
              v-if="!isPatchLocalExists(tab.patchType)"
              class="select-and-apply-btn"
              @click="selectAndApplyPatch(tab)"
              :disabled="!gamePath || applyingPatch"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                <polyline points="14 2 14 8 20 8"/>
                <line x1="12" y1="18" x2="12" y2="12"/>
                <line x1="9" y1="15" x2="15" y2="15"/>
              </svg>
              <span>{{ applyingPatch ? '应用中...' : '选择补丁文件并应用' }}</span>
            </button>

            <p class="patch-path">补丁路径: {{ tab.patchPath }}</p>
            <p class="game-path-display" v-if="gamePath">
              游戏路径: {{ gamePath }}
            </p>
            <p class="game-path-display warning" v-else>
              请先选择游戏路径
            </p>

            <!-- 应用补丁按钮 -->
            <button
              class="apply-patch-btn"
              @click="applyPatch(tab)"
              :disabled="!gamePath || applyingPatch || !isPatchLocalExists(tab.patchType)"
            >
              <span v-if="applyingPatch">应用中...</span>
              <span v-else>应用补丁</span>
            </button>

            <!-- 补丁说明（显示在应用补丁按钮下方） -->
            <div v-if="getPatchReadme(tab.patchType)" class="patch-readme">
              <h4 class="readme-title">补丁说明</h4>
              <pre class="readme-content">{{ getPatchReadme(tab.patchType) }}</pre>
            </div>

            <!-- 应用结果提示 -->
            <div v-if="patchResult" class="patch-result" :class="{ success: patchResult.success, error: !patchResult.success }">
              <p v-if="patchResult.success" class="result-title">补丁应用成功！</p>
              <p v-else class="result-title">补丁应用完成，但有一些错误</p>

              <div v-if="patchResult.backed_up_files.length > 0" class="result-section">
                <p class="section-title">已备份文件 ({{ patchResult.backed_up_files.length }}个):</p>
                <ul class="file-list">
                  <li v-for="(file, index) in patchResult.backed_up_files.slice(0, 5)" :key="index">
                    {{ getFileName(file) }}
                  </li>
                  <li v-if="patchResult.backed_up_files.length > 5">
                    ...还有 {{ patchResult.backed_up_files.length - 5 }} 个文件
                  </li>
                </ul>
              </div>

              <div v-if="patchResult.copied_files.length > 0" class="result-section">
                <p class="section-title">已复制文件 ({{ patchResult.copied_files.length }}个):</p>
                <ul class="file-list">
                  <li v-for="(file, index) in patchResult.copied_files.slice(0, 5)" :key="index">
                    {{ getFileName(file) }}
                  </li>
                  <li v-if="patchResult.copied_files.length > 5">
                    ...还有 {{ patchResult.copied_files.length - 5 }} 个文件
                  </li>
                </ul>
              </div>

              <div v-if="patchResult.errors.length > 0" class="result-section error-section">
                <p class="section-title">错误 ({{ patchResult.errors.length }}个):</p>
                <ul class="file-list error-list">
                  <li v-for="(error, index) in patchResult.errors.slice(0, 3)" :key="index">
                    {{ error }}
                  </li>
                  <li v-if="patchResult.errors.length > 3">
                    ...还有 {{ patchResult.errors.length - 3 }} 个错误
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { useGamesStore, getCategoryName, getCategoryColor } from '../stores/games'
import DownloadProgress from '../components/download/DownloadProgress.vue'
import type { DownloadProgress as DownloadProgressType, DepotProgress } from '../composables/useDownloadProgress'

const route = useRoute()
const router = useRouter()
const gamesStore = useGamesStore()

// 游戏ID
const gameId = computed(() => route.params.id as string)

// 获取游戏数据
const game = computed(() => gamesStore.getGameById(gameId.value))

// 路径数据
const downloadPath = ref('')
const gamePath = ref('')

// 清单文件夹路径（自动检测）
const manifestFolderPath = ref('')

// 清单文件夹检测状态
const manifestCheckStatus = ref<'checking' | 'found' | 'not_found'>('checking')

// 当前选中的标签页
const currentTab = ref('')

// 根据游戏数据设置默认标签页
const setDefaultTab = () => {
  if (game.value?.downloadable) {
    currentTab.value = 'download'
  } else if (game.value?.tags && game.value.tags.length > 0) {
    // 如果没有下载功能，默认选中第一个补丁标签
    currentTab.value = `patch-${game.value.tags[0].patch_type}`
  } else {
    currentTab.value = ''
  }
}

// 补丁应用状态
const applyingPatch = ref(false)
const patchResult = ref<{
  success: boolean
  backed_up_files: string[]
  copied_files: string[]
  errors: string[]
} | null>(null)

// 补丁说明缓存
const patchReadmes = ref<Map<number, string>>(new Map())

// 本地补丁文件存在状态缓存
const patchLocalStatus = ref<Map<number, boolean>>(new Map())

// 下载状态
const isDownloading = ref(false)
const downloadLogs = ref<{ time: string; message: string; type: 'info' | 'success' | 'error' | 'warning' }[]>([])

// 下载进度监控
const isMonitoring = ref(false)
const downloadProgress = ref<DownloadProgressType>({
  totalDepots: 0,
  completedDepots: 0,
  overallPercentage: 0,
  depots: [],
  isComplete: false
})
let monitorInterval: number | null = null

// 可用的标签页
const availableTabs = computed(() => {
  const tabs: { id: string; name: string }[] = []
  
  // 如果有downloadable，添加游戏下载标签
  if (game.value?.downloadable) {
    tabs.push({ id: 'download', name: '游戏下载' })
  }
  
  // 添加游戏分类标签
  game.value?.tags.forEach(tag => {
    tabs.push({
      id: `patch-${tag.patch_type}`,
      name: getCategoryName(tag.patch_type)
    })
  })
  
  return tabs
})

// 补丁标签页
const patchTabs = computed(() => {
  return game.value?.tags.map(tag => ({
    id: `patch-${tag.patch_type}`,
    name: getCategoryName(tag.patch_type),
    patchType: tag.patch_type,
    patchPath: tag.patch_source_path,
    downloadUrl: tag.download_url
  })) || []
})

// 是否可以开始下载
const canDownload = computed(() => {
  return manifestCheckStatus.value === 'found' && downloadPath.value !== '' && !isDownloading.value
})

// 返回上一页
const goBack = () => {
  router.back()
}

// 处理图片加载错误
const handleImageError = (e: Event) => {
  const img = e.target as HTMLImageElement
  img.style.display = 'none'
}

// 选择下载路径
const selectDownloadPath = async () => {
  try {
    const selected = await invoke<string | null>('select_folder', {
      title: '选择游戏下载路径'
    })
    if (selected) {
      downloadPath.value = selected
    }
  } catch (error) {
    console.error('选择文件夹失败:', error)
    alert('选择文件夹失败: ' + error)
  }
}

// 选择游戏路径
const selectGamePath = async () => {
  try {
    const selected = await invoke<string | null>('select_folder', {
      title: '选择游戏安装路径'
    })
    if (selected) {
      gamePath.value = selected
    }
  } catch (error) {
    console.error('选择文件夹失败:', error)
    alert('选择文件夹失败: ' + error)
  }
}

// 开始下载
const startDownload = async () => {
  // 检查是否有清单文件夹
  if (!manifestFolderPath.value) {
    alert('未找到清单文件夹，请先下载清单文件')
    return
  }

  // 检查下载路径
  if (!downloadPath.value) {
    alert('请先选择下载路径')
    return
  }

  isDownloading.value = true
  downloadLogs.value = [] // 清空之前的日志

  addDownloadLog('开始下载游戏...', 'info')
  addDownloadLog(`游戏: ${game.value?.game_name || gameId.value}`, 'info')
  addDownloadLog(`清单路径: ${manifestFolderPath.value}`, 'info')
  addDownloadLog(`下载路径: ${downloadPath.value}`, 'info')

  try {
    // 调用Rust后端执行下载命令
    const result = await invoke<{
      success: boolean
      message: string
    }>('start_game_download', {
      manifestPath: manifestFolderPath.value,
      downloadPath: downloadPath.value
    })

    if (result.success) {
      addDownloadLog('下载命令已启动', 'success')
      addDownloadLog(result.message, 'info')
      // 启动进度监控
      startProgressMonitoring()
    } else {
      addDownloadLog(`下载启动失败: ${result.message}`, 'error')
    }
  } catch (error) {
    addDownloadLog(`下载出错: ${error}`, 'error')
  } finally {
    isDownloading.value = false
  }
}

/**
 * 解析进度文件名获取depot ID和百分比
 * 文件名格式: "{百分比}% - {depotId}.json"
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
 * 获取depot总数（通过读取manifest文件夹中的.manifest文件数量）
 */
const getTotalDepots = async (): Promise<number> => {
  try {
    const result = await invoke<{
      jsonFiles: string[]
      vdfFiles: string[]
      manifestFiles: string[]
    }>('read_manifest_folder', { folderPath: manifestFolderPath.value })
    return result.manifestFiles.length
  } catch (error) {
    console.error('获取depot数量失败:', error)
    return 0
  }
}

/**
 * 扫描进度文件
 */
const scanProgressFiles = async () => {
  try {
    // 获取程序根目录下的进度文件
    const progressFiles = await invoke<Array<{ name: string; path: string }>>('get_download_progress_files')

    console.log('扫描到进度文件:', progressFiles)

    // 更新每个depot的进度
    const updatedDepots = [...downloadProgress.value.depots]

    for (const file of progressFiles) {
      const parsed = parseProgressFileName(file.name)
      if (parsed) {
        // 读取文件内容获取已下载文件数量
        const fileContent = await invoke<Record<string, string[]>>('read_json_file', {
          filePath: file.path
        }).catch(() => ({}))

        const downloadedFiles = Object.keys(fileContent).length

        // 查找对应的depot并更新
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
          // 如果depot不在列表中，添加它
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

    // 计算总体进度
    const completedDepots = updatedDepots.filter(d => d.status === 'completed').length
    const overallPercentage = updatedDepots.length > 0
      ? Math.round(updatedDepots.reduce((sum, d) => sum + d.percentage, 0) / updatedDepots.length)
      : 0

    downloadProgress.value = {
      totalDepots: downloadProgress.value.totalDepots || updatedDepots.length,
      completedDepots,
      overallPercentage,
      depots: updatedDepots,
      isComplete: updatedDepots.length > 0 && updatedDepots.every(d => d.status === 'completed')
    }

    // 如果下载完成，停止监控
    if (downloadProgress.value.isComplete) {
      stopProgressMonitoring()
      addDownloadLog('所有depot下载完成！', 'success')
    }
  } catch (error) {
    console.error('扫描进度文件失败:', error)
  }
}

/**
 * 开始监控下载进度
 */
const startProgressMonitoring = async () => {
  if (isMonitoring.value) return

  isMonitoring.value = true

  // 首先读取manifest文件夹，获取所有depot ID
  try {
    const result = await invoke<{
      jsonFiles: string[]
      vdfFiles: string[]
      manifestFiles: string[]
    }>('read_manifest_folder', { folderPath: manifestFolderPath.value })

    // 从manifest文件名中提取depot ID
    // 文件名格式: "{depotId}_{version}.manifest"
    // 注意: result.manifestFiles 是完整路径，如 D:\...\261550\261551_8211251051201469236.manifest
    const depotIds = result.manifestFiles.map(filePath => {
      // 使用更健壮的正则，匹配路径分隔符后的文件名
      const match = filePath.match(/[\\/](\d+)_\d+\.manifest$/)
      return match ? match[1] : null
    }).filter(id => id !== null) as string[]

    console.log('检测到depot IDs:', depotIds)

    // 初始化所有depot的进度状态
    downloadProgress.value.totalDepots = depotIds.length
    downloadProgress.value.depots = depotIds.map(depotId => ({
      depotId,
      percentage: 0,
      downloadedFiles: 0,
      totalFiles: 0,
      status: 'pending' as const
    }))

    addDownloadLog(`检测到 ${depotIds.length} 个depot`, 'info')
  } catch (error) {
    console.error('读取manifest文件夹失败:', error)
    addDownloadLog(`读取manifest文件夹失败: ${error}`, 'error')
  }

  // 立即扫描一次
  await scanProgressFiles()

  // 设置定时扫描
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

// 应用补丁
const applyPatch = async (tab: any) => {
  if (!gamePath.value) {
    alert('请先选择游戏路径')
    return
  }

  applyingPatch.value = true
  patchResult.value = null

  try {
    const result = await invoke<{
      success: boolean
      backed_up_files: string[]
      copied_files: string[]
      errors: string[]
    }>('apply_patch', {
      patchSourcePath: tab.patchPath,
      gamePath: gamePath.value
    })

    patchResult.value = result
    
    if (result.success) {
      alert('补丁应用成功！')
    } else {
      alert('补丁应用完成，但有一些错误，请查看详情')
    }
  } catch (error) {
    console.error('应用补丁失败:', error)
    alert('应用补丁失败: ' + error)
    patchResult.value = {
      success: false,
      backed_up_files: [],
      copied_files: [],
      errors: [String(error)]
    }
  } finally {
    applyingPatch.value = false
  }
}

// 获取文件名
const getFileName = (path: string): string => {
  const parts = path.split(/[\\/]/)
  return parts[parts.length - 1] || path
}

// 检查本地补丁文件是否存在
const checkPatchLocalStatus = async () => {
  if (!game.value) return

  for (const tag of game.value.tags) {
    try {
      const result = await invoke<string>('check_patch_file_exists', {
        patchSourcePath: tag.patch_source_path
      })
      patchLocalStatus.value.set(tag.patch_type, result !== '')
    } catch (error) {
      console.error(`检查补丁文件失败 (patch_type=${tag.patch_type}):`, error)
      patchLocalStatus.value.set(tag.patch_type, false)
    }
  }
}

// 获取本地补丁存在状态
const isPatchLocalExists = (patchType: number): boolean => {
  return patchLocalStatus.value.get(patchType) || false
}

// 打开下载链接
const openDownloadUrl = async (url: string) => {
  try {
    await invoke('open_external_link', { url })
  } catch (error) {
    console.error('打开下载链接失败:', error)
    alert('打开下载链接失败: ' + error)
  }
}

// 选择本地补丁文件并直接应用
const selectAndApplyPatch = async (tab: any) => {
  if (!gamePath.value) {
    alert('请先选择游戏路径')
    return
  }

  try {
    // 打开文件选择对话框
    const selected = await open({
      title: '选择补丁压缩包文件',
      filters: [{
        name: '7z压缩包',
        extensions: ['7z']
      }],
      multiple: false
    })

    if (!selected) {
      return // 用户取消了选择
    }

    const archivePath = Array.isArray(selected) ? selected[0] : selected

    // 确认应用
    const confirmApply = confirm(`确定要将补丁应用到游戏目录吗？\n\n补丁文件: ${archivePath}\n游戏路径: ${gamePath.value}\n\n应用前会自动备份原有文件。`)
    if (!confirmApply) {
      return
    }

    applyingPatch.value = true
    patchResult.value = null

    // 调用后端命令直接应用补丁
    const result = await invoke<{
      success: boolean
      backed_up_files: string[]
      copied_files: string[]
      errors: string[]
    }>('apply_patch_from_file', {
      archivePath: archivePath,
      gamePath: gamePath.value
    })

    patchResult.value = result

    if (result.success) {
      alert('补丁应用成功！')
    } else {
      alert('补丁应用完成，但有一些错误，请查看详情')
    }
  } catch (error) {
    console.error('应用补丁失败:', error)
    alert('应用补丁失败: ' + error)
    patchResult.value = {
      success: false,
      backed_up_files: [],
      copied_files: [],
      errors: [String(error)]
    }
  } finally {
    applyingPatch.value = false
  }
}

// 页面加载时确保游戏数据已加载，并自动检测清单文件夹
onMounted(async () => {
  if (gamesStore.games.length === 0) {
    await gamesStore.loadGames()
  }
  // 设置默认标签页
  setDefaultTab()
  // 自动检测清单文件夹
  await checkManifestFolder()
  // 加载所有补丁说明
  await loadPatchReadmes()
  // 检查本地补丁文件状态
  await checkPatchLocalStatus()
})

// 组件卸载时清理定时器
onUnmounted(() => {
  stopProgressMonitoring()
})

// 加载所有补丁说明
const loadPatchReadmes = async () => {
  if (!game.value) return

  for (const tag of game.value.tags) {
    try {
      const readme = await invoke<string>('get_patch_readme', {
        gameId: game.value.game_id,
        patchType: tag.patch_type
      })
      if (readme) {
        patchReadmes.value.set(tag.patch_type, readme)
      }
    } catch (error) {
      console.error(`加载补丁说明失败 (patch_type=${tag.patch_type}):`, error)
    }
  }
}

// 获取补丁说明
const getPatchReadme = (patchType: number): string => {
  return patchReadmes.value.get(patchType) || ''
}

/**
 * 检查 manifest 文件夹是否存在
 * 自动查找 resources/manifest/游戏ID 目录
 */
const checkManifestFolder = async () => {
  manifestCheckStatus.value = 'checking'
  manifestFolderPath.value = ''

  try {
    // 构建清单文件夹路径：resources/manifest/游戏ID
    const manifestPath = await invoke<string>('get_manifest_path', {
      gameId: gameId.value
    })

    if (manifestPath) {
      manifestFolderPath.value = manifestPath
      manifestCheckStatus.value = 'found'
      // 自动设置下载路径
      await autoSetDownloadPath()
    } else {
      manifestCheckStatus.value = 'not_found'
    }
  } catch (error) {
    console.error('检查清单文件夹失败:', error)
    manifestCheckStatus.value = 'not_found'
  }
}

/**
 * 清理文件夹名称，移除Windows文件系统不支持的字符
 * @param name 原始名称
 * @returns 清理后的名称
 */
const sanitizeFolderName = (name: string): string => {
  // Windows 文件系统不支持的字符: < > : " / \ | ? *
  return name.replace(/[<>:"/\\|?*]/g, '_')
}

/**
 * 自动设置下载路径
 * 优先使用用户设置的自定义路径，否则使用默认路径（D盘优先，没有D盘则用C盘）
 * 路径格式：自定义路径\游戏英文名 或 D:\SteamGame\游戏英文名
 * 仅当游戏可下载时设置
 */
const autoSetDownloadPath = async () => {
  // 如果游戏不可下载，不设置下载路径
  if (!game.value?.downloadable) {
    downloadPath.value = ''
    return
  }

  try {
    // 构建下载路径，使用游戏英文名作为文件夹名（清理非法字符）
    const rawFolderName = game.value?.game_name || gameId.value
    const folderName = sanitizeFolderName(rawFolderName)
    
    // 检查是否有用户设置的自定义下载路径
    const customPath = localStorage.getItem('customDownloadPath')
    if (customPath) {
      // 使用用户设置的自定义路径 + 游戏文件夹名
      const path = `${customPath}\\${folderName}`
      downloadPath.value = path
      console.log('已使用自定义下载路径:', path)
    } else {
      // 获取可用的游戏盘符（优先D盘，其次C盘）
      const drive = await invoke<string>('get_available_drive')
      const path = `${drive}\\SteamGame\\${folderName}`
      downloadPath.value = path
      console.log('已使用默认下载路径:', path)
    }
  } catch (error) {
    console.error('自动设置下载路径失败:', error)
  }
}

/**
 * 添加下载日志
 * @param message 日志消息
 * @param type 日志类型
 */
const addDownloadLog = (message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') => {
  const now = new Date()
  const time = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:${now.getSeconds().toString().padStart(2, '0')}`
  downloadLogs.value.push({ time, message, type })

  // 限制日志数量
  if (downloadLogs.value.length > 100) {
    downloadLogs.value = downloadLogs.value.slice(-80)
  }
}
</script>

<style scoped>
.game-detail {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  padding: 24px;
  background-color: var(--bg-primary);
}

/* 头部区域 */
.detail-header {
  display: flex;
  align-items: center;
  gap: 24px;
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--border-color);
}

.back-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  border: none;
  border-radius: 8px;
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 14px;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.back-btn:hover {
  background-color: var(--bg-surface);
}

.back-btn svg {
  width: 16px;
  height: 16px;
}

.header-info {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 16px;
}

.game-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.game-id {
  font-size: 13px;
  color: var(--text-secondary);
  font-family: 'Courier New', monospace;
}

.category-tags {
  display: flex;
  gap: 8px;
}

.category-tag {
  padding: 4px 12px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
  color: white;
}

.download-tag {
  background-color: #10b981;
}

/* 主要内容区 */
.detail-content {
  display: flex;
  gap: 24px;
  margin-bottom: 24px;
}

.cover-section {
  flex-shrink: 0;
}

.game-cover {
  width: 400px;
  height: auto;
  max-height: 300px;
  object-fit: contain;
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.paths-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.path-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.path-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.path-input-group {
  display: flex;
  gap: 8px;
}

.path-input {
  flex: 1;
  padding: 10px 12px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
}

.browse-btn {
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  background-color: var(--accent-color);
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.browse-btn:hover {
  background-color: var(--accent-hover);
}

/* 标签页区域 */
.tabs-section {
  background-color: var(--bg-secondary);
  border-radius: 12px;
  overflow: hidden;
}

.tabs-header {
  display: flex;
  border-bottom: 1px solid var(--border-color);
}

.tab-btn {
  padding: 12px 24px;
  border: none;
  background-color: transparent;
  color: var(--text-secondary);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  border-bottom: 2px solid transparent;
}

.tab-btn:hover {
  color: var(--text-primary);
  background-color: var(--bg-surface);
}

.tab-btn.active {
  color: var(--accent-color);
  border-bottom-color: var(--accent-color);
  background-color: var(--bg-surface);
}

.tabs-content {
  padding: 24px;
}

.tab-panel {
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.panel-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 16px 0;
}

.download-info {
  display: flex;
  flex-direction: column;
  gap: 16px;
  margin-bottom: 24px;
}

.info-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  border-radius: 8px;
  font-size: 13px;
}

.info-item.warning {
  background-color: rgba(245, 158, 11, 0.1);
  color: #f59e0b;
}

.info-item.success {
  background-color: rgba(16, 185, 129, 0.1);
  color: #10b981;
}

.info-item.checking {
  background-color: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
}

.status-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}

.status-icon svg {
  width: 20px;
  height: 20px;
}

.spin-icon {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.warning-icon {
  font-size: 8px;
}

.download-path-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.path-display {
  padding: 12px 16px;
  background-color: var(--bg-primary);
  border-radius: 8px;
  font-size: 13px;
  color: var(--text-secondary);
  font-family: 'Courier New', monospace;
}

.start-download-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px 32px;
  border: none;
  border-radius: 8px;
  background-color: var(--accent-color);
  color: white;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.start-download-btn svg {
  width: 18px;
  height: 18px;
}

.start-download-btn:hover:not(.disabled) {
  background-color: var(--accent-hover);
}

.start-download-btn.disabled {
  background-color: var(--text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

.start-download-btn.loading {
  cursor: wait;
}

.loading-icon {
  animation: spin 1s linear infinite;
}

.path-hint {
  margin: 4px 0 0 0;
  font-size: 12px;
  color: var(--text-secondary);
  font-style: italic;
}

/* 下载日志样式 */
.download-logs {
  margin-top: 20px;
  padding: 16px;
  background-color: var(--bg-primary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.logs-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 12px 0;
}

.logs-content {
  max-height: 200px;
  overflow-y: auto;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  line-height: 1.6;
}

.log-line {
  display: flex;
  gap: 10px;
  padding: 2px 0;
}

.log-time {
  color: var(--text-secondary);
  flex-shrink: 0;
}

.log-message {
  color: var(--text-primary);
  word-break: break-all;
}

.log-line.success .log-message {
  color: #10b981;
}

.log-line.error .log-message {
  color: #ef4444;
}

.log-line.warning .log-message {
  color: #f59e0b;
}

/* 下载进度区域样式 */
.download-progress-section {
  margin-top: 20px;
}

.patch-info {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

/* 补丁说明样式 - 显示在应用补丁按钮下方 */
.patch-readme {
  background-color: var(--bg-primary);
  border-radius: 8px;
  padding: 16px;
  border-left: 4px solid var(--accent-color);
  margin-top: 16px;
}

.readme-title {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.readme-content {
  margin: 0;
  padding: 0;
  font-size: 13px;
  line-height: 1.6;
  color: var(--text-secondary);
  white-space: pre-wrap;
  word-wrap: break-word;
  font-family: inherit;
  background: none;
  border: none;
}

/* 补丁状态样式 */
.patch-status {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 14px;
  border-radius: 8px;
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
  font-size: 13px;
  font-weight: 500;
  margin-bottom: 12px;
}

.patch-status.local-exists {
  background-color: rgba(16, 185, 129, 0.1);
  color: #10b981;
}

.status-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
}

.status-icon svg {
  width: 18px;
  height: 18px;
}

/* 下载补丁按钮 */
.download-patch-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 10px 24px;
  border: none;
  border-radius: 8px;
  background-color: #3b82f6;
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.15s ease;
  margin-bottom: 12px;
}

.download-patch-btn:hover {
  background-color: #2563eb;
}

.download-patch-btn svg {
  width: 18px;
  height: 18px;
}

/* 选择补丁文件并应用按钮 */
.select-and-apply-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 10px 24px;
  border: none;
  border-radius: 8px;
  background-color: #8b5cf6;
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.15s ease;
  margin-bottom: 12px;
}

.select-and-apply-btn:hover:not(:disabled) {
  background-color: #7c3aed;
}

.select-and-apply-btn:disabled {
  background-color: var(--text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

.select-and-apply-btn svg {
  width: 18px;
  height: 18px;
}

.patch-path {
  margin: 0;
  font-size: 13px;
  color: var(--text-secondary);
  font-family: 'Courier New', monospace;
}

.game-path-display {
  margin: 0;
  font-size: 13px;
  color: var(--text-secondary);
  font-family: 'Courier New', monospace;
}

.game-path-display.warning {
  color: #f59e0b;
}

.apply-patch-btn {
  padding: 10px 24px;
  border: none;
  border-radius: 8px;
  background-color: var(--accent-color);
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.15s ease;
  align-self: flex-start;
}

.apply-patch-btn:hover:not(:disabled) {
  background-color: var(--accent-hover);
}

.apply-patch-btn:disabled {
  background-color: var(--text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

/* 补丁结果提示 */
.patch-result {
  margin-top: 16px;
  padding: 16px;
  border-radius: 8px;
  background-color: var(--bg-primary);
}

.patch-result.success {
  border-left: 4px solid #10b981;
}

.patch-result.error {
  border-left: 4px solid #ef4444;
}

.result-title {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 600;
}

.patch-result.success .result-title {
  color: #10b981;
}

.patch-result.error .result-title {
  color: #ef4444;
}

.result-section {
  margin-bottom: 12px;
}

.result-section:last-child {
  margin-bottom: 0;
}

.section-title {
  margin: 0 0 8px 0;
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
}

.file-list {
  margin: 0;
  padding-left: 16px;
  font-size: 12px;
  color: var(--text-secondary);
}

.file-list li {
  margin-bottom: 4px;
}

.error-section .section-title {
  color: #ef4444;
}

.error-list {
  color: #ef4444;
}
</style>
