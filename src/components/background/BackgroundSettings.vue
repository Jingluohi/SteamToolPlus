<template>
  <div class="background-settings">
    <!-- 文件管理区域 -->
    <div class="file-management">
      <div class="upload-section">
        <h3 class="section-subtitle">文件库</h3>
        <div class="upload-buttons">
          <Button variant="secondary" size="small" @click="uploadImages">
            添加图片
          </Button>
        </div>
      </div>

      <!-- 文件列表 -->
      <div class="files-container">
        <div v-if="config.files.length > 0" class="file-group">
          <h4 class="file-group-title">
            图片 ({{ config.files.length }})
          </h4>
          <div class="file-grid">
            <div
              v-for="file in config.files"
              :key="file.id"
              class="file-item"
              :class="{ disabled: !file.enabled, selected: isFileSelected(file.id) }"
              @click="toggleFileSelection(file.id)"
            >
              <img :src="getFileUrl(file)" :alt="file.filename" class="file-thumb" />
              <div class="file-name">{{ file.filename }}</div>
              <div class="file-overlay">
                <span v-if="isFileSelected(file.id)" class="selected-badge">✓</span>
              </div>
              <div class="file-actions">
                <button
                  class="action-btn toggle"
                  :class="{ active: file.enabled }"
                  @click.stop="toggleFile(file.id)"
                  :title="file.enabled ? '禁用' : '启用'"
                >
                  {{ file.enabled ? '✓' : '○' }}
                </button>
                <button class="action-btn delete" @click.stop="deleteFile(file.id)" title="删除">
                  ×
                </button>
              </div>
            </div>
          </div>
        </div>

        <div v-if="config.files.length === 0" class="empty-tip">
          <p>暂无图片，请点击上方按钮添加图片</p>
        </div>
      </div>
    </div>

    <!-- 主题模式选择提示 -->
    <div v-if="currentPageConfig" class="theme-mode-notice">
      <div class="notice-content">
        <span class="notice-text">当前正在为 <strong>{{ currentThemeMode === 'light' ? '浅色' : '深色' }}模式</strong> 设置背景图片</span>
      </div>
    </div>

    <!-- 页面配置区域 -->
    <div class="page-configs">
      <h3 class="section-subtitle">页面背景配置</h3>
      <p class="section-desc">为不同页面设置专属背景，点击页面卡片进行配置</p>

      <div class="page-grid">
        <div
          v-for="pageConfig in config.pageConfigs"
          :key="pageConfig.pageType"
          class="page-card"
          :class="{ active: selectedPage === pageConfig.pageType, 'has-bg': pageConfig.lightFileIds.length > 0 || pageConfig.darkFileIds.length > 0 }"
          @click="selectPage(pageConfig.pageType)"
        >
          <div class="page-header">
            <span class="page-icon">{{ PAGE_CONFIG_MAP[pageConfig.pageType].icon }}</span>
            <span class="page-name">{{ pageConfig.pageName }}</span>
            <span v-if="pageConfig.lightFileIds.length > 0 || pageConfig.darkFileIds.length > 0" class="file-count">
              {{ pageConfig.lightFileIds.length + pageConfig.darkFileIds.length }}
            </span>
          </div>
          <div class="page-preview">
            <div v-if="getThemeFileIds(pageConfig).length === 0" class="no-bg">
              <span class="no-bg-text">未设置背景</span>
              <span v-if="pageConfig.lightFileIds.length > 0 || pageConfig.darkFileIds.length > 0" class="has-other-theme">
                (其他模式已设置)
              </span>
            </div>
            <div v-else class="bg-preview">
              <img
                :src="getFileUrl(getFirstThemeFile(pageConfig)!)"
                class="preview-img"
              />
            </div>
          </div>
          <div class="page-status">
            <Toggle v-model="pageConfig.enabled" @change="saveConfig" />
            <span class="status-text">{{ pageConfig.enabled ? '已启用' : '已禁用' }}</span>
          </div>
        </div>
      </div>

      <!-- 选中页面的详细配置 -->
      <div v-if="currentPageConfig" class="page-detail-config">
        <div class="detail-header">
          <span class="detail-icon">{{ PAGE_CONFIG_MAP[currentPageConfig.pageType].icon }}</span>
          <span class="detail-title">{{ currentPageConfig.pageName }} - 详细设置</span>
        </div>

        <!-- 主题模式切换 -->
        <div class="theme-mode-tabs">
          <button
            class="theme-tab"
            :class="{ active: currentThemeMode === 'light' }"
            @click="setThemeMode('light')"
          >
            <span class="tab-text">浅色模式</span>
            <span class="tab-count" v-if="currentPageConfig.lightFileIds.length > 0">
              {{ currentPageConfig.lightFileIds.length }}
            </span>
          </button>
          <button
            class="theme-tab"
            :class="{ active: currentThemeMode === 'dark' }"
            @click="setThemeMode('dark')"
          >
            <span class="tab-text">深色模式</span>
            <span class="tab-count" v-if="currentPageConfig.darkFileIds.length > 0">
              {{ currentPageConfig.darkFileIds.length }}
            </span>
          </button>
        </div>

        <div class="detail-content">
          <!-- 已选文件 -->
          <div class="detail-section">
            <label class="detail-label">
              {{ currentThemeMode === 'light' ? '浅色' : '深色' }}模式背景文件 ({{ currentThemeFileIds.length }})
            </label>
            <div v-if="currentThemeFileIds.length === 0" class="no-selection">
              点击上方文件库中的图片进行选择，为{{ currentThemeMode === 'light' ? '浅色' : '深色' }}模式设置专属背景
            </div>
            <div v-else class="selected-files">
              <div
                v-for="fileId in currentThemeFileIds"
                :key="fileId"
                class="selected-file-item"
              >
                <img
                  :src="getFileUrl(getFileById(fileId)!)"
                  class="selected-thumb"
                />
                <span class="selected-name">{{ getFileById(fileId)?.filename }}</span>
                <button class="remove-btn" @click="removeFileFromPage(fileId)">×</button>
              </div>
            </div>
          </div>

          <!-- 显示模式 -->
          <div class="detail-section">
            <label class="detail-label">显示模式</label>
            <div class="mode-options">
              <button
                v-for="mode in modeOptions"
                :key="mode.value"
                class="mode-btn"
                :class="{ active: currentPageConfig.mode === mode.value }"
                @click="setPageMode(mode.value)"
              >
                {{ mode.label }}
              </button>
            </div>
          </div>

          <!-- 轮播设置 -->
          <div v-if="currentPageConfig.mode !== 'single'" class="detail-section">
            <label class="detail-label">切换间隔</label>
            <select v-model="currentPageConfig.interval" @change="saveConfig" class="detail-select">
              <option v-for="opt in INTERVAL_OPTIONS" :value="opt.value">{{ opt.label }}</option>
            </select>
          </div>

          <!-- 动画效果 -->
          <div class="detail-section">
            <label class="detail-label">切换动画</label>
            <select v-model="currentPageConfig.transitionEffect" @change="saveConfig" class="detail-select">
              <option v-for="opt in TRANSITION_EFFECT_OPTIONS" :value="opt.value">{{ opt.label }}</option>
            </select>
          </div>

          <!-- 视觉效果 -->
          <div class="detail-section effects-section">
            <label class="detail-label">视觉效果</label>
            <div class="effect-controls">
              <div class="effect-item">
                <span class="effect-name">背景模糊</span>
                <select v-model="currentPageConfig.blurStrength" @change="saveConfig" class="effect-select">
                  <option v-for="opt in BLUR_STRENGTH_OPTIONS" :value="opt.value">{{ opt.label }}</option>
                </select>
              </div>
              <div class="effect-item">
                <span class="effect-name">暗化程度</span>
                <select v-model="currentPageConfig.darkness" @change="saveConfig" class="effect-select">
                  <option v-for="opt in DARKNESS_OPTIONS" :value="opt.value">{{ opt.label }}</option>
                </select>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 底部操作按钮 -->
    <div class="bottom-actions">
      <Button variant="secondary" size="small" @click="resetPersonalization">
        重置个性化
      </Button>
      <Button variant="primary" size="small" @click="saveConfiguration">
        保存配置
      </Button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import Button from '../common/Button.vue'
