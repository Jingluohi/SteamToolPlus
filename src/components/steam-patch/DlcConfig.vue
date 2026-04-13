<template>
  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <div class="header-icon dlc">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
            <line x1="12" y1="8" x2="12" y2="16"/>
            <line x1="8" y1="12" x2="16" y2="12"/>
          </svg>
        </div>
        <h3>DLC 配置</h3>
        <button class="close-btn" @click="$emit('close')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <div class="modal-body">
        <!-- 启用 DLC 系统 -->
        <div class="config-item">
          <label class="toggle-label">
            <input type="checkbox" v-model="config.enabled" class="toggle-input" />
            <span class="toggle-slider"></span>
            <span class="toggle-text">启用 DLC 系统</span>
          </label>
        </div>

        <!-- 说明 -->
        <div class="info-box">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <line x1="12" y1="16" x2="12" y2="12"/>
            <line x1="12" y1="8" x2="12.01" y2="8"/>
          </svg>
          <div class="info-content">
            <p>在此处配置已安装的 DLC，游戏将认为这些内容可用。</p>
            <p>DLC ID 可在 SteamDB 上查询游戏的 Depots 页面获得。</p>
          </div>
        </div>

        <!-- SteamDB 查询链接 -->
        <div v-if="config.enabled" class="steamdb-section">
          <div class="steamdb-header">
            <svg class="steamdb-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/>
              <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>
            </svg>
            <span class="steamdb-title">查询 DLC ID</span>
          </div>
          <p class="steamdb-desc">
            访问 SteamDB 查询游戏的 Depot ID 和 DLC 信息：
          </p>
          <a 
            :href="steamdbUrl" 
            target="_blank" 
            class="steamdb-link"
            @click.prevent="openSteamdb"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>
              <polyline points="15 3 21 3 21 9"/>
              <line x1="10" y1="14" x2="21" y2="3"/>
            </svg>
            在 SteamDB 上查询 (AppID: {{ gameId || '游戏AppID' }})
          </a>
          <p class="steamdb-hint">
            在 SteamDB 页面中找到标记为 "DLC" 的条目，记录其 ID 和名称后手动添加到下表。
          </p>
        </div>

        <!-- DLC 列表 -->
        <div v-if="config.enabled" class="dlc-section">
          <div class="section-header">
            <label class="config-label">已安装 DLC</label>
            <button class="action-btn" @click="addDlc">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="12" y1="5" x2="12" y2="19"/>
                <line x1="5" y1="12" x2="19" y2="12"/>
              </svg>
              添加 DLC
            </button>
          </div>

          <div class="dlc-list">
            <div
              v-for="(dlc, index) in config.dlcs"
              :key="index"
              class="dlc-card"
            >
              <div class="dlc-fields-row">
                <!-- DLC App ID 输入框 -->
                <div class="field-group">
                  <label>DLC App ID:</label>
                  <input
                    type="number"
                    v-model.number="dlc.appId"
                    class="field-input"
                    placeholder="如：1731080"
                  />
                </div>
                <!-- DLC 名称输入框 -->
                <div class="field-group">
                  <label>DLC 名称:</label>
                  <input
                    type="text"
                    v-model="dlc.name"
                    class="field-input"
                    placeholder="如：Winters' Expansion"
                  />
                </div>
                <!-- 删除按钮 -->
                <button class="remove-btn" @click="removeDlc(index)">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="18" y1="6" x2="6" y2="18"/>
                    <line x1="6" y1="6" x2="18" y2="18"/>
                  </svg>
                </button>
              </div>
            </div>
          </div>

          <p v-if="config.dlcs.length === 0" class="empty-hint">
            暂无 DLC 配置，点击上方按钮添加或从 SteamDB 查询后手动输入
          </p>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn-secondary" @click="$emit('close')">取消</button>
        <button class="btn-primary" :disabled="!config.enabled" @click="saveConfig">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="20 6 9 17 4 12"/>
          </svg>
          保存配置
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// ============================================
// Props 和 Emits
// ============================================

