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
        <!-- 使用说明 -->
        <div class="usage-guide">
          <div class="guide-header">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="10"/>
              <line x1="12" y1="16" x2="12" y2="12"/>
              <line x1="12" y1="8" x2="12.01" y2="8"/>
            </svg>
            <span>格式说明</span>
          </div>
          <div class="guide-content">
            <div class="guide-item">
              <span class="guide-label">配置文件</span>
              <span class="guide-value">configs.overlay.ini</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">启用字段</span>
              <span class="guide-value">enable_experimental_overlay = 1</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">快捷键格式</span>
              <span class="guide-value">shift + tab（gbe_fork 格式）</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">通知字段</span>
              <span class="guide-value">disable_achievement_notification、disable_friend_notification 等</span>
            </div>
          </div>
        </div>

        <!-- 启用开关 -->
        <div class="config-section">
          <label class="toggle-label">
            <input v-model="config.enableExperimentalOverlay" type="checkbox" class="toggle-input" />
            <span class="toggle-slider"></span>
            <span class="toggle-text">启用实验性 Overlay（Shift+Tab）</span>
          </label>
          <p class="config-hint">实验性功能，如遇到游戏崩溃或卡顿请关闭</p>
        </div>

        <template v-if="config.enableExperimentalOverlay">
          <!-- 快捷键设置 -->
          <div class="config-group">
            <label class="config-label">快捷键</label>
            <input v-model="config.overlayHotkey" type="text" class="config-input" placeholder="shift + tab" />
            <p class="config-hint">按下组合键显示/隐藏 Overlay</p>
          </div>

          <!-- Hook 延迟 -->
          <div class="config-group">
            <label>Hook 延迟（秒）</label>
            <input v-model.number="config.hookDelaySec" type="number" class="config-input" min="0" max="30" placeholder="0" />
            <p class="config-hint">游戏启动后延迟 Hook 的时间，避免冲突</p>
          </div>

          <!-- 渲染器检测超时 -->
          <div class="config-group">
            <label>渲染器检测超时（秒）</label>
            <input v-model.number="config.rendererDetectorTimeoutSec" type="number" class="config-input" min="1" max="60" placeholder="10" />
            <p class="config-hint">检测游戏渲染器的超时时间</p>
          </div>

          <!-- 通知与功能开关 -->
          <h4 class="section-title">通知与功能开关</h4>
          <div class="form-row">
            <label class="checkbox-label">
              <input v-model="config.notifications.disableAchievementNotification" type="checkbox" />
              <span>禁用成就通知</span>
            </label>
            <label class="checkbox-label">
              <input v-model="config.notifications.disableFriendNotification" type="checkbox" />
              <span>禁用好友通知</span>
            </label>
          </div>
          <div class="form-row">
            <label class="checkbox-label">
              <input v-model="config.notifications.disableAchievementProgress" type="checkbox" />
              <span>禁用成就进度</span>
            </label>
            <label class="checkbox-label">
              <input v-model="config.notifications.disableWarningAny" type="checkbox" />
              <span>禁用所有警告</span>
            </label>
          </div>
          <div class="form-row">
            <label class="checkbox-label">
              <input v-model="config.notifications.disableWarningBadAppid" type="checkbox" />
              <span>禁用 AppID 警告</span>
            </label>
            <label class="checkbox-label">
              <input v-model="config.notifications.disableWarningLocalSave" type="checkbox" />
              <span>禁用本地存档警告</span>
            </label>
          </div>
          <div class="form-row">
            <label class="checkbox-label">
              <input v-model="config.notifications.overlayAlwaysShowUserInfo" type="checkbox" />
              <span>始终显示用户信息</span>
            </label>
            <label class="checkbox-label">
              <input v-model="config.notifications.overlayAlwaysShowFps" type="checkbox" />
              <span>始终显示 FPS</span>
            </label>
          </div>
          <div class="form-row">
            <label class="checkbox-label">
              <input v-model="config.notifications.overlayAlwaysShowFrametime" type="checkbox" />
              <span>始终显示帧时间</span>
            </label>
            <label class="checkbox-label">
              <input v-model="config.notifications.overlayAlwaysShowPlaytime" type="checkbox" />
              <span>始终显示游玩时间</span>
            </label>
          </div>

          <!-- FPS 平均窗口 -->
          <div class="config-group">
            <label>FPS 平均窗口</label>
            <input v-model.number="config.fpsAveragingWindow" type="number" class="config-input" min="0.1" max="10" step="0.1" placeholder="1.0" />
            <p class="config-hint">FPS 计算的平均窗口（秒）</p>
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

  <!-- 保存成功提示 -->
  <transition name="toast">
    <div v-if="showToast" class="toast-success">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
        <polyline points="22 4 12 14.01 9 11.01"/>
      </svg>
      <span>覆盖层配置已保存成功！</span>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps<{
  gamePath: string
  gameId: string
  isExperimental?: boolean
}>()

const emit = defineEmits<{
  close: []
  saved: []
}>()

const showToast = ref(false)

/**
 * Overlay 配置对象
 * 字段名使用 camelCase，与后端 save_overlay_config 命令期望的 JSON 键名保持一致
 */
