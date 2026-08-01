<template>
  <!--
    UpdateCheck.vue - 检查更新页面
    显示本地与远程版本信息，支持手动检查更新、下载并应用更新
  -->
  <div class="update-check-page">
    <div class="update-card">
      <h1 class="page-title">检查更新</h1>
      <p class="page-subtitle">对比本地版本与 Gitee Release 上的最新版本</p>

      <!-- 版本信息面板 -->
      <div class="version-panel">
        <div class="version-section">
          <h2 class="section-title">本地版本</h2>
          <div class="version-row">
            <span class="version-label">主程序版本</span>
            <span class="version-value">{{ localAppVersion || '获取中...' }}</span>
          </div>
          <div class="version-row">
            <span class="version-label">资源版本</span>
            <span class="version-value">{{ localResourceVersion || '获取中...' }}</span>
          </div>
        </div>

        <div class="version-divider">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M5 12h14M12 5l7 7-7 7" />
          </svg>
        </div>

        <div class="version-section">
          <h2 class="section-title">远程最新版本</h2>
          <div class="version-row">
            <span class="version-label">资源版本</span>
            <span class="version-value" :class="{ 'version-newer': hasUpdate }">
              {{ remoteResourceVersion || '未检查' }}
            </span>
          </div>
          <div class="version-row">
            <span class="version-label">更新类型</span>
            <span class="version-value update-type">
              <span v-if="!hasChecked" class="type-placeholder">点击检查更新</span>
              <span v-else-if="!hasUpdate" class="type-latest">已是最新</span>
              <span v-else-if="remoteHasExeUpdate" class="type-exe">主程序 + 资源更新</span>
              <span v-else class="type-resource">仅资源更新</span>
            </span>
          </div>
        </div>
      </div>

      <!-- 更新说明 -->
      <div v-if="remoteDescription" class="description-box">
        <h3>更新说明</h3>
        <p>{{ remoteDescription }}</p>
      </div>

      <!-- 进度区域 -->
      <div v-if="isUpdating || downloadPercent > 0" class="progress-area">
        <div class="progress-header">
          <span class="progress-status">{{ statusText }}</span>
          <span class="progress-size">{{ downloadedText }} / {{ totalText }}</span>
        </div>
        <div class="progress-bar-bg">
          <div class="progress-bar-fill" :style="{ width: `${downloadPercent}%` }" />
        </div>
        <div class="progress-percent">{{ downloadPercent }}%</div>
      </div>

      <!-- 按钮区域 -->
      <div class="action-area">
        <Button
          variant="secondary"
          size="lg"
          :loading="isChecking"
          :disabled="isUpdating"
          @click="handleCheckUpdate"
        >
          {{ hasChecked ? '重新检查' : '检查更新' }}
        </Button>

        <Button
          v-if="hasUpdate"
          variant="primary"
          size="lg"
          :loading="isUpdating"
          :disabled="isUpdating"
          @click="handleUpdateNow"
        >
          {{ remoteHasExeUpdate ? '立即更新并重启' : '立即更新资源' }}
        </Button>
      </div>

      <!-- 手动下载链接 -->
      <div class="manual-links">
        <p class="manual-title">网盘备用下载</p>
        <div class="link-row">
          <button class="link-btn" @click="openQuarkLink">夸克网盘</button>
          <button class="link-btn" @click="openDownloadLink">百度网盘</button>
          <button class="link-btn" @click="openXunleiLink">迅雷网盘</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * UpdateCheck.vue - 检查更新页面
 * 提供手动检查更新、显示版本信息、下载并应用更新的完整界面
 */

import { ref, computed, onMounted, onUnmounted } from 'vue'
import { open } from '@tauri-apps/plugin-shell'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import Button from '../../components/common/Button.vue'

interface AppVersionInfo {
  app_version: string
  resource_version: string
}

interface UpdateCheckResult {
  has_update: boolean
  local_app_version: string
  local_resource_version: string
  remote_resource_version: string
  remote_description: string | null
  has_exe_update: boolean
  patch_url: string
}

interface DownloadProgress {
  url: string
  downloaded: number
  total: number
  percent: number
}

// 本地版本
const localAppVersion = ref('')
const localResourceVersion = ref('')

// 远程版本
const hasChecked = ref(false)
const hasUpdate = ref(false)
const remoteResourceVersion = ref('')
const remoteDescription = ref('')
const remoteHasExeUpdate = ref(false)
const patchUrl = ref('')

