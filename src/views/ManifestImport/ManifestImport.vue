<template>
  <div class="manifest-import-page">
    <!-- 清单下载夸克网盘二维码弹窗 -->
    <QRCodeModal
      v-model="showQRCodeModal"
      title="夸克网盘下载"
      :qr-image-url="qrCodeImageUrl"
      hint="请使用夸克APP扫码下载"
      @close="handleQRCodeClose"
    />

    <div class="page-header">
      <div class="page-title-row">
        <h1>清单入库</h1>
        <button
          class="manifest-link-btn"
          @click="openQingdanQRCode"
        >
          <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="7 10 12 15 17 10"/>
            <line x1="12" y1="15" x2="12" y2="3"/>
          </svg>
          夸克网盘
        </button>
        <span class="backup-label">备用（容易和谐）：</span>
        <button
          class="manifest-link-btn"
          @click="openExternalLink('https://pan.baidu.com/s/1FTZyknIObyzMuLAJC-Uj9g?pwd=8uwx')"
        >
          <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="7 10 12 15 17 10"/>
            <line x1="12" y1="15" x2="12" y2="3"/>
          </svg>
          百度网盘
        </button>
        <button
          class="manifest-link-btn"
          @click="openExternalLink('https://pan.xunlei.com/s/VOrmjucdcpCilK1xnElvCI9vA1?pwd=z2gb#')"
        >
          <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="7 10 12 15 17 10"/>
            <line x1="12" y1="15" x2="12" y2="3"/>
          </svg>
          迅雷网盘
        </button>
      </div>
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

        <!-- OpenSteamTool内核配置 -->
        <div class="section">
          <h3>入库方式</h3>
          <div class="setting-item">
            <div class="setting-info">
              <h4 class="setting-name">使用OpenSteamTool内核入库</h4>
              <p class="setting-desc">通过OpenSteamTool内核注入方式入库，兼容性更好，不需要manifest文件也可入库</p>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <h4 class="setting-name">热加载入库</h4>
              <p class="setting-desc">开启后入库时不重启Steam，依赖OpenSteamTool的文件监视自动加载新游戏</p>
            </div>
            <div class="setting-control">
              <Toggle v-model="hotReload" />
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <h4 class="setting-name">高级模式</h4>
              <p class="setting-desc">启用后会写入Windows注册表，通常用于Denuvo/在线游戏（需要二次确认）</p>
            </div>
            <div class="setting-control">
              <Toggle v-model="advancedMode" />
            </div>
          </div>

          <Tooltip text="锁定版本适用于限定补丁版本，防止游戏更新破坏补丁兼容性；跟随更新允许Steam自动更新游戏" position="top">
            <div class="setting-item">
              <div class="setting-info">
                <h4 class="setting-name">锁定版本</h4>
                <p class="setting-desc">开启后生成 setManifestid 强制锁定 depot 版本，适用于限定版本补丁；关闭则允许 Steam 自动更新</p>
              </div>
              <div class="setting-control">
                <Toggle v-model="lockVersion" />
              </div>
            </div>
          </Tooltip>

          <div class="setting-item setting-item-vertical">
            <div class="setting-info">
              <h4 class="setting-name">访问令牌 (Access Token)</h4>
              <p class="setting-desc">可选，用于下载受保护的游戏或DLC，仅在从VDF自动生成Lua时生效</p>
            </div>
            <div class="setting-control setting-control-full">
              <input
                v-model="accessToken"
                type="text"
                class="form-input"
                placeholder="输入访问令牌，留空则不添加 addtoken"
              />
            </div>
          </div>

          <div class="setting-item setting-item-vertical">
            <div class="setting-info">
              <h4 class="setting-name">成就数据 SteamID</h4>
              <p class="setting-desc">可选，用于拉取指定Steam账号的成就数据，仅在从VDF自动生成Lua时生效</p>
            </div>
            <div class="setting-control setting-control-full">
              <input
                v-model="statsSteamId"
                type="text"
                class="form-input"
                placeholder="输入SteamID，留空则不添加 setStat"
              />
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
            <Button variant="secondary" size="sm" @click="clearSource">
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
            {{ isImporting ? '入库中...' : '开始OpenSteamTool入库' }}
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
                <li>点击"开始OpenSteamTool入库"完成操作</li>
                <li>如果Steam已运行，可使用热加载入库避免重启</li>
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
import { open as openShell } from '@tauri-apps/plugin-shell'
import Button from '../../components/common/Button.vue'
import Toggle from '../GlobalSettings/components/Toggle.vue'
import Tooltip from '../../components/common/Tooltip.vue'
import QRCodeModal from '../../components/common/QRCodeModal.vue'
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

