<template>
  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <div class="header-icon lan">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
            <circle cx="9" cy="7" r="4"/>
            <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
            <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
          </svg>
        </div>
        <h3>局域网联机配置</h3>
        <button class="close-btn" @click="$emit('close')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <div class="modal-body">
        <!-- 启用局域网联机 -->
        <div class="config-item">
          <label class="toggle-label">
            <input type="checkbox" v-model="config.enabled" class="toggle-input" />
            <span class="toggle-slider"></span>
            <span class="toggle-text">启用局域网联机</span>
          </label>
        </div>

        <!-- 自定义广播IP -->
        <div class="config-group">
          <label class="config-label">自定义广播 IP / 域名</label>
          <p class="config-desc">添加自定义 IP 地址或域名，模拟器将向这些地址发送广播包</p>
          <div class="ip-list">
            <div v-for="(ip, index) in config.customBroadcasts" :key="index" class="ip-item">
              <input
                type="text"
                v-model="config.customBroadcasts[index]"
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

        <!-- 自动接受邀请 -->
        <div class="config-group">
          <label class="config-label">自动接受邀请</label>
          <p class="config-desc">配置自动接受来自指定 Steam ID 的游戏/大厅邀请（实验版功能）</p>
          <div class="invite-options">
            <label class="radio-label">
              <input type="radio" v-model="config.autoAcceptInvite" value="none" />
              <span>不自动接受</span>
            </label>
            <label class="radio-label">
              <input type="radio" v-model="config.autoAcceptInvite" value="all" />
              <span>接受所有人的邀请</span>
            </label>
            <label class="radio-label">
              <input type="radio" v-model="config.autoAcceptInvite" value="whitelist" />
              <span>仅接受白名单用户的邀请</span>
            </label>
          </div>

          <!-- 白名单列表 -->
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

        <!-- 监听端口配置 -->
        <div class="config-group">
          <label class="config-label">监听端口</label>
          <p class="config-desc">模拟器使用的 UDP 端口（默认 47584）</p>
          <input
            type="number"
            v-model.number="config.listenPort"
            class="config-input"
            placeholder="47584"
            min="1024"
            max="65535"
          />
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
import { ref, computed, onMounted } from 'vue'
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

interface LanConfig {
  enabled: boolean
  customBroadcasts: string[]
  autoAcceptInvite: 'none' | 'all' | 'whitelist'
  listenPort: number
}

// ============================================
// 响应式状态
// ============================================

const config = ref<LanConfig>({
  enabled: true,
  customBroadcasts: [],
  autoAcceptInvite: 'none',
  listenPort: 47584
})

const whitelistText = ref('')

// ============================================
// 方法
// ============================================

/**
 * 添加广播 IP
 */
const addBroadcastIp = () => {
  config.value.customBroadcasts.push('')
}

/**
 * 移除广播 IP
 */
const removeBroadcastIp = (index: number) => {
  config.value.customBroadcasts.splice(index, 1)
}

/**
 * 保存配置
 */
const saveConfig = async () => {
  try {
    // 过滤空值
    const filteredBroadcasts = config.value.customBroadcasts.filter(ip => ip.trim() !== '')

    // 解析白名单
    const whitelist = whitelistText.value
      .split('\n')
      .map(id => id.trim())
      .filter(id => id !== '')

    const result = await invoke<{
      success: boolean
      message: string
    }>('save_lan_multiplayer_config', {
      gamePath: props.gamePath,
      config: {
        enabled: config.value.enabled,
        customBroadcasts: filteredBroadcasts,
        autoAcceptInvite: config.value.autoAcceptInvite,
        whitelist: config.value.autoAcceptInvite === 'whitelist' ? whitelist : [],
        listenPort: config.value.listenPort
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
      config?: LanConfig & { whitelist?: string[] }
    }>('load_lan_multiplayer_config', {
      gamePath: props.gamePath
    })

    if (result.exists && result.config) {
      config.value.enabled = result.config.enabled
      config.value.customBroadcasts = result.config.customBroadcasts.length > 0
        ? result.config.customBroadcasts
        : ['']
      config.value.autoAcceptInvite = result.config.autoAcceptInvite
      config.value.listenPort = result.config.listenPort

      if (result.config.whitelist) {
        whitelistText.value = result.config.whitelist.join('\n')
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

.header-icon.lan {
  background-color: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
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
  margin-bottom: 8px;
}

.config-desc {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 0 0 12px 0;
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

/* IP 列表 */
.ip-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.ip-item {
  display: flex;
  gap: 8px;
}

.config-input {
  flex: 1;
  padding: 10px 12px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color 0.15s ease;
}

.config-input:focus {
  border-color: var(--accent-color);
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
}

.remove-btn:hover {
  background-color: rgba(239, 68, 68, 0.2);
}

.remove-btn svg {
  width: 16px;
  height: 16px;
}

.add-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 10px;
  border: 1px dashed var(--border-color);
  border-radius: 8px;
  background-color: transparent;
  color: var(--accent-color);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.add-btn:hover {
  border-color: var(--accent-color);
  background-color: rgba(59, 130, 246, 0.05);
}

.add-btn svg {
  width: 16px;
  height: 16px;
}

/* 单选按钮组 */
.invite-options {
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
  color: var(--text-primary);
}

.radio-label input[type="radio"] {
  width: 18px;
  height: 18px;
  accent-color: var(--accent-color);
}

/* 白名单区域 */
.whitelist-section {
  margin-top: 12px;
  padding: 12px;
  background-color: var(--bg-primary);
  border-radius: 8px;
}

.config-textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 13px;
  font-family: 'Courier New', monospace;
  resize: vertical;
  outline: none;
}

.config-textarea:focus {
  border-color: var(--accent-color);
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
