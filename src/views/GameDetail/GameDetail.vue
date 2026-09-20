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
      <button class="back-btn ui-btn ui-btn-sm ui-btn-outline" @click="goBack">
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
            <button class="browse-btn ui-btn ui-btn-md ui-btn-outline" @click="selectDownloadPath">浏览</button>
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
            <button class="browse-btn ui-btn ui-btn-md ui-btn-outline" @click="selectGamePath">浏览</button>
          </div>
        </div>
      </div>
    </div>

    <!-- 底部：标签页（所有标签都被配置关闭时不显示） -->
    <div v-if="availableTabs.length > 0" class="tabs-section">
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
          :is-updating="isUpdating"
          :is-monitoring="isMonitoring"
          :is-paused="isPausedDownload"
          :download-speed="downloadSpeed"
          :download-done-bytes="downloadBytes.doneBytes"
          :download-total-bytes="downloadBytes.totalBytes"
          :download-progress="downloadProgress"
          :download-logs="downloadLogs"
          :download-manifest-source-mode="downloadManifestSourceMode"
          :selected-download-manifest-path="selectedDownloadManifestPath"
          :is-preparing-download-manifest="isPreparingDownloadManifest"
          @select-manifest-archive="selectDownloadManifestArchive"
          @select-manifest-folder="selectDownloadManifestFolder"
          @update:download-manifest-source-mode="downloadManifestSourceMode = $event"
          @start-download="startDownload"
          @stop-download="stopDownload"
          @open-qingdan-qr-code="openQingdanQRCode"
          @open-download-url="openDownloadUrl"
          @verify-integrity="verifyIntegrity"
          @update-game="handleUpdateGame"
          @replace-manifest="handleReplaceManifest"
        />

        <!-- 补丁配置标签页 -->
        <PatchConfigTab
          :currentTab="currentTab"
          :patchTabs="patchTabs"
          :gamePath="gamePath"
          :applyingPatch="applyingPatch"
          :patchResult="patchResult"
          :patchLocalStatus="patchLocalStatus"
          :patchAppliedFromFile="patchAppliedFromFile"
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
          :manifestCheckStatus="manifestCheckStatus"
          :importSourceMode="importSourceMode"
          :selectedImportPath="selectedImportPath"
          @importWithOpenSteamTool="importWithOpenSteamTool"
          @restartSteam="restartSteam"
          @selectImportArchive="selectImportArchive"
          @selectImportFolder="selectImportFolder"
          @clearImportSource="clearImportSource"
          @update:lockVersion="lockVersion = $event"
          @update:importSourceMode="importSourceMode = $event"
          @replace-manifest="handleReplaceManifest"
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
                    class="download-patch-btn ui-btn ui-btn-md ui-btn-primary"
                    :class="{ disabled: extractPlayStatus !== 'available' }"
                    :disabled="extractPlayStatus !== 'available'"
                    @click="openQuarkQRCode"
                    :title="extractPlayTitle"
                  >
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                      <polyline points="7 10 12 15 17 10"/>
                      <line x1="12" y1="15" x2="12" y2="3"/>
                    </svg>
                    夸克网盘
                  </button>
                  <span v-if="extractPlayStatus === 'unavailable'" class="pwd-hint">暂未上传完游戏</span>
                  <span v-else-if="extractPlayStatus === 'loading'" class="pwd-hint">加载中...</span>
                  <span v-else-if="extractPlayStatus === 'error'" class="pwd-hint">加载失败，请检查网络</span>
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
                    class="download-patch-btn ui-btn ui-btn-md ui-btn-primary"
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

    <!-- 更新游戏弹窗：选择新版清单 → 开始更新（增量下载） -->
    <BaseModal
      v-model="showUpdateModal"
      title="更新游戏"
      width="40%"
      container-class="update-game-modal"
      :show-confirm="false"
      :show-cancel="true"
      cancel-text="取消"
      @close="showUpdateModal = false"
    >
      <template #body>
        <div class="update-game-modal-content">
          <p class="modal-main-hint">更新游戏：导入新版本清单后开始更新，程序只下载与本地不一致的文件（未变化的文件自动跳过）。</p>

          <!-- 新版清单来源模式 -->
          <div class="update-mode-select">
            <label class="radio-label">
              <input
                type="radio"
                value="7z"
                v-model="updateManifestMode"
              />
              <span>7z 清单压缩包</span>
            </label>
            <label class="radio-label">
              <input
                type="radio"
                value="folder"
                v-model="updateManifestMode"
              />
              <span>清单文件夹（.vdf / .lua + .manifest）</span>
            </label>
          </div>

          <!-- 新版清单就绪状态 -->
          <div class="update-list-status">
            <span class="update-status-label">新版清单：</span>
            <span v-if="updateManifestReady" class="update-status-ready">{{ updateManifestPath }}</span>
            <span v-else class="update-status-muted">尚未选择（选择后会替换当前清单）</span>
          </div>

          <!-- 两个操作按钮 -->
          <div class="update-actions">
            <button
              class="update-select-btn ui-btn ui-btn-lg ui-btn-outline"
              :disabled="isPreparingUpdateManifest"
              @click="handleSelectUpdateManifest"
            >
              {{ isPreparingUpdateManifest ? '处理中...' : '选择新版清单' }}
            </button>
            <button
              class="update-start-btn ui-btn ui-btn-lg ui-btn-primary"
              :class="{ loading: isUpdating }"
              :disabled="!updateManifestReady || isUpdating"
              @click="handleStartUpdate"
            >
              {{ isUpdating ? '更新中...' : '开始更新' }}
            </button>
          </div>
        </div>
      </template>
    </BaseModal>

    <!-- 入库成功弹窗 -->
    <BaseModal
      v-model="showImportSuccessModal"
      title="入库成功"
      width="300px"
      container-class="import-success-modal"
      :show-confirm="true"
      :show-cancel="false"
      confirm-text="确定"
      @confirm="showImportSuccessModal = false"
      @close="showImportSuccessModal = false"
    >
      <template #body>
        <div class="import-success-modal-content">
          <p class="modal-main-hint">{{ importSuccessMessage }}</p>
          <div class="import-success-details">
            <p><span class="detail-label">游戏：</span>{{ importSuccessGameName }}</p>
            <p><span class="detail-label">AppID：</span>{{ importSuccessAppId }}</p>
          </div>
          <p class="modal-sub-hint">请打开 Steam 的库中查看游戏。</p>
        </div>
      </template>
    </BaseModal>

    <!-- 清单文件包误当补丁提示弹窗：用户把 Steam 清单包（lua/vdf/manifest/txt）当作补丁应用时弹出 -->
    <BaseModal
      v-model="showManifestPatchAlert"
      title="提示"
      width="420px"
      :show-confirm="true"
      :show-cancel="false"
      confirm-text="知道了"
      @confirm="closeManifestPatchAlert"
      @close="closeManifestPatchAlert"
    >
      <template #body>
        <div class="manifest-patch-alert-content">
          <p class="modal-main-hint">检测到您选择的文件是「Steam 清单文件包」，不是游戏补丁，已取消应用，未写入任何文件。</p>
          <p class="modal-path-hint">请到补丁页面点击网盘下载按钮，下载真正的补丁后再来应用。</p>
        </div>
      </template>
    </BaseModal>

    <!-- Xbox 联机补丁兼容性提示弹窗：点进 Xbox 补丁标签页时弹出 -->
    <BaseModal
      v-model="showXboxPatchNotice"
      title="提示"
      width="420px"
      :show-confirm="true"
      :show-cancel="false"
      confirm-text="知道了"
      :close-on-overlay="false"
      @confirm="closeXboxPatchNotice"
      @close="closeXboxPatchNotice"
    >
      <template #body>
        <div class="xbox-patch-notice-content">
          <p class="modal-main-hint">
            该补丁只兼容微软商店游戏本体（不兼容steam下载的游戏本体），没有微软商店本体的请去解压即玩界面下载已经配置好了的游戏文件，解压后按照必看教程进行操作
          </p>
        </div>
      </template>
    </BaseModal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/plugin-dialog'
