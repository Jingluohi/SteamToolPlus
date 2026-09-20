<template>
  <!-- 游戏下载标签页 -->
  <div class="tab-panel">
    <!-- 顶部操作栏 -->
    <div class="download-tab-header">
      <h3 class="panel-title">游戏下载</h3>
      <button
        v-if="manifestCheckStatus === 'found'"
        class="replace-manifest-btn ui-btn ui-btn-sm ui-btn-warning"
        @click="handleReplaceManifest"
        title="删除本地清单文件夹，重新导入或下载清单"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2"/>
        </svg>
        <span>替换清单</span>
      </button>
    </div>

    <!-- 下载状态显示 -->
    <div v-if="existingGameData?.download_status === 'completed'" class="download-completed-notice">
      <div class="success-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
          <polyline points="22 4 12 14.01 9 11.01"/>
        </svg>
      </div>
      <div class="success-text">
        <h4>游戏已下载</h4>
        <p>下载路径: {{ existingGameData.download_path }}</p>
        <p>安装路径: {{ existingGameData.install_path }}</p>
        <p class="patch-hint">请安装对应补丁（免steam、steam联机、局域网联机等）</p>
      </div>
      <!-- 操作按钮组：更新游戏 + 验证完整性 -->
      <div class="notice-actions">
        <!-- 更新游戏按钮：仅新引擎支持增量下载（ddv20.exe 模式隐藏） -->
        <button
          v-if="configStore.config?.launch?.useNewEngine ?? false"
          class="update-game-btn ui-btn ui-btn-md ui-btn-outline"
          :class="{ loading: isUpdating }"
          :disabled="isUpdating"
          @click="handleUpdateGame"
          title="选择新版本清单文件夹，只下载与本地不一致的文件"
        >
          <svg v-if="isUpdating" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
          </svg>
          <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M23 4v6h-6M1 20v-6h6"/>
            <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/>
          </svg>
          {{ isUpdating ? '更新中...' : '更新游戏' }}
        </button>

        <!-- 验证完整性按钮 -->
        <button
          class="verify-integrity-btn ui-btn ui-btn-md ui-btn-primary"
          :class="{ loading: isVerifying }"
          :disabled="isVerifying"
          @click="handleVerifyIntegrity"
        >
          <svg v-if="isVerifying" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
          </svg>
          <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
            <polyline points="22 4 12 14.01 9 11.01"/>
          </svg>
          {{ isVerifying ? '验证中...' : '验证游戏完整性' }}
        </button>
      </div>
    </div>

    <div v-else class="download-info">
      <!-- 清单文件夹检测状态，未找到时显示下载引导和文件选择 -->
      <div
        v-if="manifestCheckStatus === 'not_found'"
        class="import-no-files"
      >
        <!-- 清单下载详细说明 -->
        <div class="manifest-guide-box">
          <h4 class="guide-title">操作步骤：</h4>
          <ol class="guide-steps">
            <li><strong>第一步：</strong>点击下方网盘按钮下载该游戏的清单7z压缩包（每个游戏对应一个清单7z文件，游戏ID就是7z文件名）</li>
            <li><strong>第二步：</strong>在下方选择"7z文件"选择7z文件，或者，在".vdf / .lua 和 .manifest所在文件夹"点击"选择"后选择7z解压后的文件夹</li>
            <li><strong>第三步：</strong>选择完成后点击"开始下载"按钮，程序会从Steam服务器下载游戏文件到您指定的路径</li>
          </ol>
        </div>

        <!-- 下载引导 -->
        <div class="download-guide">
          <p class="download-guide-title">点击下载对应清单文件7z，id：{{ gameId }}</p>
          <div class="download-buttons">
            <div class="download-btn-wrapper">
              <button
                class="download-patch-btn ui-btn ui-btn-md ui-btn-primary"
                @click="handleOpenQingdanQRCode"
              >
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                  <polyline points="7 10 12 15 17 10"/>
                  <line x1="12" y1="15" x2="12" y2="3"/>
                </svg>
                <span>夸克网盘</span>
              </button>
            </div>
            <div class="backup-label">备用（容易和谐）：</div>
            <div class="download-btn-wrapper">
              <button
                class="download-patch-btn ui-btn ui-btn-md ui-btn-primary"
                @click="handleOpenDownloadUrl('https://pan.xunlei.com/s/VOw3jTAGHqYFsm49n2t_AeVGA1?pwd=3r6n')"
              >
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                  <polyline points="7 10 12 15 17 10"/>
                  <line x1="12" y1="15" x2="12" y2="3"/>
                </svg>
                <span>迅雷网盘</span>
              </button>
              <p class="pwd-hint">提取码: 3r6n</p>
            </div>
            <div class="download-btn-wrapper">
              <button
                class="download-patch-btn ui-btn ui-btn-md ui-btn-primary"
                @click="handleOpenDownloadUrl('https://pan.baidu.com/s/1FTZyknIObyzMuLAJC-Uj9g?pwd=8uwx')"
              >
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                  <polyline points="7 10 12 15 17 10"/>
                  <line x1="12" y1="15" x2="12" y2="3"/>
                </svg>
                <span>百度网盘</span>
              </button>
              <p class="pwd-hint">提取码: 8uwx</p>
            </div>
          </div>
        </div>

        <!-- 清单文件选择 -->
        <div class="import-source-select">
          <h4 class="source-select-title">清单文件选择</h4>

          <div class="radio-group">
            <label class="radio-label">
              <input
                type="radio"
                :checked="downloadManifestSourceMode === '7z'"
                @change="handleUpdateManifestSourceMode('7z')"
                value="7z"
                name="download-manifest-source-mode"
              />
              <span>7z文件</span>
            </label>
            <label class="radio-label">
              <input
                type="radio"
                :checked="downloadManifestSourceMode === 'folder'"
                @change="handleUpdateManifestSourceMode('folder')"
                value="folder"
                name="download-manifest-source-mode"
              />
              <span>.vdf / .lua 和 .manifest所在文件夹</span>
            </label>
          </div>

          <div class="source-select-actions">
            <button
              v-if="downloadManifestSourceMode === '7z'"
              class="select-source-btn ui-btn ui-btn-sm ui-btn-outline"
              :disabled="isPreparingDownloadManifest"
              @click="handleSelectManifestArchive"
            >
              <svg v-if="isPreparingDownloadManifest" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
              </svg>
              {{ isPreparingDownloadManifest ? '处理中...' : '选择7z文件' }}
            </button>
            <button
              v-else
              class="select-source-btn ui-btn ui-btn-sm ui-btn-outline"
              :disabled="isPreparingDownloadManifest"
              @click="handleSelectManifestFolder"
            >
              <svg v-if="isPreparingDownloadManifest" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
              </svg>
              {{ isPreparingDownloadManifest ? '处理中...' : '选择文件夹' }}
            </button>
          </div>

          <p v-if="selectedDownloadManifestPath" class="source-select-info">
            已选择: {{ selectedDownloadManifestPath }}
          </p>
        </div>
      </div>
    </div>

    <!-- 下载说明 -->
    <div class="download-description">
      <div class="download-option">
        <h4>方法一【开始下载】</h4>
        <p>本界面的开始下载按钮通过清单文件直连 Steam 官方服务器下载正版分流文件（无法下载就加速一下steam），下载完成后需要 <span class="highlight-red-bold">注入补丁</span> 才能游玩（只能打一个补丁）</p>
      </div>
      <div class="download-option invalid">
        <h4>方法二【入库 Steam】<span class="invalid-label">⚠ 入库后无法下载游戏，该方法已失效</span></h4>
        <p>将游戏清单脚本导入 Steam 客户端，导入后可在 Steam 库中下载和启动游戏（优点是可选择是否自动更新），想要联机或者有第三方加密的游戏需要 <span class="highlight-red-bold">注入补丁</span> 才能游玩</p>
      </div>
      <div class="download-option">
        <h4>方法三【解压即玩】</h4>
        <p>已经测试过100%可以直接下载下来 解压后就能直接玩 的游戏（大部分 单机豪华版 ，联机版会特别标注），体积超过100GB的会有多个文件以001、002等结尾，解压需要用7zip</p>
      </div>
    </div>

    <!-- 下载按钮组 -->
    <div v-if="existingGameData?.download_status !== 'completed'" class="download-btn-group">
      <!-- 圆形下载进度条，仅下载中显示 -->
      <div
        v-if="isDownloading || existingGameData?.download_status === 'downloading'"
        class="circular-progress"
        :title="`总进度 ${downloadProgress.overallPercentage}%`"
      >
        <svg viewBox="0 0 36 36">
          <path
            class="circle-bg"
            d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
          />
          <path
            class="circle-progress"
            :stroke-dasharray="`${downloadProgress.overallPercentage}, 100`"
            d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
          />
        </svg>
        <span class="progress-text">{{ downloadProgress.overallPercentage }}%</span>
      </div>

      <!-- 开始下载 / 继续下载按钮（暂停时复用开始下载，引擎断点续传） -->
      <button
        class="start-download-btn ui-btn ui-btn-lg ui-btn-primary"
        :class="{ disabled: !canDownload, loading: isDownloading || existingGameData?.download_status === 'downloading' }"
        :disabled="!canDownload || isDownloading || existingGameData?.download_status === 'downloading'"
        @click="handleStartDownload"
        :title="isPaused ? '继续下载（断点续传）' : (!canDownload ? '未找到清单文件，无法下载' : '')"
      >
        <svg v-if="isDownloading || existingGameData?.download_status === 'downloading'" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
        </svg>
        <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
          <polyline points="7 10 12 15 17 10"/>
          <line x1="12" y1="15" x2="12" y2="3"/>
        </svg>
        {{ downloadButtonText }}
      </button>

      <!-- 暂停态立即续跑：主按钮（开始下载）在暂停时文字即"继续下载" -->

      <!-- 下载中 → 暂停 -->
      <button
        v-if="isDownloading || existingGameData?.download_status === 'downloading'"
        class="stop-download-btn ui-btn ui-btn-md ui-btn-danger"
        @click="handleStopDownload"
        title="暂停下载"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="6" y="4" width="4" height="16"/>
          <rect x="14" y="4" width="4" height="16"/>
        </svg>
        暂停
      </button>
    </div>

    <!-- 下载进度组件（置于下载日志上方，优先展示实时进度） -->
    <DownloadProgress
      v-if="isMonitoring || isPaused || downloadProgress.depots.length > 0 || existingGameData?.download_status === 'downloading'"
      :progress="downloadProgress"
      :is-monitoring="isMonitoring || existingGameData?.download_status === 'downloading'"
      :speed="downloadSpeed"
      :done-bytes="downloadDoneBytes"
      :total-bytes="downloadTotalBytes"
      class="download-progress-section"
    />

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
  </div>
