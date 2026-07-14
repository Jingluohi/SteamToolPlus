<template>
  <!-- 补丁配置标签页 -->
  <div
    v-for="tab in patchTabs"
    :key="tab.id"
    v-show="currentTab === tab.id"
    class="tab-panel"
  >
    <h3 class="panel-title">{{ tab.name }}</h3>

    <!-- 补丁操作详细说明（仅当存在下载链接时显示） -->
    <div v-if="tab.downloadUrls && tab.downloadUrls.length > 0" class="patch-guide-box">
      <h4 class="guide-title">操作步骤：</h4>
      <ol class="guide-steps">
        <li><strong>第一步：</strong>先在页面顶部选择"游戏路径"，定位到游戏主程序（exe文件）所在的文件夹</li>
        <li><strong>第二步：</strong>查看下方"本地补丁状态"，如果显示"本地补丁未下载"，请点击网盘按钮下载该补丁的7z压缩包</li>
        <li><strong>第三步：</strong>下载完成后有两种方式应用补丁：
          <ul class="sub-steps">
            <li>方式一：点击"选择补丁文件（7z）并应用"按钮，直接选择刚下载的7z文件，程序会自动解压并应用到游戏目录</li>
            <li>方式二：手动解压7z文件到下方显示的"补丁路径"目录，然后点击"应用补丁"按钮</li>
          </ul>
        </li>
        <li><strong>第四步：</strong>等待应用完成，成功后即可运行游戏</li>
      </ol>
    </div>

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

      <!-- 下载补丁区域（当本地不存在且有下载链接时显示） -->
      <div v-if="!isPatchLocalExists(tab.patchType) && tab.downloadUrls && tab.downloadUrls.length > 0" class="download-section">
        <p class="download-section-title">下载补丁：</p>
        <div class="download-buttons">
          <div
            v-for="(item, index) in tab.downloadUrls"
            :key="index"
            class="download-btn-wrapper"
          >
            <button
              class="download-patch-btn"
              @click="emit('openDownloadUrl', item.url)"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                <polyline points="7 10 12 15 17 10"/>
                <line x1="12" y1="15" x2="12" y2="3"/>
              </svg>
              <span>{{ getDownloadSourceName(item.source) }}</span>
            </button>
            <p v-if="item.pwd || item.source === 'lanzou'" class="pwd-hint">
              提取码: {{ item.pwd || '1234' }}
            </p>
          </div>
        </div>
        <p class="download-hint">
          （请先转存至您的网盘，避免和谐，也将给作者带来无限的更新动力）
        </p>
      </div>

      <!-- 未选择游戏路径时的提示 -->
      <p class="game-path-display warning" v-if="!gamePath">
        请先选择游戏路径
      </p>

      <!-- 选择补丁文件并应用按钮（当本地不存在时显示） -->
      <button
        v-if="!isPatchLocalExists(tab.patchType)"
        class="select-and-apply-btn"
        @click="emit('selectAndApplyPatch', tab)"
        :disabled="!gamePath || applyingPatch"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
          <polyline points="14 2 14 8 20 8"/>
          <line x1="12" y1="18" x2="12" y2="12"/>
          <line x1="9" y1="15" x2="15" y2="15"/>
        </svg>
        <span>{{ applyingPatch ? '应用中...' : '选择补丁文件（7z）并应用' }}</span>
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
        @click="emit('applyPatch', tab)"
        :disabled="!gamePath || applyingPatch || !isPatchLocalExists(tab.patchType)"
      >
        <span v-if="applyingPatch">应用中...</span>
        <span v-else>应用补丁</span>
      </button>

      <!-- D加密虚拟机类型显示虚拟化环境配置教程按钮 -->
      <button
        v-if="tab.patchType === 3"
        class="tutorial-btn"
        @click="emit('openVirtualizationTutorial')"
      >
        虚拟化环境配置教程
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
</template>

<script setup lang="ts">
/**
 * PatchConfigTab 组件
 * 补丁配置标签页，负责显示补丁状态、下载引导、应用补丁等功能
 */

// ==================== Props 定义 ====================

interface PatchTab {
  id: string
  name: string
  patchType: number
  patchPath: string
  downloadUrls?: { source: string; url: string; pwd?: string | null }[]
}

interface Props {
  /** 当前选中的标签页 */
  currentTab: string
  /** 补丁标签页列表 */
  patchTabs: PatchTab[]
  /** 游戏路径 */
  gamePath: string
  /** 是否正在应用补丁 */
  applyingPatch: boolean
  /** 补丁应用结果 */
  patchResult: {
    success: boolean
    backed_up_files: string[]
    copied_files: string[]
    errors: string[]
  } | null
  /** 本地补丁文件存在状态缓存 */
  patchLocalStatus: Map<number, boolean>
  /** 补丁说明缓存 */
  patchReadmes: Map<number, string>
}