import { convertFileSrc } from '@tauri-apps/api/core'
import QRCodeModal from '../../components/common/QRCodeModal.vue'
import BaseModal from '../../components/common/BaseModal.vue'
import GameImportTab from './components/GameImportTab.vue'
import GameDownloadTab from './components/GameDownloadTab.vue'
import PatchConfigTab from './components/PatchConfigTab.vue'
import { usePatch } from './composables/usePatch'
import type { GameConfigData } from '../../types'
import { getPatchSourcePath } from '../../types'
import { loadGamesConfigFromFile } from '../../api/game.api'
import { getExtractPlayImage } from '../../api/extractPlay.api'
import { getCachedCoverImage } from '../../services/imageCache.service'
import { 
  getGameData, 
  upsertGameData, 
  updateGameDownloadStatus,
  finalizeGameDownload,
  deleteGameLogCache,
  type GameData 
} from '../../api/gameData.api'
import { getCategoryName, getCategoryColor } from '../../constants/game'
import { safeAsync } from '../../utils/async-helper'
import { sanitizeGameFolderName } from '../../utils/file-helper'
import { useConfigStore } from '../../store/config.store'
import { useDownloadManagerStore } from '../../store'

const route = useRoute()
const router = useRouter()
const configStore = useConfigStore()
const downloadManagerStore = useDownloadManagerStore()

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

// Readme 云端更新事件监听句柄（组件卸载时注销）
let unlistenReadmeUpdate: UnlistenFn | null = null

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
  } else if (game.value?.has_steam_import !== false) {
    // 以上都不满足时，若有入库Steam选项，默认选中它
    // 覆盖"游戏仅有入库Steam选项"的情况
    currentTab.value = 'import'
  } else if (availableTabs.value.length > 0) {
    // 无入库Steam选项时，默认选中第一个可用的其他标签
    currentTab.value = availableTabs.value[0].id
  } else {
    // 所有标签都被关闭时，不选中任何标签
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
  patchAppliedFromFile,
  showManifestPatchAlert,
  checkPatchLocalStatus,
  applyPatch,
  selectAndApplyPatch,
  loadPatchReadmes,
  openVirtualizationTutorial,
  openDownloadUrl,
  closeManifestPatchAlert
} = usePatch(game, gamePath)

