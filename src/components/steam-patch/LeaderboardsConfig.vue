<template>
  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <div class="header-icon leaderboards">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M6 9H4.5a2.5 2.5 0 0 1 0-5H6"/>
            <path d="M18 9h1.5a2.5 2.5 0 0 0 0-5H18"/>
            <path d="M4 22h16"/>
            <path d="M10 14.66V17c0 .55-.47.98-.97 1.21C7.85 18.75 7 20.24 7 22"/>
            <path d="M14 14.66V17c0 .55.47.98.97 1.21C16.15 18.75 17 20.24 17 22"/>
            <path d="M18 2H6v7a6 6 0 0 0 12 0V2z"/>
          </svg>
        </div>
        <h3>排行榜配置</h3>
        <button class="close-btn" @click="$emit('close')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <div class="modal-body">
        <!-- 启用排行榜 -->
        <div class="config-item">
          <label class="toggle-label">
            <input type="checkbox" v-model="config.enabled" class="toggle-input" />
            <span class="toggle-slider"></span>
            <span class="toggle-text">启用排行榜系统</span>
          </label>
        </div>

        <!-- 默认行为说明 -->
        <div class="info-box">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <line x1="12" y1="16" x2="12" y2="12"/>
            <line x1="12" y1="8" x2="12.01" y2="8"/>
          </svg>
          <div class="info-content">
            <p>默认情况下，模拟器会假设所有排行榜都存在，并使用最常见的选项创建。</p>
            <p>如果游戏无法正常工作，您可以手动配置排行榜。</p>
          </div>
        </div>

        <!-- 排行榜列表 -->
        <div v-if="config.enabled" class="leaderboards-section">
          <div class="section-header">
            <label class="config-label">排行榜列表</label>
            <div class="header-actions">
              <button class="action-btn" @click="importFromSteamDB">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                  <polyline points="7 10 12 15 17 10"/>
                  <line x1="12" y1="15" x2="12" y2="3"/>
                </svg>
                从 SteamDB 导入
              </button>
              <button class="action-btn" @click="addLeaderboard">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="12" y1="5" x2="12" y2="19"/>
                  <line x1="5" y1="12" x2="19" y2="12"/>
                </svg>
                添加排行榜
              </button>
            </div>
          </div>

          <!-- 排行榜表格 -->
          <div class="leaderboards-table">
            <div class="table-header">
              <div class="col-name">排行榜名称</div>
              <div class="col-sort">排序方式</div>
              <div class="col-display">显示类型</div>
              <div class="col-actions">操作</div>
            </div>
            <div class="table-body">
              <div
                v-for="(leaderboard, index) in config.leaderboards"
                :key="index"
                class="leaderboard-row"
              >
                <div class="col-name">
                  <input
                    type="text"
                    v-model="leaderboard.name"
                    class="cell-input"
                    placeholder="排行榜名称"
                  />
                </div>
                <div class="col-sort">
                  <select v-model="leaderboard.sortMethod" class="cell-select">
                    <option :value="2">降序（分数高排名高）</option>
                    <option :value="1">升序（分数低排名高）</option>
                    <option :value="0">无</option>
                  </select>
                </div>
                <div class="col-display">
                  <select v-model="leaderboard.displayType" class="cell-select">
                    <option :value="1">数字</option>
                    <option :value="2">时间（秒）</option>
                    <option :value="3">时间（毫秒）</option>
                    <option :value="0">无</option>
                  </select>
                </div>
                <div class="col-actions">
                  <button class="remove-btn" @click="removeLeaderboard(index)">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <line x1="18" y1="6" x2="6" y2="18"/>
                      <line x1="6" y1="6" x2="18" y2="18"/>
                    </svg>
                  </button>
                </div>
              </div>
            </div>
          </div>

          <p v-if="config.leaderboards.length === 0" class="empty-hint">
            暂无排行榜定义，点击上方按钮添加
          </p>
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

interface Leaderboard {
  name: string
  sortMethod: number
  displayType: number
}

interface LeaderboardsConfig {
  enabled: boolean
  leaderboards: Leaderboard[]
}

// ============================================
// 响应式状态
// ============================================

const config = ref<LeaderboardsConfig>({
  enabled: true,
  leaderboards: []
})

// ============================================
// 方法
// ============================================

/**
 * 添加排行榜
 */
const addLeaderboard = () => {
  config.value.leaderboards.push({
    name: '',
    sortMethod: 2,
    displayType: 1
  })
}

/**
 * 移除排行榜
 */
const removeLeaderboard = (index: number) => {
  config.value.leaderboards.splice(index, 1)
}

/**
 * 从 SteamDB 导入
 */
const importFromSteamDB = async () => {
  alert('请访问 https://steamdb.info/app/' + props.gameId + '/leaderboards/ 查看排行榜列表，然后手动添加')
}

/**
 * 保存配置
 */
const saveConfig = async () => {
  try {
    // 过滤掉空的排行榜
    const validLeaderboards = config.value.leaderboards.filter(
      lb => lb.name.trim() !== ''
    )

    const result = await invoke<{
      success: boolean
      message: string
    }>('save_leaderboards_config', {
      gamePath: props.gamePath,
      config: {
        ...config.value,
        leaderboards: validLeaderboards
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
      config?: LeaderboardsConfig
    }>('load_leaderboards_config', {
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
  max-width: 700px;
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

.header-icon.leaderboards {
  background-color: rgba(236, 72, 153, 0.1);
  color: #ec4899;
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

/* 信息框 */
.info-box {
  display: flex;
  gap: 12px;
  padding: 12px 16px;
  background-color: rgba(59, 130, 246, 0.1);
  border-radius: 8px;
  margin-bottom: 20px;
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

/* 排行榜区域 */
.leaderboards-section {
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

/* 排行榜表格 */
.leaderboards-table {
  border: 1px solid var(--border-color);
  border-radius: 8px;
  overflow: hidden;
}

.table-header {
  display: grid;
  grid-template-columns: 2fr 1.5fr 1.5fr 60px;
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

.leaderboard-row {
  display: grid;
  grid-template-columns: 2fr 1.5fr 1.5fr 60px;
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
