<template>
  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <div class="header-icon achievements">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="8" r="7"/>
            <polyline points="8.21 13.89 7 23 12 20 17 23 15.79 13.88"/>
          </svg>
        </div>
        <h3>成就系统配置</h3>
        <button class="close-btn" @click="$emit('close')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <div class="modal-body">
        <!-- 启用成就系统 -->
        <div class="config-item">
          <label class="toggle-label">
            <input type="checkbox" v-model="config.enabled" class="toggle-input" />
            <span class="toggle-slider"></span>
            <span class="toggle-text">启用成就系统</span>
          </label>
        </div>

        <!-- 成就列表 -->
        <div v-if="config.enabled" class="achievements-section">
          <div class="section-header">
            <label class="config-label">成就列表</label>
            <div class="header-actions">
              <button class="action-btn" @click="importFromSteamDB">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                  <polyline points="7 10 12 15 17 10"/>
                  <line x1="12" y1="15" x2="12" y2="3"/>
                </svg>
                从 SteamDB 导入
              </button>
              <button class="action-btn" @click="addAchievement">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="12" y1="5" x2="12" y2="19"/>
                  <line x1="5" y1="12" x2="19" y2="12"/>
                </svg>
                添加成就
              </button>
            </div>
          </div>

          <!-- 成就表格 -->
          <div class="achievements-table">
            <div class="table-header">
              <div class="col-icon">图标</div>
              <div class="col-name">名称</div>
              <div class="col-description">描述</div>
              <div class="col-hidden">隐藏</div>
              <div class="col-actions">操作</div>
            </div>
            <div class="table-body">
              <div
                v-for="(achievement, index) in config.achievements"
                :key="index"
                class="achievement-row"
              >
                <div class="col-icon">
                  <div class="icon-placeholder" @click="selectAchievementIcon(index)">
                    <img v-if="achievement.icon" :src="achievement.icon" alt="" />
                    <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
                      <circle cx="8.5" cy="8.5" r="1.5"/>
                      <polyline points="21 15 16 10 5 21"/>
                    </svg>
                  </div>
                </div>
                <div class="col-name">
                  <input
                    type="text"
                    v-model="achievement.name"
                    class="cell-input"
                    placeholder="成就名称"
                  />
                </div>
                <div class="col-description">
                  <input
                    type="text"
                    v-model="achievement.description"
                    class="cell-input"
                    placeholder="成就描述"
                  />
                </div>
                <div class="col-hidden">
                  <input type="checkbox" v-model="achievement.hidden" />
                </div>
                <div class="col-actions">
                  <button class="remove-btn" @click="removeAchievement(index)">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <line x1="18" y1="6" x2="6" y2="18"/>
                      <line x1="6" y1="6" x2="18" y2="18"/>
                    </svg>
                  </button>
                </div>
              </div>
            </div>
          </div>

          <p v-if="config.achievements.length === 0" class="empty-hint">
            暂无成就，点击上方按钮添加
          </p>
        </div>

        <!-- 解锁通知设置 -->
        <div v-if="config.enabled" class="config-group">
          <label class="config-label">解锁通知设置</label>
          <div class="notification-options">
            <label class="checkbox-label">
              <input type="checkbox" v-model="config.showNotification" />
              <span>显示解锁通知</span>
            </label>
            <label class="checkbox-label">
              <input type="checkbox" v-model="config.playSound" />
              <span>播放解锁音效</span>
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

interface Achievement {
  name: string
  description: string
  hidden: boolean
  icon: string
}

interface AchievementsConfig {
  enabled: boolean
  achievements: Achievement[]
  showNotification: boolean
  playSound: boolean
}

// ============================================
// 响应式状态
// ============================================

const config = ref<AchievementsConfig>({
  enabled: true,
  achievements: [],
  showNotification: true,
  playSound: true
})

// ============================================
// 方法
// ============================================

/**
 * 添加成就
 */
const addAchievement = () => {
  config.value.achievements.push({
    name: '',
    description: '',
    hidden: false,
    icon: ''
  })
}

/**
 * 移除成就
 */
const removeAchievement = (index: number) => {
  config.value.achievements.splice(index, 1)
}

/**
 * 选择成就图标
 */
const selectAchievementIcon = async (index: number) => {
  try {
    const result = await invoke<string | null>('select_image_file', {
      title: '选择成就图标'
    })
    if (result) {
      config.value.achievements[index].icon = result
    }
  } catch (error) {
    console.error('选择文件失败:', error)
  }
}

/**
 * 从 SteamDB 导入
 */
const importFromSteamDB = async () => {
  // 这里可以实现从 SteamDB 获取成就数据的功能
  alert('请访问 https://steamdb.info/app/' + props.gameId + '/stats/ 查看成就列表，然后手动添加')
}

/**
 * 保存配置
 */
const saveConfig = async () => {
  try {
    // 过滤掉空的成就
    const validAchievements = config.value.achievements.filter(
      a => a.name.trim() !== ''
    )

    const result = await invoke<{
      success: boolean
      message: string
    }>('save_achievements_config', {
      gamePath: props.gamePath,
      config: {
        ...config.value,
        achievements: validAchievements
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
      config?: AchievementsConfig
    }>('load_achievements_config', {
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
  max-width: 800px;
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

.header-icon.achievements {
  background-color: rgba(245, 158, 11, 0.1);
  color: #f59e0b;
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

/* 成就区域 */
.achievements-section {
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

/* 成就表格 */
.achievements-table {
  border: 1px solid var(--border-color);
  border-radius: 8px;
  overflow: hidden;
}

.table-header {
  display: grid;
  grid-template-columns: 60px 1fr 1fr 60px 60px;
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

.achievement-row {
  display: grid;
  grid-template-columns: 60px 1fr 1fr 60px 60px;
  gap: 8px;
  padding: 8px 12px;
  border-top: 1px solid var(--border-color);
  align-items: center;
}

.col-icon,
.col-hidden,
.col-actions {
  display: flex;
  align-items: center;
  justify-content: center;
}

.icon-placeholder {
  width: 36px;
  height: 36px;
  border: 1px dashed var(--border-color);
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  overflow: hidden;
  transition: border-color 0.15s ease;
}

.icon-placeholder:hover {
  border-color: var(--accent-color);
}

.icon-placeholder img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.icon-placeholder svg {
  width: 18px;
  height: 18px;
  color: var(--text-secondary);
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

.col-hidden input[type="checkbox"] {
  width: 18px;
  height: 18px;
  accent-color: var(--accent-color);
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

/* 通知选项 */
.notification-options {
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
