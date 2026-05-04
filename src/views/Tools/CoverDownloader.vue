<template>
  <div class="cover-downloader-page">
    <div class="page-header">
      <h1>Steam 封面下载器</h1>
      <p class="subtitle">批量下载 Steam 游戏封面图片，支持多种尺寸</p>
    </div>

    <div class="main-content">
      <!-- 左侧：输入和控制 -->
      <div class="left-panel">
        <div class="section">
          <div class="section-header">
            <h3>游戏 ID 输入</h3>
            <span class="count">共 {{ gameIds.length }} 个游戏</span>
          </div>
          <p class="hint">每行输入一个游戏 ID，支持自动去重</p>

          <textarea
            v-model="gameInput"
            class="game-input"
            placeholder="请输入 Steam 游戏 ID，每行一个&#10;例如：&#10;730&#10;570&#10;440"
            @input="updateGameCount"
          ></textarea>

          <div class="button-group">
            <Button variant="secondary" @click="clearInput">
              清空
            </Button>
            <Button variant="secondary" @click="insertExample">
              示例
            </Button>
          </div>
        </div>

        <div class="section">
          <h3>输出目录</h3>
          <div class="output-path">
            <input
              type="text"
              v-model="outputPath"
              readonly
              placeholder="默认：下载文件夹"
            />
            <Button variant="secondary" @click="selectOutputFolder">
              选择目录
            </Button>
          </div>
        </div>

        <div class="section">
          <h3>下载设置</h3>
          <div class="setting-row">
            <label>并发数：</label>
            <input
              type="number"
              v-model.number="concurrentCount"
              min="1"
              max="20"
            />
            <span class="hint">建议 5-10</span>
          </div>
        </div>

        <div class="section">
          <div v-if="isDownloading" class="progress-bar">
            <div class="progress-fill" :style="{ width: progressPercent + '%' }"></div>
            <span class="progress-text">{{ Math.round(progressPercent) }}%</span>
          </div>

          <div class="stats-display">
            {{ isDownloading ? `下载中... 成功: ${stats.success} | 失败: ${stats.fail}` : '等待开始...' }}
          </div>

          <div class="button-group download-btns">
            <Button
              variant="primary"
              class="btn-download"
              :disabled="!canDownload || isDownloading"
              @click="startDownload"
            >
              {{ isDownloading ? '下载中...' : '开始下载' }}
            </Button>
            <Button
              variant="secondary"
              :disabled="!isDownloading"
              @click="stopDownload"
            >
              停止
            </Button>
          </div>
        </div>
      </div>

      <!-- 右侧：尺寸选择和结果 -->
      <div class="right-panel">
        <div class="section">
          <div class="section-header">
            <h3>封面尺寸选择</h3>
            <div class="select-btns">
              <Button variant="secondary" size="sm" @click="selectAll">
                全选
              </Button>
              <Button variant="secondary" size="sm" @click="deselectAll">
                全不选
              </Button>
            </div>
          </div>

          <div class="size-list">
            <label
              v-for="(size, index) in coverSizes"
              :key="index"
              class="size-item"
              :title="size.tooltip"
            >
              <input
                type="checkbox"
                v-model="selectedSizes"
                :value="index"
              />
              <div class="size-info">
                <span class="size-name">{{ size.desc }}</span>
                <span class="size-dims">{{ size.width }} × {{ size.height }}</span>
              </div>
            </label>
          </div>
        </div>

        <div class="section">
          <div class="section-header">
            <h3>下载结果</h3>
            <Button variant="secondary" size="sm" @click="clearResults">
              清空
            </Button>
          </div>

          <div ref="resultArea" class="result-area">
            <div v-if="results.length === 0" class="result-placeholder">
              下载结果将显示在这里...
            </div>
            <div
              v-for="(result, index) in results"
              :key="index"
              class="result-item"
              :class="result.type"
            >
              <span class="result-time">[{{ result.time }}]</span>
              {{ result.message }}
            </div>
          </div>
        </div>

        <div class="section info-section">
          <h3>使用说明</h3>
          <div class="info-content">
            <p><strong>如何获取游戏 ID：</strong></p>
            <ol>
              <li>打开 Steam 商店页面</li>
              <li>查看 URL 中的数字，如：<code>store.steampowered.com/app/730</code></li>
              <li>730 就是 CS2 的游戏 ID</li>
            </ol>
            <p class="hint">支持批量输入，每行一个游戏 ID</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import Button from '../../components/common/Button.vue'

interface CoverSize {
  width: number
  height: number
  desc: string
  type: string
  ext: string
  tooltip: string
}