// 下载状态（使用全局 store 按 game_id 管理，支持多游戏同时下载和页面切换状态保持）
const isDownloading = ref(false)
const isVerifying = ref(false)
/** 是否正在更新游戏（选择新清单 → 增量下载） */
const isUpdating = ref(false)
/** 更新游戏弹窗：是否显示 */
const showUpdateModal = ref(false)
/** 更新弹窗-清单源模式（7z / 文件夹） */
const updateManifestMode = ref<'7z' | 'folder'>('7z')
/** 更新弹窗-新版清单是否已导入就绪 */
const updateManifestReady = ref(false)
/** 更新弹窗-新版清单导入后的路径 */
const updateManifestPath = ref('')
/** 更新弹窗-正在导入新版清单 */
const isPreparingUpdateManifest = ref(false)
const downloadLogs = computed(() => downloadManagerStore.getGameLogs(gameId.value))

// 下载进度监控（从全局 store 读取）
const isMonitoring = computed(() => downloadManagerStore.isGameMonitoring(gameId.value))
const downloadProgress = computed(() => downloadManagerStore.getGameProgress(gameId.value))

// 是否处于暂停态（store 内存标记 或 game.json 已记录 paused，双保险）
const isPausedDownload = computed(
  () =>
    downloadManagerStore.isGamePaused(gameId.value) ||
    existingGameData.value?.download_status === 'paused'
)

// 该游戏实时下载速度（字节/秒），供"下载进度"头部展示
const downloadSpeed = computed(() => downloadManagerStore.getGameSpeed(gameId.value))

// 该游戏本次下载的已下载/总字节数，供"下载进度"头部估算剩余时间
const downloadBytes = computed(() => downloadManagerStore.getGameBytes(gameId.value))

// 入库成功弹窗
const showImportSuccessModal = ref(false)
const importSuccessMessage = ref('')
const importSuccessGameName = ref('')
const importSuccessAppId = ref<number | string>('')

// 入库Steam状态
const isImportingWithOpenSteamTool = ref(false)
const manifestExists = ref(false)
const hasLua = ref(false)
const hasVdf = ref(false)
const hasManifest = ref(false)

// OpenSteamTool高级选项
// Steam 接口加密后仅支持锁定版本入库，固定为 true，界面上开关置灰不可切换
const lockVersion = ref(true)

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

// Xbox 联机补丁兼容性提示弹窗状态
const showXboxPatchNotice = ref(false)

/**
 * 关闭 Xbox 补丁兼容性提示弹窗
 */
const closeXboxPatchNotice = () => {
  showXboxPatchNotice.value = false
}

// 解压即玩图片加载状态：loading=加载中 / available=可下载 / unavailable=暂未上传 / error=加载失败
const extractPlayStatus = ref<'loading' | 'available' | 'unavailable' | 'error'>('loading')
// 解压即玩图片本地缓存路径
const extractPlayImagePath = ref('')

// 解压即玩按钮的提示文字
const extractPlayTitle = computed(() => {
  switch (extractPlayStatus.value) {
    case 'available':
      return '点击扫码下载'
    case 'loading':
      return '加载中...'
    case 'error':
      return '加载失败，请检查网络后重试'
    default:
      return '暂未上传完游戏'
  }
})

/**
 * 拉取当前游戏的解压即玩图片
 * 后端会优先复用本会话已下载的本地缓存，否则实时从 Gitee 下载。
 * 进入解压即玩标签页或切换标签页回到该页时都会重新调用，实现实时刷新。
 */
const fetchExtractPlayImage = async (gameId: string) => {
  extractPlayStatus.value = 'loading'
  try {
    const result = await getExtractPlayImage(gameId)
    if (result.exists && result.path) {
      extractPlayImagePath.value = result.path
      extractPlayStatus.value = 'available'
    } else {
      extractPlayStatus.value = 'unavailable'
    }
  } catch {
    // 网络异常或仓库不可达时，标记为加载失败，允许用户切回标签页重试
    extractPlayStatus.value = 'error'
  }
}

// 切换到解压即玩标签页时重新拉取图片，实现实时刷新与失败重试
watch(currentTab, (tab) => {
  if (tab === 'extract-play') {
    fetchExtractPlayImage(gameId.value)
  }
  // 点进 Xbox 联机补丁标签页（patch_type=5 → tab id 为 patch-5）时弹出兼容性提示
  if (tab === 'patch-5') {
    showXboxPatchNotice.value = true
  }
})

/**
 * 打开夸克网盘二维码弹窗
 */