import Toggle from '../../views/GlobalSettings/components/Toggle.vue'
import type {
  BackgroundConfig,
  BackgroundFile,
  PageBackgroundConfig,
  PageType,
  ThemeMode
} from '../../types/background.types'
import {
  DEFAULT_BACKGROUND_CONFIG,
  PAGE_CONFIG_MAP,
  INTERVAL_OPTIONS,
  TRANSITION_EFFECT_OPTIONS,
  BLUR_STRENGTH_OPTIONS,
  DARKNESS_OPTIONS,
  IMAGE_EXTENSIONS
} from '../../types/background.types'
import {
  getBackgroundConfig,
  saveBackgroundConfig,
  addBackgroundFile,
  removeBackgroundFile,
  getBackgroundFileUrl
} from '../../api/background.api'

const emit = defineEmits<{
  refresh: []
}>()

const config = ref<BackgroundConfig>({ ...DEFAULT_BACKGROUND_CONFIG })
const isLoading = ref(false)
const selectedPage = ref<PageType | null>(null)
const currentThemeMode = ref<ThemeMode>('dark')
const fileUrlCache = ref<Map<string, string>>(new Map())

const modeOptions = [
  { label: '单张', value: 'single' },
  { label: '轮播', value: 'slideshow' },
  { label: '随机', value: 'random' }
]

