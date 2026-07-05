<template>
  <div class="lan-multiplayer-config-panel">
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
          <span class="guide-label">广播 IP 列表</span>
          <span class="guide-value">custom_broadcasts.txt（每行一个 IP/域名）</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">自动接受邀请</span>
          <span class="guide-value">auto_accept_invite.txt（每行一个 SteamID64）</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">空文件</span>
          <span class="guide-value">接受所有人的邀请</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">监听端口</span>
          <span class="guide-value">UDP 端口，默认 47584</span>
        </div>
      </div>
      <div class="guide-example">
        <div class="example-title">custom_broadcasts.txt 示例：</div>
        <pre class="example-code">192.168.1.100
192.168.1.101
friend.example.com</pre>
      </div>
      <div class="guide-example">
        <div class="example-title">auto_accept_invite.txt 示例：</div>
        <pre class="example-code">76561198000000001
76561198000000002</pre>
      </div>
      <p class="guide-tip">提示：所有玩家必须在同一局域网内，且运行相同 AppID 的游戏时不要同时运行多个实例</p>
    </div>

    <div class="config-item">
      <label class="toggle-label">
        <input v-model="config.enabled" type="checkbox" class="toggle-input" />
        <span class="toggle-slider"></span>
        <span class="toggle-text">启用局域网联机</span>
      </label>
    </div>

    <div class="config-group">
      <label class="config-label">自定义广播 IP / 域名</label>
      <p class="config-desc">添加自定义 IP 地址或域名，模拟器将向这些地址发送广播包</p>
      <div class="ip-list">
        <div v-for="(_ip, index) in config.customBroadcasts" :key="index" class="ip-item">
          <input
            v-model="config.customBroadcasts[index]"
            type="text"
            class="config-input"
            placeholder="例如: 192.168.1.100 或 friend.example.com"
          />
          <button class="remove-btn" @click="removeBroadcastIp(index)">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"/>
              <line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
          </button>
        </div>
        <button class="add-btn" @click="addBroadcastIp">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="12" y1="5" x2="12" y2="19"/>
            <line x1="5" y1="12" x2="19" y2="12"/>
          </svg>
          添加 IP / 域名
        </button>
      </div>
    </div>

    <div class="config-group">
      <label class="config-label">自动接受邀请</label>
      <p class="config-desc">配置自动接受来自指定 Steam ID 的游戏/大厅邀请（功能版）</p>
      <div class="radio-group">
        <label class="radio-label">
          <input v-model="config.autoAcceptInvite" type="radio" value="none" />
          <span>不自动接受</span>
        </label>
        <label class="radio-label">
          <input v-model="config.autoAcceptInvite" type="radio" value="all" />
          <span>接受所有人的邀请</span>
        </label>
        <label class="radio-label">
          <input v-model="config.autoAcceptInvite" type="radio" value="whitelist" />
          <span>仅接受白名单用户的邀请</span>
        </label>
      </div>

      <div v-if="config.autoAcceptInvite === 'whitelist'" class="whitelist-section">
        <p class="config-desc">输入 SteamID64（每行一个）</p>
        <textarea
          v-model="whitelistText"
          class="config-textarea"
          rows="4"
          placeholder="例如:
76561198000000001
76561198000000002"
        ></textarea>
      </div>
    </div>

    <div class="config-group">
      <label class="config-label">监听端口</label>
      <p class="config-desc">模拟器使用的 UDP 端口（默认 47584）</p>
      <input
        v-model.number="config.listenPort"
        type="number"
        class="config-input"
        placeholder="47584"
        min="1024"
        max="65535"
      />
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
        <span>局域网联机配置已保存成功！</span>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
/**
 * LanMultiplayerConfigPanel.vue - 局域网联机配置统一 Panel
 * 供单独弹窗和完整配置管理器复用
 */