</template>

<script setup lang="ts">
/**
 * GameDownloadTab 组件
 * 游戏下载标签页，负责显示下载状态、清单文件选择、下载进度等信息
 */

import { computed } from 'vue'
import DownloadProgress from '../../../components/download/DownloadProgress.vue'
import type { GameConfigData } from '../../../types'
import type { GameData } from '../../../api/gameData.api'
import type { DownloadProgress as DownloadProgressType } from '../../../types/download.types'
import { useConfigStore } from '../../../store/config.store'

// 全局配置（读"新下载引擎"开关：false=ddv20 模式，隐藏"更新游戏"按钮——增量更新仅新引擎支持）
const configStore = useConfigStore()

// ==================== Props 定义 ====================

interface Props {
  /** 游戏ID */
  gameId: string
  /** 游戏配置数据 */
  game?: GameConfigData
  /** 已存在的游戏数据（从game.json加载） */
  existingGameData: GameData | null
  /** 清单文件夹检测状态 */
  manifestCheckStatus: 'checking' | 'found' | 'not_found'
  /** 下载路径 */
  downloadPath: string
  /** 清单文件夹路径 */
  manifestFolderPath: string
  /** 是否正在下载 */
  isDownloading: boolean
  /** 是否正在验证 */
  isVerifying: boolean
  /** 是否正在更新游戏 */
  isUpdating: boolean
  /** 是否正在监控进度 */
  isMonitoring: boolean
  /** 是否处于暂停态（暂停时按钮切换为「继续下载」） */
  isPaused: boolean
  /** 实时下载速度（字节/秒），供"下载进度"头部展示 */
  downloadSpeed: number
  /** 本次下载已下载字节数（引擎全局字节模型，供估算剩余时间） */
  downloadDoneBytes: number
  /** 本次下载总字节数（引擎全局字节模型，供估算剩余时间） */
  downloadTotalBytes: number
  /** 下载进度 */
  downloadProgress: DownloadProgressType
  /** 下载日志 */
  downloadLogs: { time: string; message: string; type: 'info' | 'success' | 'error' | 'warning' }[]
  /** 清单源选择模式 */
  downloadManifestSourceMode: '7z' | 'folder'
  /** 已选择的清单源路径 */
  selectedDownloadManifestPath: string
  /** 是否正在准备清单 */
  isPreparingDownloadManifest: boolean
}

