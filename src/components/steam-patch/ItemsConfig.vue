<template>
  <!-- 保存成功提示 - 放在弹窗外部，确保关闭弹窗后仍可见 -->
  <transition name="toast">
    <div v-if="showToast" class="toast-success">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
        <polyline points="22 4 12 14.01 9 11.01"/>
      </svg>
      <span>物品库存配置已保存成功！</span>
    </div>
  </transition>

  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <div class="header-icon items">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>
            <polyline points="3.27 6.96 12 12.01 20.73 6.96"/>
            <line x1="12" y1="22.08" x2="12" y2="12"/>
          </svg>
        </div>
        <h3>物品与库存配置</h3>
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
              <span class="guide-label">物品定义文件</span>
              <span class="guide-value">items.json</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">初始库存文件</span>
              <span class="guide-value">initial_items.txt</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">物品ID</span>
              <span class="guide-value">数字或字符串，如 1001、item_sword</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">初始库存格式</span>
              <span class="guide-value">物品ID=数量，每行一个</span>
            </div>
          </div>
          <div class="guide-example">
            <div class="example-title">初始库存示例：</div>
            <pre class="example-code">1001=5
1002=10
item_sword=1
item_health_potion=20</pre>
          </div>
        </div>

        <!-- 启用开关 -->
        <div class="config-group">
          <label class="toggle-label">
            <input v-model="config.enabled" type="checkbox" class="toggle-input" />
            <span class="toggle-slider"></span>
            <span class="toggle-text">启用物品与库存系统</span>
          </label>
        </div>

        <template v-if="config.enabled">
          <!-- 物品定义 -->
          <div class="config-section">
            <h4 class="section-title">物品定义</h4>
            <p class="config-desc">定义游戏中可用的所有物品及其属性</p>
            <textarea
              v-model="itemDefsText"
              class="config-textarea"
              rows="8"
              placeholder="每行一个物品定义（JSON格式）:&#10;{&quot;id&quot;: 1001, &quot;name&quot;: &quot;铁剑&quot;, &quot;stackable&quot;: true, &quot;maxStackSize&quot;: 99}&#10;{&quot;id&quot;: 1002, &quot;name&quot;: &quot;生命药水&quot;, &quot;stackable&quot;: true, &quot;maxStackSize&quot;: 999}"
            ></textarea>
            <p class="field-hint">每行一个 JSON 对象，包含 id、name、stackable、maxStackSize</p>
          </div>

          <!-- 初始库存 -->
          <div class="config-section">
            <h4 class="section-title">初始库存</h4>
            <p class="config-desc">配置游戏启动时的初始物品和数量</p>
            <textarea
              v-model="initialItemsText"
              class="config-textarea"
              rows="5"
              placeholder="格式: 物品ID=数量&#10;例如:&#10;1001=5&#10;1002=10&#10;item_sword=1"
            ></textarea>
            <p class="field-hint">每行一个物品，格式为 "物品ID=数量"</p>
          </div>
        </template>
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
import { ref, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps<{
  gamePath: string
  gameId: string
}>()

const emit = defineEmits<{
  close: []
  saved: []
}>()

const showToast = ref(false)

/**
 * 物品配置对象
 * 与 Rust ItemsConfig 结构体一致
 */
const config = ref({
  enabled: true,
  itemDefinitions: [] as Array<{
    id: number | string
    name: string
    stackable: boolean
    maxStackSize: number
  }>,
  initialItems: [] as Array<{
    itemId: string
    quantity: number
  }>,
})

/** 物品定义文本（用于 textarea 编辑） */
const itemDefsText = ref('')

/** 初始库存文本（用于 textarea 编辑） */
const initialItemsText = ref('')

/**
 * 将数组格式的物品定义转换为文本
 */
function syncItemDefsToText() {
  itemDefsText.value = config.value.itemDefinitions
    .map((d) => JSON.stringify(d))
    .join('\n')
}

/**
 * 将文本格式的物品定义转换为数组
 */
function syncTextToItemDefs() {
  const lines = itemDefsText.value
    .split('\n')
    .map((l) => l.trim())
    .filter((l) => l)

  config.value.itemDefinitions = []
  for (const line of lines) {
    try {
      const parsed = JSON.parse(line)
      if (parsed.id !== undefined) {
        config.value.itemDefinitions.push({
          id: parsed.id,
          name: parsed.name || '',
          stackable: parsed.stackable ?? true,
          maxStackSize: parsed.maxStackSize ?? 99,
        })
      }
    } catch {
      // 跳过无效行
    }
  }
}

/**
 * 将数组格式的初始库存转换为文本
 */
function syncInitialItemsToText() {
  initialItemsText.value = config.value.initialItems
    .map((i) => `${i.itemId}=${i.quantity}`)
    .join('\n')
}

/**
 * 将文本格式的初始库存转换为数组
 */
function syncTextToInitialItems() {
  const lines = initialItemsText.value
    .split('\n')
    .map((l) => l.trim())
    .filter((l) => l && l.includes('='))

  config.value.initialItems = lines.map((line) => {
    const idx = line.indexOf('=')
    return {
      itemId: line.slice(0, idx),
      quantity: parseInt(line.slice(idx + 1), 10) || 1,
    }
  })
}

/** 监听文本变化同步到数组 */
watch(itemDefsText, syncTextToItemDefs)
watch(initialItemsText, syncTextToInitialItems)

/**
 * 保存配置
 */
async function saveConfig() {
  // 确保文本已同步到数组
  syncTextToItemDefs()
  syncTextToInitialItems()

  try {
    const result = await invoke<{ success: boolean; message: string }>('save_items_config', {
      gamePath: props.gamePath,
      config: config.value,
    })

    if (result.success) {
      showToast.value = true
      setTimeout(() => {
        showToast.value = false
      }, 3000)
      emit('saved')
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

/**
 * 加载现有配置
 */
async function loadConfig() {
  try {
    const result = await invoke<{
      exists: boolean
      config?: any
    }>('load_items_config', {
      gamePath: props.gamePath,
    })

    if (result.exists && result.config) {
      config.value.enabled = result.config.enabled ?? true

      // 加载物品定义
      if (result.config.itemDefinitions && result.config.itemDefinitions.length > 0) {
        config.value.itemDefinitions = result.config.itemDefinitions
        syncItemDefsToText()
      }

      // 加载初始库存
      if (result.config.initialItems && result.config.initialItems.length > 0) {
        config.value.initialItems = result.config.initialItems
        syncInitialItemsToText()
      }
    }
  } catch (error) {
    console.error('加载物品配置失败:', error)
  }
}

onMounted(() => {
  loadConfig()
})
</script>

<style scoped>
/* 复用与 LeaderboardsConfig 相同的样式 */
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
  max-width: 650px;
  max-height: 85vh;
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

.header-icon.items {
  background-color: rgba(16, 185, 129, 0.1);
  color: #10b981;
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

.config-section {
  margin-bottom: 24px;
  padding-bottom: 20px;
  border-bottom: 1px solid var(--steam-border);
}

.config-section:last-of-type {
  border-bottom: none;
  margin-bottom: 0;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 8px 0;
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
  font-family: 'Consolas', 'Courier New', monospace;
  resize: vertical;
  outline: none;
}

.config-textarea:focus {
  border-color: var(--steam-accent-blue);
}

.field-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 4px 0 0 0;
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
