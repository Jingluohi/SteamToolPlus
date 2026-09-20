<template>
  <!-- 入库Steam标签页 -->
  <div v-if="currentTab === 'import'" class="tab-panel">
    <!-- 顶部操作栏 -->
    <div class="import-tab-header">
      <h3 class="panel-title">入库Steam</h3>
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
    <div class="import-steam-content">
      <!-- 入库说明 -->
      <div class="import-description">
        <p>将游戏清单导入Steam客户端，导入后可在Steam库中下载和启动游戏，初次启动steam时，需要1分钟来加载游戏，如果2分钟后库还不显示游戏，请重启steam再次尝试。</p>
        <p class="import-note">注意：部分游戏入库下载后需要配合补丁才能正常游玩。</p>
      </div>

      <!-- OpenSteamTool高级选项 -->
      <div class="opensteamtool-options">
        <Tooltip text="Steam已加密内容接口，仅支持锁定清单版本入库，无法跟随更新" position="top">
          <div class="option-item">
            <label class="option-label">是否更新</label>
            <SliderToggle
              :model-value="lockVersion"
              disabled
              left-label="跟随更新"
              right-label="锁定版本"
              @update:model-value="emit('update:lockVersion', $event)"
            />
            <p class="option-hint">Steam 已加密内容接口，只能通过本地清单（manifest）连接下载，入库固定为锁定版本；未提供清单的 depot 仅解锁不锁定</p>
          </div>
        </Tooltip>
      </div>

      <!-- 入库按钮组 -->
      <div class="import-btn-group">
        <!-- OpenSteamTool内核入库按钮（推荐） -->
        <button
          class="import-opensteamtool-btn-large ui-btn ui-btn-lg ui-btn-success"
          :class="{ disabled: !canImportToSteam, loading: isImporting }"
          :disabled="!canImportToSteam || isImporting"
          @click="handleImportWithOpenSteamTool"
        >
          <svg v-if="isImporting" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
          </svg>
          <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
          </svg>
          {{ isImporting ? '入库中...' : 'opensteamtool入库（推荐）' }}
        </button>

        <!-- 重启Steam按钮 -->
        <button
          class="restart-steam-btn ui-btn ui-btn-lg ui-btn-warning"
          @click="handleRestartSteam"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M23 4v6h-6"/>
            <path d="M1 20v-6h6"/>
            <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/>
          </svg>
          重启steam
        </button>
      </div>

      <!-- 自定义源已选择提示 -->
      <div v-if="importSourceReady" class="import-source-info">
        <span class="source-label">当前使用自定义清单源:</span>
        <span class="source-path" :title="selectedImportPath">{{ selectedImportPath }}</span>
        <button class="clear-source-btn ui-btn ui-btn-sm ui-btn-ghost" @click="handleClearImportSource">清除</button>
      </div>

      <!-- 没有vdf/lua时显示下载引导和文件选择 -->
      <div v-if="!hasLua && !importSourceReady" class="import-no-files">
        <!-- 本地清单缺失提示 -->
        <div class="manifest-missing-notice">
          <p>当前不存在本地清单，请下载对应 id：{{ gameId }} 的清单后导入</p>
        </div>

        <!-- 下载引导 -->
        <div class="download-guide">
          <p class="download-guide-title">点击下载对应清单文件7z，id：{{ gameId }}</p>
          <div class="download-buttons">
            <div class="download-btn-wrapper">
              <button
                class="download-patch-btn ui-btn ui-btn-md ui-btn-primary"
                @click="openQingdanQRCode"
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
                @click="openDownloadUrl('https://pan.xunlei.com/s/VOw3jTAGHqYFsm49n2t_AeVGA1?pwd=3r6n')"
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
                @click="openDownloadUrl('https://pan.baidu.com/s/1FTZyknIObyzMuLAJC-Uj9g?pwd=8uwx')"
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

        <!-- 文件选择区域 -->
        <div class="import-source-select">
          <h4 class="source-select-title">清单文件选择</h4>

          <div class="radio-group">
            <label class="radio-label">
              <input
                type="radio"
                :checked="importSourceMode === '7z'"
                @change="emit('update:importSourceMode', '7z')"
                value="7z"
                name="import-source-mode"
              />
              <span>7z文件</span>
            </label>
            <label class="radio-label">
              <input
                type="radio"
                :checked="importSourceMode === 'folder'"
                @change="emit('update:importSourceMode', 'folder')"
                value="folder"
                name="import-source-mode"
              />
              <span>.vdf / .lua 和 .manifest所在文件夹</span>
            </label>
          </div>

          <div class="source-select-actions">
            <button
              v-if="importSourceMode === '7z'"
              class="select-source-btn ui-btn ui-btn-sm ui-btn-outline"
              :disabled="isPreparingImport"
              @click="handleSelectImportArchive"
            >
              <svg v-if="isPreparingImport" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
              </svg>
              {{ isPreparingImport ? '处理中...' : '选择7z文件' }}
            </button>
            <button
              v-else
              class="select-source-btn ui-btn ui-btn-sm ui-btn-outline"
              :disabled="isPreparingImport"
              @click="handleSelectImportFolder"
            >
              <svg v-if="isPreparingImport" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
              </svg>
              {{ isPreparingImport ? '处理中...' : '选择文件夹' }}
            </button>
          </div>

          <p v-if="selectedImportPath && !importSourceReady" class="source-select-error">
            所选位置未找到vdf或lua文件，无法入库
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * GameImportTab 组件
 * 入库Steam标签页
 */

