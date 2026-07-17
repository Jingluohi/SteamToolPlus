<template>
  <div class="game-detail-page">
    <!-- 二维码弹窗 -->
    <QRCodeModal
      v-model="showQRCodeModal"
      :title="qrCodeTitle"
      :qr-image-url="qrCodeImageUrl"
      :hint="qrCodeHint"
      @close="handleQRCodeClose"
    />

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
          :src="coverImage || '/default-cover.jpg'" 
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
          :class="{ active: currentTab === tab.id, 'patch-tab': tab.id.startsWith('patch-') }"
          @click="currentTab = tab.id"
        >
          {{ tab.name }}
        </button>
      </div>

      <div class="tabs-content">
        <!-- 游戏下载标签页 -->
        <GameDownloadTab
          v-if="currentTab === 'download'"
          :game-id="gameId"
          :game="game"
          :existing-game-data="existingGameData"
          :manifest-check-status="manifestCheckStatus"
          :download-path="downloadPath"
          :manifest-folder-path="manifestFolderPath"
          :is-downloading="isDownloading"
          :is-verifying="isVerifying"
          :is-monitoring="isMonitoring"
          :download-progress="downloadProgress"
          :download-logs="downloadLogs"
          :download-manifest-source-mode="downloadManifestSourceMode"
          :selected-download-manifest-path="selectedDownloadManifestPath"
          :is-preparing-download-manifest="isPreparingDownloadManifest"
          @select-manifest-archive="selectDownloadManifestArchive"
          @select-manifest-folder="selectDownloadManifestFolder"
          @start-download="startDownload"
          @stop-download="stopDownload"
          @open-qingdan-qr-code="openQingdanQRCode"
          @open-download-url="openDownloadUrl"
          @verify-integrity="verifyIntegrity"
        />

        <!-- 补丁配置标签页 -->
        <PatchConfigTab
          :currentTab="currentTab"
          :patchTabs="patchTabs"
          :gamePath="gamePath"
          :applyingPatch="applyingPatch"
          :patchResult="patchResult"
          :patchLocalStatus="patchLocalStatus"
          :patchReadmes="patchReadmes"
          @openDownloadUrl="openDownloadUrl"
          @selectAndApplyPatch="selectAndApplyPatch"
          @applyPatch="applyPatch"
          @openVirtualizationTutorial="openVirtualizationTutorial"
        />

        <!-- 入库Steam标签页 -->
        <GameImportTab
          :currentTab="currentTab"
          :gameId="gameId"
          :game="game"
          :hasLua="hasLua"
          :canImportToSteam="canImportToSteam"
          :isImporting="isImportingWithOpenSteamTool"
          :isPreparingImport="isPreparingImport"
          :importSourceReady="importSourceReady"
          :lockVersion="lockVersion"
          :importSourceMode="importSourceMode"
          :selectedImportPath="selectedImportPath"
          @importWithOpenSteamTool="importWithOpenSteamTool"
          @restartSteam="restartSteam"
          @selectImportArchive="selectImportArchive"
          @selectImportFolder="selectImportFolder"
          @clearImportSource="clearImportSource"
          @update:lockVersion="lockVersion = $event"
          @update:importSourceMode="importSourceMode = $event"
          @open-qingdan-qr-code="openQingdanQRCode"
          @openDownloadUrl="openDownloadUrl"
        />

        <!-- 解压即玩标签页 -->
        <div v-if="currentTab === 'extract-play'" class="tab-panel">
          <h3 class="panel-title">解压即玩</h3>
          <div class="extract-play-content">
            <div class="extract-play-description">
              <p>直接从网盘下载完整游戏文件（豪华版），下载完成后解压即可游玩，无需额外操作。</p>
              <p class="extract-play-note">（都是无法联机版，除非网盘文件特别标注或者手动注入联机补丁）</p>
            </div>

            <!-- 夸克网盘下载区域 -->
            <div class="download-section">
              <p class="download-section-title">下载游戏：</p>
              <div class="download-buttons">
                <div class="download-btn-wrapper">
                  <button
                    class="download-patch-btn"
                    :class="{ disabled: !quarkQRCodeExists }"
                    :disabled="!quarkQRCodeExists"
                    @click="openQuarkQRCode"
                    :title="quarkQRCodeExists ? '点击扫码下载' : '暂未上传完游戏'"
                  >
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                      <polyline points="7 10 12 15 17 10"/>
                      <line x1="12" y1="15" x2="12" y2="3"/>
                    </svg>
                    夸克网盘
                  </button>
                  <span v-if="!quarkQRCodeExists" class="pwd-hint">暂未上传完游戏</span>
                </div>
              </div>
              <p class="download-hint">（请先转存至您的网盘，避免和谐，也将给作者带来无限的更新动力）</p>
            </div>
          </div>
        </div>

        <!-- 修改器标签页 -->
        <div v-if="currentTab === 'trainer'" class="tab-panel">
          <h3 class="panel-title">游戏修改器</h3>
          <div class="trainer-content">
            <!-- 修改器下载区域 -->
            <div v-if="game?.trainer?.download_urls && game.trainer.download_urls.length > 0" class="download-section">
              <p class="download-section-title">修改器下载：</p>
              <div class="download-buttons">
                <div
                  v-for="(item, index) in game.trainer.download_urls"
                  :key="index"
                  class="download-btn-wrapper"
                >
                  <button
                    class="download-patch-btn"
                    @click="openDownloadUrl(item.url)"
                  >
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                      <polyline points="7 10 12 15 17 10"/>
                      <line x1="12" y1="15" x2="12" y2="3"/>
                    </svg>
                    <span>{{ getDownloadSourceName(item.source) }}</span>
                  </button>
                  <p v-if="item.pwd" class="pwd-hint">
                    提取码: {{ item.pwd }}
                  </p>
                </div>
              </div>
              <p class="download-hint">
                （请先转存至您的网盘，避免和谐，也将给作者带来无限的更新动力）
              </p>
            </div>

            <!-- 本地修改器内容 -->
            <div v-if="trainerContent" class="trainer-local-content">
              <h4 class="trainer-content-title">修改器说明</h4>
              <pre class="trainer-content-text">{{ trainerContent }}</pre>
            </div>
            <div v-else-if="game?.trainer?.local_path" class="trainer-no-content">
              <p>本地修改器文件不存在或无法读取</p>
              <p class="trainer-path">路径: {{ game.trainer.local_path }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 下载完成弹窗：提示用户前往注入补丁 -->
    <BaseModal
      v-model="showDownloadCompleteModal"
      title="下载完成"
      width="40%"
      container-class="download-complete-modal"
      :show-confirm="false"
      :show-cancel="true"
      cancel-text="稍后再说"
      @close="showDownloadCompleteModal = false"
    >
      <template #body>
        <div class="download-complete-modal-content">
          <p class="modal-main-hint">下载完成，请前往注入补丁，补丁应用成功才能游玩</p>
          <p class="modal-path-hint" v-if="downloadCompleteInstallPath">
            已自动设置安装路径：<code>{{ downloadCompleteInstallPath }}</code>
          </p>
          <p class="modal-path-hint" v-if="downloadCompleteExePath && downloadCompleteExePath !== downloadCompleteInstallPath">
            已自动定位游戏 exe 路径：<code>{{ downloadCompleteExePath }}</code>
          </p>
          <div class="patch-select-section">
            <p class="patch-select-title">补丁选择：</p>
            <div class="patch-select-buttons">
              <button
                v-for="tab in patchTabs"
                :key="tab.id"
                class="patch-select-btn"
                :style="{ backgroundColor: getCategoryColor(tab.patchType) }"
                @click="goToPatchTab(tab)"
              >
                {{ tab.name }}
              </button>
            </div>
          </div>
          <p v-if="patchTabs.length === 0" class="no-patch-hint">
            该游戏暂无补丁配置，请前往游戏库查看。
          </p>
        </div>
      </template>
    </BaseModal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { convertFileSrc } from '@tauri-apps/api/core'
import QRCodeModal from '../../components/common/QRCodeModal.vue'
import BaseModal from '../../components/common/BaseModal.vue'
import GameImportTab from './components/GameImportTab.vue'
import GameDownloadTab from './components/GameDownloadTab.vue'
import PatchConfigTab from './components/PatchConfigTab.vue'
import { usePatch } from './composables/usePatch'
import type { DownloadProgress as DownloadProgressType } from '../../types/download.types'
import type { GameConfigData } from '../../types'
import { getPatchSourcePath } from '../../types'
import { loadGamesConfigFromFile } from '../../api/game.api'
import { getCachedCoverImage } from '../../services/imageCache.service'
import { 
  getGameData, 
  upsertGameData, 
  updateGameDownloadStatus,
  finalizeGameDownload,
  type GameData 
} from '../../api/gameData.api'
import { getCategoryName, getCategoryColor } from '../../constants/game'
import { safeAsync } from '../../utils/async-helper'
import { sanitizeGameFolderName } from '../../utils/file-helper'
import { useConfigStore } from '../../store/config.store'

const route = useRoute()
const router = useRouter()
const configStore = useConfigStore()

// 游戏ID
const gameId = computed(() => route.params.id as string)

// 游戏数据
const gamesConfig = ref<GameConfigData[]>([])
const game = computed(() => gamesConfig.value.find(g => g.game_id === gameId.value))

// 已存在的游戏数据（从game.json加载）
const existingGameData = ref<GameData | null>(null)

// 封面图片
const coverImage = ref('')

// 路径数据
const downloadPath = ref('')
const gamePath = ref('')

// 清单文件夹路径（自动检测）
const manifestFolderPath = ref('')

// 清单文件夹检测状态
const manifestCheckStatus = ref<'checking' | 'found' | 'not_found'>('checking')

// 当前选中的标签页
const currentTab = ref('')

// 修改器本地文件内容
const trainerContent = ref<string>('')

// 根据游戏数据设置默认标签页
const setDefaultTab = () => {
  if (game.value?.downloadable) {
    currentTab.value = 'download'
  } else if (game.value?.has_extract_play === true) {
    // 如果没有下载功能但有解压即玩，默认选中解压即玩标签
    currentTab.value = 'extract-play'
  } else if (game.value?.tags && game.value.tags.length > 0) {
    // 如果没有下载功能和解压即玩，默认选中第一个补丁标签
    currentTab.value = `patch-${game.value.tags[0].patch_type}`
  } else {
    currentTab.value = ''
  }
}

// ==================== Composables 初始化 ====================

// 补丁应用相关状态和方法
const {
  applyingPatch,
  patchResult,
  patchReadmes,
  patchLocalStatus,
  checkPatchLocalStatus,
  applyPatch,
  selectAndApplyPatch,
  loadPatchReadmes,
  openVirtualizationTutorial,
  openDownloadUrl
} = usePatch(game, gamePath)

// 下载状态
const isDownloading = ref(false)
const isVerifying = ref(false)
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

// 下载完成弹窗
const showDownloadCompleteModal = ref(false)
const downloadCompleteInstallPath = ref('')
const downloadCompleteExePath = ref('')

// 入库Steam状态
const isImportingWithOpenSteamTool = ref(false)
const manifestExists = ref(false)
const hasLua = ref(false)
const hasVdf = ref(false)
const hasManifest = ref(false)

// OpenSteamTool高级选项
const lockVersion = ref(false)

// 自定义清单源状态
const importSourceMode = ref<'7z' | 'folder'>('7z')
const selectedImportPath = ref('')
const selectedImportTempPath = ref('')
const isPreparingImport = ref(false)
const importSourceReady = computed(() => {
  if (!selectedImportPath.value) return false
  // 7z模式需要已经解压到临时目录并检测到lua
  if (importSourceMode.value === '7z' && !selectedImportTempPath.value) return false
  return hasLua.value
})

// 游戏下载标签页的自定义清单源状态
const downloadManifestSourceMode = ref<'7z' | 'folder'>('7z')
const selectedDownloadManifestPath = ref('')
const isPreparingDownloadManifest = ref(false)

// 二维码弹窗状态
const showQRCodeModal = ref(false)
const qrCodeTitle = ref('夸克网盘下载')
const qrCodeImageUrl = ref('')
const qrCodeHint = ref('请使用夸克APP扫码下载')

// 夸克网盘二维码是否存在
const quarkQRCodeExists = ref(false)

/**
 * 获取夸克网盘二维码图片路径
 * 图片存放在 resources/pic/Quark_QR/{game_id}.png
 */
const getQuarkQRCodePath = (gameId: string): string => {
  // 使用相对路径，程序根目录下的 resources/pic/Quark_QR/
  return `resources/pic/Quark_QR/${gameId}.png`
}

/**
 * 检查夸克网盘二维码是否存在
 */
const checkQuarkQRCodeExists = async (gameId: string): Promise<boolean> => {
  try {
    const qrPath = getQuarkQRCodePath(gameId)
    const exists = await invoke<boolean>('check_file_exists', { path: qrPath })
    return exists
  } catch {
    return false
  }
}

/**
 * 打开夸克网盘二维码弹窗
 */
const openQuarkQRCode = async () => {
  // 检查二维码是否存在
  const exists = await checkQuarkQRCodeExists(gameId.value)
  if (!exists) {
    return
  }

  // 设置弹窗内容
  qrCodeTitle.value = '夸克网盘下载'
  qrCodeHint.value = '请使用夸克APP扫码下载'

  // 获取二维码图片URL
  const qrPath = getQuarkQRCodePath(gameId.value)
  qrCodeImageUrl.value = convertFileSrc(qrPath)

  // 显示弹窗
  showQRCodeModal.value = true
}

/**
 * 关闭二维码弹窗
 */
const handleQRCodeClose = () => {
  showQRCodeModal.value = false
  qrCodeImageUrl.value = ''
}

/**
 * 打开清单下载夸克网盘二维码弹窗
 * 使用程序内置的 qingdan.png 二维码图片
 */
const openQingdanQRCode = async () => {
  console.log('openQingdanQRCode called')
  try {
    qrCodeTitle.value = '夸克网盘下载'
    qrCodeHint.value = '请使用夸克APP扫码下载'
    qrCodeImageUrl.value = await invoke<string>('get_qingdan_image_base64')
    showQRCodeModal.value = true
  } catch (error) {
    console.error('打开清单二维码弹窗失败:', error)
    alert(`打开二维码弹窗失败: ${error}`)
  }
}

const canImportToSteam = computed(() => {
  // 只要有lua就可以入库（内置清单或自定义源）
  return hasLua.value || importSourceReady.value
})

// 可用的标签页
// 优先级：游戏下载 > 解压即玩 > steam入库 > 各类补丁
const availableTabs = computed(() => {
  const tabs: { id: string; name: string }[] = []

  // 1. 如果有downloadable，添加游戏下载标签
  if (game.value?.downloadable) {
    tabs.push({ id: 'download', name: '游戏下载' })
  }

  // 2. 如果有解压即玩版本，添加解压即玩标签
  if (game.value?.has_extract_play === true) {
    tabs.push({ id: 'extract-play', name: '解压即玩' })
  }

  // 3. 入库Steam标签始终显示
  tabs.push({ id: 'import', name: '入库Steam' })

  // 4. 添加游戏分类标签（各类补丁）
  game.value?.tags.forEach(tag => {
    tabs.push({
      id: `patch-${tag.patch_type}`,
      name: getCategoryName(tag.patch_type)
    })
  })

  // 5. 如果有修改器配置且包含下载链接，添加修改器标签
  if (game.value?.trainer?.download_urls && game.value.trainer.download_urls.length > 0) {
    tabs.push({ id: 'trainer', name: '修改器' })
  }

  return tabs
})

// 补丁标签页
const patchTabs = computed(() => {
  return game.value?.tags.map(tag => ({
    id: `patch-${tag.patch_type}`,
    name: getCategoryName(tag.patch_type),
    patchType: tag.patch_type,
    patchPath: getPatchSourcePath(tag, game.value?.game_id || ''),
    downloadUrls: tag.download_urls || []
  })) || []
})

/**
 * 获取下载来源显示名称
 * @param source 网盘来源标识
 */
const getDownloadSourceName = (source: string): string => {
  const sourceMap: Record<string, string> = {
    'baidu': '百度网盘',
    'thunder': '迅雷网盘',
    'lanzou': '蓝奏云',
    'quark': '夸克网盘',
    'other': '其他网盘'
  }
  return sourceMap[source] || source || '未知网盘'
}

// 返回上一页
const goBack = () => {
  router.back()
}

// 处理图片加载错误 - 尝试重新加载
const handleImageError = async (e: Event) => {
  const img = e.target as HTMLImageElement
  // 延迟后尝试重新加载图片
  if (gameId.value) {
    setTimeout(async () => {
      try {
        const { getGameCoverImage } = await import('../../api/game.api')
        const { convertFileSrc } = await import('@tauri-apps/api/core')
        const filePath = await getGameCoverImage(gameId.value)
        if (filePath) {
          img.src = convertFileSrc(filePath)
        }
      } catch {
        // 重试失败则隐藏图片
        img.style.display = 'none'
      }
    }, 500)
  } else {
    img.style.display = 'none'
  }
}

// 分类名称和颜色从 constants/game 导入

// 选择下载路径
const selectDownloadPath = async () => {
  try {
    const selected = await open({
      directory: true,
      title: '选择游戏下载路径'
    })
    if (selected) {
      downloadPath.value = selected
    }
  } catch (error) {
    alert('选择文件夹失败: ' + error)
  }
}

// 选择游戏路径
const selectGamePath = async () => {
  try {
    const selected = await open({
      directory: true,
      title: '选择游戏安装路径'
    })
    if (selected) {
      gamePath.value = selected
    }
  } catch (error) {
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
    // 先保存游戏数据到game.json
    const gameData: GameData = {
      game_id: gameId.value,
      game_name: game.value?.game_name || '',
      chinese_name: game.value?.chinese_name || '',
      game_type: 'downloaded',
      install_path: downloadPath.value,
      exe_path: '', // 下载完成后需要用户设置
      is_favorite: false,
      is_installed: false,
      play_time: 0,
      added_date: new Date().toISOString(),
      download_status: 'downloading',
      download_progress: 0,
      download_path: downloadPath.value
    }
    await upsertGameData(gameData)
    existingGameData.value = gameData

    // 调用Rust后端执行下载命令
    const result = await invoke<{
      success: boolean
      message: string
    }>('start_game_download', {
      manifestPath: manifestFolderPath.value,
      downloadPath: downloadPath.value,
      gameId: gameId.value
    })

    if (result.success) {
      addDownloadLog('下载命令已启动', 'success')
      addDownloadLog(result.message, 'info')
      // 启动进度监控
      startProgressMonitoring()
    } else {
      addDownloadLog(`下载启动失败: ${result.message}`, 'error')
      await updateGameDownloadStatus(gameId.value, 'error', 0)
    }
  } catch (error) {
    addDownloadLog(`下载出错: ${error}`, 'error')
    await updateGameDownloadStatus(gameId.value, 'error', 0)
  } finally {
    isDownloading.value = false
  }
}

/**
 * 停止下载
 * 终止 ddv20.exe 进程，并将游戏状态设置为未下载
 */
const stopDownload = async () => {
  try {
    addDownloadLog('正在停止下载...', 'info')
    await invoke('stop_download', {
      gameId: gameId.value
    })
    addDownloadLog('下载已停止', 'success')

    // 停止进度监控
    stopProgressMonitoring()

    // 重新加载游戏数据以更新状态显示
    existingGameData.value = await getGameData(gameId.value)

    isDownloading.value = false
  } catch (error) {
    addDownloadLog(`停止下载失败: ${error}`, 'error')
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
 * 验证游戏完整性
 * 使用与下载相同的参数重新运行 ddv20.exe，验证并补全缺失文件
 */
const verifyIntegrity = async () => {
  if (!manifestFolderPath.value) {
    alert('未找到清单文件夹')
    return
  }

  if (!existingGameData.value?.download_path) {
    alert('未找到下载路径')
    return
  }

  isVerifying.value = true
  downloadLogs.value = []
  addDownloadLog('开始验证游戏完整性...', 'info')
  addDownloadLog(`游戏: ${game.value?.game_name || gameId.value}`, 'info')
  addDownloadLog(`清单路径: ${manifestFolderPath.value}`, 'info')
  addDownloadLog(`下载路径: ${existingGameData.value.download_path}`, 'info')

  try {
    // 调用与下载相同的命令，ddv20.exe 会自动验证并补全
    const result = await invoke<{
      success: boolean
      message: string
    }>('start_game_download', {
      manifestPath: manifestFolderPath.value,
      downloadPath: existingGameData.value.download_path,
      gameId: gameId.value
    })

    if (result.success) {
      addDownloadLog('验证命令已启动', 'success')
      addDownloadLog(result.message, 'info')
      // 启动进度监控
      startProgressMonitoring()
    } else {
      addDownloadLog(`验证启动失败: ${result.message}`, 'error')
    }
  } catch (error) {
    addDownloadLog(`验证出错: ${error}`, 'error')
  } finally {
    isVerifying.value = false
  }
}

/**
 * 扫描进度文件
 */
const scanProgressFiles = async () => {
  try {
    // 获取指定游戏的进度文件
    const progressFiles = await invoke<Array<{ name: string; path: string }>>('get_download_progress_files', {
      gameId: gameId.value
    })

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

    // 更新下载状态到game.json
    if (existingGameData.value) {
      await updateGameDownloadStatus(gameId.value, downloadProgress.value.isComplete ? 'completed' : 'downloading', overallPercentage)
    }

    // 如果下载完成，停止监控
    if (downloadProgress.value.isComplete) {
      stopProgressMonitoring()
      addDownloadLog('所有depot下载完成！', 'success')
      // 更新游戏数据为已完成
      await updateGameDownloadStatus(gameId.value, 'completed', 100)

      // 执行下载完成收尾：扫描目录、定位 exe、标记已安装
      let finalizedGame = null
      try {
        finalizedGame = await finalizeGameDownload(gameId.value)
        existingGameData.value = finalizedGame
        addDownloadLog('游戏已入库，可前往游戏库查看', 'success')
      } catch (finalizeError) {
        addDownloadLog(`入库处理失败: ${finalizeError}`, 'error')
        existingGameData.value = await getGameData(gameId.value)
      }

      // 如果该游戏存在补丁配置，弹出补丁选择弹窗
      if (patchTabs.value.length > 0) {
        downloadCompleteInstallPath.value = downloadPath.value
        downloadCompleteExePath.value = finalizedGame?.install_path || existingGameData.value?.install_path || downloadPath.value
        showDownloadCompleteModal.value = true
      }
    }
  } catch (error) {
    // 扫描进度文件失败时静默处理
  }
}

/**
 * 跳转并应用指定补丁类型
 * @param tab 补丁标签页
 */
const goToPatchTab = (tab: { id: string; name: string; patchType: number; patchPath: string; downloadUrls: { source: string; url: string; pwd?: string | null }[] }) => {
  // 跳转补丁页时，优先使用 exe 路径；如果不存在，则回退到安装路径
  gamePath.value = downloadCompleteExePath.value || downloadCompleteInstallPath.value || downloadPath.value
  currentTab.value = tab.id
  showDownloadCompleteModal.value = false
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
// 加载封面图片 - 带重试机制确保100%加载成功
const loadCoverImage = async (retryCount = 0): Promise<void> => {
  if (!gameId.value) return

  try {
    const cachedUrl = await getCachedCoverImage(gameId.value)
    if (cachedUrl) {
      coverImage.value = cachedUrl
      return
    }
  } catch {
    // 缓存获取失败，继续尝试直接加载
  }

  // 如果缓存未返回URL，尝试直接调用后端获取路径
  try {
    const { getGameCoverImage } = await import('../../api/game.api')
    const { convertFileSrc } = await import('@tauri-apps/api/core')
    const filePath = await getGameCoverImage(gameId.value)
    if (filePath) {
      coverImage.value = convertFileSrc(filePath)
      return
    }
  } catch {
    // 直接加载也失败
  }

  // 如果都失败了且重试次数小于3次，延迟后重试
  if (retryCount < 3) {
    setTimeout(() => loadCoverImage(retryCount + 1), 300 * (retryCount + 1))
  }
}

/**
 * 加载修改器本地文件内容
 */
const loadTrainerContent = async () => {
  if (!game.value?.trainer?.local_path) {
    return
  }

  try {
    const content = await invoke<string>('read_text_file', {
      filePath: game.value.trainer.local_path
    })
    trainerContent.value = content
  } catch {
    // 读取失败时保持为空字符串
    trainerContent.value = ''
  }
}

// 页面加载时确保游戏数据已加载，并自动检测清单文件夹
onMounted(async () => {
  // 加载游戏配置
  if (gamesConfig.value.length === 0) {
    const config = await safeAsync(() => loadGamesConfigFromFile())
    if (config) gamesConfig.value = config
  }

  // 加载已存在的游戏数据
  const gameData = await safeAsync(() => getGameData(gameId.value))
  if (gameData) {
    // 检查游戏目录是否实际存在（防止用户手动删除游戏文件）
    let isGameDirExists = false
    if (gameData.install_path) {
      try {
        isGameDirExists = await invoke<boolean>('check_directory_exists', {
          path: gameData.install_path
        })
      } catch {
        isGameDirExists = false
      }
    }

    // 如果记录为已下载但实际目录不存在，重置状态
    if (gameData.download_status === 'completed' && !isGameDirExists) {
      gameData.download_status = ''
      gameData.is_installed = false
      // 更新到 game.json
      await safeAsync(() => upsertGameData(gameData))
    }

    existingGameData.value = gameData
    // 恢复下载路径：如果游戏已下载且目录仍存在，使用保存的路径；否则优先使用全局默认下载路径
    const defaultPath = configStore.config?.gameDirs?.defaultDownloadPath
    if (gameData.download_status === 'completed' && isGameDirExists && gameData.download_path) {
      downloadPath.value = gameData.download_path
    } else if (defaultPath) {
      const folderName = sanitizeGameFolderName(gameData.game_name || gameId.value)
      downloadPath.value = `${defaultPath}\\${folderName}`
    } else if (gameData.download_path) {
      downloadPath.value = gameData.download_path
    }
    if (gameData.install_path) {
      gamePath.value = gameData.install_path
    }
    // 如果正在下载中，恢复监控
    if (gameData.download_status === 'downloading') {
      startProgressMonitoring()
    }

    // 补偿处理：之前版本下载完成后可能只设置了 completed 但没有标记 is_installed
    // 如果检测到已完成但未安装，自动执行收尾
    if (gameData.download_status === 'completed' && !gameData.is_installed) {
      try {
        const finalizedGame = await finalizeGameDownload(gameId.value)
        existingGameData.value = finalizedGame
      } catch {
        // 补偿失败时保持原数据，不影响页面显示
      }
    }
  }

  // 加载封面图片 - 使用重试机制确保100%加载
  await loadCoverImage()

  // 设置默认标签页
  setDefaultTab()
  // 自动填充下载路径（不依赖清单文件夹是否存在）
  await autoSetDownloadPath()
  // 自动检测清单文件夹
  await checkManifestFolder()
  // 加载所有补丁说明
  await loadPatchReadmes()
  // 检查夸克网盘二维码是否存在
  quarkQRCodeExists.value = await checkQuarkQRCodeExists(gameId.value)
  // 检查本地补丁文件状态
  await checkPatchLocalStatus()
  // 检查游戏清单文件是否存在（用于入库Steam按钮）
  await checkGameManifest()
  // 加载修改器本地文件内容
  await loadTrainerContent()
})

// 组件卸载时清理定时器
onUnmounted(() => {
  stopProgressMonitoring()
})

/**
 * 检查 manifest 文件夹是否存在
 * 自动查找 resources/manifest/游戏ID 目录
 * 并自动转换格式（Lua -> VDF）
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

      // 扫描并转换清单文件格式（如果没有VDF但有Lua，自动转换）
      const scanResult = await invoke<{
        success: boolean
        hasVdf: boolean
        hasLua: boolean
        hasManifest: boolean
        converted: boolean
        message: string
      }>('scan_and_convert_manifest_for_download', {
        folderPath: manifestPath
      })

      if (scanResult.success) {
        manifestCheckStatus.value = 'found'
        if (scanResult.converted) {
          addDownloadLog(`已自动将Lua转换为VDF格式`, 'success')
        }
        // 自动设置下载路径
        await autoSetDownloadPath()
      } else {
        manifestCheckStatus.value = 'not_found'
        addDownloadLog(`清单文件检查失败: ${scanResult.message}`, 'error')
      }
    } else {
      manifestCheckStatus.value = 'not_found'
    }
  } catch (error) {
    manifestCheckStatus.value = 'not_found'
  }
}

/**
 * 自动设置下载路径
 * 如果游戏已下载且未卸载，使用保存的路径；否则优先使用全局配置的默认下载路径
 * 路径格式：默认路径\游戏英文名 或 D:\SteamGame\游戏英文名
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
    const folderName = sanitizeGameFolderName(rawFolderName)

    // 如果游戏已下载且未卸载，使用保存的路径
    if (existingGameData.value?.download_status === 'completed' && existingGameData.value?.download_path) {
      downloadPath.value = existingGameData.value.download_path
      return
    }

    // 优先使用全局配置中的默认下载路径
    const defaultPath = configStore.config?.gameDirs?.defaultDownloadPath
    if (defaultPath) {
      const path = `${defaultPath}\\${folderName}`
      downloadPath.value = path
    } else {
      // 获取可用的游戏盘符（优先D盘，其次C盘）
      const drive = await invoke<string>('get_available_drive')
      const path = `${drive}\\SteamGame\\${folderName}`
      downloadPath.value = path
    }
  } catch (error) {
    // 自动设置下载路径失败时静默处理
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

/**
 * 检查游戏清单文件状态
 * 有vdf时自动转换为lua，有lua时直接使用
 */
const checkGameManifest = async () => {
  try {
    const result = await invoke<{
      exists: boolean
      hasLua: boolean
      hasVdf: boolean
      hasManifest: boolean
      canImport: boolean
    }>('check_game_manifest_exists', {
      gameId: gameId.value
    })

    manifestExists.value = result.exists
    hasLua.value = result.hasLua
    hasVdf.value = result.hasVdf
    hasManifest.value = result.hasManifest

    // 有vdf但没有lua时，自动转换vdf为lua
    if (result.hasVdf && !result.hasLua) {
      await convertVdfToLuaInManifestFolder()
    }
  } catch (error) {
    manifestExists.value = false
    hasLua.value = false
    hasVdf.value = false
    hasManifest.value = false
  }
}

/**
 * 将内置清单文件夹中的vdf转换为lua
 */
const convertVdfToLuaInManifestFolder = async () => {
  try {
    const manifestPath = await invoke<string>('get_manifest_path', {
      gameId: gameId.value
    })

    if (!manifestPath) return

    const vdfFiles = await invoke<string[]>('get_vdf_files_in_folder', {
      folder: manifestPath
    })

    for (const vdfFile of vdfFiles) {
      try {
        const convertResult = await invoke<{
          success: boolean
          outputPath: string
          message: string
        }>('convert_vdf_to_lua', {
          filePath: vdfFile
        })

        if (convertResult.success) {
          hasLua.value = true
        }
      } catch {
        // 单个文件转换失败继续处理下一个
      }
    }
  } catch {
    // 转换失败时保持当前状态
  }
}

/**
 * 扫描指定文件夹并转换vdf为lua
 * 返回扫描后的文件状态
 */
const scanAndConvertFolder = async (folderPath: string) => {
  hasLua.value = false
  hasVdf.value = false
  hasManifest.value = false

  const scanResult = await invoke<{
    luaFiles: string[]
    manifestFiles: string[]
    vdfFiles: string[]
  }>('scan_manifest_folder', {
    folderPath
  })

  hasLua.value = scanResult.luaFiles.length > 0
  hasVdf.value = scanResult.vdfFiles.length > 0
  hasManifest.value = scanResult.manifestFiles.length > 0

  // 有vdf但没有lua时，自动转换
  if (hasVdf.value && !hasLua.value) {
    for (const vdfFile of scanResult.vdfFiles) {
      try {
        const convertResult = await invoke<{
          success: boolean
          outputPath: string
          message: string
        }>('convert_vdf_to_lua', {
          filePath: vdfFile
        })

        if (convertResult.success) {
          hasLua.value = true
        }
      } catch {
        // 单个文件转换失败继续处理下一个
      }
    }
  }

  return scanResult
}

/**
 * 选择7z压缩包作为清单源
 */
const selectImportArchive = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [
        { name: '7z压缩包', extensions: ['7z'] }
      ],
      title: '选择清单7z压缩包'
    })

    if (!selected || typeof selected !== 'string') {
      return
    }

    selectedImportPath.value = selected
    selectedImportTempPath.value = ''
    isPreparingImport.value = true

    try {
      // 解压到临时目录
      const tempFolder = await invoke<string>('extract_archive', {
        archivePath: selected
      })

      selectedImportTempPath.value = tempFolder

      // 扫描并转换
      await scanAndConvertFolder(tempFolder)

      if (!hasLua.value) {
        alert('未在压缩包中找到vdf或lua文件，无法入库')
      }
    } catch (error) {
      alert(`解压失败: ${error}`)
      selectedImportPath.value = ''
      selectedImportTempPath.value = ''
    } finally {
      isPreparingImport.value = false
    }
  } catch (error) {
    alert(`选择文件失败: ${error}`)
  }
}

