<template>
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
          <label class="config-label">便携存档路径（可选）</label>
          <input v-model="config.localSavePath" type="text" class="config-input" placeholder="留空使用系统默认路径，例如：./saves" />
          <p class="field-hint">设置后，存档将保存在此相对路径下，实现便携存档</p>
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
import { ref } from 'vue'
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
  language: 'schinese',
  localSavePath: ''
})

async function saveConfig() {
  try {
    const result = await invoke<{ success: boolean; message: string }>('save_user_config', {
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
</style>