const props = defineProps<Props>()

// ==================== Emits 定义 ====================

interface Emits {
  /** 选择下载路径 */
  (e: 'selectDownloadPath'): void
  /** 选择清单7z文件 */
  (e: 'selectManifestArchive'): void
  /** 选择清单文件夹 */
  (e: 'selectManifestFolder'): void
  /** 开始下载 */
  (e: 'startDownload'): void
  /** 停止下载 */
  (e: 'stopDownload'): void
  /** 打开清单下载二维码弹窗 */
  (e: 'open-qingdan-qr-code'): void
  /** 打开下载链接 */
  (e: 'openDownloadUrl', url: string): void
  /** 验证游戏完整性 */
  (e: 'verifyIntegrity'): void
  /** 更新游戏：选择新版本清单后增量下载 */
  (e: 'updateGame'): void
  /** 替换清单：删除本地清单文件夹 */
  (e: 'replace-manifest'): void
  /** 更新清单源选择模式 */
  (e: 'update:downloadManifestSourceMode', mode: '7z' | 'folder'): void
}

const emit = defineEmits<Emits>()

// ==================== 计算属性 ====================

/**
 * 是否可以开始下载
 * 条件：清单文件夹已找到、下载路径已设置、不在下载中
 */
const canDownload = computed(() => {
  return props.manifestCheckStatus === 'found' && props.downloadPath !== '' && !props.isDownloading
})

