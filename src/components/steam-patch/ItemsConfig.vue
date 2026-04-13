<template>
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
        <!-- 启用物品系统 -->
        <div class="config-item">
          <label class="toggle-label">
            <input type="checkbox" v-model="config.enabled" class="toggle-input" />
            <span class="toggle-slider"></span>
            <span class="toggle-text">启用物品与库存系统</span>
          </label>
        </div>

        <!-- 物品定义 -->
        <div v-if="config.enabled" class="items-section">
          <div class="section-header">
            <label class="config-label">物品定义</label>
            <div class="header-actions">
              <button class="action-btn" @click="importFromSteamDB">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                  <polyline points="7 10 12 15 17 10"/>
                  <line x1="12" y1="15" x2="12" y2="3"/>
                </svg>
                从 SteamDB 导入
              </button>
              <button class="action-btn" @click="addItem">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="12" y1="5" x2="12" y2="19"/>
                  <line x1="5" y1="12" x2="19" y2="12"/>
                </svg>
                添加物品
              </button>
            </div>
          </div>

          <!-- 物品列表 -->
          <div class="items-list">
            <div
              v-for="(item, index) in config.items"
              :key="index"
              class="item-card"
            >
              <div class="item-header">
                <input
                  type="text"
                  v-model="item.name"
                  class="item-name-input"
                  placeholder="物品名称"
                />
                <button class="remove-btn" @click="removeItem(index)">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="18" y1="6" x2="6" y2="18"/>
                    <line x1="6" y1="6" x2="18" y2="18"/>
                  </svg>
                </button>
              </div>
              <div class="item-fields">
                <div class="field-row">
                  <label>Item ID:</label>
                  <input
                    type="number"
                    v-model.number="item.itemId"
                    class="field-input"
                    placeholder="物品ID"
                  />
                </div>
                <div class="field-row">
                  <label>数量:</label>
                  <input
                    type="number"
                    v-model.number="item.quantity"
                    class="field-input"
                    placeholder="初始数量"
                    min="0"
                  />
                </div>
                <div class="field-row">
                  <label>属性:</label>
                  <textarea
                    v-model="item.attributes"
                    class="field-textarea"
                    rows="2"
                    placeholder='JSON格式属性，例如: {&quot;quality&quot;: 100, &quot;level&quot;: 5}'
                  ></textarea>
                </div>
              </div>
            </div>
          </div>

          <p v-if="config.items.length === 0" class="empty-hint">
            暂无物品定义，点击上方按钮添加
          </p>
        </div>

        <!-- 初始库存 -->
        <div v-if="config.enabled" class="config-group">
          <label class="config-label">初始库存设置</label>
          <p class="config-desc">设置玩家初始拥有的物品和数量</p>
          <div class="inventory-options">
            <label class="checkbox-label">
              <input type="checkbox" v-model="config.giveAllItems" />
              <span>给予所有定义的物品（使用上方定义的数量）</span>
            </label>
            <label class="checkbox-label">
              <input type="checkbox" v-model="config.clearOnStart" />
              <span>游戏启动时清空库存</span>
            </label>
          </div>
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

interface Item {
  name: string
  itemId: number
  quantity: number
  attributes: string
}

interface ItemsConfig {
  enabled: boolean
  items: Item[]
  giveAllItems: boolean
  clearOnStart: boolean
}

// ============================================
// 响应式状态
// ============================================

const config = ref<ItemsConfig>({
  enabled: true,
  items: [],
  giveAllItems: false,
  clearOnStart: false
})

// ============================================
// 方法
// ============================================

/**
 * 添加物品
 */
const addItem = () => {
  config.value.items.push({
    name: '',
    itemId: 0,
    quantity: 1,
    attributes: ''
  })
}

/**
 * 移除物品
 */
const removeItem = (index: number) => {
  config.value.items.splice(index, 1)
}

/**
 * 从 SteamDB 导入
 */
const importFromSteamDB = async () => {
  alert('请访问 https://steamdb.info/app/' + props.gameId + '/items/ 查看物品列表，然后手动添加')
}

/**
 * 保存配置
 */
const saveConfig = async () => {
  try {
    // 验证并解析物品属性
    const validItems = config.value.items
      .filter(item => item.name.trim() !== '' && item.itemId > 0)
      .map(item => {
        let attributes = {}
        if (item.attributes.trim()) {
          try {
            attributes = JSON.parse(item.attributes)
          } catch (e) {
            // 如果解析失败，使用空对象
            attributes = {}
          }
        }
        return {
          ...item,
          attributes
        }
      })

    const result = await invoke<{
      success: boolean
      message: string
    }>('save_items_config', {
      gamePath: props.gamePath,
      config: {
        ...config.value,
        items: validItems
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
      config?: ItemsConfig
    }>('load_items_config', {
      gamePath: props.gamePath
    })

    if (result.exists && result.config) {
      // 将属性对象转换回字符串
      const items = result.config.items.map(item => ({
        ...item,
        attributes: typeof item.attributes === 'object'
          ? JSON.stringify(item.attributes, null, 2)
          : String(item.attributes)
      }))
      config.value = { ...result.config, items }
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
  margin-bottom: 20px;
}

.config-label {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.config-desc {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 4px 0 12px 0;
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

/* 物品区域 */
.items-section {
  margin-bottom: 20px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.action-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.action-btn:hover {
  border-color: var(--accent-color);
  color: var(--accent-color);
}

.action-btn svg {
  width: 14px;
  height: 14px;
}

/* 物品列表 */
.items-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  max-height: 300px;
  overflow-y: auto;
}

.item-card {
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 12px;
}

.item-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.item-name-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 14px;
  font-weight: 500;
  outline: none;
}

.item-name-input:focus {
  border-color: var(--accent-color);
}

.remove-btn {
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 6px;
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
  width: 14px;
  height: 14px;
}

.item-fields {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.field-row {
  display: flex;
  align-items: center;
  gap: 10px;
}

.field-row label {
  width: 60px;
  font-size: 12px;
  color: var(--text-secondary);
  flex-shrink: 0;
}

.field-input {
  flex: 1;
  padding: 6px 10px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
}

.field-input:focus {
  border-color: var(--accent-color);
}

.field-textarea {
  flex: 1;
  padding: 6px 10px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 12px;
  font-family: 'Courier New', monospace;
  resize: vertical;
  outline: none;
}

.field-textarea:focus {
  border-color: var(--accent-color);
}

.empty-hint {
  text-align: center;
  padding: 40px;
  color: var(--text-secondary);
  font-size: 14px;
}

/* 库存选项 */
.inventory-options {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  font-size: 14px;
  color: var(--text-primary);
}

.checkbox-label input[type="checkbox"] {
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
