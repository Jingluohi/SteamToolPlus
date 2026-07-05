<template>
  <div class="leaderboards-config-panel">
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

    <!-- 启用开关 -->
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

    <!-- 保存按钮 -->
    <div class="panel-actions">
      <button class="btn-primary" @click="saveConfig">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="20 6 9 17 4 12"/>
        </svg>
        保存配置
      </button>
    </div>

    <!-- 保存成功提示 -->
    <transition name="toast">
      <div v-if="showToast" class="toast-success">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
          <polyline points="22 4 12 14.01 9 11.01"/>
        </svg>
        <span>排行榜配置已保存成功！</span>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
/**
 * LeaderboardsConfigPanel.vue - 排行榜配置统一 Panel
 * 供单独弹窗和完整配置管理器复用
 */

import { shallowReactive, ref, onMounted, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { LeaderboardsConfig, Leaderboard } from '../../../types/steam-config.types'
import { CONFIG_EVENTS } from '../../../constants/config-events'

const props = defineProps<{
  gamePath: string
}>()

const emit = defineEmits<{
  saved: []
}>()

const showToast = ref(false)

/**
 * 排行榜配置对象
 * 与 Rust LeaderboardsConfig 结构体一致。
 * 使用 shallowReactive 减少深层响应式代理开销；
 * 数组/对象通过不可变更新替换引用。
 */
const config = shallowReactive<LeaderboardsConfig>({
  enabled: true,
  leaderboards: [],
})

/** 排行榜文本（用于 textarea 编辑） */
const leaderboardsText = ref('')

/**
 * 将数组格式的排行榜转换为文本
 * 输出格式: NAME=sort=display（gbe_fork 标准格式）
 */
function syncLeaderboardsToText() {
  leaderboardsText.value = config.leaderboards
    .map((lb) => `${lb.name}=${lb.sortMethod}=${lb.displayType}`)
    .join('\n')
}

/**
 * 将文本格式的排行榜转换为数组
 * 输入格式: NAME=sort=display（gbe_fork 标准格式）
 * 第一个 '=' 前是 name，中间是 sortMethod，最后一个是 displayType
 * 使用局部数组收集后再整体赋值，适配 shallowReactive 的不可变更新
 */
function syncTextToLeaderboards() {
  const lines = leaderboardsText.value
    .split('\n')
    .map((l) => l.trim())
    .filter((l) => l)

  const leaderboards: Leaderboard[] = []
  for (const line of lines) {
    const parts = line.split('=')
    if (parts.length >= 3) {
      leaderboards.push({
        name: parts[0].trim(),
        displayName: parts[0].trim(),
        sortMethod: parts[1].trim() as Leaderboard['sortMethod'],
        displayType: parts[2].trim() as Leaderboard['displayType'],
        entries: [],
      })
    }
  }
  config.leaderboards = leaderboards
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
      config,
    })

    if (result.success) {
      showToast.value = true
      setTimeout(() => {
        showToast.value = false
      }, 3000)
      emit('saved')
      // 广播排行榜配置已保存事件，通知完整配置管理器等其它窗口刷新
      window.dispatchEvent(new CustomEvent(CONFIG_EVENTS.LEADERBOARDS_SAVED, {
        detail: { gamePath: props.gamePath }
      }))
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
      config?: LeaderboardsConfig
    }>('load_leaderboards_config', {
      gamePath: props.gamePath,
    })

    if (result.exists && result.config) {
      config.enabled = result.config.enabled ?? true

      // 同步排行榜定义（包括空数组，确保外部清空后文本区同步）
      config.leaderboards = Array.isArray(result.config.leaderboards)
        ? result.config.leaderboards
        : []
      syncLeaderboardsToText()
    }
  } catch (error) {
    // 加载失败时使用默认值
  }
}

/**
 * 统一配置保存事件处理器：仅当事件携带的 gamePath 与当前 Panel 匹配时重新加载
 */
function onConfigSavedEvent(e: Event) {
  const customEvent = e as CustomEvent<{ gamePath?: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    loadConfig()
  }
}

onMounted(() => {
  loadConfig()
  // 监听排行榜配置保存事件，与完整配置管理器实时同步
  window.addEventListener(CONFIG_EVENTS.LEADERBOARDS_SAVED, onConfigSavedEvent)
})

onUnmounted(() => {
  window.removeEventListener(CONFIG_EVENTS.LEADERBOARDS_SAVED, onConfigSavedEvent)
})

defineExpose({
  load: loadConfig,
  save: saveConfig
})
</script>

<style scoped>
.leaderboards-config-panel {
  width: 100%;
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
  font-family: 'Consolas', 'Courier New', monospace;
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

.panel-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 24px;
  padding-top: 20px;
  border-top: 1px solid var(--steam-border);
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

.guide-example {
  background-color: var(--steam-bg-primary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  padding: 12px 14px;
  margin-bottom: 10px;
}

.guide-example:last-of-type {
  margin-bottom: 0;
}

.example-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin-bottom: 8px;
}

.example-code {
  font-size: 12px;
  color: var(--steam-text-primary);
  background-color: rgba(0, 0, 0, 0.2);
  padding: 10px 14px;
  border-radius: 6px;
  overflow-x: auto;
  line-height: 1.6;
  margin: 0;
  white-space: pre-wrap;
  word-break: break-all;
}

.field-hint {
  font-size: 12px;
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