// 清单下载夸克网盘二维码弹窗状态
const showQRCodeModal = ref(false)
const qrCodeImageUrl = ref('')

/**
 * 打开清单下载夸克网盘二维码弹窗
 * 使用程序内置的 qingdan.png 二维码图片
 */
const openQingdanQRCode = async () => {
  qrCodeImageUrl.value = await invoke<string>('get_qingdan_image_base64')
  showQRCodeModal.value = true
}

/**
 * 关闭二维码弹窗
 */
const handleQRCodeClose = () => {
  showQRCodeModal.value = false
  qrCodeImageUrl.value = ''
}

// 状态
const steamPath = ref('')
const selectedSource = ref('')
const sourceType = ref<'folder' | 'archive' | null>(null)
// 实际用于入库的文件夹路径（压缩包解压后的临时目录）
const extractedFolderPath = ref('')
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

// 高级模式开关（写入注册表等）
const advancedMode = ref(false)
// 热加载开关（Steam运行时不重启）
const hotReload = ref(true)
// 访问令牌（用于受保护的游戏/DLC）
const accessToken = ref('')
// 成就数据SteamID（用于setStat）
const statsSteamId = ref('')
// 锁定版本开关（生成 setManifestid）
const lockVersion = ref(false)

// 计算属性
const sourceTypeText = computed(() => {
  return sourceType.value === 'folder' ? '文件夹' : '压缩包'
})

