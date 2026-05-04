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
        <!-- 启用开关 -->
        <div class="config-section">
          <label class="toggle-label">
            <input v-model="config.enabled" type="checkbox" class="toggle-input" />
            <span class="toggle-slider"></span>
            <span class="toggle-text">启用游戏内 Overlay（Shift+Tab）</span>
          </label>
        </div>

        <template v-if="config.enabled">
          <!-- 快捷键设置 -->
          <div class="config-group">
            <label class="config-label">快捷键</label>
            <input v-model="config.hotkey" type="text" class="config-input" placeholder="Shift+Tab" />
            <p class="config-hint">按下组合键显示/隐藏 Overlay</p>
          </div>

          <!-- 通知设置 -->
          <div class="config-section">
            <h4 class="section-title">通知设置</h4>
            
            <div class="config-row">
              <label class="checkbox-label">
                <input v-model="config.notifications.achievement" type="checkbox" />
                <span>成就解锁通知</span>
              </label>
            </div>
            
            <div class="config-row">
              <label class="checkbox-label">
                <input v-model="config.notifications.friend" type="checkbox" />
                <span>好友上线通知</span>
              </label>
            </div>
            
            <div class="config-row">
              <label class="checkbox-label">
                <input v-model="config.notifications.message" type="checkbox" />
                <span>消息通知</span>
              </label>
            </div>

            <div class="config-row two-col">
              <div class="config-item">
                <label class="config-label">显示时长（秒）</label>
                <input 
                  v-model.number="config.notifications.duration" 
                  type="number" 
                  class="config-input" 
                  min="1" 
                  max="30" 
                />
              </div>
              
              <div class="config-item">
                <label class="config-label">通知位置</label>
                <select v-model="config.notifications.position" class="config-select">
                  <option value="top-left">左上</option>
                  <option value="top-right">右上</option>
                  <option value="bottom-left">左下</option>
                  <option value="bottom-right">右下</option>
                </select>
              </div>
            </div>
          </div>

          <!-- 外观设置 -->
          <div class="config-section">
            <h4 class="section-title">外观设置</h4>
            
            <div class="config-row two-col">
              <div class="config-item">
                <label class="config-label">主题</label>
                <select v-model="config.appearance.theme" class="config-select">
                  <option value="dark">深色</option>
                  <option value="light">浅色</option>
                  <option value="auto">跟随系统</option>
                </select>
              </div>
              
              <div class="config-item">
                <label class="config-label">缩放比例</label>
                <input 
                  v-model.number="config.appearance.scale" 
                  type="number" 
                  class="config-input" 
                  min="0.5" 
                  max="2.0" 
                  step="0.1"
                />
              </div>
            </div>

            <div class="config-row two-col">
              <div class="config-item">
                <label class="config-label">透明度</label>
                <div class="slider-group">
                  <input 
                    v-model.number="config.appearance.opacity" 
                    type="range" 
                    min="0.1" 
                    max="1.0" 
                    step="0.05"
                    class="config-slider"
                  />
                  <span class="slider-value">{{ Math.round(config.appearance.opacity * 100) }}%</span>
                </div>
              </div>
              
              <div class="config-item">
                <label class="checkbox-label">
                  <input v-model="config.appearance.blur" type="checkbox" />
                  <span>背景模糊效果</span>
                </label>
              </div>
            </div>
          </div>

          <!-- 性能设置 -->
          <div class="config-section">
            <h4 class="section-title">性能设置</h4>
            
            <div class="config-row">
              <label class="checkbox-label">
                <input v-model="config.performance.hardwareAcceleration" type="checkbox" />
                <span>硬件加速</span>
              </label>
            </div>
            
            <div class="config-row">
              <label class="checkbox-label">
                <input v-model="config.performance.lowPerformanceMode" type="checkbox" />
                <span>低性能模式（减少动画和特效）</span>
              </label>
            </div>

            <div class="config-row">
              <div class="config-item">
                <label class="config-label">帧率限制</label>
                <select v-model="config.performance.fpsLimit" class="config-select">
                  <option :value="30">30 FPS</option>
                  <option :value="60">60 FPS</option>
                  <option :value="120">120 FPS</option>
                  <option :value="144">144 FPS</option>
                  <option :value="0">无限制</option>
                </select>
              </div>
            </div>
          </div>

          <!-- 功能开关 -->
          <div class="config-section">
            <h4 class="section-title">功能开关</h4>
            
            <div class="feature-grid">
              <label class="checkbox-label">
                <input v-model="config.features.achievements" type="checkbox" />
                <span>成就页面</span>
              </label>
              
              <label class="checkbox-label">
                <input v-model="config.features.friends" type="checkbox" />
                <span>好友列表</span>
              </label>
              
              <label class="checkbox-label">
                <input v-model="config.features.chat" type="checkbox" />
                <span>聊天功能</span>
              </label>
              
              <label class="checkbox-label">
                <input v-model="config.features.browser" type="checkbox" />
                <span>内置浏览器</span>
              </label>
              
              <label class="checkbox-label">
                <input v-model="config.features.settings" type="checkbox" />
                <span>设置页面</span>
              </label>
            </div>
          </div>
        </template>
      </div>

      <div class="modal-footer">
        <button class="btn-secondary" @click="$emit('close')">取消</button>
        <button class="btn-primary" @click="saveConfig">
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
import type { OverlayConfig } from '../../../src/types/steam-config.types'