const config = ref({
  enableExperimentalOverlay: false,
  hookDelaySec: undefined,
  rendererDetectorTimeoutSec: undefined,
  overlayHotkey: 'shift + tab',
  fpsAveragingWindow: undefined,
  notifications: {
    disableAchievementNotification: false,
    disableFriendNotification: false,
    disableAchievementProgress: false,
    disableWarningAny: false,
    disableWarningBadAppid: false,
    disableWarningLocalSave: false,
    uploadAchievementsIconsToGpu: true,
    overlayAlwaysShowUserInfo: false,
    overlayAlwaysShowFps: false,
    overlayAlwaysShowFrametime: false,
    overlayAlwaysShowPlaytime: false,
  },
})

/**
 * 将 snake_case 键名递归转换为 camelCase
 * 用于把后端返回的 OverlayConfig 数据转换为前端表单结构
 */
function snakeToCamel(obj: any): any {
  if (obj === null || typeof obj !== 'object') return obj
  if (Array.isArray(obj)) return obj.map(snakeToCamel)
  const result: any = {}
  for (const [key, value] of Object.entries(obj)) {
    const camelKey = key.replace(/_([a-z])/g, (_, letter: string) => letter.toUpperCase())
    result[camelKey] = snakeToCamel(value)
  }
  return result
}

/**
 * 保存配置
 * 将配置发送到后端 save_overlay_config 命令
 */
async function saveConfig() {
  try {
    const result = await invoke<{ success: boolean; message: string }>('save_overlay_config', {
      gamePath: props.gamePath,
      config: config.value,
    })

    if (result.success) {
      showToast.value = true
      setTimeout(() => {
        showToast.value = false
      }, 3000)
      emit('saved')
      // 广播覆盖层配置已保存事件，使其他弹窗/页面同步刷新
      window.dispatchEvent(new CustomEvent('overlay-config-saved', {
        detail: { gamePath: props.gamePath }
      }))
      // 延迟关闭弹窗，等待 Toast 消失后再关闭
      setTimeout(() => {
        emit('close')
      }, 3000)
    } else {
      alert(`保存失败: ${result.message}`)
    }
  } catch (error) {
    alert(`保存失败: ${error}`)
  }
}

/**
 * 加载现有配置
 * 从后端读取 configs.overlay.ini 并填充到表单
 */
async function loadConfig() {
  try {
    const result = await invoke<{
      exists: boolean
      config?: any
    }>('load_overlay_config', {
      gamePath: props.gamePath,
    })

    if (result.exists && result.config) {
      // 后端返回 snake_case 键名，需转换为 camelCase 以匹配前端表单结构
      const camelConfig = snakeToCamel(result.config)
      // 合并已有配置，保留默认值
      config.value = { ...config.value, ...camelConfig }
      // 确保 notifications 子对象也存在
      if (camelConfig.notifications) {
        config.value.notifications = { ...config.value.notifications, ...camelConfig.notifications }
      }
    }
  } catch (error) {
    // 加载失败时使用默认值
  }
}

/**
 * 处理覆盖层配置同步事件
 * 当其他组件（如完整配置管理器）保存 overlay 配置时，若 gamePath 匹配则自动重载
 */
function handleOverlayConfigSaved(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    loadConfig()
  }
}

onMounted(() => {
  loadConfig()
  window.addEventListener('overlay-config-saved', handleOverlayConfigSaved)
})

onUnmounted(() => {
  window.removeEventListener('overlay-config-saved', handleOverlayConfigSaved)
})
</script>

<style scoped>
/* 复用已有样式 */
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

.form-row {
  display: flex;
  gap: 20px;
  margin-bottom: 12px;
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

/* 使用说明 */
.usage-guide {
  background-color: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 10px;
  padding: 16px 20px;
  margin-bottom: 20px;
}

.guide-header {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 14px;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-accent-blue);
}

.guide-header svg {
  width: 18px;
  height: 18px;
  flex-shrink: 0;
}

.guide-content {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-bottom: 16px;
}

.guide-item {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  font-size: 13px;
  line-height: 1.6;
}

.guide-item::before {
  content: '';
  display: block;
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background-color: var(--steam-accent-blue);
  flex-shrink: 0;
  margin-top: 7px;
}

.guide-label {
  color: var(--steam-text-secondary);
  white-space: nowrap;
  min-width: 100px;
  flex-shrink: 0;
}

.guide-value {
  color: var(--steam-text-primary);
  font-family: 'Consolas', 'Courier New', monospace;
  font-size: 12px;
  word-break: break-all;
}

/* 保存成功提示 */
.toast-success {
  position: fixed;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  background-color: #10b981;
  color: white;
  padding: 12px 24px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  font-weight: 500;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 9999;
}

.toast-success svg {
  width: 20px;
  height: 20px;
}

.toast-enter-active {
  animation: toast-in 0.3s ease;
}

.toast-leave-active {
  animation: toast-out 0.3s ease;
}

@keyframes toast-in {
  from {
    opacity: 0;
    transform: translateX(-50%) translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateX(-50%) translateY(0);
  }
}

@keyframes toast-out {
  from {
    opacity: 1;
    transform: translateX(-50%) translateY(0);
  }
  to {
    opacity: 0;
    transform: translateX(-50%) translateY(-20px);
  }
}
</style>