const canImport = computed(() => {
  if (!steamPath.value || !selectedSource.value || isImporting.value) {
    return false
  }
  // OpenSteamTool内核模式只需要Lua或VDF文件（manifest可选）
  return scanResults.value.luaFiles.length > 0 || scanResults.value.vdfFiles.length > 0
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
  // 从全局配置读取OpenSteamTool设置
  if (configStore.config?.opensteamtool) {
    advancedMode.value = configStore.config.opensteamtool.advancedMode || false
    hotReload.value = configStore.config.opensteamtool.hotReload ?? true
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
    extractedFolderPath.value = selected
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
    extractedFolderPath.value = tempFolder
    addLog(`解压完成: ${tempFolder}`, 'success')
    await scanFolder(tempFolder)
  } catch (error) {
    addLog(`解压失败: ${error}`, 'error')
    extractedFolderPath.value = ''
  } finally {
    isScanning.value = false
  }
}

// 清除选择
function clearSource() {
  selectedSource.value = ''
  extractedFolderPath.value = ''
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

  if (!selectedSource.value || !extractedFolderPath.value) {
    addLog('错误: 未选择清单文件', 'error')
    return
  }

  // OpenSteamTool高级模式二次确认
  if (advancedMode.value) {
    const confirmAdvanced = confirm(
      '高级模式已启用，将写入Windows注册表。\n\n' +
      '这通常用于Denuvo/在线游戏，但也可能带来风险。\n\n' +
      '是否继续？'
    )
    if (!confirmAdvanced) {
      return
    }
  }

  isImporting.value = true
  importStats.value = { total: 0, success: 0, fail: 0 }
  progressPercent.value = 0

  addLog('='.repeat(60), 'info')
  addLog('开始OpenSteamTool入库操作', 'info')
  addLog('='.repeat(60), 'info')

  try {
    const result = await invoke<{
      success: boolean
      message: string
      kernelInstalled: boolean
      luaWritten: boolean
      manifestCopied: number
      steamRestarted: boolean
      advancedEnabled: boolean
    }>('import_manifest_with_opensteamtool', {
      steamPath: steamPath.value,
      folderPath: extractedFolderPath.value,
      advancedMode: advancedMode.value,
      hotReload: hotReload.value,
      accessToken: accessToken.value || undefined,
      statsSteamId: statsSteamId.value || undefined,
      lockVersion: lockVersion.value
    })

    if (result.success) {
      addLog('', 'info')
      addLog('OpenSteamTool入库操作完成！', 'success')
      addLog(`  - 内核DLL: ${result.kernelInstalled ? '已安装' : '未安装'}`, 'success')
      addLog(`  - Lua文件: ${result.luaWritten ? '已写入' : '未写入'}`, 'success')
      addLog(`  - Manifest文件: ${result.manifestCopied}个`, 'success')
      addLog(`  - Steam: ${result.steamRestarted ? '已重启' : '未重启'}`, 'success')
      if (result.advancedEnabled) {
        addLog('  - 高级模式: 已启用（写入注册表）', 'info')
      }
      addLog('', 'info')
    } else {
      addLog(`OpenSteamTool入库失败: ${result.message}`, 'error')
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

// 打开外部链接
async function openExternalLink(url: string) {
  try {
    await openShell(url)
  } catch (error) {
    addLog(`打开链接失败: ${error}`, 'error')
  }
}
</script>

<style scoped>
.manifest-import-page {
  max-width: 1400px;
  margin: 0 auto;
  padding: 16px;
  height: 100%;
  overflow-y: auto;
  overflow-x: hidden;
}

.page-header {
  text-align: center;
  margin-bottom: 19px;
  padding-bottom: 13px;
  border-bottom: 1px solid var(--steam-border);
}

.page-title-row {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 13px;
  margin-bottom: 6px;
}

.page-header h1 {
  font-size: 24px;
  color: var(--steam-text-primary);
  margin: 0;
}

.backup-label {
  font-size: 13px;
  color: var(--steam-text-secondary);
  white-space: nowrap;
}

/* 网盘链接按钮 */
.manifest-link-btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background: var(--steam-accent-blue);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.manifest-link-btn:hover {
  background: var(--steam-accent-blue-hover, #4aa8ff);
  transform: translateY(-1px);
}

.btn-icon {
  width: 16px;
  height: 16px;
}

.subtitle {
  color: var(--steam-text-secondary);
  font-size: 14px;
}

.main-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 19px;
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
  padding: 13px;
  margin-bottom: 13px;
  border: 1px solid var(--steam-border);
}

.section h3 {
  font-size: 14px;
  color: var(--steam-text-primary);
  font-weight: 600;
  margin-bottom: 10px;
}

.section h4 {
  font-size: 12px;
  color: var(--steam-text-secondary);
  font-weight: 500;
  margin-bottom: 6px;
}

/* 设置项样式（用于OpenSteamTool开关） */
.setting-item {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid rgba(255,255,255,0.05);
  gap: 13px;
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-info {
  flex: 1;
  min-width: 0;
  padding-right: 13px;
}

.setting-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
  margin: 0 0 3px 0;
  line-height: 1.4;
}

.setting-desc {
  font-size: 12px;
  color: var(--steam-text-muted);
  line-height: 1.5;
  word-wrap: break-word;
  margin: 0;
}

.setting-control {
  min-width: 44px;
  display: flex;
  justify-content: flex-end;
  align-items: center;
  padding-top: 2px;
}

.setting-item-vertical {
  flex-direction: column;
  align-items: flex-start;
  gap: 8px;
}

.setting-item-vertical .setting-info {
  padding-right: 0;
}

.setting-control-full {
  width: 100%;
  min-width: auto;
  justify-content: flex-start;
}

.setting-control-full .form-input {
  width: 100%;
  padding: 8px 10px;
  background: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border);
  border-radius: 4px;
  color: var(--steam-text-primary);
  font-size: 13px;
}

.setting-control-full .form-input::placeholder {
  color: var(--steam-text-muted);
}

.hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin-bottom: 10px;
}

/* Steam路径显示 */
.steam-path-display {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.path-info {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 10px;
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
  gap: 10px;
  margin-bottom: 10px;
}

.selected-source {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px;
  background: var(--steam-bg-tertiary);
  border-radius: 4px;
  border: 1px solid var(--steam-border);
}

.source-info {
  display: flex;
  flex-direction: column;
  gap: 3px;
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
  gap: 13px;
  margin-bottom: 13px;
  padding: 10px;
  background: var(--steam-bg-tertiary);
  border-radius: 4px;
}

.stat-item {
  display: flex;
  flex-direction: column;
  gap: 3px;
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
  gap: 10px;
}

.file-list-section {
  border: 1px solid var(--steam-border);
  border-radius: 4px;
  padding: 10px;
  background: var(--steam-bg-tertiary);
}

.file-list {
  max-height: 120px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.file-item {
  padding: 5px 6px;
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
  margin-bottom: 10px;
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
  margin-bottom: 10px;
}

.stats {
  display: flex;
  gap: 13px;
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
  margin-bottom: 10px;
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
  padding: 10px;
  font-family: Consolas, Monaco, monospace;
  font-size: 12px;
  line-height: 1.6;
  border: 1px solid var(--steam-border);
}

.log-placeholder {
  color: var(--steam-text-muted);
  text-align: center;
  padding: 19px;
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
  gap: 6px;
  margin-top: 10px;
}

/* 信息区域 */
.info-section {
  margin-top: 13px;
}

.info-content {
  font-size: 13px;
}

.info-block {
  margin-bottom: 13px;
}

.info-block:last-child {
  margin-bottom: 0;
}

.info-block p {
  color: var(--steam-text-secondary);
  margin-bottom: 6px;
}

.info-block ol,
.info-block ul {
  margin: 6px 0;
  padding-left: 20px;
  color: var(--steam-text-secondary);
}

.info-block li {
  margin: 3px 0;
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