import type { GameConfigData } from '../../../types'
import Tooltip from '@/components/common/Tooltip.vue'
import SliderToggle from '@/components/common/SliderToggle.vue'

// ==================== Props ====================

const props = defineProps<{
  /** 游戏ID */
  gameId: string
  /** 游戏配置数据 */
  game: GameConfigData | undefined
  /** 是否有lua文件 */
  hasLua: boolean
  /** 是否可以入库Steam */
  canImportToSteam: boolean
  /** 当前选中的标签页 */
  currentTab: string
  /** 是否正在入库 */
  isImporting: boolean
  /** 是否正在准备导入 */
  isPreparingImport: boolean
  /** 自定义清单源是否准备就绪 */
  importSourceReady: boolean
  /** 锁定版本 */
  lockVersion: boolean
  /** 清单检查状态（用于显示替换清单按钮） */
  manifestCheckStatus: 'checking' | 'found' | 'not_found'
  /** 自定义清单源模式 */
  importSourceMode: '7z' | 'folder'
  /** 选中的导入路径 */
  selectedImportPath: string
}>()

// ==================== Emits ====================

const emit = defineEmits<{
  /** 使用OpenSteamTool入库 */
  (e: 'importWithOpenSteamTool'): void
  /** 重启Steam */
  (e: 'restartSteam'): void
  /** 选择7z压缩包 */
  (e: 'selectImportArchive'): void
  /** 选择文件夹 */
  (e: 'selectImportFolder'): void
  /** 清除导入源 */
  (e: 'clearImportSource'): void
  /** 更新锁定版本 */
  (e: 'update:lockVersion', value: boolean): void
  /** 更新导入源模式 */
  (e: 'update:importSourceMode', value: '7z' | 'folder'): void
  /** 替换清单：删除本地清单文件夹 */
  (e: 'replace-manifest'): void
  /** 打开清单下载二维码 */
  (e: 'open-qingdan-qr-code'): void
  /** 打开下载链接 */
  (e: 'openDownloadUrl', url: string): void
}>()

// ==================== 事件处理 ====================

function handleImportWithOpenSteamTool() {
  emit('importWithOpenSteamTool')
}

function handleRestartSteam() {
  emit('restartSteam')
}

function handleSelectImportArchive() {
  emit('selectImportArchive')
}

function handleSelectImportFolder() {
  emit('selectImportFolder')
}

function handleClearImportSource() {
  emit('clearImportSource')
}

function handleReplaceManifest() {
  emit('replace-manifest')
}

function openQingdanQRCode() {
  emit('open-qingdan-qr-code')
}

function openDownloadUrl(url: string) {
  emit('openDownloadUrl', url)
}
</script>

<style scoped>
/* 入库标签页顶部操作栏 */
.import-tab-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 5px;
  padding-bottom: 10px;
  border-bottom: 1px solid var(--steam-border);
}

