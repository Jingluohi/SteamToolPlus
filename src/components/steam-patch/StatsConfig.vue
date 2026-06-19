<template>
  <!-- 保存成功提示 - 放在弹窗外部，确保关闭弹窗后仍可见 -->
  <transition name="toast">
    <div v-if="showToast" class="toast-success">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
        <polyline points="22 4 12 14.01 9 11.01"/>
      </svg>
      <span>统计数据配置已保存成功！</span>
    </div>
  </transition>

  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <div class="header-icon stats">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="20" x2="18" y2="10"/>
            <line x1="12" y1="20" x2="12" y2="4"/>
            <line x1="6" y1="20" x2="6" y2="14"/>
          </svg>
        </div>
        <h3>统计数据配置</h3>
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
              <span class="guide-label">统计数据文件</span>
              <span class="guide-value">stats.json</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">统计类型</span>
              <span class="guide-value">int / float / avgrate</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">统计名称</span>
              <span class="guide-value">英文字母+下划线，如 kills、deaths</span>
            </div>
            <div class="guide-item">
              <span class="guide-label">默认值</span>
              <span class="guide-value">整数填 0，浮点数填 0.0</span>
            </div>
          </div>
          <div class="guide-example">
            <div class="example-title">JSON 示例：</div>
            <pre class="example-code">[
  {
    "name": "kills",
    "type": "int",
    "defaultValue": 0
  },
  {
    "name": "playtime",
    "type": "float",
    "defaultValue": 0.0,
    "minValue": 0.0,
    "maxValue": 99999.0
  },
  {
    "name": "accuracy",
    "type": "avgrate",
    "defaultValue": 0.0,
    "windowSize": 100
  }
]</pre>
          </div>
        </div>

        <!-- 启用开关 -->
        <div class="config-section">
          <label class="toggle-label">
            <input v-model="config.enabled" type="checkbox" class="toggle-input" />
            <span class="toggle-slider"></span>
            <span class="toggle-text">启用统计系统</span>
          </label>
        </div>

        <template v-if="config.enabled">
          <!-- 统计列表 -->
          <div class="config-section">
            <div class="section-header">
              <h4 class="section-title">统计项列表</h4>
              <button class="btn-add" @click="addStat">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="12" y1="5" x2="12" y2="19"/>
                  <line x1="5" y1="12" x2="19" y2="12"/>
                </svg>
                添加统计项
              </button>
            </div>

            <div class="stats-list">
              <div 
                v-for="(stat, index) in config.stats" 
                :key="index"
                class="stat-card"
                :class="{ expanded: expandedIndex === index }"
              >
                <div class="stat-header" @click="toggleExpand(index)">
                  <div class="stat-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <line x1="18" y1="20" x2="18" y2="10"/>
                      <line x1="12" y1="20" x2="12" y2="4"/>
                      <line x1="6" y1="20" x2="6" y2="14"/>
                    </svg>
                  </div>
                  <div class="stat-info">
                    <span class="stat-name">{{ stat.name || '未命名统计' }}</span>
                    <span class="stat-type">{{ getStatTypeLabel(stat.type) }}</span>
                  </div>
                  <div class="stat-actions">
                    <button class="btn-icon" @click.stop="removeStat(index)">
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <line x1="18" y1="6" x2="6" y2="18"/>
                        <line x1="6" y1="6" x2="18" y2="18"/>
                      </svg>
                    </button>
                  </div>
                </div>

                <div v-if="expandedIndex === index" class="stat-details">
                  <div class="form-row">
                    <div class="form-group">
                      <label>统计名称</label>
                      <input 
                        v-model="stat.name" 
                        type="text" 
                        placeholder="例如：kills"
                      />
                    </div>
                    <div class="form-group">
                      <label>统计类型</label>
                      <select v-model="stat.type">
                        <option value="int">整数 (int)</option>
                        <option value="float">浮点数 (float)</option>
                        <option value="avgrate">平均值 (avgrate)</option>
                      </select>
                      <p class="field-hint">int=计数, float=小数, avgrate=速率平均</p>
                    </div>
                  </div>

                  <div class="form-row">
                    <div class="form-group">
                      <label>默认值</label>
                      <input 
                        v-model.number="stat.defaultValue" 
                        type="number" 
                        step="0.01"
                        placeholder="0"
                      />
                    </div>
                  </div>

                  <template v-if="stat.type === 'float'">
                    <div class="form-row">
                      <div class="form-group">
                        <label>最小值（可选）</label>
                        <input v-model.number="stat.minValue" type="number" step="0.01" placeholder="无限制" />
                        <p class="field-hint">不填则无最小值限制</p>
                      </div>
                      <div class="form-group">
                        <label>最大值（可选）</label>
                        <input v-model.number="stat.maxValue" type="number" step="0.01" placeholder="无限制" />
                        <p class="field-hint">不填则无最大值限制</p>
                      </div>
                    </div>
                  </template>

                  <template v-if="stat.type === 'avgrate'">
                    <div class="form-row">
                      <div class="form-group">
                        <label>窗口大小</label>
                        <input v-model.number="stat.windowSize" type="number" placeholder="100" />
                        <p class="field-hint">计算平均值的样本窗口大小</p>
                      </div>
                    </div>
                  </template>
                </div>
              </div>

              <div v-if="config.stats.length === 0" class="empty-state">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="18" y1="20" x2="18" y2="10"/>
                  <line x1="12" y1="20" x2="12" y2="4"/>
                  <line x1="6" y1="20" x2="6" y2="14"/>
                </svg>
                <p>暂无统计项，点击上方按钮添加</p>
              </div>
            </div>
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

  <!-- 保存成功提示 -->
  <transition name="toast">
    <div v-if="showToast" class="toast-success">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
        <polyline points="22 4 12 14.01 9 11.01"/>
      </svg>
      <span>统计数据配置已保存成功！</span>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { StatsConfig, StatItem } from '../../../src/types/steam-config.types'

