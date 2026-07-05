<template>
  <div class="user-config-panel">
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
          <span class="guide-label">本地存档</span>
          <span class="guide-value">留空使用 %appdata%/GSE Saves，填写相对路径启用便携存档</span>
        </div>
      </div>
      <div class="guide-example">
        <div class="example-title">configs.user.ini 示例：</div>
        <pre class="example-code">[user]
# 用户名（游戏中显示的名称）
user_name = Player
# 语言（使用 Steam API 语言代码）
language = schinese
# 本地存档路径（留空则使用 %appdata%/GSE Saves）
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
      <label class="config-label">本地存档路径（可选）</label>
      <input v-model="config.localSavePath" type="text" class="config-input" placeholder="留空则使用默认路径 %appdata%/GSE Saves" />
      <p class="field-hint">留空使用默认路径 %appdata%/GSE Saves；填写相对路径则启用便携存档</p>
    </div>

    <div class="config-group">
      <label class="config-label">EncryptedAppTicket（Base64，可选）</label>
      <textarea v-model="config.ticket" class="config-textarea" rows="3" placeholder="用于需要票据验证的游戏"></textarea>
      <p class="field-hint">部分游戏需要 EncryptedAppTicket 才能正常运行</p>
    </div>

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
        <span>用户配置已保存成功！</span>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
/**
 * UserConfigPanel.vue - 用户配置 Panel
 * 供 UserConfig.vue 单独弹窗和 CompleteConfigManager.vue 完整管理器复用
 * 统一加载、保存、默认值和同步逻辑
 */

import { ref, shallowReactive, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { CONFIG_EVENTS } from '../../../constants/config-events'

const props = defineProps<{
  gamePath: string
}>()

const emit = defineEmits<{
  saved: []
}>()

/**
 * 用户配置数据模型
 * 默认值与 Rust 后端 UserConfig::default_config() 保持一致。
 * 所有字段均为顶层标量，使用 shallowReactive 减少响应式代理开销。
 */
const config = shallowReactive({
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
async function loadUserConfig() {
  try {
    const result = await invoke<{ exists: boolean; config?: any }>('load_user_config', {
      gamePath: props.gamePath
    })

    if (result.exists && result.config) {
      const cfg = snakeToCamel(result.config)
      config.username = cfg.username || 'Player'
      config.accountSteamid = cfg.accountSteamid || ''
      config.language = cfg.language || 'schinese'
      config.ipCountry = cfg.ipCountry || 'CN'
      config.savesFolderName = cfg.savesFolderName || ''
      // 清理示例或无效路径，确保默认使用 %appdata%/GSE Saves
      const rawPath = cfg.localSavePath || ''
      config.localSavePath =
        rawPath === '' ||
        rawPath.toLowerCase().includes('path/relative') ||
        rawPath.toLowerCase().includes('relative/to/dll') ||
        rawPath.toLowerCase() === 'gse saves'
          ? ''
          : rawPath
      config.ticket = cfg.ticket || ''
      config.altSteamid = cfg.altSteamid || ''
      config.altSteamidCount = cfg.altSteamidCount || 0
    }
  } catch (error) {
    // 加载配置失败时静默处理，使用默认值
  }
}

/**
 * 统一配置保存事件处理器
 * 仅当事件携带的 gamePath 与当前 Panel 匹配时重新加载配置
 */
function onConfigSavedEvent(e: Event) {
  const customEvent = e as CustomEvent<{ gamePath?: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    loadUserConfig()
  }
}

onMounted(() => {
  loadUserConfig()
  // 监听用户配置保存事件，与完整配置管理器/其它单独窗口实时同步
  window.addEventListener(CONFIG_EVENTS.USER_SAVED, onConfigSavedEvent)
})

onUnmounted(() => {
  window.removeEventListener(CONFIG_EVENTS.USER_SAVED, onConfigSavedEvent)
})

/**
 * 保存配置
 * 调用 save_user_config 将表单数据写入 configs.user.ini
 */
async function saveConfig() {
  try {
    const payload = {
      username: config.username,
      accountSteamid: config.accountSteamid,
      language: config.language,
      ipCountry: config.ipCountry,
      savesFolderName: config.savesFolderName,
      localSavePath: config.localSavePath,
      ticket: config.ticket,
      altSteamid: config.altSteamid,
      altSteamidCount: config.altSteamidCount
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
      window.dispatchEvent(new CustomEvent(CONFIG_EVENTS.USER_SAVED, {
        detail: { gamePath: props.gamePath }
      }))
    } else {
      alert(`保存失败: ${result.message}`)
    }
  } catch (error) {
    alert(`保存失败: ${error}`)
  }
}

// 暴露方法供父组件调用
defineExpose({
  load: loadUserConfig,
  save: saveConfig
})
</script>

<style scoped>
.user-config-panel {
  position: relative;
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

.config-input,
select {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 13px;
  outline: none;
}

.config-input:focus,
select:focus {
  border-color: var(--steam-accent-blue);
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

.field-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 6px 0 0 0;
}

.btn-primary {
  display: inline-flex;
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
  margin-top: 24px;
  padding-top: 16px;
  border-top: 1px solid var(--steam-border);
  display: flex;
  justify-content: flex-end;
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
  margin-bottom: 10px;
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
  margin: 0;
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
