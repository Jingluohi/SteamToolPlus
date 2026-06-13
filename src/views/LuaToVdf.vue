<template>
  <div class="lua-to-vdf-page">
    <div class="page-header">
      <h1>📄 Lua to VDF Converter</h1>
      <p class="subtitle">将 Steam 格式的 Lua 文件批量转换为 config.vdf 文件</p>
    </div>

    <div class="main-content">
      <!-- 左侧：文件列表 -->
      <div class="left-panel">
        <div class="section">
          <div class="section-header">
            <h3>📋 Lua 文件列表</h3>
            <span class="file-count">共 {{ fileList.length }} 个文件</span>
          </div>
          <p class="hint">拖拽文件到下方区域，或点击添加按钮</p>

          <div
            ref="dropZone"
            class="drop-zone"
            :class="{ 'drag-over': isDragging }"
            @dragenter.prevent="isDragging = true"
            @dragleave.prevent="isDragging = false"
            @dragover.prevent
            @drop.prevent="handleDrop"
          >
            <div class="drop-hint">
              <span class="icon">📁</span>
              <p>拖拽 Lua 文件到这里</p>
              <p class="sub-hint">支持批量拖拽，自动去重</p>
            </div>
          </div>

          <div class="file-list">
            <div v-if="fileList.length === 0" class="empty-state">
              暂无文件，请添加 Lua 文件
            </div>
            <div
              v-for="(file, index) in fileList"
              :key="index"
              class="file-item"
              :class="{ selected: file.selected, [file.status]: true }"
              @click="toggleSelect(index)"
            >
              <input
                type="checkbox"
                v-model="file.selected"
                @click.stop
              />
              <span class="file-icon">📄</span>
              <div class="file-info">
                <span class="file-name" :title="file.path">{{ file.name }}</span>
                <span class="file-size">{{ formatFileSize(file.size) }}</span>
              </div>
              <span class="file-status">{{ getStatusIcon(file.status) }}</span>
            </div>
          </div>

          <div class="button-group">
            <Button variant="secondary" @click="addFiles">
              ➕ 添加文件
            </Button>
            <Button variant="secondary" @click="addFolder">
              📁 添加文件夹
            </Button>
            <Button
              variant="secondary"
              :disabled="!hasSelected"
              @click="removeSelected"
            >
              ➖ 移除选中
            </Button>
            <Button
              variant="secondary"
              :disabled="fileList.length === 0"
              @click="clearAll"
            >
              清空列表
            </Button>
          </div>
        </div>

        <div class="section">
          <h3>转换控制</h3>
          <Button
            variant="primary"
            class="btn-convert"
            :disabled="fileList.length === 0 || isConverting"
            @click="startConvert"
          >
            {{ isConverting ? '转换中...' : '开始批量转换' }}
          </Button>
        </div>
      </div>

      <!-- 右侧：结果和日志 -->
      <div class="right-panel">
        <div class="section">
          <div class="section-header">
            <h3>转换结果</h3>
            <div class="stats">
              <span>总计: <strong>{{ stats.total }}</strong></span>
              <span class="success">成功: <strong>{{ stats.success }}</strong></span>
              <span class="error">失败: <strong>{{ stats.fail }}</strong></span>
            </div>
          </div>

          <div v-if="isConverting" class="progress-bar">
            <div class="progress-fill" :style="{ width: progressPercent + '%' }"></div>
            <span class="progress-text">{{ Math.round(progressPercent) }}%</span>
          </div>

          <div ref="logArea" class="log-area">
            <div v-if="logs.length === 0" class="log-placeholder">
              等待开始转换...
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
          <h3>格式说明</h3>
          <div class="info-content">
            <div class="format-block">
              <p><strong>支持的Lua格式：</strong></p>
              <code>addappid(depot_id, 1, "decryption_key")</code>
              <p class="example">示例：addappid(123456, 1, "a1b2c3d4e5f6...")</p>
            </div>

            <div class="format-block">
              <p><strong>生成的VDF格式：</strong></p>
              <pre>"depots"
{
    "depot_id"
    {
        "DecryptionKey" "key"
    }
}</pre>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import Button from '../../components/common/Button.vue'

interface FileItem {
  name: string
  path: string
  size: number
  selected: boolean
  status: 'pending' | 'converting' | 'success' | 'error'
}

interface LogItem {
  time: string
  message: string
  type: 'info' | 'success' | 'error'
}

const fileList = ref<FileItem[]>([])
const logs = ref<LogItem[]>([])
const isConverting = ref(false)
const isDragging = ref(false)
const progressPercent = ref(0)
const stats = ref({ total: 0, success: 0, fail: 0 })

const dropZone = ref<HTMLDivElement>()
const logArea = ref<HTMLDivElement>()

