<template>
  <div class="mods-config-panel">
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
          <span class="guide-label">模组定义文件</span>
          <span class="guide-value">mods.json</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">模组目录结构</span>
          <span class="guide-value">steam_settings/mods/&lt;MOD编号&gt;/</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">模组ID</span>
          <span class="guide-value">Steam 创意工坊文件 ID（纯数字）</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">可见性</span>
          <span class="guide-value">public / friends / private</span>
        </div>
      </div>
      <div class="guide-example">
        <div class="example-title">JSON 示例：</div>
        <pre class="example-code">[
  {
    "publishedFileId": "123456789",
    "title": "My Awesome Mod",
    "visibility": "public"
  }
]</pre>
      </div>
      <p class="guide-tip">提示：在 Steam 创意工坊页面 URL 中即可查看模组文件 ID</p>
    </div>

    <!-- 启用开关 -->
    <div class="config-group">
      <label class="toggle-label">
        <input v-model="config.enabled" type="checkbox" class="toggle-input" />
        <span class="toggle-slider"></span>
        <span class="toggle-text">启用创意工坊模组</span>
      </label>
    </div>

    <template v-if="config.enabled">
      <!-- 模组列表 -->
      <div class="config-section">
        <div class="section-header">
          <h4 class="section-title">已订阅模组</h4>
          <button class="btn-add" @click="addMod">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="12" y1="5" x2="12" y2="19"/>
              <line x1="5" y1="12" x2="19" y2="12"/>
            </svg>
            添加模组
          </button>
        </div>

        <div class="list-container">
          <div
            v-for="(mod, index) in config.subscribedMods"
            :key="index"
            class="list-item expandable"
          >
            <div class="item-header" @click="toggleExpand(index)">
              <span class="item-title">{{ mod.title || '未命名模组' }}</span>
              <span class="item-badge">{{ mod.visibility }}</span>
              <button class="btn-icon" @click.stop="removeMod(index)">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="18" y1="6" x2="6" y2="18"/>
                  <line x1="6" y1="6" x2="18" y2="18"/>
                </svg>
              </button>
            </div>
            <div v-if="expandedItems[index]" class="item-body">
              <div class="form-group">
                <label>模组ID</label>
                <input
                  v-model="mod.publishedFileId"
                  type="text"
                  placeholder="Steam 创意工坊文件 ID"
                />
              </div>
              <div class="form-group">
                <label>模组标题</label>
                <input
                  v-model="mod.title"
                  type="text"
                  placeholder="模组标题"
                />
              </div>
              <div class="form-group">
                <label>模组描述</label>
                <textarea
                  v-model="mod.description"
                  rows="2"
                  placeholder="模组描述（可选）"
                ></textarea>
              </div>
              <div class="form-group">
                <label>可见性</label>
                <select v-model="mod.visibility">
                  <option value="public">公开</option>
                  <option value="friends">好友</option>
                  <option value="private">私有</option>
                </select>
              </div>
            </div>
          </div>
          <div v-if="config.subscribedMods.length === 0" class="empty-state">
            <p>暂无模组配置，点击"添加模组"开始配置</p>
          </div>
        </div>
      </div>

      <!-- 自动更新 -->
      <div class="config-group">
        <label class="checkbox-label">
          <input v-model="config.autoUpdate" type="checkbox" />
          <span>自动更新模组</span>
        </label>
      </div>
    </template>

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
        <span>模组配置已保存成功！</span>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
/**
 * ModsConfigPanel.vue - 创意工坊模组配置统一 Panel
 * 供单独弹窗和完整配置管理器复用
 */