/**
 * 选择lua所在文件夹作为清单源
 */
const selectImportFolder = async () => {
  try {
    const selected = await open({
      directory: true,
      title: '选择包含lua/vdf文件的文件夹'
    })

    if (!selected || typeof selected !== 'string') {
      return
    }

    selectedImportPath.value = selected
    selectedImportTempPath.value = ''
    isPreparingImport.value = true

    try {
      // 扫描并转换
      await scanAndConvertFolder(selected)

      if (!hasLua.value) {
        alert('未在文件夹中找到vdf或lua文件，无法入库')
      }
    } catch (error) {
      alert(`扫描失败: ${error}`)
      selectedImportPath.value = ''
    } finally {
      isPreparingImport.value = false
    }
  } catch (error) {
    alert(`选择文件夹失败: ${error}`)
  }
}

/**
 * 清除自定义清单源选择
 */
const clearImportSource = () => {
  selectedImportPath.value = ''
  selectedImportTempPath.value = ''
  // 重新检查内置清单
  checkGameManifest()
}

/**
 * 选择 7z 压缩包作为游戏下载的清单源
 * 解压到 resources/manifest/{game_id}/ 后重新检测
 */
const selectDownloadManifestArchive = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [
        { name: '7z压缩包', extensions: ['7z'] }
      ],
      title: '选择清单7z压缩包'
    })

    if (!selected || typeof selected !== 'string') {
      return
    }

    selectedDownloadManifestPath.value = selected
    isPreparingDownloadManifest.value = true

    try {
      addDownloadLog(`正在解压清单压缩包...`, 'info')
      const targetFolder = await invoke<string>('extract_manifest_archive', {
        archivePath: selected,
        gameId: gameId.value
      })

      addDownloadLog(`清单已解压到: ${targetFolder}`, 'success')

      // 记录手动导入的清单游戏ID到缓存
      try {
        await invoke('add_imported_manifest_game_id', {
          gameId: gameId.value
        })
      } catch {
        // 清单导入缓存失败不影响主流程
      }

      // 重新检测清单文件夹状态
      await checkManifestFolder()
    } catch (error) {
      addDownloadLog(`解压清单压缩包失败: ${error}`, 'error')
      alert(`解压失败: ${error}`)
      selectedDownloadManifestPath.value = ''
    } finally {
      isPreparingDownloadManifest.value = false
    }
  } catch (error) {
    alert(`选择文件失败: ${error}`)
  }
}

