<template>
  <div class="other-config-panel">
    <!-- 格式说明 -->
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
          <span class="guide-label">已安装应用</span>
          <span class="guide-value">installed_app_ids.txt（每行一个AppID）</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">订阅群组</span>
          <span class="guide-value">subscribed_groups.txt（每行一个群组ID）</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">购买密钥</span>
          <span class="guide-value">purchased_keys.txt（appid=KEY）</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">支持语言</span>
          <span class="guide-value">supported_languages.txt（每行一个语言代码）</span>
        </div>
      </div>
      <div class="guide-example">
        <div class="example-title">purchased_keys.txt 示例：</div>
        <pre class="example-code"># appid=KEY
123456=ABCDE-FGHIJ-KLMNO
789012=PQRST-UVWXY-Z1234</pre>
      </div>
      <p class="guide-tip">提示：详细格式说明请参考 gbe_fork 官方文档或 steam_settings.EXAMPLE 目录</p>
    </div>

    <!-- 已安装应用ID -->
    <div class="config-section">
      <h4>已安装应用ID</h4>
      <p class="field-hint">每行一个纯数字，如 480、730</p>
      <textarea
        v-model="otherConfigs.installedAppIds"
        class="config-textarea"
        rows="3"
        placeholder="480&#10;730"
      ></textarea>
    </div>

    <!-- 订阅群组 -->
    <div class="config-section">
      <h4>订阅群组ID</h4>
      <p class="field-hint">每行一个纯数字，Steam 群组 ID</p>
      <textarea
        v-model="otherConfigs.subscribedGroups"
        class="config-textarea"
        rows="3"
      ></textarea>
    </div>

    <!-- 购买密钥 -->
    <div class="config-section">
      <h4>CD密钥</h4>
      <p class="field-hint">每行一个密钥，如 XXXXX-XXXXX-XXXXX</p>
      <textarea
        v-model="otherConfigs.purchasedKeys"
        class="config-textarea"
        rows="3"
      ></textarea>
    </div>

    <!-- 支持语言 -->
    <div class="config-section">
      <h4>支持语言</h4>
      <p class="field-hint">每行一个语言代码，如 schinese、english</p>
      <textarea
        v-model="otherConfigs.supportedLanguages"
        class="config-textarea"
        rows="3"
        placeholder="schinese&#10;english"
      ></textarea>
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
        <span>其他配置已保存成功！</span>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
/**
 * OtherConfigPanel.vue - 其他配置统一 Panel
 * 处理 installed_app_ids.txt / subscribed_groups.txt / purchased_keys.txt / supported_languages.txt
 * 供 AdvancedConfig.vue 和 CompleteConfigManager.vue 复用
 */

import { ref, shallowReactive, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { CONFIG_EVENTS } from '../../../constants/config-events'

export interface OtherConfigData {
  installedAppIds: string
  subscribedGroups: string
  purchasedKeys: string
  supportedLanguages: string
}

const props = defineProps<{
  gamePath: string
}>()

const emit = defineEmits<{
  saved: []
}>()

const showToast = ref(false)

// 使用 shallowReactive 减少深层响应式代理开销；
// 所有字段均为顶层字符串，v-model 可直接使用。
const otherConfigs = shallowReactive<OtherConfigData>({
  installedAppIds: '',
  subscribedGroups: '',
  purchasedKeys: '',
  supportedLanguages: '',
})

async function saveConfig() {
  const appIds = otherConfigs.installedAppIds
    .split('\n')
    .map((s) => s.trim())
    .filter((s) => s !== '')

  const groupIds = otherConfigs.subscribedGroups
    .split('\n')
    .map((s) => s.trim())
    .filter((s) => s !== '')

  const keys = otherConfigs.purchasedKeys
    .split('\n')
    .map((s) => s.trim())
    .filter((s) => s !== '' && !s.startsWith('#'))

  const languages = otherConfigs.supportedLanguages
    .split('\n')
    .map((s) => s.trim())
    .filter((s) => s !== '')

  await Promise.all([
    invoke('save_installed_app_ids', { gamePath: props.gamePath, appIds }),
    invoke('save_subscribed_groups', { gamePath: props.gamePath, groupIds }),
    invoke('save_purchased_keys', { gamePath: props.gamePath, keys }),
    invoke('save_supported_languages', { gamePath: props.gamePath, languages }),
  ])

  showToast.value = true
  setTimeout(() => {
    showToast.value = false
  }, 3000)
  emit('saved')
  window.dispatchEvent(
    new CustomEvent(CONFIG_EVENTS.OTHER_SAVED, {
      detail: { gamePath: props.gamePath },
    })
  )
}

async function loadConfig() {
  const [appIds, groupIds, keys, languages] = await Promise.all([
    invoke<string[]>('load_installed_app_ids', { gamePath: props.gamePath }),
    invoke<string[]>('load_subscribed_groups', { gamePath: props.gamePath }),
    invoke<string[]>('load_purchased_keys', { gamePath: props.gamePath }),
    invoke<string[]>('load_supported_languages', { gamePath: props.gamePath }),
  ])

  otherConfigs.installedAppIds = Array.isArray(appIds) ? appIds.join('\n') : ''
  otherConfigs.subscribedGroups = Array.isArray(groupIds) ? groupIds.join('\n') : ''
  otherConfigs.purchasedKeys = Array.isArray(keys) ? keys.join('\n') : ''
  otherConfigs.supportedLanguages = Array.isArray(languages) ? languages.join('\n') : ''
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
  window.addEventListener(CONFIG_EVENTS.OTHER_SAVED, onConfigSavedEvent)
})

onUnmounted(() => {
  window.removeEventListener(CONFIG_EVENTS.OTHER_SAVED, onConfigSavedEvent)
})

defineExpose({
  load: loadConfig,
  save: saveConfig,
})
</script>

<style scoped>
.other-config-panel {
  width: 100%;
}

.config-section {
  margin-bottom: 20px;
  padding-bottom: 20px;
  border-bottom: 1px solid var(--steam-border);
}

.config-section:last-of-type {
  border-bottom: none;
  margin-bottom: 0;
  padding-bottom: 0;
}

.config-section h4 {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 8px 0;
}

.field-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 0 0 10px 0;
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
  transition: border-color 0.15s ease;
  box-sizing: border-box;
}

.config-textarea:focus {
  border-color: var(--steam-accent-blue);
}

.panel-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 24px;
  padding-top: 20px;
  border-top: 1px solid var(--steam-border);
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

.guide-tip {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  font-size: 12px;
  color: var(--steam-accent-blue);
  margin-top: 14px;
  line-height: 1.5;
  padding: 8px 12px;
  background-color: rgba(59, 130, 246, 0.08);
  border-radius: 6px;
}

.guide-tip::before {
  content: '';
  display: block;
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background-color: var(--steam-accent-blue);
  flex-shrink: 0;
  margin-top: 6px;
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
