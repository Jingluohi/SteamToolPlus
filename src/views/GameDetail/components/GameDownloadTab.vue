<template>
  <!-- 游戏下载标签页 -->
  <div class="tab-panel">
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
      <!-- 验证完整性按钮 -->
      <button
        class="verify-integrity-btn"
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
                class="download-patch-btn"
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
                class="download-patch-btn"
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
                class="download-patch-btn"
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
              class="select-source-btn"
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
              class="select-source-btn"
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

      <!-- 下载路径显示 -->
      <div class="download-path-section">
        <label class="path-label">下载路径</label>
        <div class="path-display">{{ downloadPath || '未选择' }}</div>
        <p v-if="downloadPath" class="path-hint">自动设置为: {{ downloadPath }}</p>
      </div>
    </div>

    <!-- 下载说明 -->
    <div class="download-description">
      <div class="download-option">
        <h4>方法一【开始下载】</h4>
        <p>通过清单文件直连 Steam 官方服务器下载正版分流文件，下载完成后需要 <span class="highlight-red-bold">注入补丁</span> 才能游玩</p>
      </div>
      <div class="download-option">
        <h4>方法二【入库 Steam】</h4>
        <p>将游戏清单脚本导入 Steam 客户端，导入后可在 Steam 库中下载和启动游戏（优点是可自动更新）</p>
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

      <!-- 开始下载按钮 -->
      <button
        class="start-download-btn"
        :class="{ disabled: !canDownload, loading: isDownloading || existingGameData?.download_status === 'downloading' }"
        :disabled="!canDownload || isDownloading || existingGameData?.download_status === 'downloading'"
        @click="handleStartDownload"
        :title="!canDownload ? '未找到清单文件，无法下载' : ''"
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

      <!-- 暂停/停止下载按钮 -->
      <button
        v-if="isDownloading || existingGameData?.download_status === 'downloading'"
        class="stop-download-btn"
        @click="handleStopDownload"
        title="停止下载"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="6" y="4" width="4" height="16"/>
          <rect x="14" y="4" width="4" height="16"/>
        </svg>
        暂停
      </button>
    </div>

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
      v-if="isMonitoring || downloadProgress.depots.length > 0 || existingGameData?.download_status === 'downloading'"
      :progress="downloadProgress"
      :is-monitoring="isMonitoring || existingGameData?.download_status === 'downloading'"
      class="download-progress-section"
    />
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
  /** 是否正在监控进度 */
  isMonitoring: boolean
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
 */
const downloadButtonText = computed(() => {
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
</script>

<style scoped>
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

.loading-icon {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
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

.select-source-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
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

.select-source-btn:hover:not(:disabled) {
  background-color: var(--steam-accent-hover);
}

.select-source-btn:disabled {
  background-color: var(--steam-text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

.select-source-btn svg {
  width: 16px;
  height: 16px;
}

.source-select-info {
  margin: 0;
  font-size: 12px;
  color: var(--steam-text-secondary);
  word-break: break-all;
}
</style>