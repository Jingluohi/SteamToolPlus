<template>
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
        <h3>游戏统计配置</h3>
        <button class="close-btn" @click="$emit('close')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <div class="modal-body">
        <!-- 启用统计 -->
        <div class="config-item">
          <label class="toggle-label">
            <input type="checkbox" v-model="config.enabled" class="toggle-input" />
            <span class="toggle-slider"></span>
            <span class="toggle-text">启用游戏统计系统</span>
          </label>
        </div>

        <!-- 允许未知统计 -->
        <div class="config-item">
          <label class="toggle-label">
            <input type="checkbox" v-model="config.allowUnknownStats" class="toggle-input" />
            <span class="toggle-slider"></span>
            <span class="toggle-text">允许未知统计（自动创建未定义的统计）</span>
          </label>
          <p class="config-hint">如果禁用，只有下方定义的统计才会被游戏识别</p>
        </div>

        <!-- 统计列表 -->
        <div v-if="config.enabled" class="stats-section">
          <div class="section-header">
            <label class="config-label">统计定义列表</label>
            <div class="header-actions">
              <button class="action-btn" @click="importFromSteamDB">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                  <polyline points="7 10 12 15 17 10"/>
                  <line x1="12" y1="15" x2="12" y2="3"/>
                </svg>
                从 SteamDB 导入
              </button>
              <button class="action-btn" @click="addStat">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="12" y1="5" x2="12" y2="19"/>
                  <line x1="5" y1="12" x2="19" y2="12"/>
                </svg>
                添加统计
              </button>
            </div>
          </div>

          <!-- 统计表格 -->
          <div class="stats-table">
            <div class="table-header">
              <div class="col-name">统计名称</div>
              <div class="col-type">类型</div>
              <div class="col-default">默认值</div>
              <div class="col-global">全局值</div>
              <div class="col-actions">操作</div>
            </div>
            <div class="table-body">
              <div
                v-for="(stat, index) in config.stats"
                :key="index"
                class="stat-row"
              >
                <div class="col-name">
                  <input
                    type="text"
                    v-model="stat.name"
                    class="cell-input"
                    placeholder="统计名称"
                  />
                </div>
                <div class="col-type">
                  <select v-model="stat.type" class="cell-select">
                    <option value="int">整数 (int)</option>
                    <option value="float">浮点数 (float)</option>
                    <option value="avgrate">平均值 (avgrate)</option>
                  </select>
                </div>
                <div class="col-default">
                  <input
                    type="number"
                    v-model.number="stat.default"
                    class="cell-input small"
                    placeholder="0"
                  />
                </div>
                <div class="col-global">
                  <input
                    type="number"
                    v-model.number="stat.global"
                    class="cell-input small"
                    placeholder="0"
                  />
                </div>
                <div class="col-actions">
                  <button class="remove-btn" @click="removeStat(index)">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <line x1="18" y1="6" x2="6" y2="18"/>
                      <line x1="6" y1="6" x2="18" y2="18"/>
                    </svg>
                  </button>
                </div>
              </div>
            </div>
          </div>

          <p v-if="config.stats.length === 0" class="empty-hint">
            暂无统计定义，点击上方按钮添加
          </p>
        </div>

        <!-- 说明 -->
        <div class="info-box">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <line x1="12" y1="16" x2="12" y2="12"/>
            <line x1="12" y1="8" x2="12.01" y2="8"/>
          </svg>
          <div class="info-content">
            <p><strong>统计类型说明：</strong></p>
            <p>• <strong>整数 (int)</strong>：整数值，如击杀数、收集物品数</p>
            <p>• <strong>浮点数 (float)</strong>：小数值，如游戏时间、距离</p>
            <p>• <strong>平均值 (avgrate)</strong>：平均值，如平均得分</p>
            <p><strong>全局值</strong>用于某些游戏的 DLC 解锁等功能</p>
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

interface Stat {
  name: string
  type: 'int' | 'float' | 'avgrate'
  default: number
  global: number
}

interface StatsConfig {
  enabled: boolean
  allowUnknownStats: boolean
  stats: Stat[]
}

// ============================================
// 响应式状态
// ============================================

const config = ref<StatsConfig>({
  enabled: true,
  allowUnknownStats: false,
  stats: []
})

// ============================================
// 方法
// ============================================

/**
 * 添加统计
 */
const addStat = () => {
  config.value.stats.push({
    name: '',
    type: 'int',
    default: 0,
    global: 0
  })
}

/**
 * 移除统计
 */
const removeStat = (index: number) => {
  config.value.stats.splice(index, 1)
}

/**
 * 从 SteamDB 导入
 */
const importFromSteamDB = async () => {
  alert('请访问 https://steamdb.info/app/' + props.gameId + '/stats/ 查看统计列表，然后手动添加')
}

/**
 * 保存配置
 */
const saveConfig = async () => {
  try {
    // 过滤掉空的统计
    const validStats = config.value.stats.filter(
      stat => stat.name.trim() !== ''
    )

    const result = await invoke<{
      success: boolean
      message: string
    }>('save_stats_config', {
      gamePath: props.gamePath,
      config: {
        ...config.value,
        stats: validStats
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
      config?: StatsConfig
    }>('load_stats_config', {
      gamePath: props.gamePath
    })

    if (result.exists && result.config) {
      config.value = result.config
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
  max-width: 750px;
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

.header-icon.stats {
  background-color: rgba(14, 165, 233, 0.1);
  color: #0ea5e9;
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
  margin-bottom: 16px;
}

.config-label {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.config-hint {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 6px 0 0 60px;
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
  background-color: var(--accent-color);
}

.toggle-input:checked + .toggle-slider::after {
  transform: translateX(22px);
}

.toggle-text {
  font-size: 14px;
  color: var(--text-primary);
}

/* 信息框 */
.info-box {
  display: flex;
  gap: 12px;
  padding: 12px 16px;
  background-color: rgba(59, 130, 246, 0.1);
  border-radius: 8px;
  margin-top: 20px;
}

.info-box svg {
  width: 20px;
  height: 20px;
  color: #3b82f6;
  flex-shrink: 0;
}

.info-content p {
  font-size: 13px;
  color: var(--text-primary);
  margin: 0 0 4px 0;
}

.info-content p:last-child {
  margin-bottom: 0;
}

/* 统计区域 */
.stats-section {
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

/* 统计表格 */
.stats-table {
  border: 1px solid var(--border-color);
  border-radius: 8px;
  overflow: hidden;
}

.table-header {
  display: grid;
  grid-template-columns: 2fr 1fr 80px 80px 60px;
  gap: 8px;
  padding: 10px 12px;
  background-color: var(--bg-primary);
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
}

.table-body {
  max-height: 300px;
  overflow-y: auto;
}

.stat-row {
  display: grid;
  grid-template-columns: 2fr 1fr 80px 80px 60px;
  gap: 8px;
  padding: 8px 12px;
  border-top: 1px solid var(--border-color);
  align-items: center;
}

.col-actions {
  display: flex;
  align-items: center;
  justify-content: center;
}

.cell-input {
  width: 100%;
  padding: 6px 10px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
}

.cell-input.small {
  padding: 6px 8px;
  text-align: center;
}

.cell-input:focus {
  border-color: var(--accent-color);
}

.cell-select {
  width: 100%;
  padding: 6px 10px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-size: 12px;
  outline: none;
  cursor: pointer;
}

.cell-select:focus {
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

.empty-hint {
  text-align: center;
  padding: 40px;
  color: var(--text-secondary);
  font-size: 14px;
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