const props = defineProps<{
  gamePath: string
  gameId: string
}>()

const emit = defineEmits<{
  close: []
  saved: []
}>()

// ============================================
// 类型定义
// ============================================

interface Dlc {
  name: string
  appId: number
}

interface DlcConfig {
  enabled: boolean
  dlcs: Dlc[]
}

// ============================================
// 响应式状态
// ============================================

const config = ref<DlcConfig>({
  enabled: true,
  dlcs: []
})

// ============================================
// 计算属性
// ============================================

const steamdbUrl = computed(() => {
  if (props.gameId) {
    return `https://steamdb.info/app/${props.gameId}/depots/`
  }
  return 'https://steamdb.info/'
})

// ============================================
// 方法
// ============================================

/**
 * 打开 SteamDB 链接
 */
const openSteamdb = async () => {
  try {
    await invoke('open_external_link', { url: steamdbUrl.value })
  } catch (error) {
    // 如果后端命令失败，使用前端打开
    window.open(steamdbUrl.value, '_blank')
  }
}

/**
 * 添加 DLC
 */
const addDlc = () => {
  config.value.dlcs.push({
    name: '',
    appId: 0
  })
}

/**
 * 移除 DLC
 */
const removeDlc = (index: number) => {
  config.value.dlcs.splice(index, 1)
}

/**
 * 保存配置
 */
const saveConfig = async () => {
  try {
    // 过滤掉空的配置
    const validDlcs = config.value.dlcs.filter(dlc => dlc.appId > 0 && dlc.name.trim())

    const result = await invoke<{
      success: boolean
      message: string
    }>('save_dlc_config', {
      gamePath: props.gamePath,
      config: {
        enabled: config.value.enabled,
        unlockAll: false, // 手动配置时不使用 unlock_all
        dlcs: validDlcs,
        depots: [],
        installedApps: []
      }
    })

    if (result.success) {
      emit('saved')
      emit('close')
    } else {
      alert(`保存失败: ${result.message}`)
    }
  } catch (error) {
    alert(`保存失败: ${error}`)
  }
}

/**
 * 加载现有配置
 */
const loadConfig = async () => {
  try {
    const result = await invoke<{
      exists: boolean
      config?: DlcConfig
    }>('load_dlc_config', {
      gamePath: props.gamePath
    })

    if (result.exists && result.config) {
      // 只加载手动配置的DLC（unlockAll=false时的DLC）
      config.value.enabled = result.config.enabled
      if (result.config.dlcs && result.config.dlcs.length > 0) {
        config.value.dlcs = result.config.dlcs
      }
    }
  } catch (error) {
    console.error('加载配置失败:', error)
  }
}

// ============================================
// 生命周期
// ============================================

onMounted(() => {
  loadConfig()
})
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background-color: var(--bg-secondary);
  border-radius: 12px;
  border: 1px solid var(--border-color);
  width: 90%;
  max-width: 650px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 20px;
  border-bottom: 1px solid var(--border-color);
}