interface ResultItem {
  time: string
  message: string
  type: 'info' | 'success' | 'error'
}

const coverSizes: CoverSize[] = [
  { width: 600, height: 900, desc: '库封面 (600x900)', type: 'library_600x900', ext: 'jpg', tooltip: 'Steam 库中显示的竖版封面，最常用 ★推荐' },
  { width: 1920, height: 620, desc: '库英雄图 (1920x620)', type: 'library_hero', ext: 'jpg', tooltip: '游戏详情页顶部横幅 ★推荐' },
  { width: 460, height: 215, desc: '库横幅 (460x215)', type: 'library_header', ext: 'jpg', tooltip: 'Steam库中显示的游戏横幅 ★推荐' },
  { width: 32, height: 32, desc: '小图标 (32x32)', type: 'icon', ext: 'jpg', tooltip: '游戏小图标' },
  { width: 1920, height: 1080, desc: '透明宣传图', type: 'logo', ext: 'png', tooltip: '透明背景宣传图' }
]

const gameInput = ref('')
const gameIds = ref<string[]>([])
const selectedSizes = ref<number[]>([0, 1, 2]) // 默认选中前三个
const outputPath = ref('')
const concurrentCount = ref(5)
const isDownloading = ref(false)
const shouldStop = ref(false)
const progressPercent = ref(0)
const stats = ref({ total: 0, success: 0, fail: 0 })
const results = ref<ResultItem[]>([])

const resultArea = ref<HTMLDivElement>()

const canDownload = computed(() => {
  return gameIds.value.length > 0 && selectedSizes.value.length > 0
})

function updateGameCount() {
  const lines = gameInput.value.split('\n')
  const ids: string[] = []
  for (const line of lines) {
    const trimmed = line.trim()
    if (!trimmed) continue
    const match = trimmed.match(/^(\d+)/)
    if (match && !ids.includes(match[1])) {
      ids.push(match[1])
    }
  }
  gameIds.value = ids
}

function clearInput() {
  gameInput.value = ''
  gameIds.value = []
}

function insertExample() {
  gameInput.value = '730\n570\n440\n' + gameInput.value
  updateGameCount()
}

async function selectOutputFolder() {
  const folder = await open({ directory: true })
  if (folder) {
    outputPath.value = folder
    addResult(`输出目录已设置: ${folder}`, 'info')
  }
}

function selectAll() {
  selectedSizes.value = coverSizes.map((_, i) => i)
}

function deselectAll() {
  selectedSizes.value = []
}

function addResult(message: string, type: 'info' | 'success' | 'error' = 'info') {
  const time = new Date().toLocaleTimeString()
  results.value.push({ time, message, type })
  setTimeout(() => {
    if (resultArea.value) {
      resultArea.value.scrollTop = resultArea.value.scrollHeight
    }
  }, 0)
}

function clearResults() {
  results.value = []
  stats.value = { total: 0, success: 0, fail: 0 }
}

function stopDownload() {
  shouldStop.value = true
}

async function startDownload() {
  if (gameIds.value.length === 0 || selectedSizes.value.length === 0) return

  stats.value = { total: gameIds.value.length * selectedSizes.value.length, success: 0, fail: 0 }
  isDownloading.value = true
  shouldStop.value = false
  progressPercent.value = 0

  addResult('='.repeat(60), 'info')
  addResult(`开始下载 ${gameIds.value.length} 个游戏的 ${selectedSizes.value.length} 种尺寸封面`, 'info')
  addResult('='.repeat(60), 'info')

  const tasks: { gameId: string; sizeIndex: number }[] = []
  for (const gameId of gameIds.value) {
    for (const sizeIndex of selectedSizes.value) {
      tasks.push({ gameId, sizeIndex })
    }
  }

  let completed = 0
  const activeDownloads = new Set<Promise<void>>()

  for (const task of tasks) {
    if (shouldStop.value) {
      addResult('下载已取消', 'info')
      break
    }

    while (activeDownloads.size >= concurrentCount.value) {
      await Promise.race(activeDownloads)
    }

    const promise = downloadSingleCover(task.gameId, task.sizeIndex)
      .then(() => {
        completed++
        progressPercent.value = (completed / tasks.length) * 100
      })
      .finally(() => {
        activeDownloads.delete(promise)
      })

    activeDownloads.add(promise)
  }

  await Promise.all(activeDownloads)

  addResult('', 'info')
  addResult('='.repeat(60), 'info')
  addResult(`下载完成: 成功 ${stats.value.success} 个, 失败 ${stats.value.fail} 个`, 'info')
  addResult('='.repeat(60), 'info')

  isDownloading.value = false

  if (stats.value.fail === 0) {
    alert(`批量下载完成！\n\n成功: ${stats.value.success} 张图片`)
  } else {
    alert(`批量下载完成，但部分图片下载失败。\n\n成功: ${stats.value.success} 张\n失败: ${stats.value.fail} 张`)
  }
}

