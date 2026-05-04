<template>
  <div class="manifest-import-page">
    <div class="page-header">
      <h1>清单入库</h1>
      <p class="subtitle">将清单文件导入Steam，使游戏库中显示对应游戏</p>
    </div>

    <div class="main-content">
      <!-- 左侧：文件选择和配置 -->
      <div class="left-panel">
        <!-- Steam路径显示 -->
        <div class="section">
          <h3>Steam路径</h3>
          <div class="steam-path-display">
            <div v-if="steamPath" class="path-info">
              <span class="path-value">{{ steamPath }}</span>
              <span class="path-status success">已配置</span>
            </div>
            <div v-else class="path-info">
              <span class="path-value empty">未配置Steam路径</span>
              <span class="path-status error">请先前往全局设置配置Steam路径</span>
            </div>
          </div>
        </div>

        <!-- 文件选择 -->
        <div class="section">
          <h3>选择清单文件</h3>
          <p class="hint">支持选择文件夹或7z/zip压缩包</p>

          <div class="file-select-options">
            <Button variant="secondary" @click="selectFolder">
              选择文件夹
            </Button>
            <Button variant="secondary" @click="selectArchive">
              选择压缩包
            </Button>
          </div>

          <div v-if="selectedSource" class="selected-source">
            <div class="source-info">
              <span class="source-type">{{ sourceTypeText }}</span>
              <span class="source-path" :title="selectedSource">{{ selectedSource }}</span>
            </div>
            <Button variant="secondary" size="small" @click="clearSource">
              清除
            </Button>
          </div>
        </div>

        <!-- 扫描结果 -->
        <div class="section" v-if="scanResults.luaFiles.length > 0 || scanResults.manifestFiles.length > 0 || scanResults.vdfFiles.length > 0">
          <h3>扫描结果</h3>
          <div class="scan-stats">
            <div class="stat-item">
              <span class="stat-label">Lua文件:</span>
              <span class="stat-value">{{ scanResults.luaFiles.length }}个</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Manifest文件:</span>
              <span class="stat-value">{{ scanResults.manifestFiles.length }}个</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">VDF文件:</span>
              <span class="stat-value">{{ scanResults.vdfFiles.length }}个</span>
            </div>
          </div>

          <!-- 文件列表 -->
          <div class="file-lists">
            <div v-if="scanResults.luaFiles.length > 0" class="file-list-section">
              <h4>Lua文件</h4>
              <div class="file-list">
                <div
                  v-for="(file, index) in scanResults.luaFiles"
                  :key="'lua-'+index"
                  class="file-item"
                >
                  <span class="file-name">{{ getFileName(file) }}</span>
                </div>
              </div>
            </div>

            <div v-if="scanResults.manifestFiles.length > 0" class="file-list-section">
              <h4>Manifest文件</h4>
              <div class="file-list">
                <div
                  v-for="(file, index) in scanResults.manifestFiles"
                  :key="'manifest-'+index"
                  class="file-item"
                >
                  <span class="file-name">{{ getFileName(file) }}</span>
                </div>
              </div>
            </div>

            <div v-if="scanResults.vdfFiles.length > 0" class="file-list-section">
              <h4>VDF文件 (将自动转换)</h4>
              <div class="file-list">
                <div
                  v-for="(file, index) in scanResults.vdfFiles"
                  :key="'vdf-'+index"
                  class="file-item vdf-item"
                >
                  <span class="file-name">{{ getFileName(file) }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="section">
          <Button
            variant="primary"
            class="btn-import"
            :disabled="!canImport"
            :loading="isImporting"
            @click="startImport"
          >
            {{ isImporting ? '入库中...' : '开始清单入库' }}
          </Button>
          <Button
            variant="secondary"
            class="btn-restart-steam"
            :disabled="!steamPath"
            @click="restartSteam"
          >
            重启Steam
          </Button>
        </div>
      </div>

      <!-- 右侧：日志和结果 -->
      <div class="right-panel">
        <div class="section">
          <div class="section-header">
            <h3>操作日志</h3>
            <div class="stats" v-if="importStats.total > 0">
              <span>总计: <strong>{{ importStats.total }}</strong></span>
              <span class="success">成功: <strong>{{ importStats.success }}</strong></span>
              <span class="error">失败: <strong>{{ importStats.fail }}</strong></span>
            </div>
          </div>

          <div v-if="isImporting" class="progress-bar">
            <div class="progress-fill" :style="{ width: progressPercent + '%' }"></div>
            <span class="progress-text">{{ Math.round(progressPercent) }}%</span>
          </div>

          <div ref="logArea" class="log-area">
            <div v-if="logs.length === 0" class="log-placeholder">
              等待开始操作...
            </div>
            <div
              v-for="(log, index) in logs"
              :key="index"
              class="log-item"
              :class="log.type"
            >
              <span class="log-time">[{{ log.time }}]</span>
              {{ log.message }}
            </div>
          </div>

          <div class="button-group">
            <Button variant="secondary" @click="clearLogs">
              清空日志
            </Button>
          </div>
        </div>

        <div class="section info-section">
          <h3>使用说明</h3>
          <div class="info-content">
            <div class="info-block">
              <p><strong>入库流程：</strong></p>
              <ol>
                <li>配置Steam安装路径</li>
                <li>选择包含清单文件的文件夹或压缩包</li>
                <li>程序自动扫描并识别文件</li>
                <li>点击"开始清单入库"完成操作</li>
              </ol>
            </div>

            <div class="info-block">
              <p><strong>文件说明：</strong></p>
              <ul>
                <li><code>.lua</code> 文件 - Steam清单脚本文件</li>
                <li><code>.manifest</code> 文件 - 游戏清单数据文件</li>
                <li><code>.vdf</code> 文件 - 配置文件（将自动转换为lua）</li>
              </ul>
            </div>


          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import Button from '../../components/common/Button.vue'
import { useConfigStore } from '../../store/config.store'

interface LogItem {
  time: string
  message: string
  type: 'info' | 'success' | 'error'
}

interface ScanResults {
  luaFiles: string[]
  manifestFiles: string[]
  vdfFiles: string[]
}

interface ImportStats {
  total: number
  success: number
  fail: number
}

// Store
const configStore = useConfigStore()

// 状态
const steamPath = ref('')
const selectedSource = ref('')
const sourceType = ref<'folder' | 'archive' | null>(null)
const scanResults = ref<ScanResults>({
  luaFiles: [],
  manifestFiles: [],
  vdfFiles: []
})
const isScanning = ref(false)
const isImporting = ref(false)
const logs = ref<LogItem[]>([])
const progressPercent = ref(0)
const importStats = ref<ImportStats>({ total: 0, success: 0, fail: 0 })
const logArea = ref<HTMLDivElement>()

// 计算属性
const sourceTypeText = computed(() => {
  return sourceType.value === 'folder' ? '文件夹' : '压缩包'
})

const canImport = computed(() => {
  return steamPath.value &&
         selectedSource.value &&
         (scanResults.value.luaFiles.length > 0 || scanResults.value.vdfFiles.length > 0) &&
         scanResults.value.manifestFiles.length > 0 &&
         !isImporting.value
})

// 监听Steam路径变化（从全局配置读取）
watch(() => configStore.config?.gameDirs?.steamPath, (newPath) => {
  if (newPath) {
    steamPath.value = newPath
  }
}, { immediate: true })

// 初始化
onMounted(async () => {
  // 确保配置已加载
  if (!configStore.config) {
    await configStore.loadConfig()
  }
  // 从全局配置读取Steam路径
  if (configStore.config?.gameDirs?.steamPath) {
    steamPath.value = configStore.config.gameDirs.steamPath
  }
})

// 添加日志
function addLog(message: string, type: 'info' | 'success' | 'error' = 'info') {
  const time = new Date().toLocaleTimeString()
  logs.value.push({ time, message, type })
  nextTick(() => {
    if (logArea.value) {
      logArea.value.scrollTop = logArea.value.scrollHeight
    }
  })
}

// 选择文件夹
async function selectFolder() {
  const selected = await open({
    directory: true,
    title: '选择包含清单文件的文件夹'
  })

  if (selected && typeof selected === 'string') {
    selectedSource.value = selected
    sourceType.value = 'folder'
    addLog(`已选择文件夹: ${selected}`, 'info')
    await scanFolder(selected)
  }
}

// 选择压缩包
async function selectArchive() {
  const selected = await open({
    multiple: false,
    filters: [
      { name: '压缩文件', extensions: ['7z', 'zip', 'rar'] }
    ],
    title: '选择清单压缩包'
  })

  if (selected && typeof selected === 'string') {
    selectedSource.value = selected
    sourceType.value = 'archive'
    addLog(`已选择压缩包: ${selected}`, 'info')
    await extractAndScan(selected)
  }
}

// 扫描文件夹
async function scanFolder(folderPath: string) {
  isScanning.value = true
  addLog('开始扫描文件夹...', 'info')

  try {
    const results = await invoke<ScanResults>('scan_manifest_folder', {
      folderPath
    })

    scanResults.value = results

    const totalFiles = results.luaFiles.length + results.manifestFiles.length + results.vdfFiles.length
    addLog(`扫描完成，共找到 ${totalFiles} 个文件`, 'success')

    if (results.luaFiles.length > 0) {
      addLog(`  - Lua文件: ${results.luaFiles.length}个`, 'info')
    }
    if (results.manifestFiles.length > 0) {
      addLog(`  - Manifest文件: ${results.manifestFiles.length}个`, 'info')
    }
    if (results.vdfFiles.length > 0) {
      addLog(`  - VDF文件: ${results.vdfFiles.length}个 (将自动转换)`, 'info')
    }
  } catch (error) {
    addLog(`扫描失败: ${error}`, 'error')
  } finally {
    isScanning.value = false
  }
}

// 解压并扫描压缩包
async function extractAndScan(archivePath: string) {
  isScanning.value = true
  addLog('正在解压压缩包...', 'info')

  try {
    const tempFolder = await invoke<string>('extract_archive', { archivePath })
    addLog(`解压完成: ${tempFolder}`, 'success')
    await scanFolder(tempFolder)
  } catch (error) {
    addLog(`解压失败: ${error}`, 'error')
  } finally {
    isScanning.value = false
  }
}

// 清除选择
function clearSource() {
  selectedSource.value = ''
  sourceType.value = null
  scanResults.value = { luaFiles: [], manifestFiles: [], vdfFiles: [] }
  addLog('已清除选择', 'info')
}

// 获取文件名
function getFileName(path: string): string {
  const parts = path.split(/[\\/]/)
  return parts[parts.length - 1] || path
}

// 开始入库
async function startImport() {
  if (!steamPath.value) {
    addLog('错误: 未配置Steam路径', 'error')
    return
  }

  if (!selectedSource.value) {
    addLog('错误: 未选择清单文件', 'error')
    return
  }

  isImporting.value = true
  importStats.value = { total: 0, success: 0, fail: 0 }
  progressPercent.value = 0

  addLog('='.repeat(60), 'info')
  addLog('开始清单入库操作', 'info')
  addLog('='.repeat(60), 'info')

  try {
    const result = await invoke<{
      success: boolean
      message: string
      importedLua: number
      importedManifest: number
      convertedVdf: number
    }>('import_manifest_to_steam', {
      steamPath: steamPath.value,
      luaFiles: scanResults.value.luaFiles,
      manifestFiles: scanResults.value.manifestFiles,
      vdfFiles: scanResults.value.vdfFiles
    })

    if (result.success) {
      addLog('', 'info')
      addLog('入库操作完成！', 'success')
      addLog(`  - Lua文件: ${result.importedLua}个`, 'success')
      addLog(`  - Manifest文件: ${result.importedManifest}个`, 'success')
      if (result.convertedVdf > 0) {
        addLog(`  - VDF转换: ${result.convertedVdf}个`, 'success')
      }
      addLog('', 'info')
      addLog('提示: 请重启Steam以查看导入的游戏', 'info')
    } else {
      addLog(`入库失败: ${result.message}`, 'error')
    }
  } catch (error) {
    addLog(`入库操作失败: ${error}`, 'error')
  } finally {
    isImporting.value = false
    progressPercent.value = 100
  }
}

// 清空日志
function clearLogs() {
  logs.value = []
  importStats.value = { total: 0, success: 0, fail: 0 }
  progressPercent.value = 0
}

// 重启Steam
async function restartSteam() {
  if (!steamPath.value) {
    addLog('错误: 未配置Steam路径', 'error')
    return
  }

  addLog('正在重启Steam...', 'info')

  try {
    const result = await invoke<{ success: boolean; message: string }>('restart_steam', {
      steamPath: steamPath.value
    })

    if (result.success) {
      addLog('Steam重启成功', 'success')
    } else {
      addLog(`Steam重启失败: ${result.message}`, 'error')
    }
  } catch (error) {
    addLog(`Steam重启失败: ${error}`, 'error')
  }
}
</script>

<style scoped>
.manifest-import-page {
  max-width: 1400px;
  margin: 0 auto;
  padding: 24px;
}

.page-header {
  text-align: center;
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--steam-border);
}