const currentPageConfig = computed(() =>
  selectedPage.value
    ? config.value.pageConfigs.find(p => p.pageType === selectedPage.value) || null
    : null
)

const currentThemeFileIds = computed(() => {
  if (!currentPageConfig.value) return []
  return currentThemeMode.value === 'light'
    ? currentPageConfig.value.lightFileIds
    : currentPageConfig.value.darkFileIds
})

async function loadFileUrl(file: BackgroundFile): Promise<string> {
  if (fileUrlCache.value.has(file.id)) {
    return fileUrlCache.value.get(file.id)!
  }

  try {
    const url = await getBackgroundFileUrl(file.path)
    fileUrlCache.value.set(file.id, url)
    return url
  } catch (err) {
    return ''
  }
}

function getFileUrl(file: BackgroundFile): string {
  const cached = fileUrlCache.value.get(file.id)
  if (cached) {
    return cached
  }

  loadFileUrl(file).then(() => {
    fileUrlCache.value = new Map(fileUrlCache.value)
  })

  return ''
}

function getFileById(fileId: string): BackgroundFile | undefined {
  return config.value.files.find(f => f.id === fileId)
}

function getFirstFile(pageConfig: PageBackgroundConfig): BackgroundFile | undefined {
  if (pageConfig.fileIds.length === 0) return undefined
  return getFileById(pageConfig.fileIds[0])
}

function getThemeFileIds(pageConfig: PageBackgroundConfig): string[] {
  // 获取程序当前实际使用的主题（从DOM读取）
  const currentTheme = document.documentElement.getAttribute('data-theme')
  const isDarkMode = currentTheme !== 'light'
  return isDarkMode ? pageConfig.darkFileIds : pageConfig.lightFileIds
}

function getFirstThemeFile(pageConfig: PageBackgroundConfig): BackgroundFile | undefined {
  const fileIds = getThemeFileIds(pageConfig)
  if (fileIds.length === 0) return undefined
  return getFileById(fileIds[0])
}

function isFileSelected(fileId: string): boolean {
  if (!currentPageConfig.value) return false
  return currentThemeMode.value === 'light'
    ? currentPageConfig.value.lightFileIds.includes(fileId)
    : currentPageConfig.value.darkFileIds.includes(fileId)
}

function toggleFileSelection(fileId: string) {
  if (!currentPageConfig.value) {
    if (config.value.pageConfigs.length > 0) {
      selectedPage.value = config.value.pageConfigs[0].pageType
    } else {
      return
    }
  }

  if (!currentPageConfig.value) return

  const fileIds = currentThemeMode.value === 'light'
    ? currentPageConfig.value.lightFileIds
    : currentPageConfig.value.darkFileIds

  const index = fileIds.indexOf(fileId)
  if (index === -1) {
    fileIds.push(fileId)
  } else {
    fileIds.splice(index, 1)
  }
  saveConfig()
}

