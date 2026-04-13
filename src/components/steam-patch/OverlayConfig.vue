<template>
  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <div class="header-icon overlay">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="2" y="3" width="20" height="14" rx="2" ry="2"/>
            <line x1="8" y1="21" x2="16" y2="21"/>
            <line x1="12" y1="17" x2="12" y2="21"/>
          </svg>
        </div>
        <h3>游戏内 Overlay 配置</h3>
        <button class="close-btn" @click="$emit('close')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <div class="modal-body">
        <!-- 实验版警告 -->
        <div class="experimental-warning">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
            <line x1="12" y1="9" x2="12" y2="13"/>
            <line x1="12" y1="17" x2="12.01" y2="17"/>
          </svg>
          <span>Overlay 是实验性功能，使用 Shift+Tab 打开</span>
        </div>

        <!-- 启用 Overlay -->
        <div class="config-item">
          <label class="toggle-label">
            <input type="checkbox" v-model="config.enabled" class="toggle-input" />
            <span class="toggle-slider"></span>
            <span class="toggle-text">启用游戏内 Overlay</span>
          </label>
        </div>

        <!-- 快捷键配置 -->
        <div class="config-group">
          <label class="config-label">打开 Overlay 快捷键</label>
          <div class="hotkey-display">
            <span class="hotkey-key">Shift</span>
            <span class="hotkey-plus">+</span>
            <span class="hotkey-key">Tab</span>
          </div>
          <p class="config-desc">快捷键固定为 Shift+Tab，不可修改</p>
        </div>

        <!-- 通知声音 -->
        <div class="config-group">
          <label class="config-label">通知声音</label>
          <div class="sound-options">
            <div class="sound-item">
              <label class="toggle-label small">
                <input type="checkbox" v-model="config.achievementSound" class="toggle-input" />
                <span class="toggle-slider small"></span>
                <span class="toggle-text">成就解锁通知音</span>
              </label>
              <button
                v-if="config.achievementSound"
                class="file-btn"
                @click="selectAchievementSound"
              >
                {{ config.achievementSoundPath ? '更改文件' : '选择文件' }}
              </button>
            </div>
            <p v-if="config.achievementSoundPath" class="file-path">
              {{ config.achievementSoundPath }}
            </p>

            <div class="sound-item">
              <label class="toggle-label small">
                <input type="checkbox" v-model="config.friendSound" class="toggle-input" />
                <span class="toggle-slider small"></span>
                <span class="toggle-text">好友邀请通知音</span>
              </label>
              <button
                v-if="config.friendSound"
                class="file-btn"
                @click="selectFriendSound"
              >
                {{ config.friendSoundPath ? '更改文件' : '选择文件' }}
              </button>
            </div>
            <p v-if="config.friendSoundPath" class="file-path">
              {{ config.friendSoundPath }}
            </p>
          </div>
          <p class="config-desc">支持 .wav 格式的音频文件</p>
        </div>

        <!-- 字体配置 -->
        <div class="config-group">
          <label class="config-label">自定义字体</label>
          <div class="font-config">
            <button class="file-btn" @click="selectFontFile">
              {{ config.fontPath ? '更改字体文件' : '选择字体文件' }}
            </button>
            <p v-if="config.fontPath" class="file-path">{{ config.fontPath }}</p>
          </div>
          <p class="config-desc">支持 .ttf 格式的字体文件（留空使用默认字体）</p>
        </div>

        <!-- 界面设置 -->
        <div class="config-group">
          <label class="config-label">界面设置</label>
          <div class="ui-options">
            <label class="checkbox-label">
              <input type="checkbox" v-model="config.showFPS" />
              <span>显示 FPS 计数器</span>
            </label>
            <label class="checkbox-label">
              <input type="checkbox" v-model="config.showClock" />
              <span>显示时钟</span>
            </label>
            <label class="checkbox-label">
              <input type="checkbox" v-model="config.showNotifications" />
              <span>显示通知弹窗</span>
            </label>
          </div>
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
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// ============================================
// Props 和 Emits
// ============================================

const props = defineProps<{
  gamePath: string
  gameId: string
  isExperimental: boolean
}>()

const emit = defineEmits<{
  close: []
  saved: []
}>()

// ============================================
// 类型定义
// ============================================