.page-header h1 {
  font-size: 24px;
  margin-bottom: 8px;
  color: var(--steam-text-primary);
}

.subtitle {
  color: var(--steam-text-secondary);
  font-size: 14px;
}

.main-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
}

@media (max-width: 1000px) {
  .main-content {
    grid-template-columns: 1fr;
  }
}

.section {
  background: rgba(var(--steam-bg-primary-rgb), 0.15);
  backdrop-filter: blur(10px);
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 16px;
  border: 1px solid var(--steam-border);
}

.section h3 {
  font-size: 14px;
  color: var(--steam-text-primary);
  font-weight: 600;
  margin-bottom: 12px;
}

.section h4 {
  font-size: 12px;
  color: var(--steam-text-secondary);
  font-weight: 500;
  margin-bottom: 8px;
}

.hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin-bottom: 12px;
}

/* Steam路径显示 */
.steam-path-display {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.path-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 12px;
  background: var(--steam-bg-tertiary);
  border-radius: 4px;
  border: 1px solid var(--steam-border);
}

.path-value {
  font-size: 13px;
  color: var(--steam-text-primary);
  word-break: break-all;
}

.path-value.empty {
  color: var(--steam-text-muted);
  font-style: italic;
}

.path-status {
  font-size: 12px;
  font-weight: 500;
  padding: 4px 8px;
  border-radius: 4px;
  display: inline-block;
  align-self: flex-start;
}