.header-icon {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.header-icon.dlc {
  background-color: rgba(168, 85, 247, 0.1);
  color: #a855f7;
}

.header-icon svg {
  width: 24px;
  height: 24px;
}

.modal-header h3 {
  flex: 1;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.close-btn {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 8px;
  background-color: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.close-btn:hover {
  background-color: var(--bg-surface);
  color: var(--text-primary);
}

.close-btn svg {
  width: 18px;
  height: 18px;
}

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid var(--border-color);
}

/* 配置项样式 */
.config-item {
  margin-bottom: 16px;
}

.config-group {
  margin-bottom: 20px;
}

.config-label {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

/* 切换开关 */
.toggle-label {
  display: flex;
  align-items: center;
  gap: 12px;
  cursor: pointer;
}

.toggle-input {
  display: none;
}

.toggle-slider {
  width: 48px;
  height: 26px;
  background-color: var(--border-color);
  border-radius: 13px;
  position: relative;
  transition: background-color 0.2s ease;
}

.toggle-slider::after {
  content: '';
  position: absolute;
  width: 22px;
  height: 22px;
  background-color: white;
  border-radius: 50%;
  top: 2px;
  left: 2px;
  transition: transform 0.2s ease;
}

.toggle-input:checked + .toggle-slider {
  background-color: var(--accent-color);
}

.toggle-input:checked + .toggle-slider::after {
  transform: translateX(22px);
}

.toggle-text {
  font-size: 14px;
  color: var(--text-primary);
}

/* 信息框 */
.info-box {
  display: flex;
  gap: 12px;
  padding: 12px 16px;
  background-color: rgba(59, 130, 246, 0.1);
  border-radius: 8px;
  margin-bottom: 20px;
}

.info-box svg {
  width: 20px;
  height: 20px;
  color: #3b82f6;
  flex-shrink: 0;
}

.info-content p {
  font-size: 13px;
  color: var(--text-primary);
  margin: 0 0 4px 0;
}

.info-content p:last-child {
  margin-bottom: 0;
}

/* SteamDB 区域 */
.steamdb-section {
  margin-bottom: 24px;
  padding: 16px;
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.05) 0%, rgba(168, 85, 247, 0.05) 100%);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.steamdb-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
}

.steamdb-icon {
  width: 20px;
  height: 20px;
  color: #3b82f6;
}

.steamdb-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.steamdb-desc {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 12px;
}

.steamdb-link {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  background-color: rgba(59, 130, 246, 0.1);
  border: 1px solid rgba(59, 130, 246, 0.3);
  border-radius: 6px;
  color: #3b82f6;
  font-size: 13px;
  font-weight: 500;
  text-decoration: none;
  cursor: pointer;
  transition: all 0.15s ease;
}

.steamdb-link:hover {
  background-color: rgba(59, 130, 246, 0.2);
  border-color: rgba(59, 130, 246, 0.5);
}

.steamdb-link svg {
  width: 14px;
  height: 14px;
}

.steamdb-hint {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 12px;
  line-height: 1.5;
}

/* 区域样式 */
.dlc-section {
  margin-bottom: 24px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.action-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.action-btn:hover {
  border-color: var(--accent-color);
  color: var(--accent-color);
}

.action-btn svg {
  width: 14px;
  height: 14px;
}

/* DLC 卡片 */
.dlc-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.dlc-card {
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 12px;
}

/* DLC 字段行 - 并排布局 */
.dlc-fields-row {
  display: flex;
  align-items: flex-end;
  gap: 12px;
}

.field-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
  flex: 1;
}

.field-group label {
  font-size: 12px;
  color: var(--text-secondary);
  font-weight: 500;
}

.field-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color 0.15s ease;
}

.field-input:focus {
  border-color: var(--accent-color);
}

/* 删除按钮 */
.remove-btn {
  width: 36px;
  height: 36px;
  border: none;
  border-radius: 6px;
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
  flex-shrink: 0;
}

.remove-btn:hover {
  background-color: rgba(239, 68, 68, 0.2);
}

.remove-btn svg {
  width: 16px;
  height: 16px;
}

.empty-hint {
  text-align: center;
  padding: 30px;
  color: var(--text-secondary);
  font-size: 14px;
}

/* 按钮样式 */
.btn-primary,
.btn-secondary {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.btn-primary {
  background-color: var(--accent-color);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background-color: var(--accent-hover);
}

.btn-primary:disabled {
  background-color: var(--text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

.btn-secondary {
  background-color: var(--bg-surface);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.btn-secondary:hover {
  background-color: var(--border-color);
}

.btn-primary svg {
  width: 16px;
  height: 16px;
}
</style>