const props = defineProps<{
  gamePath: string
  gameId: string
  isExperimental?: boolean
}>()

const emit = defineEmits<{
  close: []
  saved: []
}>()

const config = ref<OverlayConfig>({
  enabled: true,
  hotkey: 'Shift+Tab',
  notifications: {
    achievement: true,
    friend: true,
    message: true,
    duration: 5,
    position: 'bottom-right'
  },
  appearance: {
    theme: 'dark',
    opacity: 0.95,
    scale: 1.0,
    blur: true
  },
  performance: {
    hardwareAcceleration: true,
    fpsLimit: 60,
    lowPerformanceMode: false
  },
  features: {
    achievements: true,
    friends: true,
    chat: true,
    browser: true,
    settings: true
  }
})

async function saveConfig() {
  try {
    const result = await invoke<{ success: boolean; message: string }>('save_overlay_config', {
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

async function loadConfig() {
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
    // 加载失败时使用默认值
  }
}

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
  background-color: var(--steam-bg-primary);
  border-radius: 12px;
  border: 1px solid var(--steam-border);
  width: 90%;
  max-width: 700px;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 20px;
  border-bottom: 1px solid var(--steam-border);
  flex-shrink: 0;
}

.header-icon {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
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
  color: var(--steam-text-primary);
  margin: 0;
}

.close-btn {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 8px;
  background-color: transparent;
  color: var(--steam-text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
  flex-shrink: 0;
}

.close-btn:hover {
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
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
  border-top: 1px solid var(--steam-border);
  flex-shrink: 0;
}

/* 配置区域 */
.config-section {
  margin-bottom: 24px;
  padding-bottom: 20px;
  border-bottom: 1px solid var(--steam-border);
}

.config-section:last-of-type {
  border-bottom: none;
  margin-bottom: 0;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 16px 0;
}

.config-group {
  margin-bottom: 20px;
}

.config-label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-text-primary);
  margin-bottom: 8px;
}

.config-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 6px 0 0 0;
}

.config-input {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color 0.15s ease;
}

.config-input:focus {
  border-color: var(--steam-accent-blue);
}

.config-select {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 14px;
  outline: none;
  cursor: pointer;
}

.config-select:focus {
  border-color: var(--steam-accent-blue);
}

/* 开关样式 */
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
  background-color: var(--steam-border);
  border-radius: 13px;
  position: relative;
  transition: background-color 0.2s ease;
  flex-shrink: 0;
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
  background-color: var(--steam-accent-blue);
}

.toggle-input:checked + .toggle-slider::after {
  transform: translateX(22px);
}

.toggle-text {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

/* 复选框样式 */
.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 14px;
  color: var(--steam-text-primary);
}

.checkbox-label input[type="checkbox"] {
  width: 18px;
  height: 18px;
  accent-color: var(--steam-accent-blue);
}

/* 行布局 */
.config-row {
  margin-bottom: 16px;
}

.config-row:last-child {
  margin-bottom: 0;
}

.config-row.two-col {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

/* 功能网格 */
.feature-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

/* 滑块样式 */
.slider-group {
  display: flex;
  align-items: center;
  gap: 12px;
}

.config-slider {
  flex: 1;
  height: 6px;
  -webkit-appearance: none;
  appearance: none;
  background: var(--steam-border);
  border-radius: 3px;
  outline: none;
}

.config-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  background: var(--steam-accent-blue);
  border-radius: 50%;
  cursor: pointer;
}

.config-slider::-moz-range-thumb {
  width: 18px;
  height: 18px;
  background: var(--steam-accent-blue);
  border-radius: 50%;
  cursor: pointer;
  border: none;
}

.slider-value {
  font-size: 13px;
  color: var(--steam-text-secondary);
  min-width: 40px;
  text-align: right;
}

/* 按钮样式 */
.btn-primary {
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
  background-color: var(--steam-accent-blue);
  color: white;
}

.btn-primary:hover {
  background-color: var(--steam-accent-hover);
}

.btn-primary svg {
  width: 16px;
  height: 16px;
}

.btn-secondary {
  padding: 10px 20px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
}

.btn-secondary:hover {
  background-color: var(--steam-border);
}

/* 响应式 */
@media (max-width: 600px) {
  .config-row.two-col {
    grid-template-columns: 1fr;
  }
  
  .feature-grid {
    grid-template-columns: 1fr;
  }
}
</style>