.path-status.success {
  color: var(--steam-accent-green);
  background: rgba(76, 175, 80, 0.1);
}

.path-status.error {
  color: var(--steam-accent-red);
  background: rgba(244, 67, 54, 0.1);
}

/* 文件选择 */
.file-select-options {
  display: flex;
  gap: 12px;
  margin-bottom: 12px;
}

.selected-source {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: var(--steam-bg-tertiary);
  border-radius: 4px;
  border: 1px solid var(--steam-border);
}

.source-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
  min-width: 0;
}

.source-type {
  font-size: 12px;
  color: var(--steam-accent-blue);
  font-weight: 500;
}

.source-path {
  font-size: 12px;
  color: var(--steam-text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* 扫描结果 */
.scan-stats {
  display: flex;
  gap: 16px;
  margin-bottom: 16px;
  padding: 12px;
  background: var(--steam-bg-tertiary);
  border-radius: 4px;
}

.stat-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.stat-label {
  font-size: 11px;
  color: var(--steam-text-muted);
}

.stat-value {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.file-lists {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.file-list-section {
  border: 1px solid var(--steam-border);
  border-radius: 4px;
  padding: 12px;
  background: var(--steam-bg-tertiary);
}

.file-list {
  max-height: 120px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.file-item {
  padding: 6px 8px;
  background: var(--steam-bg-primary);
  border-radius: 4px;
  font-size: 12px;
}

.file-item.vdf-item {
  border-left: 3px solid var(--steam-accent-orange);
}

.file-name {
  color: var(--steam-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* 操作按钮 */
.btn-import {
  width: 100%;
  padding: 12px;
  font-size: 14px;
  margin-bottom: 12px;
}

.btn-restart-steam {
  width: 100%;
  padding: 12px;
  font-size: 14px;
}

/* 右侧面板 */
.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.stats {
  display: flex;
  gap: 16px;
  font-size: 12px;
}

.stats .success {
  color: var(--steam-accent-green);
}

.stats .error {
  color: var(--steam-accent-red);
}

.progress-bar {
  height: 28px;
  background: var(--steam-bg-tertiary);
  border-radius: 4px;
  overflow: hidden;
  position: relative;
  margin-bottom: 12px;
}

.progress-fill {
  height: 100%;
  background: var(--steam-accent-blue);
  transition: width 0.3s ease;
}

.progress-text {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 12px;
  font-weight: 600;
  color: #000;
}

.log-area {
  max-height: 300px;
  overflow-y: auto;
  background: var(--steam-bg-tertiary);
  border-radius: 4px;
  padding: 12px;
  font-family: Consolas, Monaco, monospace;
  font-size: 12px;
  line-height: 1.6;
  border: 1px solid var(--steam-border);
}

.log-placeholder {
  color: var(--steam-text-muted);
  text-align: center;
  padding: 24px;
}

.log-item {
  padding: 2px 0;
}

.log-time {
  color: var(--steam-text-muted);
  margin-right: 8px;
}

.log-item.success {
  color: var(--steam-accent-green);
}

.log-item.error {
  color: var(--steam-accent-red);
}

.button-group {
  display: flex;
  gap: 8px;
  margin-top: 12px;
}

/* 信息区域 */
.info-section {
  margin-top: 16px;
}

.info-content {
  font-size: 13px;
}

.info-block {
  margin-bottom: 16px;
}

.info-block:last-child {
  margin-bottom: 0;
}

.info-block p {
  color: var(--steam-text-secondary);
  margin-bottom: 8px;
}

.info-block ol,
.info-block ul {
  margin: 8px 0;
  padding-left: 20px;
  color: var(--steam-text-secondary);
}

.info-block li {
  margin: 4px 0;
}

.info-block code {
  background: var(--steam-bg-tertiary);
  padding: 2px 6px;
  border-radius: 3px;
  font-family: Consolas, Monaco, monospace;
  font-size: 12px;
  color: var(--steam-text-primary);
}
</style>