const props = defineProps<Props>()

// ==================== Emits 定义 ====================

interface Emits {
  /** 打开下载链接 */
  (e: 'openDownloadUrl', url: string): void
  /** 选择并应用补丁 */
  (e: 'selectAndApplyPatch', tab: PatchTab): void
  /** 应用补丁 */
  (e: 'applyPatch', tab: PatchTab): void
  /** 打开虚拟化环境配置教程 */
  (e: 'openVirtualizationTutorial'): void
}

const emit = defineEmits<Emits>()

// ==================== 辅助方法 ====================

/**
 * 获取本地补丁存在状态（从 props 中的 Map 查询）
 */
function isPatchLocalExists(patchType: number): boolean {
  return props.patchLocalStatus.get(patchType) || false
}

/**
 * 获取补丁说明（从 props 中的 Map 查询）
 */
function getPatchReadme(patchType: number): string {
  return props.patchReadmes.get(patchType) || ''
}

/**
 * 获取下载源名称
 */
function getDownloadSourceName(source: string): string {
  const sourceNames: Record<string, string> = {
    'lanzou': '蓝奏云',
    'quark': '夸克网盘',
    'baidu': '百度网盘',
    'xunlei': '迅雷网盘',
    '123': '123云盘'
  }
  return sourceNames[source] || source
}

/**
 * 获取文件名（从路径中提取）
 */
function getFileName(path: string): string {
  const parts = path.split(/[/\\]/)
  return parts[parts.length - 1] || path
}
</script>

<style scoped>
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

/* 补丁状态样式 */
.patch-status {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 10px 14px;
  border-radius: 8px;
  background-color: rgba(239, 68, 68, 0.1);
  font-size: 13px;
  font-weight: 500;
  margin-bottom: 7px;
}

.patch-status.local-exists {
  background-color: rgba(16, 185, 129, 0.1);
  color: #10b981;
}

.status-icon svg {
  width: 18px;
  height: 18px;
}

.status-text {
  font-size: 13px;
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
  gap: 7px;
  margin-bottom: 7px;
}

.download-btn-wrapper {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.download-patch-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px 16px;
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
  border: 1px solid var(--steam-border);
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.download-patch-btn:hover {
  background-color: var(--steam-bg-hover);
}

.download-patch-btn svg {
  width: 16px;
  height: 16px;
}

.pwd-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 2px 0 0 0;
  text-align: center;
}

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
  background-color: #8b5cf6;
  color: white;
  border: none;
  border-radius: 4px;
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
  font-size: 13px;
  color: var(--steam-text-secondary);
  margin: 0 0 7px 0;
}

.game-path-display {
  font-size: 13px;
  color: var(--steam-text-secondary);
  margin: 0 0 7px 0;
}

.game-path-display.warning {
  color: #ef4444;
  font-weight: 500;
}

/* 应用补丁按钮 */
.apply-patch-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 10px 24px;
  background-color: var(--steam-accent-blue);
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.15s ease;
  margin-bottom: 7px;
}

.apply-patch-btn:hover:not(:disabled) {
  background-color: var(--steam-bg-hover);
}

.apply-patch-btn:disabled {
  background-color: var(--steam-text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

/* 虚拟化环境配置教程按钮 */
.tutorial-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px 16px;
  background-color: rgba(156, 39, 176, 0.3);
  color: var(--steam-text-primary);
  border: 1px solid rgba(156, 39, 176, 0.7);
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  transition: background-color 0.15s ease;
  margin-bottom: 10px;
}

.tutorial-btn:hover {
  background: rgba(156, 39, 176, 0.4);
  border-color: rgba(156, 39, 176, 0.8);
}

/* 补丁说明 */
.patch-readme {
  margin-top: 10px;
  padding: 10px;
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
}

.readme-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 7px 0;
}

.readme-content {
  font-size: 12px;
  color: var(--steam-text-secondary);
  white-space: pre-wrap;
  word-wrap: break-word;
  line-height: 1.6;
  margin: 0;
  padding: 0;
  background: none;
  font-family: inherit;
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

.result-section {
  margin-top: 7px;
}

.section-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-text-primary);
  margin: 0 0 4px 0;
}

.file-list {
  margin: 0;
  padding-left: 20px;
  font-size: 12px;
  color: var(--steam-text-secondary);
}

.file-list li {
  margin-bottom: 2px;
}

.error-section {
  background-color: rgba(239, 68, 68, 0.1);
  padding: 7px;
  border-radius: 4px;
}

.error-list li {
  color: #ef4444;
}
</style>