async function downloadSingleCover(gameId: string, sizeIndex: number) {
  if (shouldStop.value) return

  const size = coverSizes[sizeIndex]

  try {
    const result = await invoke<{ success: boolean; message: string }>('download_steam_cover', {
      gameId,
      sizeType: size.type,
      sizeDesc: size.desc,
      width: size.width,
      height: size.height,
      outputPath: outputPath.value || null
    })

    if (result.success) {
      addResult(`[成功] ${gameId} - ${size.desc}`, 'success')
      stats.value.success++
    } else {
      addResult(`[失败] ${gameId} - ${size.desc}: ${result.message}`, 'error')
      stats.value.fail++
    }
  } catch (e) {
    addResult(`[失败] ${gameId} - ${size.desc}: ${e}`, 'error')
    stats.value.fail++
  }
}
</script>

<style scoped>
.cover-downloader-page {
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
  background: var(--steam-bg-primary);
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

.count {
  font-size: 12px;
  color: var(--steam-text-secondary);
}

.hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin-bottom: 12px;
}

.game-input {
  width: 100%;
  height: 150px;
  background: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border);
  border-radius: 4px;
  padding: 12px;
  color: var(--steam-text-primary);
  font-family: Consolas, Monaco, monospace;
  font-size: 13px;
  resize: vertical;
  margin-bottom: 12px;
}

.output-path {
  display: flex;
  gap: 8px;
}

.output-path input {
  flex: 1;
  padding: 8px 12px;
  background: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border);
  border-radius: 4px;
  color: var(--steam-text-primary);
  font-size: 13px;
}

.setting-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.setting-row label {
  font-size: 13px;
  color: var(--steam-text-secondary);
}

.setting-row input[type="number"] {
  width: 60px;
  padding: 6px 8px;
  background: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border);
  border-radius: 4px;
  color: var(--steam-text-primary);
  font-size: 13px;
}

.button-group {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.btn-download {
  flex: 2;
  padding: 12px;
  font-size: 14px;
}

.download-btns {
  margin-top: 12px;
}

.select-btns {
  display: flex;
  gap: 8px;
}

.size-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.size-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px;
  background: var(--steam-bg-tertiary);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.size-item:hover {
  background: var(--steam-bg-secondary);
}

.size-item input[type="checkbox"] {
  width: 18px;
  height: 18px;
}

.size-info {
  flex: 1;
}

.size-name {
  display: block;
  font-size: 13px;
  color: var(--steam-text-primary);
}

.size-dims {
  font-size: 11px;
  color: var(--steam-text-muted);
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
  text-shadow: 0 0 2px rgba(255, 255, 255, 0.5);
}

.stats-display {
  padding: 10px;
  background: var(--steam-bg-tertiary);
  border-radius: 4px;
  font-size: 13px;
  color: var(--steam-text-secondary);
  text-align: center;
  margin-bottom: 12px;
}

.result-area {
  max-height: 250px;
  overflow-y: auto;
  background: var(--steam-bg-tertiary);
  border-radius: 4px;
  padding: 12px;
  font-family: Consolas, Monaco, monospace;
  font-size: 12px;
  line-height: 1.6;
  border: 1px solid var(--steam-border);
}

.result-placeholder {
  color: var(--steam-text-muted);
  text-align: center;
  padding: 24px;
}

.result-item {
  padding: 2px 0;
}

.result-time {
  color: var(--steam-text-muted);
  margin-right: 8px;
}

.result-item.success {
  color: var(--steam-accent-green);
}

.result-item.error {
  color: var(--steam-accent-red);
}

.info-section {
  margin-top: 16px;
}

.info-content {
  font-size: 13px;
}

.info-content ol {
  margin: 8px 0;
  padding-left: 20px;
  color: var(--steam-text-secondary);
}

.info-content li {
  margin: 4px 0;
}

.info-content code {
  display: inline;
  background: var(--steam-bg-tertiary);
  padding: 2px 6px;
  border-radius: 3px;
  font-family: Consolas, Monaco, monospace;
  font-size: 12px;
  color: var(--steam-accent-blue);
}
</style>
