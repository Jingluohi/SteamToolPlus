<template>
  <!-- 入库Steam标签页 -->
  <div v-if="currentTab === 'import'" class="tab-panel">
    <h3 class="panel-title">入库Steam</h3>
    <div class="import-steam-content">
      <!-- 入库说明 -->
      <div class="import-description">
        <p>将游戏清单导入Steam客户端，导入后可在Steam库中下载和启动游戏，如果库不显示游戏，请重启steam。</p>
        <p class="import-note">注意：部分游戏入库下载后需要配合补丁才能正常游玩。</p>
      </div>

      <!-- OpenSteamTool高级选项 -->
      <div class="opensteamtool-options">
        <Tooltip text="勾选则锁定当前清单版本，不跟随Steam自动更新；不勾选则允许Steam自动更新游戏" position="top">
          <div class="option-item">
            <label class="option-label">是否更新</label>
            <div class="option-toggle">
              <input 
                type="checkbox" 
                :checked="lockVersion" 
                @change="emit('update:lockVersion', ($event.target as HTMLInputElement).checked)"
                class="toggle-checkbox" 
              />
              <span class="toggle-label">{{ lockVersion ? '锁定' : '跟随更新' }}</span>
            </div>
            <p class="option-hint">勾选：不跟随Steam更新，固定当前版本；不勾选：允许Steam自动更新</p>
          </div>
        </Tooltip>
      </div>

      <!-- 入库按钮组 -->
      <div class="import-btn-group">
        <!-- OpenSteamTool内核入库按钮（推荐） -->
        <button
          class="import-opensteamtool-btn-large"
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
          class="restart-steam-btn"
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
        <button class="clear-source-btn" @click="handleClearImportSource">清除</button>
      </div>

      <!-- 没有vdf/lua时显示下载引导和文件选择 -->
      <div v-if="!hasLua && !importSourceReady" class="import-no-files">
        <!-- 下载引导 -->
        <div class="download-guide">
          <p class="download-guide-title">点击下载对应清单文件7z，id：{{ gameId }}</p>
          <div class="download-buttons">
            <div class="download-btn-wrapper">
              <button
                class="download-patch-btn"
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
                class="download-patch-btn"
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
                class="download-patch-btn"
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
              class="select-source-btn"
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
              class="select-source-btn"
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
import Tooltip from '../../../components/common/Tooltip.vue'

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
  /** 打开清单下载二维码 */
  (e: 'openQingdanQRCode'): void
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

function openQingdanQRCode() {
  emit('openQingdanQRCode')
}

function openDownloadUrl(url: string) {
  emit('openDownloadUrl', url)
}
</script>

<style scoped>
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

.import-opensteamtool-btn-large {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 12px 26px;
  border: none;
  border-radius: 8px;
  background-color: #10b981;
  color: white;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.import-opensteamtool-btn-large:hover:not(.disabled) {
  background-color: #059669;
}

.import-opensteamtool-btn-large.disabled {
  background-color: var(--steam-text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

.import-opensteamtool-btn-large.loading {
  cursor: wait;
}

.import-opensteamtool-btn-large svg {
  width: 16px;
  height: 16px;
}

.restart-steam-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 13px 26px;
  border: none;
  border-radius: 8px;
  background-color: #64748b;
  color: white;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.restart-steam-btn:hover {
  background-color: #475569;
}

.restart-steam-btn svg {
  width: 16px;
  height: 16px;
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

.download-patch-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 10px 18px;
  border: none;
  border-radius: 8px;
  background-color: var(--steam-accent-blue);
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