const hasSelected = computed(() => fileList.value.some(f => f.selected))

function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

function getStatusIcon(status: string): string {
  const icons: Record<string, string> = {
    pending: '⏳',
    converting: '🔄',
    success: '✓',
    error: '✗'
  }
  return icons[status] || ''
}

function addLog(message: string, type: 'info' | 'success' | 'error' = 'info') {
  const time = new Date().toLocaleTimeString()
  logs.value.push({ time, message, type })
  // 滚动到底部
  setTimeout(() => {
    if (logArea.value) {
      logArea.value.scrollTop = logArea.value.scrollHeight
    }
  }, 0)
}

async function handleDrop(e: DragEvent) {
  isDragging.value = false
  const files = Array.from(e.dataTransfer?.files || [])
  await processFiles(files)
}

async function processFiles(files: File[]) {
  let addedCount = 0
  for (const file of files) {
    if (!file.name.toLowerCase().endsWith('.lua')) continue

    const exists = fileList.value.some(f => f.name === file.name && f.size === file.size)
    if (!exists) {
      fileList.value.push({
        name: file.name,
        path: file.name,
        size: file.size,
        selected: false,
        status: 'pending'
      })
      addedCount++
    }
  }
  if (addedCount > 0) {
    addLog(`✓ 成功添加 ${addedCount} 个 Lua 文件`, 'success')
  }
}

async function addFiles() {
  const selected = await open({
    multiple: true,
    filters: [{ name: 'Lua文件', extensions: ['lua'] }]
  })
  if (selected && Array.isArray(selected)) {
    // 读取文件信息
    for (const path of selected) {
      try {
        const content = await invoke<string>('read_file_content', { path })
        const exists = fileList.value.some(f => f.path === path)
        if (!exists) {
          fileList.value.push({
            name: path.split(/[\\/]/).pop() || path,
            path,
            size: content.length,
            selected: false,
            status: 'pending'
          })
        }
      } catch (e) {
        addLog(`无法读取文件: ${path}`, 'error')
      }
    }
    addLog(`✓ 成功添加 ${selected.length} 个文件`, 'success')
  }
}

async function addFolder() {
  const folder = await open({ directory: true })
  if (folder) {
    try {
      const files = await invoke<string[]>('get_lua_files_in_folder', { folder })
      for (const path of files) {
        const exists = fileList.value.some(f => f.path === path)
        if (!exists) {
          fileList.value.push({
            name: path.split(/[\\/]/).pop() || path,
            path,
            size: 0,
            selected: false,
            status: 'pending'
          })
        }
      }
      addLog(`✓ 从文件夹添加 ${files.length} 个 Lua 文件`, 'success')
    } catch (e) {
      addLog('扫描文件夹失败', 'error')
    }
  }
}

function toggleSelect(index: number) {
  fileList.value[index].selected = !fileList.value[index].selected
}

function removeSelected() {
  fileList.value = fileList.value.filter(f => !f.selected)
}

function clearAll() {
  if (confirm(`确定要清空所有 ${fileList.value.length} 个文件吗？`)) {
    fileList.value = []
    addLog('文件列表已清空', 'info')
  }
}

function clearLogs() {
  logs.value = []
  stats.value = { total: 0, success: 0, fail: 0 }
  progressPercent.value = 0
}

// 监听拖放的Lua文件
let unlisten: UnlistenFn | null = null

onMounted(async () => {
  unlisten = await listen<string[]>('lua-files-dropped', (event) => {
    const files = event.payload
    addLog(`检测到 ${files.length} 个Lua文件被拖入`, 'info')
    processDroppedFiles(files)
  })
})

onUnmounted(() => {
  if (unlisten) {
    unlisten()
  }
})

async function processDroppedFiles(paths: string[]) {
  let addedCount = 0
  for (const path of paths) {
    const exists = fileList.value.some(f => f.path === path)
    if (!exists) {
      try {
        const content = await invoke<string>('read_file_content', { path })
        fileList.value.push({
          name: path.split(/[\\/]/).pop() || path,
          path,
          size: content.length,
          selected: false,
          status: 'pending'
        })
        addedCount++
      } catch (e) {
        addLog(`无法读取文件: ${path}`, 'error')
      }
    }
  }
  if (addedCount > 0) {
    addLog(`✓ 成功添加 ${addedCount} 个 Lua 文件`, 'success')
  }
}