// 状态
const isChecking = ref(false)
const isUpdating = ref(false)
const statusText = ref('')
const downloadPercent = ref(0)
const downloadedBytes = ref(0)
const totalBytes = ref(0)

// 事件监听取消函数
let unlistenProgress: UnlistenFn | null = null
let unlistenFinished: UnlistenFn | null = null
let unlistenError: UnlistenFn | null = null

/**
 * 格式化字节数为可读文本
 */
function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(1024))
  return `${(bytes / Math.pow(1024, i)).toFixed(2)} ${units[i]}`
}

const downloadedText = computed(() => formatBytes(downloadedBytes.value))
const totalText = computed(() => formatBytes(totalBytes.value))

/**
 * 加载本地版本信息
 */
async function loadLocalVersions() {
  try {
    const info = await invoke<AppVersionInfo>('get_app_versions')
    localAppVersion.value = info.app_version
    localResourceVersion.value = info.resource_version
  } catch (error) {
    localAppVersion.value = '获取失败'
    localResourceVersion.value = '获取失败'
  }
}

/**
 * 检查更新
 */
async function handleCheckUpdate() {
  if (isChecking.value || isUpdating.value) return

  isChecking.value = true
  hasChecked.value = false
  statusText.value = '正在检查更新...'

  try {
    const result = await invoke<UpdateCheckResult>('check_for_update')

    hasChecked.value = true
    hasUpdate.value = result.has_update
    remoteResourceVersion.value = result.remote_resource_version
    remoteDescription.value = result.remote_description ?? ''
    remoteHasExeUpdate.value = result.has_exe_update
    patchUrl.value = result.patch_url

    statusText.value = result.has_update ? '发现新版本' : '当前已是最新版本'
  } catch (error) {
    hasChecked.value = true
    hasUpdate.value = false
    statusText.value = `检查更新失败: ${error}`
  } finally {
    isChecking.value = false
  }
}

/**
 * 立即更新
 */
async function handleUpdateNow() {
  if (!hasUpdate.value || isUpdating.value) return

  isUpdating.value = true
  downloadPercent.value = 0
  downloadedBytes.value = 0
  totalBytes.value = 0
  statusText.value = '正在下载更新...'

  try {
    const willRestart = await invoke<boolean>('apply_update', {
      patchUrl: patchUrl.value,
      hasExeUpdate: remoteHasExeUpdate.value,
    })

    if (!willRestart) {
      statusText.value = '资源更新完成，正在刷新...'
      await loadLocalVersions()
      hasUpdate.value = false
      downloadPercent.value = 0
      statusText.value = '更新完成'
    }
  } catch (error) {
    statusText.value = `更新失败: ${error}`
  } finally {
    isUpdating.value = false
  }
}

/**
 * 打开夸克网盘下载链接
 */
async function openQuarkLink() {
  try {
    await open('https://pan.quark.cn/s/bc38612b683f')
  } catch (error) {
    // 打开链接失败时静默处理
  }
}

/**
 * 打开百度网盘下载链接
 */
async function openDownloadLink() {
  try {
    await open('https://pan.baidu.com/s/1w5Cm4OYT97g3kl1g6WPk5Q?pwd=vfub')
  } catch (error) {
    // 打开链接失败时静默处理
  }
}

/**
 * 打开迅雷网盘下载链接
 */
async function openXunleiLink() {
  try {
    await open('https://pan.xunlei.com/s/VOrkq4Tq0c0Sootmhpp4433yA1?pwd=2tmn#')
  } catch (error) {
    // 打开链接失败时静默处理
  }
}

/**
 * 组件挂载时加载本地版本并注册事件监听
 */
onMounted(async () => {
  await loadLocalVersions()

  unlistenProgress = await listen<DownloadProgress>('download-progress', (event) => {
    downloadedBytes.value = event.payload.downloaded
    totalBytes.value = event.payload.total
    downloadPercent.value = event.payload.percent
    statusText.value = '正在下载更新...'
  })

  unlistenFinished = await listen<{ version: string; has_exe_update: boolean }>('resource-update-finished', (event) => {
    if (event.payload.has_exe_update) {
      statusText.value = 'exe 更新完成，正在重启...'
    } else {
      statusText.value = '资源更新完成'
      loadLocalVersions()
      hasUpdate.value = false
      downloadPercent.value = 0
    }
  })

  unlistenError = await listen<{ error: string }>('resource-update-error', (event) => {
    statusText.value = `更新失败: ${event.payload.error}`
    isUpdating.value = false
  })
})