/**
 * 选择 lua 所在文件夹作为游戏下载的清单源
 * 直接复制/使用 resources/manifest/{game_id}/ 目录
 */
const selectDownloadManifestFolder = async () => {
  try {
    const selected = await open({
      directory: true,
      title: '选择包含lua/vdf文件的文件夹'
    })

    if (!selected || typeof selected !== 'string') {
      return
    }

    selectedDownloadManifestPath.value = selected
    isPreparingDownloadManifest.value = true

    try {
      addDownloadLog(`正在复制清单文件夹...`, 'info')
      const targetFolder = await invoke<string>('copy_folder_to_manifest', {
        sourcePath: selected,
        gameId: gameId.value
      })

      addDownloadLog(`清单已复制到: ${targetFolder}`, 'success')
      // 重新检测清单文件夹状态
      await checkManifestFolder()
    } catch (error) {
      addDownloadLog(`复制清单文件夹失败: ${error}`, 'error')
      alert(`复制失败: ${error}`)
      selectedDownloadManifestPath.value = ''
    } finally {
      isPreparingDownloadManifest.value = false
    }
  } catch (error) {
    alert(`选择文件夹失败: ${error}`)
  }
}

/**
 * 使用OpenSteamTool内核导入游戏到Steam
 */
const importWithOpenSteamTool = async () => {
  if (isImportingWithOpenSteamTool.value) return

  if (!game.value) {
    alert('游戏数据未加载')
    return
  }

  // 解析AppID
  const appId = parseInt(game.value.game_id, 10)
  if (isNaN(appId) || appId <= 0) {
    alert('游戏ID无效，无法作为Steam AppID使用')
    return
  }

  // 检查Steam路径
  let steamPath = configStore.config?.gameDirs?.steamPath
  if (!steamPath) {
    const selected = await open({
      directory: true,
      title: '请选择Steam安装目录'
    })

    if (!selected) {
      return
    }

    steamPath = selected
    await configStore.updateConfig({
      gameDirs: {
        steamPath: selected,
        coversPath: configStore.config?.gameDirs?.coversPath || 'data/covers'
      }
    })
  }

  // 高级模式确认
  const advancedMode = configStore.config?.opensteamtool?.advancedMode ?? false
  const hotReload = configStore.config?.opensteamtool?.hotReload ?? true
  if (advancedMode) {
    const confirmAdvanced = confirm(
      '高级模式已启用，将写入Windows注册表。\n\n' +
      '这通常用于Denuvo/在线游戏，但也可能带来风险。\n\n' +
      '是否继续？'
    )
    if (!confirmAdvanced) {
      return
    }
  }

  isImportingWithOpenSteamTool.value = true

  try {
    let result: {
      success: boolean
      message: string
      kernelInstalled: boolean
      luaWritten: boolean
      manifestCopied: number
      steamRestarted: boolean
      advancedEnabled: boolean
    }

    if (importSourceReady.value) {
      // 使用自定义清单源
      const folderPath = importSourceMode.value === '7z'
        ? selectedImportTempPath.value
        : selectedImportPath.value

      result = await invoke<{
        success: boolean
        message: string
        kernelInstalled: boolean
        luaWritten: boolean
        manifestCopied: number
        steamRestarted: boolean
        advancedEnabled: boolean
      }>('import_manifest_with_opensteamtool', {
        steamPath: steamPath,
        folderPath: folderPath,
        gameName: game.value.chinese_name || game.value.game_name || gameId.value,
        appId: appId,
        advancedMode: advancedMode,
        hotReload: hotReload,
        lockVersion: lockVersion.value
      })
    } else {
      // 使用内置清单
      result = await invoke<{
        success: boolean
        message: string
        kernelInstalled: boolean
        luaWritten: boolean
        manifestCopied: number
        steamRestarted: boolean
        advancedEnabled: boolean
      }>('import_game_with_opensteamtool', {
        steamPath: steamPath,
        gameId: gameId.value,
        gameName: game.value.chinese_name || game.value.game_name || gameId.value,
        appId: appId,
        advancedMode: advancedMode,
        hotReload: hotReload,
        lockVersion: lockVersion.value
      })
    }

    if (result.success) {
      const message =
        `入库成功，请打开 Steam 的库中查看游戏！\n\n` +
        `游戏: ${game.value.chinese_name || game.value.game_name}\n` +
        `AppID: ${appId}`
      alert(message)
    } else {
      alert(`OpenSteamTool入库失败: ${result.message}`)
    }
  } catch (error) {
    alert(`OpenSteamTool入库失败: ${error}`)
  } finally {
    isImportingWithOpenSteamTool.value = false
  }
}