/**
 * 下载按钮文本
 * 暂停态优先显示"继续下载"（点击复用开始下载，引擎断点续传）
 */
const downloadButtonText = computed(() => {
  if (props.isPaused) {
    return '继续下载'
  }
  if (props.existingGameData?.download_status === 'downloading') {
    return '下载中...'
  }
  if (props.isDownloading) {
    return '下载中...'
  }
  if (props.existingGameData?.download_status === 'completed') {
    return '已下载'
  }
  return '开始下载'
})

// ==================== 方法 ====================

/**
 * 处理打开清单下载二维码弹窗
 */
function handleOpenQingdanQRCode() {
  emit('open-qingdan-qr-code')
}

/**
 * 处理打开下载链接
 * @param url 下载链接地址
 */
function handleOpenDownloadUrl(url: string) {
  emit('openDownloadUrl', url)
}

/**
 * 处理选择清单7z文件
 */
function handleSelectManifestArchive() {
  emit('selectManifestArchive')
}

/**
 * 处理选择清单文件夹
 */
function handleSelectManifestFolder() {
  emit('selectManifestFolder')
}

/**
 * 处理开始下载
 */
function handleStartDownload() {
  emit('startDownload')
}

/**
 * 处理停止下载
 */
function handleStopDownload() {
  emit('stopDownload')
}