interface OverlayConfig {
  enabled: boolean
  achievementSound: boolean
  achievementSoundPath: string
  friendSound: boolean
  friendSoundPath: string
  fontPath: string
  showFPS: boolean
  showClock: boolean
  showNotifications: boolean
}

// ============================================
// 响应式状态
// ============================================

const config = ref<OverlayConfig>({
  enabled: false,
  achievementSound: true,
  achievementSoundPath: '',
  friendSound: true,
  friendSoundPath: '',
  fontPath: '',
  showFPS: false,
  showClock: true,
  showNotifications: true
})

// ============================================
// 方法
// ============================================

/**
 * 选择成就通知音文件
 */
const selectAchievementSound = async () => {
  try {
    const result = await invoke<string | null>('select_wav_file', {
      title: '选择成就解锁通知音'
    })
    if (result) {
      config.value.achievementSoundPath = result
    }
  } catch (error) {
    console.error('选择文件失败:', error)
  }
}

/**
 * 选择好友邀请通知音文件
 */
const selectFriendSound = async () => {
  try {
    const result = await invoke<string | null>('select_wav_file', {
      title: '选择好友邀请通知音'
    })
    if (result) {
      config.value.friendSoundPath = result
    }
  } catch (error) {
    console.error('选择文件失败:', error)
  }
}

/**
 * 选择字体文件
 */
const selectFontFile = async () => {
  try {
    const result = await invoke<string | null>('select_ttf_file', {
      title: '选择字体文件'
    })
    if (result) {
      config.value.fontPath = result
    }
  } catch (error) {
    console.error('选择文件失败:', error)
  }
}

/**
 * 保存配置
 */
const saveConfig = async () => {
  try {
    const result = await invoke<{
      success: boolean
      message: string
    }>('save_overlay_config', {
      gamePath: props.gamePath,
      config: config.value
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
      config?: OverlayConfig
    }>('load_overlay_config', {
      gamePath: props.gamePath
    })

    if (result.exists && result.config) {
      config.value = { ...config.value, ...result.config }
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
  max-width: 500px;
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

.header-icon.overlay {
  background-color: rgba(139, 92, 246, 0.1);
  color: #8b5cf6;
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

/* 实验版警告 */
.experimental-warning {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 16px;
  background-color: rgba(245, 158, 11, 0.1);
  border-radius: 8px;
  margin-bottom: 20px;
}

.experimental-warning svg {
  width: 20px;
  height: 20px;
  color: #f59e0b;
  flex-shrink: 0;
}

.experimental-warning span {
  font-size: 13px;
  color: #f59e0b;
}

/* 配置项样式 */
.config-item {
  margin-bottom: 20px;
}

.config-group {
  margin-bottom: 24px;
}

.config-label {
  display: block;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 12px;
}

.config-desc {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 8px 0 0 0;
}

/* 切换开关 */
.toggle-label {
  display: flex;
  align-items: center;
  gap: 12px;
  cursor: pointer;
}

.toggle-label.small {
  gap: 8px;
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
  flex-shrink: 0;
}

.toggle-slider.small {
  width: 40px;
  height: 22px;
  border-radius: 11px;
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

.toggle-slider.small::after {
  width: 18px;
  height: 18px;
}

.toggle-input:checked + .toggle-slider {
  background-color: var(--accent-color);
}

.toggle-input:checked + .toggle-slider::after {
  transform: translateX(22px);
}

.toggle-input:checked + .toggle-slider.small::after {
  transform: translateX(18px);
}

.toggle-text {
  font-size: 14px;
  color: var(--text-primary);
}

/* 快捷键显示 */
.hotkey-display {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background-color: var(--bg-primary);
  border-radius: 8px;
  justify-content: center;
}

.hotkey-key {
  padding: 6px 12px;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.hotkey-plus {
  font-size: 14px;
  color: var(--text-secondary);
}

/* 声音选项 */
.sound-options {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.sound-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.file-btn {
  padding: 6px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-size: 12px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.file-btn:hover {
  border-color: var(--accent-color);
  color: var(--accent-color);
}

.file-path {
  font-size: 11px;
  color: var(--text-secondary);
  font-family: 'Courier New', monospace;
  margin: 4px 0 0 48px;
  word-break: break-all;
}

.font-config {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

/* 界面选项 */
.ui-options {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  font-size: 14px;
  color: var(--text-primary);
}

.checkbox-label input[type="checkbox"] {
  width: 18px;
  height: 18px;
  accent-color: var(--accent-color);
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