import { shallowReactive, ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { CONFIG_EVENTS } from '../../../constants/config-events'

export interface LanMultiplayerConfig {
  enabled: boolean
  customBroadcasts: string[]
  autoAcceptInvite: 'none' | 'all' | 'whitelist'
  listenPort: number
  whitelist?: string[]
}

const props = defineProps<{
  gamePath: string
}>()

const emit = defineEmits<{
  saved: []
}>()

const showToast = ref(false)

// 使用 shallowReactive 减少深层响应式代理开销；
// 顶层属性变更仍保持响应式，数组/对象通过不可变更新替换引用。
const config = shallowReactive<LanMultiplayerConfig>({
  enabled: false,
  customBroadcasts: [],
  autoAcceptInvite: 'none',
  listenPort: 47584,
})

const whitelistText = ref('')

function addBroadcastIp() {
  config.customBroadcasts = [...config.customBroadcasts, '']
}

function removeBroadcastIp(index: number) {
  config.customBroadcasts = config.customBroadcasts.filter((_, i) => i !== index)
}

async function saveConfig() {
  const filteredBroadcasts = config.customBroadcasts.filter((ip) => ip.trim() !== '')
  const whitelist = whitelistText.value
    .split('\n')
    .map((id) => id.trim())
    .filter((id) => id !== '')

  const result = await invoke<{ success: boolean; message: string }>(
    'save_lan_multiplayer_config',
    {
      gamePath: props.gamePath,
      config: {
        enabled: config.enabled,
        customBroadcasts: filteredBroadcasts,
        autoAcceptInvite: config.autoAcceptInvite,
        whitelist: config.autoAcceptInvite === 'whitelist' ? whitelist : [],
        listenPort: config.listenPort,
      },
    }
  )

  if (result.success) {
    showToast.value = true
    setTimeout(() => {
      showToast.value = false
    }, 3000)
    emit('saved')
    window.dispatchEvent(
      new CustomEvent(CONFIG_EVENTS.LAN_SAVED, {
        detail: { gamePath: props.gamePath },
      })
    )
  } else {
    alert(`保存失败: ${result.message}`)
  }
}

async function loadConfig() {
  const result = await invoke<{
    exists: boolean
    config?: LanMultiplayerConfig
  }>('load_lan_multiplayer_config', {
    gamePath: props.gamePath,
  })

  if (result.exists && result.config) {
    const cfg = result.config
    config.enabled = cfg.enabled ?? false
    config.customBroadcasts = Array.isArray(cfg.customBroadcasts) ? cfg.customBroadcasts : []
    config.autoAcceptInvite = cfg.autoAcceptInvite || 'none'
    config.listenPort = cfg.listenPort ?? 47584

    if (cfg.whitelist) {
      whitelistText.value = cfg.whitelist.join('\n')
    } else {
      whitelistText.value = ''
    }
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
  window.addEventListener(CONFIG_EVENTS.LAN_SAVED, onConfigSavedEvent)
})

onUnmounted(() => {
  window.removeEventListener(CONFIG_EVENTS.LAN_SAVED, onConfigSavedEvent)
})

defineExpose({
  load: loadConfig,
  save: saveConfig,
})
</script>

<style scoped>
.lan-multiplayer-config-panel {
  width: 100%;
}

.config-item {
  margin-bottom: 20px;
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

.config-input,
.config-textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color 0.15s ease;
  box-sizing: border-box;
}

.config-input:focus,
.config-textarea:focus {
  border-color: var(--steam-accent-blue);
}

.config-textarea {
  font-family: 'Consolas', 'Courier New', monospace;
  resize: vertical;
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

.ip-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.ip-item {
  display: flex;
  gap: 8px;
  align-items: center;
}

.ip-item .config-input {
  flex: 1;
}

.add-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 10px;
  border: 1px dashed var(--steam-border);
  border-radius: 8px;
  background-color: transparent;
  color: var(--steam-accent-blue);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  width: 100%;
}

.add-btn:hover {
  border-color: var(--steam-accent-blue);
  background-color: rgba(59, 130, 246, 0.05);
}

.add-btn svg {
  width: 16px;
  height: 16px;
}

.remove-btn {
  width: 36px;
  height: 36px;
  border: none;
  border-radius: 8px;
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

.radio-group {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.radio-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 14px;
  color: var(--steam-text-primary);
}

.radio-label input[type='radio'] {
  width: 18px;
  height: 18px;
  accent-color: var(--steam-accent-blue);
}

.whitelist-section {
  margin-top: 12px;
  padding: 12px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
}

.whitelist-section .config-textarea {
  background-color: var(--steam-bg-primary);
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

.btn-primary:hover:not(:disabled) {
  background-color: var(--steam-accent-hover);
}

.btn-primary:disabled {
  background-color: var(--steam-text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
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
