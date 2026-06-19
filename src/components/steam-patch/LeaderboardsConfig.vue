<template>
  <!-- 保存成功提示 - 放在弹窗外部，确保关闭弹窗后仍可见 -->
  <transition name="toast">
    <div v-if="showToast" class="toast-success">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
        <polyline points="22 4 12 14.01 9 11.01"/>
      </svg>
      <span>排行榜配置已保存成功！</span>
    </div>
  </transition>

  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <div class="header-icon leaderboards">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M6 9H4.5a2.5 2.5 0 0 1 0-5H6"/>
            <path d="M18 9h1.5a2.5 2.5 0 0 0 0-5H18"/>
            <path d="M4 22h16"/>
            <path d="M10 14.66V17c0 .55-.47.98-.97 1.21C7.85 18.75 7 20.24 7 22"/>
            <path d="M14 14.66V17c0 .55.47.98.97 1.21C16.15 18.75 17 20.24 17 22"/>
            <path d="M18 2H6v7a6 6 0 0 0 12 0V2z"/>
          </svg>
        </div>
        <h3>排行榜配置</h3>
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
              <span class="guide-label">排行榜定义文件</span>
              <span class="guide-value">leaderboards.txt</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">排行榜名称</span>
              <span class="guide-value">英文字母+下划线，如 high_score</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">排序类型</span>
              <span class="guide-value">数字 0~3（0=无,1=升序,2=降序,3=近邻）</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">显示类型</span>
              <span class="guide-value">数字 0~3（0=无,1=数字,2=时间秒,3=毫秒）</span>
            </div>
          </div>
          <div class="guide-example">
            <div class="example-title">排行榜定义示例（gbe_fork 标准格式）：</div>
            <pre class="example-code">high_score=2=1
best_time=1=2
level_reached=2=1</pre>
          </div>
        </div>

        <div class="config-group">
          <label class="toggle-label">
            <input v-model="config.enabled" type="checkbox" class="toggle-input" />
            <span class="toggle-slider"></span>
            <span class="toggle-text">启用排行榜系统</span>
          </label>
        </div>

        <div v-if="config.enabled" class="config-group">
          <label class="config-label">排行榜数据配置</label>
          <p class="config-desc">配置本地排行榜的定义，用于模拟离线排行榜</p>
          <textarea
            v-model="leaderboardsText"
            class="config-textarea"
            rows="8"
            placeholder="格式: 排行榜名称=排序类型=显示类型&#10;例如:&#10;high_score=2=1&#10;best_time=1=2"
          ></textarea>
          <p class="field-hint">每行一个排行榜，格式: NAME=sort=display（gbe_fork 标准格式）</p>
        </div>
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
import { ref, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps<{
  gamePath: string
  gameId: string
}>()

const emit = defineEmits<{
  close: []
  saved: []
}>()

const showToast = ref(false)

/**
 * 排行榜配置对象
 * 与 Rust LeaderboardsConfig 结构体一致
 */
const config = ref({
  enabled: true,
  leaderboards: [] as Array<{
    name: string
    displayName: string
    sortMethod: string
    displayType: string
  }>,
})

/** 排行榜文本（用于 textarea 编辑） */
const leaderboardsText = ref('')

/**
 * 将数组格式的排行榜转换为文本
 * 输出格式: NAME=sort=display（gbe_fork 标准格式）
 */
function syncLeaderboardsToText() {
  leaderboardsText.value = config.value.leaderboards
    .map((lb) => `${lb.name}=${lb.sortMethod}=${lb.displayType}`)
    .join('\n')
}

/**
 * 将文本格式的排行榜转换为数组
 * 输入格式: NAME=sort=display（gbe_fork 标准格式）
 * 第一个 '=' 前是 name，中间是 sortMethod，最后一个是 displayType
 */
function syncTextToLeaderboards() {
  const lines = leaderboardsText.value
    .split('\n')
    .map((l) => l.trim())
    .filter((l) => l)

  config.value.leaderboards = []
  for (const line of lines) {
    const parts = line.split('=')
    if (parts.length >= 3) {
      config.value.leaderboards.push({
        name: parts[0].trim(),
        displayName: parts[0].trim(),
        sortMethod: parts[1].trim(),
        displayType: parts[2].trim(),
      })
    }
  }
}

/** 监听文本变化同步到数组 */
watch(leaderboardsText, syncTextToLeaderboards)

/**
 * 保存配置
 */
async function saveConfig() {
  // 确保文本已同步到数组
  syncTextToLeaderboards()

  try {
    const result = await invoke<{ success: boolean; message: string }>('save_leaderboards_config', {
      gamePath: props.gamePath,
      config: config.value,
    })

    if (result.success) {
      showToast.value = true
      setTimeout(() => {
        showToast.value = false
      }, 3000)
      emit('saved')
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
 */
async function loadConfig() {
  try {
    const result = await invoke<{
      exists: boolean
      config?: any
    }>('load_leaderboards_config', {
      gamePath: props.gamePath,
    })

    if (result.exists && result.config) {
      config.value.enabled = result.config.enabled ?? true

      // 加载排行榜定义
      if (result.config.leaderboards && result.config.leaderboards.length > 0) {
        config.value.leaderboards = result.config.leaderboards
        syncLeaderboardsToText()
      }
    }
  } catch (error) {
    console.error('加载排行榜配置失败:', error)
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
  max-width: 600px;
  max-height: 80vh;
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

.header-icon.leaderboards {
  background-color: rgba(236, 72, 153, 0.1);
  color: #ec4899;
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

.config-group {
  margin-bottom: 20px;
}

.config-label {
  display: block;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin-bottom: 8px;
}

.config-desc {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 0 0 12px 0;
}

.config-textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 13px;
  font-family: 'Courier New', monospace;
  resize: vertical;
  outline: none;
}

.config-textarea:focus {
  border-color: var(--steam-accent-blue);
}

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
  color: var(--steam-text-primary);
}

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
  border-radius: 8px;
  padding: 12px 16px;
  margin-bottom: 20px;
}

.guide-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 10px;
  font-size: 13px;
  font-weight: 600;
  color: var(--steam-accent-blue);
}

.guide-header svg {
  width: 16px;
  height: 16px;
}

.guide-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
  margin-bottom: 12px;
}

.guide-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
}

.guide-label {
  color: var(--steam-text-secondary);
  white-space: nowrap;
}

.guide-value {
  color: var(--steam-text-primary);
  font-family: 'Courier New', monospace;
}

.guide-example {
  background-color: var(--steam-bg-primary);
  border-radius: 6px;
  padding: 10px 12px;
}

.example-title {
  font-size: 12px;
  font-weight: 500;
  color: var(--steam-text-secondary);
  margin-bottom: 6px;
}

.example-code {
  font-size: 12px;
  color: #e2e8f0;
  background-color: #1e293b;
  padding: 8px 12px;
  border-radius: 4px;
  overflow-x: auto;
  line-height: 1.5;
  margin: 0;
}

.field-hint {
  font-size: 11px;
  color: var(--steam-text-secondary);
  margin: 4px 0 0 0;
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
