<template>
  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <div class="header-icon main">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="3"/>
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
          </svg>
        </div>
        <h3>主配置</h3>
        <button class="close-btn" @click="$emit('close')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <div class="modal-body">
        <!-- 认证票据设置 -->
        <div class="config-section">
          <h4 class="section-title">认证票据设置</h4>

          <div class="config-group">
            <label class="toggle-label">
              <input type="checkbox" v-model="config.newAppTicket" class="toggle-input" />
              <span class="toggle-slider"></span>
              <span class="toggle-text">使用新的 App Ticket 格式</span>
            </label>
            <p class="config-hint">某些游戏需要新的票据格式才能正常工作</p>
          </div>

          <div class="config-group">
            <label class="toggle-label">
              <input type="checkbox" v-model="config.gcToken" class="toggle-input" />
              <span class="toggle-slider"></span>
              <span class="toggle-text">生成 GC Token</span>
            </label>
            <p class="config-hint">为使用 Steam 游戏协调器的游戏生成 GC Token</p>
          </div>

          <div class="config-group">
            <label class="config-label">加密 App Ticket (Base64)</label>
            <textarea
              v-model="config.encryptedAppTicket"
              class="config-textarea"
              rows="3"
              placeholder="输入 Base64 编码的加密票据（可选）"
            ></textarea>
            <p class="config-hint">某些游戏需要特定的加密票据格式</p>
          </div>
        </div>

        <!-- 联机设置 -->
        <div class="config-section">
          <h4 class="section-title">联机设置</h4>

          <div class="config-group">
            <label class="toggle-label">
              <input type="checkbox" v-model="config.disableNetworking" class="toggle-input" />
              <span class="toggle-slider"></span>
              <span class="toggle-text">禁用网络功能</span>
            </label>
            <p class="config-hint">完全禁用所有网络相关功能</p>
          </div>

          <div class="config-group">
            <label class="toggle-label">
              <input type="checkbox" v-model="config.offlineMode" class="toggle-input" />
              <span class="toggle-slider"></span>
              <span class="toggle-text">离线模式</span>
            </label>
            <p class="config-hint">模拟 Steam 离线状态</p>
          </div>
        </div>

        <!-- 日志设置 -->
        <div class="config-section">
          <h4 class="section-title">日志设置</h4>

          <div class="config-group">
            <label class="toggle-label">
              <input type="checkbox" v-model="config.enableLogging" class="toggle-input" />
              <span class="toggle-slider"></span>
              <span class="toggle-text">启用日志记录</span>
            </label>
          </div>

          <div v-if="config.enableLogging" class="config-group">
            <label class="config-label">日志级别</label>
            <select v-model="config.logLevel" class="config-select">
              <option value="0">错误 (Error)</option>
              <option value="1">警告 (Warning)</option>
              <option value="2">信息 (Info)</option>
              <option value="3">调试 (Debug)</option>
            </select>
          </div>
        </div>

        <!-- 高级设置 -->
        <div class="config-section">
          <h4 class="section-title">高级设置</h4>

          <div class="config-group">
            <label class="toggle-label">
              <input type="checkbox" v-model="config.disableAccountLimit" class="toggle-input" />
              <span class="toggle-slider"></span>
              <span class="toggle-text">禁用账户限制检查</span>
            </label>
          </div>

          <div class="config-group">
            <label class="toggle-label">
              <input type="checkbox" v-model="config.forceOffline" class="toggle-input" />
              <span class="toggle-slider"></span>
              <span class="toggle-text">强制离线（即使网络可用）</span>
            </label>
          </div>

          <div class="config-group">
            <label class="toggle-label">
              <input type="checkbox" v-model="config.disableCloud" class="toggle-input" />
              <span class="toggle-slider"></span>
              <span class="toggle-text">禁用 Steam 云存档</span>
            </label>
          </div>

          <div class="config-group">
            <label class="config-label">额外命令行参数</label>
            <input
              type="text"
              v-model="config.extraArgs"
              class="config-input"
              placeholder="传递给游戏的额外参数"
            />
          </div>
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
import { ref, onMounted } from 'vue'
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

interface MainConfig {
  newAppTicket: boolean
  gcToken: boolean
  encryptedAppTicket: string
  disableNetworking: boolean
  offlineMode: boolean
  enableLogging: boolean
  logLevel: string
  disableAccountLimit: boolean
  forceOffline: boolean
  disableCloud: boolean
  extraArgs: string
}

// ============================================
// 响应式状态
// ============================================

const config = ref<MainConfig>({
  newAppTicket: false,
  gcToken: false,
  encryptedAppTicket: '',
  disableNetworking: false,
  offlineMode: false,
  enableLogging: false,
  logLevel: '2',
  disableAccountLimit: false,
  forceOffline: false,
  disableCloud: false,
  extraArgs: ''
})

// ============================================
// 方法
// ============================================

/**
 * 保存配置
 */
const saveConfig = async () => {
  try {
    const result = await invoke<{
      success: boolean
      message: string
    }>('save_main_config', {
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
      config?: MainConfig
    }>('load_main_config', {
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
  max-width: 550px;
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

.header-icon.main {
  background-color: rgba(100, 116, 139, 0.1);
  color: #64748b;
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

/* 配置区域 */
.config-section {
  margin-bottom: 24px;
  padding-bottom: 20px;
  border-bottom: 1px solid var(--border-color);
}

.config-section:last-child {
  margin-bottom: 0;
  padding-bottom: 0;
  border-bottom: none;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 16px 0;
}

/* 配置组 */
.config-group {
  margin-bottom: 16px;
}

.config-group:last-child {
  margin-bottom: 0;
}

.config-label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 8px;
}

.config-hint {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 6px 0 0 60px;
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
  background-color: var(--accent-color);
}

.toggle-input:checked + .toggle-slider::after {
  transform: translateX(22px);
}

.toggle-text {
  font-size: 14px;
  color: var(--text-primary);
}

/* 输入框 */
.config-input,
.config-select {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color 0.15s ease;
}

.config-input:focus,
.config-select:focus {
  border-color: var(--accent-color);
}

.config-select {
  cursor: pointer;
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%236b7280' stroke-width='2'%3E%3Cpath d='M6 9l6 6 6-6'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 12px center;
  padding-right: 36px;
}

.config-textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-size: 13px;
  font-family: 'Courier New', monospace;
  resize: vertical;
  outline: none;
  transition: border-color 0.15s ease;
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

.btn-primary:hover {
  background-color: var(--accent-hover);
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