.import-tab-header .panel-title {
  margin: 0;
  font-size: 16px;
  color: var(--steam-text-primary);
}

/* 替换清单按钮 */
.replace-manifest-btn {
  display: inline-flex;
  align-items: center;
  gap: 5px;
}

.replace-manifest-btn svg {
  width: 14px;
  height: 14px;
}

/* 入库Steam标签页样式 */
.import-steam-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
  align-items: flex-start;
}

.import-btn-group {
  display: flex;
  flex-direction: row;
  gap: 7px;
  align-items: center;
  flex-wrap: wrap;
}

.import-description {
  padding: 10px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  border-left: 4px solid #10b981;
}

.import-description p {
  margin: 0 0 5px 0;
  font-size: 14px;
  color: var(--steam-text-primary);
  line-height: 1.5;
}

.import-description p:last-child {
  margin-bottom: 0;
}

.import-note {
  font-size: 13px;
  color: var(--steam-text-secondary);
  font-style: italic;
}

.opensteamtool-options {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
  width: 100%;
  margin-bottom: 8px;
}

/* 防止 Tooltip 被 grid item 拉伸导致提示框偏移到右上角 */
.opensteamtool-options > .tooltip-wrapper {
  justify-self: start;
}

.option-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.option-label {
  font-size: 12px;
  color: var(--steam-text-secondary);
  font-weight: 500;
}

.option-hint {
  font-size: 11px;
  color: var(--steam-text-muted);
  margin: 0;
}

.option-toggle {
  display: flex;
  align-items: center;
  gap: 8px;
}

.toggle-checkbox {
  width: 16px;
  height: 16px;
  accent-color: var(--steam-accent);
}

.toggle-label {
  font-size: 12px;
  color: var(--steam-text-primary);
}

.option-input {
  width: 100%;
  padding: 8px 10px;
  background: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border);
  border-radius: 4px;
  color: var(--steam-text-primary);
  font-size: 13px;
}

.option-input::placeholder {
  color: var(--steam-text-muted);
}

@media (max-width: 700px) {
  .opensteamtool-options {
    grid-template-columns: 1fr;
  }
}

.import-error {
  color: #ef4444;
  font-size: 14px;
  margin: 0;
}

/* 自定义清单源信息 */
.import-source-info {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 10px 14px;
  background-color: rgba(59, 130, 246, 0.1);
  border-radius: 8px;
  border-left: 3px solid #3b82f6;
  font-size: 13px;
}

.import-source-info .source-label {
  color: var(--steam-text-secondary);
  flex-shrink: 0;
}

.import-source-info .source-path {
  color: var(--steam-text-primary);
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-family: 'Courier New', monospace;
}

.import-source-info .clear-source-btn {
  padding: 4px 10px;
  border: none;
  border-radius: 4px;
  background-color: rgba(239, 68, 68, 0.2);
  color: #ef4444;
  font-size: 12px;
  cursor: pointer;
  transition: background-color 0.15s ease;
  flex-shrink: 0;
}

.import-source-info .clear-source-btn:hover {
  background-color: rgba(239, 68, 68, 0.3);
}

/* 无清单文件时的引导区域 */
.import-no-files {
  display: flex;
  flex-direction: column;
  gap: 10px;
  width: 100%;
}

.manifest-missing-notice {
  padding: 10px;
  background-color: rgba(239, 68, 68, 0.1);
  border-radius: 8px;
  border-left: 4px solid #ef4444;
}

.manifest-missing-notice p {
  margin: 0;
  font-size: 14px;
  color: var(--steam-text-primary);
  line-height: 1.5;
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
  gap: 7px;
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
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 13px;
  color: var(--steam-text-primary);
  cursor: pointer;
}

.radio-label input[type="radio"] {
  width: 14px;
  height: 14px;
  accent-color: var(--steam-accent-blue);
}

.source-select-actions {
  display: flex;
  gap: 7px;
  align-items: center;
}

.source-select-error {
  margin: 0;
  font-size: 13px;
  color: #ef4444;
}

/* 加载图标动画 */
.loading-icon {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>