const openQuarkQRCode = () => {
  // 仅当图片加载成功时才允许打开
  if (extractPlayStatus.value !== 'available' || !extractPlayImagePath.value) {
    return
  }

  // 设置弹窗内容
  qrCodeTitle.value = '夸克网盘下载'
  qrCodeHint.value = '请使用夸克APP扫码下载'

  // 使用本地缓存的图片路径（convertFileSrc 转换为可访问的 URL）
  qrCodeImageUrl.value = convertFileSrc(extractPlayImagePath.value)

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

  // 3. 入库Steam标签（可通过 games_config.json 的 has_steam_import 关闭显示，缺省为 true 兼容旧版配置）
  if (game.value?.has_steam_import !== false) {
    tabs.push({ id: 'import', name: '入库Steam' })
  }

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
  downloadManagerStore.clearLogs(gameId.value) // 清空之前的日志

  addDownloadLog('开始下载游戏...', 'info')
  addDownloadLog(`游戏: ${game.value?.game_name || gameId.value}`, 'info')
  addDownloadLog(`清单路径: ${manifestFolderPath.value}`, 'info')
  addDownloadLog(`下载路径: ${downloadPath.value}`, 'info')

  try {
    // 先保存游戏数据到game.json。
    // 注意：game_type 不在此处标为 'downloaded'（"已下载"应代表下载完成，
    // 由后端 finalize_download 在整包下载通过校验后写入；否则没下完/暂停/失败
    // 时游戏库会错误显示"已下载"）。此处优先沿用库中已有 game_type（重下场景），
    // 首次下载则留空，完成后再由 finalize 补标。
    const gameData: GameData = {
      game_id: gameId.value,
      game_name: game.value?.game_name || '',
      chinese_name: game.value?.chinese_name || '',
      game_type: existingGameData.value?.game_type || '',
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
      processId?: number
    }>('start_game_download', {
      manifestPath: manifestFolderPath.value,
      downloadPath: downloadPath.value,
      gameId: gameId.value,
      // 下载引擎模式：false=ddv20.exe（默认稳定）/ true=新引擎（设置内开启）
      useEngine: configStore.config?.launch?.useNewEngine ?? false
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
 * 触发引擎取消令牌，并将游戏状态设置为未下载
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
 * 进度变化处理（由全局 store 的进度回调触发）
 * 将中间下载进度持久化到 game.json
 */
const handleDownloadProgress = async () => {
  const progress = downloadManagerStore.getGameProgress(gameId.value)

  if (progress.isComplete) return

  if (existingGameData.value) {
    await updateGameDownloadStatus(gameId.value, 'downloading', progress.overallPercentage)
  }
}

/**
 * 下载完成处理（由全局 store 的完成回调触发）
 */
const handleDownloadComplete = async () => {
  stopProgressMonitoring()
  addDownloadLog('所有depot下载完成！', 'success')
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

  const finalInstallPath = finalizedGame?.install_path || existingGameData.value?.install_path || downloadPath.value
  if (finalInstallPath) {
    gamePath.value = finalInstallPath
  }

  // 「下载完成 → 提示打补丁」弹窗由全局 App 层统一弹出（任意界面均可出现），
  // 此处不再展示本地弹窗，避免重复
}

/**
 * 下载结束处理（由全局 store 的结束回调触发，取消/失败统一收到）
 * 成功路径由 handleDownloadComplete 收尾，这里只处理取消/失败：
 * 复位下载中标记并刷新游戏数据，让界面状态与后端（idle/error）一致
 */
const handleDownloadFinish = async (success: boolean, message: string) => {
  if (success) return // 成功由 handleDownloadComplete 收尾

  // 取消类（download-cancelled / 下载已暂停 等）视为"暂停"而非失败
  const isCancelled = /cancel|取消|暂停/i.test(message)
  if (isCancelled) {
    addDownloadLog('游戏下载已暂停', 'warning')
  } else {
    addDownloadLog(`下载失败: ${message}`, 'error')
  }
  isDownloading.value = false
  existingGameData.value = await getGameData(gameId.value)
  // 清理监控状态（完成回调只在成功时触发，取消/失败在此显式停止）
  stopProgressMonitoring()
}

/**
 * 验证游戏完整性
 * 使用与下载相同的参数重新调用下载引擎，校验并补全缺失文件
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
  downloadManagerStore.clearLogs(gameId.value)
  addDownloadLog('开始验证游戏完整性...', 'info')
  addDownloadLog(`游戏: ${game.value?.game_name || gameId.value}`, 'info')
  addDownloadLog(`清单路径: ${manifestFolderPath.value}`, 'info')
  addDownloadLog(`下载路径: ${existingGameData.value.download_path}`, 'info')

  try {
    const result = await invoke<{
      success: boolean
      message: string
      processId?: number
    }>('start_game_download', {
      manifestPath: manifestFolderPath.value,
      downloadPath: existingGameData.value.download_path,
      gameId: gameId.value,
      // 校验完整性模式：忽略断点续传整包跳过，强制文件级哈希检查补下缺失/损坏文件
      verify: true,
      // 引擎模式：ddv20 模式也能校验（重跑清单补下缺失/损坏文件，支持续传）
      useEngine: configStore.config?.launch?.useNewEngine ?? false
    })

    if (result.success) {
      addDownloadLog('验证命令已启动', 'success')
      addDownloadLog(result.message, 'info')
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
 * 更新游戏（按钮入口）
 * 打开更新弹窗，走「选择新版清单 → 开始更新」两步：
 *   1. 选择新版清单：删除原清单目录，把新版清单导入 resources/manifest/{game_id}；
 *   2. 开始更新：用新版清单 + 已安装目录启动增量下载（引擎整文件哈希跳过，
 *      只下载变化/缺失/损坏文件）。
 */
const handleUpdateGame = async () => {
  if (!existingGameData.value?.download_path) {
    alert('未找到下载路径，无法更新')
    return
  }

  // 每次打开重置就绪状态（防止上次导入结果残留）
  updateManifestReady.value = false
  updateManifestPath.value = ''
  showUpdateModal.value = true
}

/**
 * 弹窗-选择新版清单
 * 按模式（7z / 文件夹）选择新清单 → 删除原清单目录 → 导入新版清单 → 校验就绪
 */
const handleSelectUpdateManifest = async () => {
  // 1. 按模式选择新清单源
  let selected: string | null = null
  try {
    if (updateManifestMode.value === '7z') {
      const picked = await open({
        multiple: false,
        filters: [{ name: '7z压缩包', extensions: ['7z'] }],
        title: '选择新版本清单7z压缩包'
      })
      selected = typeof picked === 'string' ? picked : null
    } else {
      const picked = await open({
        multiple: false,
        directory: true,
        title: '选择新版本清单文件夹（含 .vdf / .lua 和 .manifest）'
      })
      selected = typeof picked === 'string' ? picked : null
    }
  } catch (error) {
    alert(`选择清单失败: ${error}`)
    return
  }
  if (!selected) return

  // 2. 删除原清单 → 导入新版清单
  isPreparingUpdateManifest.value = true
  addDownloadLog('正在替换清单为新版本...', 'info')
  try {
    // 先删除原清单目录：避免新旧 .manifest 混在同一目录导致 depot 任务重复
    await invoke('delete_game_manifest_folder', { gameId: gameId.value })

    if (updateManifestMode.value === '7z') {
      addDownloadLog(`解压新版清单: ${selected}`, 'info')
      await invoke<string>('extract_manifest_archive', {
        archivePath: selected,
        gameId: gameId.value
      })
    } else {
      addDownloadLog(`复制新版清单: ${selected}`, 'info')
      await invoke<string>('copy_folder_to_manifest', {
        sourcePath: selected,
        gameId: gameId.value
      })
    }

    // 3. 重新检测清单状态
    await checkManifestFolder()

    // 4. 校验新清单已就绪（含 .manifest 文件）
    const folder = await invoke<{
      jsonFiles: string[]
      vdfFiles: string[]
      manifestFiles: string[]
    }>('read_manifest_folder', { folderPath: manifestFolderPath.value })

    if (folder.manifestFiles.length === 0) {
      alert('新清单中没有 .manifest 清单文件，请检查')
      return
    }

    updateManifestPath.value = manifestFolderPath.value
    updateManifestReady.value = true
    addDownloadLog(`新版清单已就绪: ${manifestFolderPath.value}`, 'success')
  } catch (error) {
    addDownloadLog(`替换新版清单失败: ${error}`, 'error')
    alert(`替换新版清单失败: ${error}`)
    // 失败后重新检测清单状态（原清单已删，可能变为未找到）
    await checkManifestFolder()
  } finally {
    isPreparingUpdateManifest.value = false
  }
}

/**
 * 弹窗-开始更新
 * 用已导入的新版清单 + 已安装目录启动增量下载
 */
const handleStartUpdate = async () => {
  if (!updateManifestReady.value) {
    alert('请先选择新版清单')
    return
  }
  const gameData = existingGameData.value
  if (!gameData?.download_path) {
    alert('未找到下载路径，无法更新')
    return
  }

  // 关闭弹窗，进入下载流程
  showUpdateModal.value = false

  isUpdating.value = true
  downloadManagerStore.clearLogs(gameId.value)
  addDownloadLog('开始更新游戏...', 'info')
  addDownloadLog(`游戏: ${game.value?.game_name || gameId.value}`, 'info')
  addDownloadLog(`新版清单: ${manifestFolderPath.value}`, 'info')
  addDownloadLog(`下载路径: ${gameData.download_path}`, 'info')

  try {
    // 游戏状态置为下载中
    await upsertGameData({ ...gameData, download_status: 'downloading' })
    existingGameData.value = { ...gameData, download_status: 'downloading' }

    const result = await invoke<{
      success: boolean
      message: string
      processId?: number
    }>('start_game_download', {
      manifestPath: manifestFolderPath.value,
      downloadPath: gameData.download_path,
      gameId: gameId.value,
      // 引擎模式：更新(增量下载)只有新引擎支持，ddv20 模式下此按钮已隐藏
      useEngine: configStore.config?.launch?.useNewEngine ?? false
    })

    if (result.success) {
      addDownloadLog('更新命令已启动', 'success')
      addDownloadLog(result.message, 'info')
      // 进度监控按新版清单初始化 depot 列表
      startProgressMonitoring()
    } else {
      addDownloadLog(`更新启动失败: ${result.message}`, 'error')
      await updateGameDownloadStatus(gameId.value, 'error', 0)
    }
  } catch (error) {
    addDownloadLog(`更新出错: ${error}`, 'error')
    await updateGameDownloadStatus(gameId.value, 'error', 0)
  } finally {
    isUpdating.value = false
  }
}

/**
 * 替换清单
 * 删除本地清单文件夹，并重新检查清单状态，恢复为未导入状态
 */
const handleReplaceManifest = async () => {
  const confirmed = confirm(
    `确定要替换 "${game.value?.chinese_name || gameId.value}" 的清单吗？\n\n` +
    '这将删除本地已导入的清单文件夹，之后需要重新选择或下载清单文件。'
  )
  if (!confirmed) return

  try {
    await invoke('delete_game_manifest_folder', { gameId: gameId.value })
    addDownloadLog('已删除本地清单文件夹', 'success')

    manifestCheckStatus.value = 'checking'
    manifestFolderPath.value = ''
    selectedDownloadManifestPath.value = ''
    downloadManifestSourceMode.value = '7z'

    await checkManifestFolder()
  } catch (error) {
    addDownloadLog(`替换清单失败: ${error}`, 'error')
    alert(`替换清单失败: ${error}`)
  }
}

/**
 * 开始监控下载进度
 * 使用全局 store 的统一定时器，页面切换不丢失，支持多游戏同时监控
 */
const startProgressMonitoring = async (manifestPathOverride?: string) => {
  if (isMonitoring.value) return

  // 更新游戏时传入新版本清单路径覆盖默认清单（默认是当前已导入清单）
  const folderPath = manifestPathOverride || manifestFolderPath.value

  // 首先读取manifest文件夹，获取所有depot ID
  try {
    const result = await invoke<{
      jsonFiles: string[]
      vdfFiles: string[]
      manifestFiles: string[]
    }>('read_manifest_folder', { folderPath })

    // 从manifest文件名中提取depot ID
    // 文件名格式: "{depotId}_{version}.manifest"
    // 注意: result.manifestFiles 是完整路径，如 D:\...\261550\261551_8211251051201469236.manifest
    const depotIds = result.manifestFiles.map(filePath => {
      // 使用更健壮的正则，匹配路径分隔符后的文件名
      const match = filePath.match(/[\\/](\d+)_\d+\.manifest$/)
      return match ? match[1] : null
    }).filter(id => id !== null) as string[]

    // 初始化所有depot的进度状态到全局 store
    downloadManagerStore.initDepots(gameId.value, depotIds)

    addDownloadLog(`检测到 ${depotIds.length} 个depot`, 'info')
  } catch (error) {
    addDownloadLog(`读取manifest文件夹失败: ${error}`, 'error')
  }

  // 注册进度变化、完成与结束回调
  downloadManagerStore.onProgress(gameId.value, handleDownloadProgress)
  downloadManagerStore.onComplete(gameId.value, handleDownloadComplete)
  downloadManagerStore.onFinish(gameId.value, handleDownloadFinish)

  // 快照兜底：页面刷新/进入时引擎可能已跑了好一会，先拉一次当前进度
  await downloadManagerStore.fetchSnapshot(gameId.value)

  // 建桥订阅引擎事件并置监控标记
  await downloadManagerStore.startMonitoring(gameId.value)
}

/**
 * 停止监控下载进度
 */
const stopProgressMonitoring = () => {
  downloadManagerStore.stopMonitoring(gameId.value)
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

  // 初始化全局 store 中该游戏的状态
  downloadManagerStore.setGameName(gameId.value, game.value?.game_name || gameId.value)

  // 恢复上次会话遗留的「已暂停」任务（程序重启后详情页仍能显示"已暂停"+ 总进度）
  await downloadManagerStore.restorePausedGames()

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

    // 如果记录为已下载但实际目录不存在，重置状态并清理缓存
    if (gameData.download_status === 'completed' && !isGameDirExists) {
      gameData.download_status = ''
      gameData.is_installed = false
      // 更新到 game.json
      await safeAsync(() => upsertGameData(gameData))
      // 删除该游戏的下载日志缓存目录
      try {
        await deleteGameLogCache(gameId.value)
      } catch {
        // 缓存删除失败不影响主流程
      }
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
    // 恢复游戏路径：下载完成后优先使用下载路径（安装根目录）
    // 并同步修正旧版本可能把 install_path 写成 exe 目录的问题
    if (gameData.download_status === 'completed' && gameData.download_path) {
      gamePath.value = gameData.download_path
      if (gameData.install_path !== gameData.download_path) {
        gameData.install_path = gameData.download_path
        await safeAsync(() => upsertGameData(gameData))
      }
    } else if (gameData.install_path) {
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
        // 同步设置游戏路径为安装根目录
        if (finalizedGame.install_path) {
          gamePath.value = finalizedGame.install_path
        } else if (finalizedGame.download_path) {
          gamePath.value = finalizedGame.download_path
        }
      } catch {
        // 补偿失败时保持原数据，不影响页面显示
      }
    }
  }

  // 加载封面图片 - 使用重试机制确保100%加载
  await loadCoverImage()

  // 设置默认标签页
  setDefaultTab()
  // 支持从全局「下载完成」弹窗直达某补丁标签（URL 带 ?patch=patch-xxx）
  const patchParam = route.query.patch as string | undefined
  if (patchParam && patchParam.startsWith('patch-')) {
    currentTab.value = patchParam
  }
  // 自动填充下载路径（不依赖清单文件夹是否存在）
  await autoSetDownloadPath()
  // 自动检测清单文件夹
  await checkManifestFolder()
  // 加载所有补丁说明
  await loadPatchReadmes()
  // 监听 Readme 云端更新事件：后台下载替换完成后静默刷新显示（不展示任何提示）
  unlistenReadmeUpdate = await listen('readme-updated', () => {
    loadPatchReadmes()
  })
  // 拉取解压即玩（夸克网盘二维码）图片
  await fetchExtractPlayImage(gameId.value)
  // 检查本地补丁文件状态
  await checkPatchLocalStatus()
  // 检查游戏清单文件是否存在（用于入库Steam按钮）
  await checkGameManifest()
  // 加载修改器本地文件内容
  await loadTrainerContent()
})

// 组件卸载时不再停止监控，保持全局 store 中的下载状态，
// 支持页面切换后仍能查看和恢复下载进度。
onUnmounted(() => {
  // 注销 Readme 云端更新事件监听
  unlistenReadmeUpdate?.()
  unlistenReadmeUpdate = null
})

// ==================== 路由级联：补丁直达（任意界面都可跳到对应补丁标签） ====================

/**
 * 应用 URL 上的补丁直达参数（?patch=patch-xxx）。
 * 首次挂载由 onMounted 处理；此处兜底等待游戏配置就绪（最多 1s 轮询），
 * 防止 game 尚未加载时设置标签被 setDefaultTab 覆盖。
 *
 * 额外动作：从「下载完成提示补丁」弹窗跳转时，组件复用（仅 query 变化、
 * onMounted 不重跑），此处主动刷新一次游戏数据回填游戏路径——
 * 否则下载完成刚写入的 install_path 不会反映到路径输入框。
 */
function applyPatchTabFromQuery() {
  const apply = () => {
    if (!game.value) return false
    const patchParam = route.query.patch as string | undefined
    if (patchParam && patchParam.startsWith('patch-')) {
      currentTab.value = patchParam
      refreshGamePathFromData()
    } else {
      setDefaultTab()
    }
    return true
  }
  if (apply()) return
  // 游戏配置尚未就绪：短轮询等待（apply 幂等，重复触发无副作用）
  for (let i = 1; i <= 20; i++) {
    setTimeout(() => {
      if (game.value) apply()
    }, 50 * i)
  }
}

/**
 * 从后端最新游戏数据回填游戏路径（安装路径优先，下载路径兜底）。
 * 用于「下载完成提示补丁」跳转等组件复用场景，补齐 onMounted 不重跑
 * 造成的路径未刷新问题。
 */
async function refreshGamePathFromData() {
  try {
    const data = await getGameData(gameId.value)
    if (!data) return
    if (data.install_path) {
      gamePath.value = data.install_path
    } else if (data.download_path) {
      gamePath.value = data.download_path
    }
    existingGameData.value = data
  } catch {
    // 刷新失败保持原值，不影响补丁标签切换
  }
}

// 补丁参数变化（在同一详情页点击弹窗补丁按钮时组件复用、onMounted 不重跑，靠这里切标签）
watch(
  () => route.query.patch,
  () => applyPatchTabFromQuery()
)

// 跨游戏跳转（/game/A → /game/B，组件复用）：重置与旧游戏绑定的展示状态，
// 再应用新游戏的默认/补丁标签
watch(
  () => route.params.id,
  (newId, oldId) => {
    if (newId === oldId) return
    gamePath.value = ''
    downloadPath.value = ''
    existingGameData.value = null
    downloadManagerStore.setGameName(gameId.value, game.value?.game_name || gameId.value)
    void loadCoverImage()
    // 下一轮渲染后应用标签（此时 game.value 已切换到新游戏）
    applyPatchTabFromQuery()
  }
)

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
 * 写入全局 store，确保页面切换后日志不丢失
 * @param message 日志消息
 * @param type 日志类型
 */
const addDownloadLog = (message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') => {
  const now = new Date()
  const time = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:${now.getSeconds().toString().padStart(2, '0')}`
  downloadManagerStore.addLog(gameId.value, { time, message, type })
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
      // 使用 BaseModal 弹窗替代 alert，展示更友好的入库成功提示
      importSuccessMessage.value = '游戏已成功入库'
      importSuccessGameName.value = game.value.chinese_name || game.value.game_name
      importSuccessAppId.value = appId
      showImportSuccessModal.value = true
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
  /* 玻璃水晶背景：半透明冷蓝底让全局背景透出，
     叠加右上方淡蓝氛围光与顶部高光，模拟光线在玻璃切面的折射 */
  background:
    radial-gradient(120% 90% at 88% -8%, rgba(87, 140, 220, 0.08) 0%, transparent 55%),
    linear-gradient(180deg, rgba(255, 255, 255, 0.04) 0%, rgba(255, 255, 255, 0) 14%),
    rgba(var(--steam-bg-primary-rgb), 0.55);
  backdrop-filter: blur(20px) saturate(150%);
  -webkit-backdrop-filter: blur(20px) saturate(150%);
}

/* 头部区域 */
.detail-header {
  display: flex;
  align-items: center;
  gap: 24px;
  margin-bottom: 14px;
  padding-bottom: 10px;
  border-bottom: 1px solid var(--glass-border);
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
  /* 玻璃切面般的细描边 + 更柔和的深阴影 */
  border: 1px solid var(--glass-border-strong);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.45);
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
  border: 1px solid var(--glass-border);
  border-radius: 8px;
  /* 半透明玻璃输入框，聚焦时呈现蓝色光晕 */
  background: rgba(var(--steam-bg-tertiary-rgb), 0.45);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  color: var(--steam-text-primary);
  font-size: 13px;
  outline: none;
  transition: border-color 0.15s ease, box-shadow 0.15s ease, background-color 0.15s ease;
}

.path-input:focus {
  border-color: rgba(var(--steam-accent-blue-rgb), 0.6);
  background: rgba(var(--steam-bg-tertiary-rgb), 0.6);
  box-shadow: 0 0 0 3px rgba(var(--steam-accent-blue-rgb), 0.15);
}

/* 标签页区域 - 玻璃面板 */
.tabs-section {
  background: rgba(var(--steam-bg-secondary-rgb), 0.45);
  backdrop-filter: blur(16px) saturate(140%);
  -webkit-backdrop-filter: blur(16px) saturate(140%);
  border: 1px solid var(--glass-border);
  border-radius: 12px;
  overflow: hidden;
}

.tabs-header {
  display: flex;
  border-bottom: 1px solid var(--glass-border);
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
  /* 激活标签使用半透明玻璃底 */
  background-color: rgba(var(--steam-bg-primary-rgb), 0.4);
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
  background: rgba(var(--steam-bg-secondary-rgb), 0.55);
  border: 1px solid var(--glass-border);
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
  background: rgba(var(--steam-bg-secondary-rgb), 0.55);
  border: 1px solid var(--glass-border);
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
  background: rgba(var(--steam-bg-secondary-rgb), 0.55);
  border-radius: 8px;
  border: 1px solid var(--glass-border);
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
  background: rgba(var(--steam-bg-secondary-rgb), 0.55);
  border: 1px solid var(--glass-border);
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

/* 更新游戏弹窗 */
:global(.update-game-modal) {
  width: 40% !important;
  max-width: 40% !important;
  height: auto !important;
  max-height: 50vh !important;
}

.update-game-modal-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
  justify-content: flex-start;
}

/* 新版清单来源模式选择 */
.update-mode-select {
  display: flex;
  gap: 18px;
  flex-wrap: wrap;
  padding: 10px 12px;
  border-radius: 8px;
  background-color: var(--steam-bg-tertiary);
}

.update-mode-select .radio-label {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--steam-text-primary);
  cursor: pointer;
}

/* 新版清单就绪状态 */
.update-list-status {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  line-height: 1.4;
}

.update-status-label {
  color: var(--steam-text-secondary);
  flex-shrink: 0;
}

.update-status-ready {
  color: var(--steam-accent-green);
  word-break: break-all;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.update-status-muted {
  color: var(--steam-text-subtle);
}

/* 两个操作按钮 */
.update-actions {
  display: flex;
  gap: 10px;
  margin-top: 4px;
}

/* 开始更新 - 主操作样式：保留右对齐 */
.update-start-btn {
  margin-left: auto;
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

/* 清单文件包误当补丁提示弹窗 */
.manifest-patch-alert-content {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.manifest-patch-alert-content .modal-main-hint {
  font-weight: 600;
}

/* Xbox 联机补丁兼容性提示弹窗 */
.xbox-patch-notice-content {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.xbox-patch-notice-content .modal-main-hint {
  font-weight: 600;
}

/* 入库成功弹窗 */
.import-success-modal-content {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.import-success-modal-content .modal-main-hint {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.import-success-details {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 12px;
  border-radius: 8px;
  background: var(--steam-bg-primary);
}

.import-success-details p {
  margin: 0;
  font-size: 14px;
  color: var(--steam-text-primary);
  word-break: break-all;
}

.import-success-details .detail-label {
  color: var(--steam-text-secondary);
  font-weight: 500;
}

.import-success-modal-content .modal-sub-hint {
  margin: 0;
  font-size: 13px;
  color: var(--steam-text-muted);
}
</style>