/**
 * 重启Steam
 */
const restartSteam = async () => {
  try {
    const result = await invoke<{
      success: boolean
      message: string
    }>('restart_steam')

    if (result.success) {
      alert('Steam重启成功！')
    } else {
      alert(`重启Steam失败: ${result.message}`)
    }
  } catch (error) {
    alert(`重启Steam失败: ${error}`)
  }
}
</script>

<style scoped>
.game-detail-page {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  padding: 14px;
  background-color: var(--steam-bg-secondary);
}

/* 头部区域 */
.detail-header {
  display: flex;
  align-items: center;
  gap: 24px;
  margin-bottom: 14px;
  padding-bottom: 10px;
  border-bottom: 1px solid var(--steam-border);
}

.back-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  border: none;
  border-radius: 8px;
  background-color: var(--steam-bg-primary);
  color: var(--steam-text-primary);
  font-size: 14px;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.back-btn:hover {
  background-color: var(--steam-bg-hover);
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
  color: var(--steam-text-primary);
  margin: 0;
}

.game-id {
  font-size: 13px;
  color: var(--steam-text-secondary);
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

/* 主要内容区 */
.detail-content {
  display: flex;
  gap: 24px;
  margin-bottom: 14px;
  align-items: center;
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
  color: var(--steam-text-primary);
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
  background-color: var(--steam-bg-primary);
  color: var(--steam-text-primary);
  font-size: 13px;
  outline: none;
}

.browse-btn {
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  background-color: var(--steam-accent-blue);
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.browse-btn:hover {
  background-color: var(--steam-accent-hover);
}

/* 标签页区域 */
.tabs-section {
  background-color: var(--steam-bg-primary);
  border-radius: 12px;
  overflow: hidden;
}

.tabs-header {
  display: flex;
  border-bottom: 1px solid var(--steam-border);
}

.tab-btn {
  padding: 12px 24px;
  border: none;
  background-color: transparent;
  color: var(--steam-text-secondary);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  border-bottom: 2px solid transparent;
}

.tab-btn:hover {
  color: var(--steam-text-primary);
  background-color: var(--steam-bg-hover);
}

.tab-btn.active {
  color: var(--steam-accent-blue);
  border-bottom-color: var(--steam-accent-blue);
  background-color: var(--steam-bg-secondary);
}

/* 补丁类标签页使用红色粗体显示 */
.tab-btn.patch-tab {
  color: #ef4444;
  font-weight: 700;
}

.tab-btn.patch-tab:hover {
  color: #f87171;
  background-color: rgba(239, 68, 68, 0.1);
}

.tab-btn.patch-tab.active {
  color: #ef4444;
  border-bottom-color: #ef4444;
  background-color: rgba(239, 68, 68, 0.08);
}

.tabs-content {
  padding: 14px;
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
  color: var(--steam-text-primary);
  margin: 0 0 10px 0;
}

/* 下载完成提示 */
.download-completed-notice {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px;
  background-color: rgba(16, 185, 129, 0.1);
  border-radius: 8px;
  border-left: 4px solid #10b981;
  margin-bottom: 12px;
}

.success-icon {
  width: 48px;
  height: 48px;
  color: #10b981;
  flex-shrink: 0;
}

.success-icon svg {
  width: 100%;
  height: 100%;
}

.success-text h4 {
  margin: 0 0 5px 0;
  font-size: 16px;
  color: #10b981;
}

.success-text p {
  margin: 0 0 3px 0;
  font-size: 13px;
  color: var(--steam-text-secondary);
}

.success-text .patch-hint {
  margin-top: 8px;
  font-size: 15px;
  font-weight: 700;
  color: #ef4444;
}

/* 通用红色加粗高亮文本 */
.highlight-red-bold {
  color: #ef4444;
  font-weight: 700;
}

/* 验证完整性按钮 - 样式与开始下载按钮一致 */
.verify-integrity-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px 32px;
  border: none;
  border-radius: 8px;
  background-color: var(--steam-accent-blue);
  color: white;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: background-color 0.15s ease;
  margin-left: auto;
}

.verify-integrity-btn:hover:not(:disabled) {
  background-color: var(--steam-accent-hover);
}

.verify-integrity-btn:disabled {
  background-color: var(--steam-text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

.verify-integrity-btn svg {
  width: 18px;
  height: 18px;
}

.download-info {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-bottom: 14px;
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
  font-size: 12px;
}

.download-path-section {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.path-display {
  padding: 12px 16px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  font-size: 13px;
  color: var(--steam-text-secondary);
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
  background-color: var(--steam-accent-blue);
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
  background-color: var(--steam-accent-hover);
}

.start-download-btn.disabled {
  background-color: var(--steam-text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

.start-download-btn.loading {
  cursor: wait;
}

/* 下载按钮组 */
.download-btn-group {
  display: flex;
  gap: 7px;
  align-items: center;
  position: relative;
  flex-wrap: wrap;
}

/* 圆形下载进度条 */
.circular-progress {
  position: relative;
  width: 40px;
  height: 40px;
  flex-shrink: 0;
}

.circular-progress svg {
  width: 100%;
  height: 100%;
  transform: rotate(-90deg);
}

.circle-bg {
  fill: none;
  stroke: var(--steam-border);
  stroke-width: 3;
}

.circle-progress {
  fill: none;
  stroke: var(--steam-accent-blue);
  stroke-width: 3;
  stroke-linecap: round;
  transition: stroke-dasharray 0.3s ease;
}

.progress-text {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 11px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

/* 暂停下载按钮 */
.stop-download-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px 24px;
  border: none;
  border-radius: 8px;
  background-color: #dc3545;
  color: white;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.stop-download-btn svg {
  width: 18px;
  height: 18px;
}

.stop-download-btn:hover {
  background-color: #c82333;
}

/* 下载说明样式 */
.download-description {
  margin: 12px 0;
  padding: 10px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  border-left: 4px solid #3b82f6;
}

.download-option {
  margin-bottom: 10px;
}

.download-option:last-child {
  margin-bottom: 0;
}

.download-option h4 {
  margin: 0 0 5px 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.download-option p {
  margin: 0;
  font-size: 13px;
  color: var(--steam-text-secondary);
  line-height: 1.5;
}

/* 解压即玩标签页样式 */
.extract-play-content {
  padding: 12px;
}

.extract-play-description {
  margin-bottom: 14px;
  padding: 10px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  border-left: 4px solid #8b5cf6;
}

.extract-play-description p {
  margin: 0 0 5px 0;
  font-size: 14px;
  color: var(--steam-text-primary);
  line-height: 1.6;
}

.extract-play-description p:last-child {
  margin-bottom: 0;
}

.extract-play-note {
  font-size: 13px;
  color: var(--steam-text-secondary);
  font-style: italic;
}

.loading-icon {
  animation: spin 1s linear infinite;
}

.path-hint {
  margin: 3px 0 0 0;
  font-size: 12px;
  color: var(--steam-text-secondary);
  font-style: italic;
}

/* 下载日志样式 */
.download-logs {
  margin-top: 12px;
  padding: 10px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--steam-border);
}

.logs-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 7px 0;
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
  gap: 12px;
  padding: 2px 0;
}

.log-time {
  color: var(--steam-text-muted);
  flex-shrink: 0;
}

.log-message {
  color: var(--steam-text-primary);
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
  margin-top: 12px;
}

.patch-info {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

/* 补丁说明样式 - 显示在应用补丁按钮下方 */
.patch-readme {
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  padding: 10px;
  border-left: 4px solid var(--steam-accent-blue);
  margin-top: 10px;
}

.readme-title {
  margin: 0 0 7px 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.readme-content {
  margin: 0;
  padding: 0;
  font-size: 13px;
  line-height: 1.6;
  color: var(--steam-text-secondary);
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
  gap: 5px;
  padding: 10px 14px;
  border-radius: 8px;
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
  font-size: 13px;
  font-weight: 500;
  margin-bottom: 7px;
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

/* 下载区域样式 */
.download-section {
  margin-bottom: 10px;
}

.download-section-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
  margin: 0 0 7px 0;
}

.download-buttons {
  display: flex;
  flex-wrap: wrap;
  align-items: flex-start;
  gap: 10px;
  margin-bottom: 7px;
}

.backup-label {
  font-size: 13px;
  color: var(--steam-text-secondary);
  white-space: nowrap;
  margin-left: 6px;
}

.download-btn-wrapper {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 7px;
}

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
}

.download-patch-btn:hover {
  background-color: #2563eb;
}

.download-patch-btn svg {
  width: 18px;
  height: 18px;
}

.pwd-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 0;
}

/* 下载提示文字 */
.download-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 4px 0 7px 0;
  font-style: italic;
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
  margin-bottom: 7px;
}

.select-and-apply-btn:hover:not(:disabled) {
  background-color: #7c3aed;
}

.select-and-apply-btn:disabled {
  background-color: var(--steam-text-secondary);
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
  color: var(--steam-text-secondary);
  font-family: 'Courier New', monospace;
}

.game-path-display {
  margin: 0;
  font-size: 13px;
  color: var(--steam-text-secondary);
  font-family: 'Courier New', monospace;
}

.game-path-display.warning {
  color: #f59e0b;
}

.apply-patch-btn {
  padding: 10px 24px;
  border: none;
  border-radius: 8px;
  background-color: var(--steam-accent-blue);
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.15s ease;
  align-self: flex-start;
}

.apply-patch-btn:hover:not(:disabled) {
  background-color: var(--steam-accent-hover);
}

.apply-patch-btn:disabled {
  background-color: var(--steam-text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

/* 虚拟化环境配置教程按钮 */
.tutorial-btn {
  margin-top: 7px;
  padding: 12px 24px;
  background: rgba(156, 39, 176, 0.2);
  color: #ce93d8;
  border: 1px solid rgba(156, 39, 176, 0.5);
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  align-self: flex-start;
}

.tutorial-btn:hover {
  background: rgba(156, 39, 176, 0.3);
  border-color: rgba(156, 39, 176, 0.7);
}

/* 补丁结果提示 */
.patch-result {
  margin-top: 10px;
  padding: 10px;
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
}

.patch-result.success {
  border-left: 4px solid #10b981;
}

.patch-result.error {
  border-left: 4px solid #ef4444;
}

.result-title {
  margin: 0 0 7px 0;
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
  margin-bottom: 7px;
}

.result-section:last-child {
  margin-bottom: 0;
}

.section-title {
  margin: 0 0 5px 0;
  font-size: 12px;
  font-weight: 500;
  color: var(--steam-text-secondary);
}

.file-list {
  margin: 0;
  padding-left: 16px;
  font-size: 12px;
  color: var(--steam-text-secondary);
}

.file-list li {
  margin-bottom: 2px;
}

.error-section .section-title {
  color: #ef4444;
}

.error-list {
  color: #ef4444;
}

/* 修改器标签页样式 */
.trainer-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.trainer-local-content {
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  padding: 10px;
  border-left: 4px solid #f59e0b;
}

.trainer-content-title {
  margin: 0 0 7px 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.trainer-content-text {
  margin: 0;
  padding: 0;
  font-size: 13px;
  line-height: 1.6;
  color: var(--steam-text-secondary);
  white-space: pre-wrap;
  word-wrap: break-word;
  font-family: inherit;
  background: none;
  border: none;
  max-height: 400px;
  overflow-y: auto;
}

.trainer-no-content {
  padding: 12px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  text-align: center;
  color: var(--steam-text-secondary);
}

.trainer-no-content p {
  margin: 0 0 5px 0;
}

.trainer-path {
  font-size: 12px;
  font-family: 'Courier New', monospace;
  color: var(--steam-text-muted);
}

/* 清单下载详细说明框 */
.manifest-guide-box {
  background: rgba(102, 192, 244, 0.08);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  padding: 12px 16px;
  margin-bottom: 10px;
}

.manifest-guide-box .guide-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 6px 0;
}

.manifest-guide-box .guide-title:not(:first-child) {
  margin-top: 10px;
}

.manifest-guide-box .guide-text {
  font-size: 13px;
  color: var(--steam-text-secondary);
  line-height: 1.6;
  margin: 0;
}

.manifest-guide-box .guide-steps {
  margin: 6px 0 0 0;
  padding-left: 20px;
  color: var(--steam-text-secondary);
  font-size: 13px;
  line-height: 1.7;
}

.manifest-guide-box .guide-steps li {
  margin-bottom: 6px;
}

.manifest-guide-box .guide-steps strong {
  color: var(--steam-accent-blue);
}

/* 补丁操作详细说明框 */
.patch-guide-box {
  background: rgba(102, 192, 244, 0.08);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  padding: 12px 16px;
  margin-bottom: 10px;
}

.patch-guide-box .guide-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 6px 0;
}

.patch-guide-box .guide-title:not(:first-child) {
  margin-top: 10px;
}

.patch-guide-box .guide-text {
  font-size: 13px;
  color: var(--steam-text-secondary);
  line-height: 1.6;
  margin: 0;
}

.patch-guide-box .guide-steps {
  margin: 6px 0 0 0;
  padding-left: 20px;
  color: var(--steam-text-secondary);
  font-size: 13px;
  line-height: 1.7;
}

.patch-guide-box .guide-steps li {
  margin-bottom: 6px;
}

.patch-guide-box .guide-steps strong {
  color: var(--steam-accent-blue);
}

.patch-guide-box .sub-steps {
  margin: 4px 0 0 0;
  padding-left: 20px;
  font-size: 12px;
  line-height: 1.6;
}

.patch-guide-box .sub-steps li {
  margin-bottom: 3px;
}

.download-guide {
  display: flex;
  flex-direction: column;
  gap: 7px;
}

.download-guide-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
  margin: 0;
}

/* 下载完成弹窗 - 宽度40%，最大高度50%，内容紧凑 */
:global(.download-complete-modal) {
  width: 40% !important;
  max-width: 40% !important;
  height: auto !important;
  max-height: 50vh !important;
}

.download-complete-modal-content {
  display: flex;
  flex-direction: column;
  gap: 10px;
  justify-content: flex-start;
}

.modal-main-hint {
  margin: 0;
  font-size: 15px;
  color: var(--steam-text-primary);
  line-height: 1.3;
}

.modal-path-hint {
  margin: 0;
  font-size: 13px;
  color: var(--steam-text-secondary);
}

.modal-path-hint code {
  padding: 2px 6px;
  background-color: var(--steam-bg-tertiary);
  border-radius: 4px;
  font-family: 'Courier New', monospace;
  word-break: break-all;
}

.patch-select-section {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.patch-select-title {
  margin: 0;
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

.patch-select-buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  width: 100%;
}

.patch-select-btn {
  flex: 1 1 calc(50% - 3px);
  min-width: 100px;
  padding: 8px 12px;
  border: none;
  border-radius: 6px;
  color: white;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: opacity 0.15s ease, transform 0.15s ease;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.patch-select-btn:hover {
  opacity: 0.85;
  transform: translateY(-1px);
}

.no-patch-hint {
  margin: 0;
  font-size: 13px;
  color: var(--steam-text-muted);
}
</style>