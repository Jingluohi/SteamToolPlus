<template>
  <!-- 保存成功提示 - 放在弹窗外部，确保关闭弹窗后仍可见 -->
  <transition name="toast">
    <div v-if="showToast" class="toast-success">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
        <polyline points="22 4 12 14.01 9 11.01"/>
      </svg>
      <span>用户配置已保存成功！</span>
    </div>
  </transition>

  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <div class="header-icon user">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
            <circle cx="12" cy="7" r="4"/>
          </svg>
        </div>
        <h3>用户配置</h3>
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
              <span class="guide-label">用户配置文件</span>
              <span class="guide-value">configs.user.ini</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">头像文件</span>
              <span class="guide-value">account_avatar.png/jpg/jpeg（放在 GSE Saves/settings/）</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">语言代码</span>
              <span class="guide-value">schinese/tchinese/english/japanese 等</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">便携存档</span>
              <span class="guide-value">在 configs.user.ini 中设置 local_save_path=相对路径</span>
            </div>
          </div>
          <div class="guide-example">
            <div class="example-title">configs.user.ini 示例：</div>
            <pre class="example-code">[user]
# 用户名（游戏中显示的名称）
user_name = Player
# 语言（使用 Steam API 语言代码）
language = schinese
# 便携存档路径（留空则使用系统默认路径）
local_save_path = 
# 自定义存档文件夹名称
saves_folder_name = </pre>
          </div>
          <p class="guide-tip">提示：头像文件放在 GSE Saves/settings/ 目录下，命名为 account_avatar</p>
        </div>

        <div class="config-group">
          <label class="config-label">用户名</label>
          <input v-model="config.username" type="text" class="config-input" placeholder="输入用户名..." />
          <p class="field-hint">游戏中显示的名称，可自定义</p>
        </div>

        <div class="config-group">
          <label class="config-label">SteamID（Steam64 格式，可选）</label>
          <input v-model="config.accountSteamid" type="text" class="config-input" placeholder="76561197960287930" />
          <p class="field-hint">无效 ID 会被模拟器忽略，留空则自动生成</p>
        </div>

        <div class="config-group">
          <label class="config-label">语言</label>
          <select v-model="config.language" class="config-input">
            <option value="schinese">简体中文</option>
            <option value="tchinese">繁体中文</option>
            <option value="english">English</option>
            <option value="japanese">日本語</option>
            <option value="korean">한국어</option>
            <option value="russian">русский</option>
            <option value="french">français</option>
            <option value="german">Deutsch</option>
            <option value="spanish">español</option>
            <option value="brazilian">português-Brasil</option>
            <option value="polish">polski</option>
            <option value="turkish">Türkçe</option>
          </select>
          <p class="field-hint">游戏界面语言，部分游戏可能不支持所有语言</p>
        </div>

        <div class="config-group">
          <label class="config-label">IP 国家代码</label>
          <input v-model="config.ipCountry" type="text" class="config-input" placeholder="CN" />
          <p class="field-hint">ISO 3166-1-alpha-2 格式，游戏查询 IP 时上报的国家代码</p>
        </div>

        <div class="config-group">
          <label class="config-label">存档文件夹名称（可选）</label>
          <input v-model="config.savesFolderName" type="text" class="config-input" placeholder="覆盖默认的 GSE Saves" />
          <p class="field-hint">设置后会覆盖默认的存档文件夹名称</p>
        </div>

        <div class="config-group">
          <label class="config-label">本地存档路径（便携模式）</label>
          <input v-model="config.localSavePath" type="text" class="config-input" placeholder="设置后完全便携，例如：./saves" />
          <p class="field-hint">设置后，存档将保存在此相对路径下，实现便携存档</p>
        </div>

        <div class="config-group">
          <label class="config-label">EncryptedAppTicket（Base64，可选）</label>
          <textarea v-model="config.ticket" class="config-textarea" rows="3" placeholder="用于需要票据验证的游戏"></textarea>
          <p class="field-hint">部分游戏需要 EncryptedAppTicket 才能正常运行</p>
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
  username: 'Player',
  accountSteamid: '',
  language: 'schinese',
  ipCountry: 'CN',
  savesFolderName: '',
  localSavePath: '',
  ticket: '',
  altSteamid: '',
  altSteamidCount: 0
})

const showToast = ref(false)

/**
 * 将 snake_case 键名递归转换为 camelCase
 * Rust 后端默认返回 snake_case，但前端类型与表单统一使用 camelCase
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
 * 加载已有配置
 * 调用 load_user_config 读取 configs.user.ini 并填充表单
 * 后端返回 snake_case，通过 snakeToCamel 转换为 camelCase 后赋值
 */
async function loadConfig() {
  try {
    const result = await invoke<{ exists: boolean; config?: any }>('load_user_config', {
      gamePath: props.gamePath
    })

    if (result.exists && result.config) {
      const cfg = snakeToCamel(result.config)
      config.value.username = cfg.username || 'Player'
      config.value.accountSteamid = cfg.accountSteamid || ''
      config.value.language = cfg.language || 'schinese'
      config.value.ipCountry = cfg.ipCountry || 'CN'
      config.value.savesFolderName = cfg.savesFolderName || ''
      config.value.localSavePath = cfg.localSavePath || ''
      config.value.ticket = cfg.ticket || ''
      config.value.altSteamid = cfg.altSteamid || ''
      config.value.altSteamidCount = cfg.altSteamidCount || 0
    }
  } catch (error) {
    // 加载配置失败时静默处理，使用默认值
  }
}

/**
 * 保存配置
 * 调用 save_user_config 将表单数据写入 configs.user.ini
 */
async function saveConfig() {
  try {
    const payload = {
      username: config.value.username,
      accountSteamid: config.value.accountSteamid,
      language: config.value.language,
      ipCountry: config.value.ipCountry,
      savesFolderName: config.value.savesFolderName,
      localSavePath: config.value.localSavePath,
      ticket: config.value.ticket,
      altSteamid: config.value.altSteamid,
      altSteamidCount: config.value.altSteamidCount
    }
    const result = await invoke<{ success: boolean; message: string }>('save_user_config', {
      gamePath: props.gamePath,
      config: payload
    })

    if (result.success) {
      showToast.value = true
      setTimeout(() => {
        showToast.value = false
      }, 3000)
      emit('saved')
      // 广播用户配置已保存事件，通知完整配置管理器等其它窗口刷新
      window.dispatchEvent(new CustomEvent('user-config-saved', {
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

let configSyncHandler: ((e: Event) => void) | null = null

onMounted(() => {
  loadConfig()

  configSyncHandler = (e: Event) => {
    const customEvent = e as CustomEvent<{ gamePath?: string }>
    if (customEvent.detail?.gamePath === props.gamePath) {
      loadConfig()
    }
  }
  // 监听用户配置保存事件，与完整配置管理器实时同步
  window.addEventListener('user-config-saved', configSyncHandler)
})

onUnmounted(() => {
  if (configSyncHandler) {
    window.removeEventListener('user-config-saved', configSyncHandler)
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

.header-icon.user {
  background-color: rgba(99, 102, 241, 0.1);
  color: #6366f1;
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

.config-input {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 14px;
  outline: none;
}

.config-input:focus {
  border-color: var(--steam-accent-blue);
}

.config-textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 12px;
  font-family: 'Consolas', 'Courier New', monospace;
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
.field-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 6px 0 0 0;
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

@media (max-width: 600px) {
  .guide-content {
    grid-template-columns: 1fr;
  }
}

/* 响应式 */
@media (max-width: 600px) {
  .guide-content {
    grid-template-columns: 1fr;
  }
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