async function startConvert() {
  if (fileList.value.length === 0) return

  stats.value = { total: fileList.value.length, success: 0, fail: 0 }
  isConverting.value = true
  progressPercent.value = 0

  addLog('='.repeat(60), 'info')
  addLog(`🚀 开始批量转换 - 共 ${fileList.value.length} 个文件`, 'info')
  addLog('='.repeat(60), 'info')

  for (let i = 0; i < fileList.value.length; i++) {
    const file = fileList.value[i]
    file.status = 'converting'

    addLog('', 'info')
    addLog(`[${i + 1}/${fileList.value.length}] 正在处理: ${file.name}`, 'info')

    try {
      const result = await invoke<{ success: boolean; message: string; depotCount: number }>(
        'convert_lua_to_vdf',
        { filePath: file.path }
      )

      if (result.success) {
        addLog(`    ✓ 成功 (${result.depotCount} 个 depot)`, 'success')
        file.status = 'success'
        stats.value.success++
      } else {
        addLog(`    ✗ ${result.message}`, 'error')
        file.status = 'error'
        stats.value.fail++
      }
    } catch (e) {
      addLog(`    ✗ 错误: ${e}`, 'error')
      file.status = 'error'
      stats.value.fail++
    }

    progressPercent.value = ((i + 1) / fileList.value.length) * 100
  }

  addLog('', 'info')
  addLog('='.repeat(60), 'info')
  addLog(`📊 转换完成: 成功 ${stats.value.success} 个, 失败 ${stats.value.fail} 个`, 'info')
  addLog('='.repeat(60), 'info')

  isConverting.value = false

  if (stats.value.fail === 0) {
    alert(`批量转换完成！\n\n成功: ${stats.value.success} 个文件`)
  } else {
    alert(`批量转换完成，但部分文件处理失败。\n\n成功: ${stats.value.success} 个\n失败: ${stats.value.fail} 个`)
  }
}
</script>

<style scoped>
.lua-to-vdf-page {
  max-width: 1400px;
  margin: 0 auto;
  padding: 24px;
  height: 100%;
  overflow-y: auto;
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

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.section h3 {
  font-size: 14px;
  color: var(--steam-text-primary);
  font-weight: 600;
}

.file-count {
  font-size: 12px;
  color: var(--steam-text-secondary);
}

.hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin-bottom: 12px;
}

.drop-zone {
  border: 2px dashed var(--steam-border);
  border-radius: 8px;
  padding: 32px;
  text-align: center;
  background: var(--steam-bg-tertiary);
  transition: all 0.2s;
  margin-bottom: 12px;
  cursor: pointer;
}

.drop-zone.drag-over {
  border-color: var(--steam-accent-blue);
  background: rgba(102, 192, 244, 0.1);
}

.drop-hint .icon {
  font-size: 36px;
  display: block;
  margin-bottom: 8px;
}

.drop-hint p {
  margin: 4px 0;
  color: var(--steam-text-secondary);
}

.drop-hint .sub-hint {
  font-size: 12px;
  color: var(--steam-text-muted);
}

.file-list {
  max-height: 200px;
  overflow-y: auto;
  background: var(--steam-bg-tertiary);
  border-radius: 4px;
  padding: 8px;
  margin-bottom: 12px;
  border: 1px solid var(--steam-border);
}

.empty-state {
  text-align: center;
  color: var(--steam-text-secondary);
  padding: 24px;
  font-size: 13px;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.file-item:hover {
  background: var(--steam-bg-secondary);
}

.file-item.selected {
  background: rgba(102, 192, 244, 0.2);
}

.file-item.success {
  border-left: 3px solid var(--steam-accent-green);
}

.file-item.error {
  border-left: 3px solid var(--steam-accent-red);
}

.file-icon {
  font-size: 16px;
}

.file-info {
  flex: 1;
  min-width: 0;
}

.file-name {
  display: block;
  font-weight: 500;
  color: var(--steam-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-size {
  font-size: 11px;
  color: var(--steam-text-muted);
}

.file-status {
  font-size: 14px;
}

.button-group {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.btn-convert {
  width: 100%;
  padding: 12px;
  font-size: 14px;
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

.info-section {
  margin-top: 16px;
}

.info-content {
  font-size: 13px;
}

.format-block {
  margin-bottom: 16px;
}

.format-block p {
  color: var(--steam-text-secondary);
  margin-bottom: 8px;
}

.format-block code {
  display: block;
  background: var(--steam-bg-tertiary);
  padding: 10px;
  border-radius: 4px;
  font-family: Consolas, Monaco, monospace;
  font-size: 12px;
  color: var(--steam-text-primary);
  border: 1px solid var(--steam-border);
  margin: 8px 0;
}

.format-block .example {
  font-size: 11px;
  color: var(--steam-text-muted);
  margin-top: 4px;
}

.format-block pre {
  background: var(--steam-bg-tertiary);
  padding: 12px;
  border-radius: 4px;
  font-family: Consolas, Monaco, monospace;
  font-size: 12px;
  overflow-x: auto;
  color: var(--steam-text-primary);
  border: 1px solid var(--steam-border);
  margin: 8px 0;
}
</style>
