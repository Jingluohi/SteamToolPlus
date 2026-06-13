<template>
  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <div class="header-icon dlc">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
            <line x1="12" y1="8" x2="12" y2="16"/>
            <line x1="8" y1="12" x2="16" y2="12"/>
          </svg>
        </div>
        <h3>DLC 与 Depot 配置</h3>
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
              <span class="guide-label">DLC 配置文件</span>
              <span class="guide-value">dlc.json / dlcs.txt</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">DLC ID</span>
              <span class="guide-value">纯数字，如 123456</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">获取方式</span>
              <span class="guide-value">在 SteamDB 上搜索游戏查看 DLC 列表</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">输入格式</span>
              <span class="guide-value">每行一个 DLC ID，或勾选"解锁所有"</span>
            </div>
          </div>
          <div class="guide-example">
            <div class="example-title">DLC 列表示例：</div>
            <pre class="example-code">123456
789012
345678</pre>
          </div>
        </div>

        <div class="config-group">
          <label class="toggle-label">
            <input v-model="config.unlockAll" type="checkbox" class="toggle-input" />
            <span class="toggle-slider"></span>
            <span class="toggle-text">解锁所有 DLC</span>
          </label>
          <p class="field-hint">部分游戏不适用"解锁所有"，可手动指定需要解锁的 DLC</p>
        </div>

        <div v-if="!config.unlockAll" class="config-group">
          <label class="config-label">DLC 列表</label>
          <p class="config-desc">输入要解锁的 DLC ID（每行一个纯数字）</p>
          <textarea
            v-model="config.dlcList"
            class="config-textarea"
            rows="8"
            placeholder="例如:&#10;123456&#10;789012"
          ></textarea>
          <p class="field-hint">在 SteamDB 搜索游戏可查看所有 DLC ID</p>
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
  unlockAll: true,
  dlcList: ''
})

async function saveConfig() {
  try {
    const result = await invoke<{ success: boolean; message: string }>('save_dlc_config', {
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

.header-icon.dlc {
  background-color: rgba(168, 85, 247, 0.1);
  color: #a855f7;
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
  margin: 6px 0 0 0;
}
</style>