/**
 * 处理更新清单源选择模式
 * @param mode 清单源模式
 */
function handleUpdateManifestSourceMode(mode: '7z' | 'folder') {
  emit('update:downloadManifestSourceMode', mode)
}

/**
 * 处理验证完整性按钮点击
 */
function handleVerifyIntegrity() {
  emit('verifyIntegrity')
}

/**
 * 处理更新游戏按钮点击
 * 由父组件选择新版本清单并启动增量下载
 */
function handleUpdateGame() {
  emit('updateGame')
}

/**
 * 处理替换清单按钮点击
 * 删除本地清单文件夹，恢复为清单未导入状态
 */
function handleReplaceManifest() {
  emit('replace-manifest')
}
</script>

<style scoped>
/* 下载标签页顶部操作栏 */
.download-tab-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 5px;
  padding-bottom: 10px;
  border-bottom: 1px solid var(--steam-border);
}

.download-tab-header .panel-title {
  margin: 0;
  font-size: 16px;
  color: var(--steam-text-primary);
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

/* 操作按钮组：更新游戏 + 验证完整性 */
.notice-actions {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-left: auto;
}

.download-info {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-bottom: 6px;
}

.download-btn-group {
  display: flex;
  gap: 7px;
  align-items: center;
  position: relative;
  flex-wrap: wrap;
}

/* 详情页下载按钮精调：字号 -1、高度略减、左右留白缩至 60% */
/* 开始下载 / 继续下载（lg 档覆盖） */
.start-download-btn {
  height: 44px;         /* 原 48px */
  padding: 0 16px;      /* 原 26px → 60% */
  font-size: 14px;      /* 原 15px → -1 */
  gap: 6px;
}

.start-download-btn svg {
  width: 16px;
  height: 16px;
}

/* 暂停（md 档覆盖） */
.stop-download-btn {
  height: 36px;         /* 原 40px */
  padding: 0 11px;      /* 原 18px → 60% */
  font-size: 13px;      /* 原 14px → -1 */
  gap: 6px;
}

.stop-download-btn svg {
  width: 14px;
  height: 14px;
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

/* 下载说明样式 */
.download-description {
  margin: 5px 0 7px; /* 底部 7px：与下方下载按钮组间隔 +2px */
  padding: 6px 10px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  border-left: 4px solid #3b82f6;
}

.download-option {
  margin-bottom: 4px;
}

.download-option:last-child {
  margin-bottom: 0;
}

.download-option h4 {
  margin: 0 0 2px 0;
  font-size: 13px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.download-option p {
  margin: 0;
  font-size: 12px;
  color: var(--steam-text-secondary);
  line-height: 1.5;
}

/* 方法二失效样式：标题与正文整块加删除线（多行文字每行都会划线） */
.download-option.invalid h4,
.download-option.invalid p {
  text-decoration: line-through;
}

/* 失效标注：红色加粗，紧贴标题右侧，且不被删除线穿过
   display: inline-block 使父级 h4 的 line-through 无法传播进该元素内部 */
.invalid-label {
  display: inline-block;
  margin-left: 8px;
  font-size: 12px;
  font-weight: 700;
  color: #ef4444;
  text-decoration: none;
}

.loading-icon {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
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

/* 无清单文件时的引导区域 */
.import-no-files {
  display: flex;
  flex-direction: column;
  gap: 10px;
  width: 100%;
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

/* 清单源选择区域 */
.import-source-select {
  padding: 10px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--steam-border);
  display: flex;
  flex-direction: column;
  gap: 7px;
}

.source-select-title {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.radio-group {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.radio-label {
  display: inline-flex;
  align-items: center;
  gap: 7px;
  cursor: pointer;
  font-size: 13px;
  color: var(--steam-text-primary);
}

.radio-label input[type="radio"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
  accent-color: var(--steam-accent-blue);
}

.source-select-actions {
  display: flex;
  gap: 7px;
  align-items: center;
}

.source-select-info {
  margin: 0;
  font-size: 12px;
  color: var(--steam-text-secondary);
  word-break: break-all;
}
</style>