import { shallowReactive, ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { CONFIG_EVENTS } from '../../../constants/config-events'
import type { ModsConfig, WorkshopMod } from '../../../types/steam-config.types'

const props = defineProps<{
  gamePath: string
}>()

const emit = defineEmits<{
  saved: []
}>()

const showToast = ref(false)

/**
 * 模组配置对象
 * 与 Rust ModsConfig 结构体一致。
 * 使用 shallowReactive 减少深层响应式代理开销；
 * 数组/对象通过不可变更新替换引用。
 */
const config = shallowReactive<ModsConfig>({
  enabled: true,
  subscribedMods: [],
  autoUpdate: false,
})

/** 展开状态索引 */
const expandedItems = ref<Record<number, boolean>>({})

/**
 * 默认空模组对象
 * 使用常量避免重复创建
 */
const EMPTY_MOD: WorkshopMod = {
  publishedFileId: '',
  title: '',
  description: '',
  visibility: 'public',
  files: [],
}

/**
 * 添加模组
 */
function addMod() {
  config.subscribedMods = [...config.subscribedMods, { ...EMPTY_MOD }]
  // 新添加的项默认展开
  expandedItems.value[config.subscribedMods.length - 1] = true
}

/**
 * 删除模组
 */
function removeMod(index: number) {
  config.subscribedMods = config.subscribedMods.filter((_, i) => i !== index)
  delete expandedItems.value[index]
}

/**
 * 切换模组展开/收起状态
 */
function toggleExpand(index: number) {
  expandedItems.value[index] = !expandedItems.value[index]
}

/**
 * 保存配置
 */
async function saveConfig() {
  try {
    const result = await invoke<{ success: boolean; message: string }>('save_mods_config', {
      gamePath: props.gamePath,
      config,
    })

    if (result.success) {
      showToast.value = true
      setTimeout(() => {
        showToast.value = false
      }, 3000)
      emit('saved')
      // 广播模组配置已保存事件，通知完整配置管理器等其它窗口刷新
      window.dispatchEvent(new CustomEvent(CONFIG_EVENTS.MODS_SAVED, {
        detail: { gamePath: props.gamePath }
      }))
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
async function loadConfig() {
  try {
    const result = await invoke<{
      exists: boolean
      config?: ModsConfig
    }>('load_mods_config', {
      gamePath: props.gamePath,
    })

    if (result.exists && result.config) {
      config.enabled = result.config.enabled ?? true
      config.autoUpdate = result.config.autoUpdate ?? false
      config.subscribedMods = Array.isArray(result.config.subscribedMods)
        ? result.config.subscribedMods
        : []
    }
  } catch (error) {
    // 加载失败时使用默认值
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
  // 监听模组配置保存事件，与完整配置管理器实时同步
  window.addEventListener(CONFIG_EVENTS.MODS_SAVED, onConfigSavedEvent)
})

onUnmounted(() => {
  window.removeEventListener(CONFIG_EVENTS.MODS_SAVED, onConfigSavedEvent)
})

defineExpose({
  load: loadConfig,
  save: saveConfig
})
</script>

<style scoped>
.mods-config-panel {
  width: 100%;
}

.config-group {
  margin-bottom: 20px;
}

.config-section {
  margin-bottom: 24px;
  padding-bottom: 20px;
  border-bottom: 1px solid var(--steam-border);
}

.config-section:last-of-type {
  border-bottom: none;
  margin-bottom: 0;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0;
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

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 14px;
  color: var(--steam-text-primary);
}

.checkbox-label input[type="checkbox"] {
  width: 18px;
  height: 18px;
  accent-color: var(--steam-accent-blue);
  flex-shrink: 0;
  margin: 0;
}

.btn-add {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  background-color: var(--steam-accent-blue);
  color: white;
}

.btn-add:hover {
  background-color: var(--steam-accent-hover);
}

.btn-add svg {
  width: 16px;
  height: 16px;
}

.list-container {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.list-item {
  background-color: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  padding: 12px;
}

.list-item.expandable {
  display: flex;
  flex-direction: column;
}

.item-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  cursor: pointer;
  gap: 8px;
}

.item-title {
  font-weight: 500;
  color: var(--steam-text-primary);
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.item-badge {
  font-size: 11px;
  font-weight: 500;
  color: var(--steam-text-secondary);
  background-color: var(--steam-bg-tertiary);
  padding: 2px 8px;
  border-radius: 4px;
  text-transform: capitalize;
  flex-shrink: 0;
}

.btn-icon {
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 6px;
  background-color: transparent;
  color: var(--steam-text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
  flex-shrink: 0;
}

.btn-icon:hover {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.btn-icon svg {
  width: 16px;
  height: 16px;
}

.item-body {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--steam-border);
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-group label {
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

.form-group input,
.form-group select,
.form-group textarea {
  padding: 8px 10px;
  border: 1px solid var(--steam-border);
  border-radius: 6px;
  background-color: var(--steam-bg-primary);
  color: var(--steam-text-primary);
  font-size: 13px;
  outline: none;
  box-sizing: border-box;
}

.form-group input:focus,
.form-group select:focus,
.form-group textarea:focus {
  border-color: var(--steam-accent-blue);
}

.empty-state {
  text-align: center;
  padding: 32px 16px;
  color: var(--steam-text-secondary);
  font-size: 13px;
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

.panel-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 24px;
  padding-top: 20px;
  border-top: 1px solid var(--steam-border);
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
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 8px 0 0 0;
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
