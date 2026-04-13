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
        <!-- 用户名 -->
        <div class="config-group">
          <label class="config-label">Steam 用户名</label>
          <input
            type="text"
            v-model="config.username"
            class="config-input"
            placeholder="输入你的用户名"
          />
          <p class="config-desc">在游戏中显示的用户名</p>
        </div>

        <!-- 用户头像 -->
        <div class="config-group">
          <label class="config-label">用户头像</label>
          <div class="avatar-section">
            <div class="avatar-preview" @click="selectAvatar">
              <img v-if="config.avatarPath" :src="config.avatarPath" alt="Avatar" />
              <div v-else class="avatar-placeholder">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
                  <circle cx="12" cy="7" r="4"/>
                </svg>
              </div>
            </div>
            <div class="avatar-actions">
              <button class="file-btn" @click="selectAvatar">
                {{ config.avatarPath ? '更换头像' : '选择头像' }}
              </button>
              <button v-if="config.avatarPath" class="remove-file-btn" @click="config.avatarPath = ''">
                移除
              </button>
            </div>
          </div>
          <p class="config-desc">支持 .png, .jpg, .jpeg 格式，建议尺寸 64x64 或 128x128</p>
        </div>

        <!-- 语言设置 -->
        <div class="config-group">
          <label class="config-label">界面语言</label>
          <select v-model="config.language" class="config-select">
            <option value="schinese">简体中文</option>
            <option value="tchinese">繁體中文</option>
            <option value="english">English</option>
            <option value="japanese">日本語</option>
            <option value="koreana">한국어</option>
            <option value="russian">Русский</option>
            <option value="german">Deutsch</option>
            <option value="french">Français</option>
            <option value="spanish">Español</option>
            <option value="portuguese">Português</option>
            <option value="brazilian">Português-Brasil</option>
            <option value="polish">Polski</option>
            <option value="turkish">Türkçe</option>
            <option value="thai">ไทย</option>
            <option value="vietnamese">Tiếng Việt</option>
          </select>
          <p class="config-desc">模拟器的界面语言</p>
        </div>

        <!-- 存档文件夹名称 -->
        <div class="config-group">
          <label class="config-label">存档文件夹名称</label>
          <input
            type="text"
            v-model="config.savesFolderName"
            class="config-input"
            placeholder="GSE Saves"
          />
          <p class="config-desc">自定义存档文件夹名称（留空使用默认 GSE Saves）</p>
          <p class="config-path-hint">
            默认存档路径：C:\Users\[用户名]\AppData\Roaming\GSE Saves\[AppID]\
          </p>
        </div>

        <!-- 本地存档路径 -->
        <div class="config-group">
          <label class="config-label">本地存档路径</label>
          <div class="path-input-group">
            <input
              type="text"
              v-model="config.localSavePath"
              class="config-input"
              placeholder="留空使用默认路径"
              readonly
            />
            <button class="browse-btn" @click="selectLocalSavePath">浏览</button>
            <button v-if="config.localSavePath" class="clear-btn" @click="config.localSavePath = ''">清除</button>
          </div>
          <p class="config-desc">设置本地存档路径可实现便携模式（存档与游戏在同一文件夹）</p>
        </div>

        <!-- Steam ID -->
        <div class="config-group">
          <label class="config-label">Steam ID</label>
          <input
            type="text"
            v-model="config.steamId"
            class="config-input"
            placeholder="76561198000000000"
          />
          <p class="config-desc">自定义 Steam ID（留空使用随机生成的 ID）</p>
        </div>

        <!-- 在线状态 -->
        <div class="config-group">
          <label class="config-label">在线状态</label>
          <div class="status-options">
            <label class="radio-label">
              <input type="radio" v-model="config.personaState" value="0" />
              <span>离线</span>
            </label>
            <label class="radio-label">
              <input type="radio" v-model="config.personaState" value="1" />
              <span>在线</span>
            </label>
            <label class="radio-label">
              <input type="radio" v-model="config.personaState" value="2" />
              <span>忙碌</span>
            </label>
            <label class="radio-label">
              <input type="radio" v-model="config.personaState" value="3" />
              <span>离开</span>
            </label>
            <label class="radio-label">
              <input type="radio" v-model="config.personaState" value="4" />
              <span>隐身</span>
            </label>
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

interface UserConfig {
  username: string
  avatarPath: string
  language: string
  savesFolderName: string
  localSavePath: string
  steamId: string
  personaState: string
}

// ============================================
// 响应式状态
// ============================================

const config = ref<UserConfig>({
  username: 'Player',
  avatarPath: '',
  language: 'schinese',
  savesFolderName: '',
  localSavePath: '',
  steamId: '',
  personaState: '1'
})

// ============================================
// 方法
// ============================================

/**
 * 选择头像
 */
const selectAvatar = async () => {
  try {
    const result = await invoke<string | null>('select_image_file', {
      title: '选择用户头像'
    })
    if (result) {
      config.value.avatarPath = result
    }
  } catch (error) {
    console.error('选择文件失败:', error)
  }
}

/**
 * 选择本地存档路径
 */
const selectLocalSavePath = async () => {
  try {
    const result = await invoke<string | null>('select_folder', {
      title: '选择本地存档文件夹'
    })
    if (result) {
      config.value.localSavePath = result
    }
  } catch (error) {
    console.error('选择文件夹失败:', error)
  }
}

/**
 * 保存配置
 */
const saveConfig = async () => {
  try {
    const result = await invoke<{
      success: boolean
      message: string
    }>('save_user_config', {
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
      config?: UserConfig
    }>('load_user_config', {
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
  max-width: 500px;
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

/* 配置组 */
.config-group {
  margin-bottom: 20px;
}

.config-label {
  display: block;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 8px;
}

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

.config-desc {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 6px 0 0 0;
}

.config-path-hint {
  font-size: 11px;
  color: var(--text-secondary);
  margin: 4px 0 0 0;
  padding: 8px 12px;
  background-color: var(--bg-primary);
  border-radius: 6px;
  border-left: 3px solid var(--accent-color);
  font-family: 'Courier New', monospace;
  word-break: break-all;
}

/* 头像区域 */
.avatar-section {
  display: flex;
  align-items: center;
  gap: 16px;
}

.avatar-preview {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  overflow: hidden;
  cursor: pointer;
  border: 2px solid var(--border-color);
  transition: border-color 0.15s ease;
}

.avatar-preview:hover {
  border-color: var(--accent-color);
}

.avatar-preview img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.avatar-placeholder {
  width: 100%;
  height: 100%;
  background-color: var(--bg-primary);
  display: flex;
  align-items: center;
  justify-content: center;
}

.avatar-placeholder svg {
  width: 40px;
  height: 40px;
  color: var(--text-secondary);
}

.avatar-actions {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.file-btn {
  padding: 8px 16px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.file-btn:hover {
  border-color: var(--accent-color);
  color: var(--accent-color);
}

.remove-file-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  background-color: transparent;
  color: #ef4444;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.remove-file-btn:hover {
  background-color: rgba(239, 68, 68, 0.1);
}

/* 路径输入组 */
.path-input-group {
  display: flex;
  gap: 8px;
}

.path-input-group .config-input {
  flex: 1;
}

.browse-btn,
.clear-btn {
  padding: 10px 16px;
  border: none;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.browse-btn {
  background-color: var(--accent-color);
  color: white;
}

.browse-btn:hover {
  background-color: var(--accent-hover);
}

.clear-btn {
  background-color: var(--bg-surface);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.clear-btn:hover {
  background-color: var(--border-color);
}

/* 状态选项 */
.status-options {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
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