/**
 * 组件卸载时清理事件监听
 */
onUnmounted(() => {
  unlistenProgress?.()
  unlistenFinished?.()
  unlistenError?.()
})
</script>

<style scoped>
.update-check-page {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
  overflow-y: auto;
}

.update-card {
  width: 100%;
  max-width: 680px;
  background: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border-color);
  border-radius: 4px;
  padding: 32px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.25);
}

.page-title {
  margin: 0 0 8px;
  font-size: 20px;
  font-weight: 600;
  color: var(--steam-text-primary);
  text-align: center;
}

.page-subtitle {
  margin: 0 0 28px;
  font-size: 14px;
  color: var(--steam-text-secondary);
  text-align: center;
}

.version-panel {
  display: flex;
  align-items: stretch;
  gap: 20px;
  margin-bottom: 24px;
  padding: 20px;
  background: var(--steam-bg-tertiary);
  border-radius: 4px;
}

.version-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.section-title {
  margin: 0 0 4px;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-secondary);
}

.version-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  font-size: 14px;
}

.version-label {
  color: var(--steam-text-secondary);
}

.version-value {
  font-weight: 500;
  color: var(--steam-text-primary);
}

.version-newer {
  color: var(--steam-accent-green);
  font-weight: 600;
}

.update-type {
  display: flex;
  align-items: center;
}

.type-placeholder {
  color: var(--steam-text-secondary);
}

.type-latest {
  color: var(--steam-accent-green);
}

.type-exe {
  color: #ff9f43;
  font-weight: 600;
}

.type-resource {
  color: var(--steam-accent-blue);
  font-weight: 600;
}

.version-divider {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  color: var(--steam-text-secondary);
}

.version-divider svg {
  width: 24px;
  height: 24px;
}

.description-box {
  margin-bottom: 24px;
  padding: 16px;
  background: var(--steam-bg-tertiary);
  border-radius: 4px;
  border-left: 3px solid var(--steam-accent-blue);
}

.description-box h3 {
  margin: 0 0 8px;
  font-size: 14px;
  color: var(--steam-text-primary);
}

.description-box p {
  margin: 0;
  font-size: 14px;
  color: var(--steam-text-secondary);
  line-height: 1.5;
}

.progress-area {
  margin-bottom: 24px;
  padding: 16px;
  background: var(--steam-bg-tertiary);
  border-radius: 4px;
}

.progress-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 10px;
  font-size: 13px;
}

.progress-status {
  color: var(--steam-text-primary);
}

.progress-size {
  color: var(--steam-text-secondary);
  font-family: monospace;
}

.progress-bar-bg {
  height: 8px;
  background: var(--steam-bg-primary);
  border-radius: 4px;
  overflow: hidden;
}

.progress-bar-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--steam-accent-blue), var(--steam-accent-green));
  transition: width 0.2s ease-out;
}

.progress-percent {
  margin-top: 8px;
  text-align: right;
  font-size: 13px;
  color: var(--steam-text-secondary);
  font-family: monospace;
}

.action-area {
  display: flex;
  gap: 16px;
  justify-content: center;
  margin-bottom: 32px;
}

.manual-links {
  padding-top: 24px;
  border-top: 1px solid var(--steam-border-color);
  text-align: center;
}

.manual-title {
  margin: 0 0 12px;
  font-size: 13px;
  color: var(--steam-text-secondary);
}

.link-row {
  display: flex;
  gap: 12px;
  justify-content: center;
}

.link-btn {
  padding: 6px 14px;
  font-size: 13px;
  color: var(--steam-text-secondary);
  background: transparent;
  border: 1px solid var(--steam-border-color);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s ease-out;
}

.link-btn:hover {
  color: var(--steam-text-primary);
  background: var(--steam-bg-tertiary);
  border-color: var(--steam-text-secondary);
}

@media (max-width: 600px) {
  .version-panel {
    flex-direction: column;
  }

  .version-divider {
    width: auto;
    transform: rotate(90deg);
  }

  .action-area {
    flex-direction: column;
  }
}
</style>
