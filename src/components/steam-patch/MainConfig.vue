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
              <span class="guide-label">主配置文件</span>
              <span class="guide-value">configs.main.ini</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">文件格式</span>
              <span class="guide-value">INI 格式（键=值）</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">布尔值</span>
              <span class="guide-value">1=启用，0=禁用</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">数值</span>
              <span class="guide-value">直接填写数字，如 300、32</span>
            </div>
          </div>
          <div class="guide-example">
            <div class="example-title">INI 格式示例：</div>
            <pre class="example-code">[main::general]
# 生成新版认证票据（多数游戏需要）
new_app_ticket = 1
# 启用游戏协调器令牌（如 CS:GO、Dota2 需要）
gc_token = 1
# 阻止未知客户端连接
block_unknown_clients = 0

[main::connectivity]
# 监听端口
listen_port = 47584</pre>
          </div>
          <p class="guide-tip">提示：下方已提供推荐配置模板，可直接使用或按需修改</p>
        </div>

        <div class="config-group">
          <label class="config-label">核心配置</label>
          <p class="config-desc">编辑 configs.main.ini 文件内容（与完整配置管理器使用相同的 gbe_fork 默认格式）</p>
          <textarea
            v-model="config.mainIni"
            class="config-textarea"
            rows="10"
            placeholder="[main::general]&#10;new_app_ticket = 1&#10;gc_token = 1&#10;..."
          ></textarea>
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

  <!-- 保存成功提示 -->
  <transition name="toast">
    <div v-if="showToast" class="toast-success">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
        <polyline points="22 4 12 14.01 9 11.01"/>
      </svg>
      <span>主配置已保存成功！</span>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps<{
  gamePath: string
  gameId: string
}>()

const emit = defineEmits<{
  close: []
  saved: []
}>()

const config = ref({
  mainIni: ''
})

const showToast = ref(false)

async function saveConfig() {
  try {
    const result = await invoke<{ success: boolean; message: string }>('save_main_config', {
      gamePath: props.gamePath,
      config: config.value
    })

    if (result.success) {
      showToast.value = true
      setTimeout(() => {
        showToast.value = false
      }, 3000)
      emit('saved')
      // 广播主配置已保存事件，通知完整配置管理器等其它窗口刷新
      window.dispatchEvent(new CustomEvent('main-config-saved', {
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

async function loadConfig() {
  try {
    const result = await invoke<{ exists: boolean; content?: string | null }>('load_main_config', {
      gamePath: props.gamePath
    })

    // 无论文件是否已存在，后端都会返回默认或现有的 INI 内容
    if (result.content) {
      config.value.mainIni = result.content
    }
  } catch (error) {
    // 加载配置失败时静默处理
  }
}

let configSyncHandler: ((e: Event) => void) | null = null

onMounted(() => {
  loadConfig()

  configSyncHandler = (e: Event) => {
    const customEvent = e as CustomEvent<{ gamePath?: string }>
    if (customEvent.detail?.gamePath === props.gamePath) {
      loadConfig()
    }
  }
  // 监听主配置保存事件，与完整配置管理器实时同步
  window.addEventListener('main-config-saved', configSyncHandler)
})

onUnmounted(() => {
  if (configSyncHandler) {
    window.removeEventListener('main-config-saved', configSyncHandler)
  }
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

.guide-tip {
  font-size: 12px;
  color: var(--steam-accent-blue);
  margin: 10px 0 0 0;
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