const props = defineProps<{
  gamePath: string
}>()

const emit = defineEmits<{
  close: []
  saved: []
}>()

const showToast = ref(false)

const config = ref<StatsConfig>({
  enabled: true,
  stats: []
})

const expandedIndex = ref<number | null>(null)

const statTypeLabels: Record<string, string> = {
  int: '整数',
  float: '浮点数',
  avgrate: '平均值',
  achievements: '成就进度'
}

function getStatTypeLabel(type: string): string {
  return statTypeLabels[type] || type
}

function toggleExpand(index: number) {
  expandedIndex.value = expandedIndex.value === index ? null : index
}

function addStat() {
  const newStat: StatItem = {
    name: '',
    type: 'int',
    defaultValue: 0
  }
  config.value.stats.push(newStat)
  expandedIndex.value = config.value.stats.length - 1
}

function removeStat(index: number) {
  if (confirm('确定要删除这个统计项吗？')) {
    config.value.stats.splice(index, 1)
    if (expandedIndex.value === index) {
      expandedIndex.value = null
    }
  }
}

async function saveConfig() {
  const invalidStats = config.value.stats.filter(s => !s.name)
  if (invalidStats.length > 0) {
    alert('请为所有统计项填写名称')
    return
  }

  try {
    const result = await invoke<{ success: boolean; message: string }>('save_stats_config', {
      gamePath: props.gamePath,
      config: config.value
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

async function loadConfig() {
  try {
    const result = await invoke<{ 
      exists: boolean
      config?: StatsConfig 
    }>('load_stats_config', {
      gamePath: props.gamePath
    })

    if (result.exists && result.config) {
      config.value = { ...config.value, ...result.config }
    }
  } catch (error) {
    // 加载失败时使用默认值
  }
}

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
  background-color: var(--steam-bg-primary);
  border-radius: 12px;
  border: 1px solid var(--steam-border);
  width: 90%;
  max-width: 700px;
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

.header-icon.stats {
  background-color: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
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
  font-weight: 500;
  color: var(--steam-text-primary);
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
  width: 14px;
  height: 14px;
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
}

.btn-icon:hover {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.btn-icon svg {
  width: 16px;
  height: 16px;
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

.stats-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.stat-card {
  background-color: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 10px;
  overflow: hidden;
  transition: all 0.15s ease;
}

.stat-card.expanded {
  border-color: var(--steam-accent-blue);
}

.stat-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.stat-header:hover {
  background-color: var(--steam-bg-tertiary);
}

.stat-icon {
  width: 36px;
  height: 36px;
  border-radius: 8px;
  background-color: var(--steam-bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.stat-icon svg {
  width: 20px;
  height: 20px;
  color: var(--steam-text-secondary);
}

.stat-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.stat-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

.stat-type {
  font-size: 12px;
  color: var(--steam-text-secondary);
}

.stat-details {
  padding: 16px;
  border-top: 1px solid var(--steam-border);
  background-color: var(--steam-bg-primary);
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
  margin-bottom: 16px;
}

.form-row:last-child {
  margin-bottom: 0;
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
.form-group select {
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color 0.15s ease;
}

.form-group input:focus,
.form-group select:focus {
  border-color: var(--steam-accent-blue);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px;
  color: var(--steam-text-secondary);
  gap: 12px;
}

.empty-state svg {
  width: 48px;
  height: 48px;
  opacity: 0.5;
}

.empty-state p {
  margin: 0;
  font-size: 14px;
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

.field-hint {
  font-size: 11px;
  color: var(--steam-text-secondary);
  margin: 4px 0 0 0;
}

@media (max-width: 600px) {
  .form-row {
    grid-template-columns: 1fr;
  }
  
  .section-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
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