function removeFileFromPage(fileId: string) {
  if (!currentPageConfig.value) return
  const fileIds = currentThemeMode.value === 'light'
    ? currentPageConfig.value.lightFileIds
    : currentPageConfig.value.darkFileIds
  const index = fileIds.indexOf(fileId)
  if (index !== -1) {
    fileIds.splice(index, 1)
    saveConfig()
  }
}

function setThemeMode(mode: ThemeMode) {
  currentThemeMode.value = mode
}

function selectPage(pageType: PageType) {
  selectedPage.value = pageType
}

function setPageMode(mode: string) {
  if (currentPageConfig.value) {
    currentPageConfig.value.mode = mode as any
    saveConfig()
  }
}

async function loadConfig() {
  try {
    isLoading.value = true
    const savedConfig = await getBackgroundConfig()

    if (savedConfig) {
      const files = savedConfig.files || []

      config.value = {
        ...DEFAULT_BACKGROUND_CONFIG,
        ...savedConfig,
        files: files,
        pageConfigs: DEFAULT_BACKGROUND_CONFIG.pageConfigs.map(defaultPage => {
          const savedPage = savedConfig.pageConfigs?.find(p => p.pageType === defaultPage.pageType)
          return savedPage ? { ...defaultPage, ...savedPage } : defaultPage
        })
      }
    }
  } catch (err) {
    // 加载失败时静默处理
  } finally {
    isLoading.value = false
  }
}

async function saveConfig() {
  try {
    await saveBackgroundConfig(config.value)
    emit('refresh')
  } catch (err) {
    // 保存失败时静默处理
  }
}

async function uploadImages() {
  const filters = [{
    name: '图片文件',
    extensions: IMAGE_EXTENSIONS
  }]

  try {
    const selected = await open({
      multiple: true,
      filters
    })

    if (selected) {
      const files = Array.isArray(selected) ? selected : [selected]
      let successCount = 0
      let errorMsg = ''
      const addedFileIds: string[] = []

      for (const filePath of files) {
        try {
          const result = await addBackgroundFile(filePath)
          addedFileIds.push(result.id)
          successCount++
        } catch (err: any) {
          errorMsg = err.message || String(err)
        }
      }

      await loadConfig()

      if (addedFileIds.length > 0 && currentPageConfig.value) {
        for (const fileId of addedFileIds) {
          if (!currentPageConfig.value.fileIds.includes(fileId)) {
            currentPageConfig.value.fileIds.push(fileId)
          }
        }
        await saveConfig()
      }

      if (successCount > 0) {
        emit('refresh')
        alert(`成功添加 ${successCount} 个图片到当前页面`)
      } else if (errorMsg) {
        alert(`添加失败: ${errorMsg}`)
      }
    }
  } catch (err) {
    // 选择文件失败时静默处理
  }
}

async function toggleFile(fileId: string) {
  const file = config.value.files.find(f => f.id === fileId)
  if (file) {
    file.enabled = !file.enabled
    await saveConfig()
  }
}

async function deleteFile(fileId: string) {
  if (!confirm('确定要删除这个文件吗？')) return

  try {
    await removeBackgroundFile(fileId)
    config.value.pageConfigs.forEach(page => {
      const index = page.fileIds.indexOf(fileId)
      if (index !== -1) {
        page.fileIds.splice(index, 1)
      }
    })
    await loadConfig()
  } catch (err) {
    alert('删除文件失败')
  }
}

onMounted(async () => {
  await loadConfig()
  if (!selectedPage.value && config.value.pageConfigs.length > 0) {
    selectedPage.value = config.value.pageConfigs[0].pageType
  }
})

/**
 * 重置个性化设置
 * 删除 background.json 配置文件并重启程序
 */
async function resetPersonalization() {
  if (!confirm('确定要重置所有个性化设置吗？这将删除所有背景配置并重启程序。')) {
    return
  }

  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('reset_background_config')
    alert('个性化设置已重置，程序将重启')
    // 重启程序
    await invoke('restart_app')
  } catch (err) {
    alert('重置失败: ' + err)
  }
}

/**
 * 保存配置
 * 由于配置已实时保存，此按钮仅作为用户确认操作的反馈
 */
function saveConfiguration() {
  // 配置已实时保存，无需额外操作
  alert('配置已保存')
}
</script>

<style scoped>
.background-settings {
  display: flex;
  flex-direction: column;
  gap: 20px;
  width: 100%;
}

.file-management {
  background: var(--steam-bg-primary);
  border-radius: 12px;
  padding: 16px;
  border: 1px solid var(--steam-border);
}

.upload-section {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--steam-border);
}

.section-subtitle {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0;
}

.upload-buttons {
  display: flex;
  gap: 10px;
}

.btn-icon {
  margin-right: 4px;
}

.files-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.file-group {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.file-group-title {
  font-size: 12px;
  font-weight: 500;
  color: var(--steam-text-secondary);
  margin: 0;
  display: flex;
  align-items: center;
  gap: 6px;
}

.group-icon {
  font-size: 14px;
}

.file-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
  gap: 10px;
}

.file-item {
  aspect-ratio: 16/10;
  border-radius: 8px;
  overflow: hidden;
  position: relative;
  cursor: pointer;
  border: 2px solid transparent;
  transition: all 0.2s ease;
  background: var(--steam-bg-tertiary);
}

.file-item:hover {
  transform: scale(1.02);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

.file-item.disabled {
  opacity: 0.4;
}

.file-item.selected {
  border-color: var(--steam-accent);
  box-shadow: 0 0 0 3px rgba(27, 159, 255, 0.3);
}

.file-thumb {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.file-name {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 4px 6px;
  font-size: 12px;
  color: var(--steam-text-primary);
  background: linear-gradient(transparent, rgba(0, 0, 0, 0.7));
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-overlay {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  pointer-events: none;
}

.selected-badge {
  width: 28px;
  height: 28px;
  background: var(--steam-accent);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 14px;
  font-weight: bold;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

.file-actions {
  position: absolute;
  top: 4px;
  right: 4px;
  display: flex;
  gap: 4px;
  opacity: 0;
  transition: opacity 0.2s;
}

.file-item:hover .file-actions {
  opacity: 1;
}

.action-btn {
  width: 22px;
  height: 22px;
  border-radius: 50%;
  border: none;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.action-btn.toggle {
  background: rgba(255, 255, 255, 0.9);
  color: var(--steam-text-secondary);
}

.action-btn.toggle.active {
  background: var(--steam-accent-green);
  color: white;
}

.action-btn.delete {
  background: rgba(220, 53, 69, 0.9);
  color: white;
}

.empty-tip {
  text-align: center;
  padding: 40px;
  color: var(--steam-text-muted);
  font-size: 13px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.empty-icon {
  font-size: 32px;
}

.page-configs {
  background: var(--steam-bg-primary);
  border-radius: 12px;
  padding: 16px;
  border: 1px solid var(--steam-border);
}

.section-desc {
  font-size: 12px;
  color: var(--steam-text-muted);
  margin: -8px 0 12px 0;
}

.page-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: 12px;
  margin-bottom: 20px;
}

.page-card {
  background: var(--steam-bg-tertiary);
  border-radius: 10px;
  padding: 12px;
  cursor: pointer;
  border: 2px solid transparent;
  transition: all 0.2s ease;
  display: flex;
  flex-direction: column;
  gap: 8px;
  min-width: 120px;
}

.page-card:hover {
  background: var(--steam-bg-hover);
  transform: translateY(-2px);
}

.page-card.active {
  border-color: var(--steam-accent);
  background: rgba(27, 159, 255, 0.1);
}

.page-card.has-bg {
  border-color: rgba(27, 159, 255, 0.3);
}

.page-header {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-text-primary);
  white-space: nowrap;
  overflow: hidden;
}

.page-icon {
  font-size: 16px;
}

.page-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-count {
  font-size: 12px;
  background: var(--steam-accent);
  color: white;
  padding: 2px 6px;
  border-radius: 10px;
}

.page-preview {
  aspect-ratio: 16/10;
  background: var(--steam-bg-secondary);
  border-radius: 6px;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
}

.no-bg {
  font-size: 12px;
  color: var(--steam-text-muted);
}

.bg-preview {
  width: 100%;
  height: 100%;
}

.preview-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.page-status {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--steam-text-secondary);
  white-space: nowrap;
}

.page-detail-config {
  background: var(--steam-bg-secondary);
  border-radius: 10px;
  padding: 16px;
  border: 1px solid var(--steam-border);
}

.detail-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--steam-border);
}

.detail-icon {
  font-size: 18px;
}

.detail-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.detail-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.detail-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.detail-label {
  font-size: 12px;
  font-weight: 500;
  color: var(--steam-text-secondary);
}

.no-selection {
  padding: 20px;
  text-align: center;
  color: var(--steam-text-muted);
  font-size: 12px;
  background: var(--steam-bg-tertiary);
  border-radius: 8px;
  border: 1px dashed var(--steam-border);
}

.selected-files {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.selected-file-item {
  display: flex;
  align-items: center;
  gap: 8px;
  background: var(--steam-bg-tertiary);
  padding: 6px 10px;
  border-radius: 6px;
  border: 1px solid var(--steam-border);
}

.selected-thumb {
  width: 40px;
  height: 24px;
  border-radius: 4px;
  object-fit: cover;
}

.selected-name {
  font-size: 12px;
  color: var(--steam-text-primary);
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.remove-btn {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  border: none;
  background: rgba(220, 53, 69, 0.8);
  color: white;
  font-size: 12px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}

.mode-options {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.mode-btn {
  padding: 6px 14px;
  background: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border);
  border-radius: 6px;
  color: var(--steam-text-secondary);
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.mode-btn:hover {
  border-color: var(--steam-accent);
}

.mode-btn.active {
  background: var(--steam-accent);
  border-color: var(--steam-accent);
  color: white;
}

.detail-select,
.effect-select {
  padding: 6px 10px;
  background: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border);
  border-radius: 6px;
  color: var(--steam-text-primary);
  font-size: 12px;
  outline: none;
  width: 140px;
}

.detail-select:focus,
.effect-select:focus {
  border-color: var(--steam-accent);
}

.effect-controls {
  display: flex;
  gap: 20px;
  flex-wrap: wrap;
}

.effect-item {
  display: flex;
  align-items: center;
  gap: 10px;
}

.effect-name {
  font-size: 12px;
  color: var(--steam-text-secondary);
}

/* 主题模式提示 */
.theme-mode-notice {
  background: linear-gradient(135deg, rgba(27, 159, 255, 0.1), rgba(138, 43, 226, 0.1));
  border: 1px solid var(--steam-accent);
  border-radius: 10px;
  padding: 12px 16px;
  margin: 0;
}

.notice-content {
  display: flex;
  align-items: center;
  gap: 10px;
}

.notice-icon {
  font-size: 18px;
}

.notice-text {
  font-size: 13px;
  color: var(--steam-text-primary);
}

.notice-text strong {
  color: var(--steam-accent);
}

/* 主题模式切换标签 */
.theme-mode-tabs {
  display: flex;
  gap: 8px;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--steam-border);
}

.theme-tab {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 10px 16px;
  background: var(--steam-bg-tertiary);
  border: 2px solid transparent;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.theme-tab:hover {
  background: var(--steam-bg-hover);
}

.theme-tab.active {
  background: rgba(27, 159, 255, 0.15);
  border-color: var(--steam-accent);
}

.theme-tab .tab-icon {
  font-size: 16px;
}

.theme-tab .tab-text {
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

.theme-tab .tab-count {
  font-size: 11px;
  background: var(--steam-accent);
  color: white;
  padding: 2px 6px;
  border-radius: 10px;
  min-width: 18px;
  text-align: center;
}

/* 页面预览其他模式提示 */
.no-bg {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.no-bg-text {
  font-size: 12px;
  color: var(--steam-text-muted);
}

.has-other-theme {
  font-size: 10px;
  color: var(--steam-accent);
}

/* 底部操作按钮 */
.bottom-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 8px;
  padding-top: 16px;
  border-top: 1px solid var(--steam-border);
}
</style>
