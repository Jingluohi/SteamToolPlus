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
        <!-- 启用开关 -->
        <div class="config-section">
          <label class="toggle-label">
            <input v-model="config.enabled" type="checkbox" class="toggle-input" />
            <span class="toggle-slider"></span>
            <span class="toggle-text">启用成就系统</span>
          </label>
        </div>

        <template v-if="config.enabled">
          <!-- 通知设置 -->
          <div class="config-section">
            <h4 class="section-title">通知设置</h4>
            <label class="checkbox-label">
              <input v-model="config.showNotifications" type="checkbox" />
              <span>显示成就解锁通知</span>
            </label>
          </div>

          <!-- 成就列表 -->
          <div class="config-section">
            <div class="section-header">
              <h4 class="section-title">成就列表</h4>
              <button class="btn-add" @click="addAchievement">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="12" y1="5" x2="12" y2="19"/>
                  <line x1="5" y1="12" x2="19" y2="12"/>
                </svg>
                添加成就
              </button>
            </div>

            <div class="achievements-list">
              <div 
                v-for="(achievement, index) in config.achievements" 
                :key="index"
                class="achievement-card"
                :class="{ expanded: expandedIndex === index }"
              >
                <div class="achievement-header" @click="toggleExpand(index)">
                  <div class="achievement-icon">
                    <svg v-if="!achievement.icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <circle cx="12" cy="8" r="7"/>
                      <polyline points="8.21 13.89 7 23 12 20 17 23 15.79 13.88"/>
                    </svg>
                    <img v-else :src="achievement.icon" alt="achievement" />
                  </div>
                  <div class="achievement-info">
                    <span class="achievement-name">{{ achievement.displayName || achievement.name || '未命名成就' }}</span>
                    <span class="achievement-id">{{ achievement.name || '无ID' }}</span>
                  </div>
                  <div class="achievement-actions">
                    <span v-if="achievement.hidden" class="badge hidden">隐藏</span>
                    <button class="btn-icon" @click.stop="removeAchievement(index)">
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <line x1="18" y1="6" x2="6" y2="18"/>
                        <line x1="6" y1="6" x2="18" y2="18"/>
                      </svg>
                    </button>
                  </div>
                </div>

                <div v-if="expandedIndex === index" class="achievement-details">
                  <div class="form-row">
                    <div class="form-group">
                      <label>成就ID（唯一标识）</label>
                      <input 
                        v-model="achievement.name" 
                        type="text" 
                        placeholder="例如：achievement_first_blood"
                      />
                    </div>
                    <div class="form-group">
                      <label>显示名称</label>
                      <input 
                        v-model="achievement.displayName" 
                        type="text" 
                        placeholder="例如：第一滴血"
                      />
                    </div>
                  </div>

                  <div class="form-group">
                    <label>成就描述</label>
                    <textarea 
                      v-model="achievement.description" 
                      rows="2"
                      placeholder="描述如何解锁此成就..."
                    ></textarea>
                  </div>

                  <div class="form-row">
                    <div class="form-group">
                      <label class="checkbox-label">
                        <input v-model="achievement.hidden" type="checkbox" />
                        <span>隐藏成就（解锁前不显示描述）</span>
                      </label>
                    </div>
                  </div>

                  <div class="form-row">
                    <div class="form-group">
                      <label>解锁图标</label>
                      <div class="file-input-group">
                        <input 
                          v-model="achievement.icon" 
                          type="text" 
                          placeholder="图标路径（可选）"
                          readonly
                        />
                        <button class="btn-browse" @click="selectIcon(index, 'icon')">浏览</button>
                        <button v-if="achievement.icon" class="btn-clear" @click="achievement.icon = ''">清除</button>
                      </div>
                    </div>
                    <div class="form-group">
                      <label>未解锁图标</label>
                      <div class="file-input-group">
                        <input 
                          v-model="achievement.iconGray" 
                          type="text" 
                          placeholder="灰色图标路径（可选）"
                          readonly
                        />
                        <button class="btn-browse" @click="selectIcon(index, 'iconGray')">浏览</button>
                        <button v-if="achievement.iconGray" class="btn-clear" @click="achievement.iconGray = ''">清除</button>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <div v-if="config.achievements.length === 0" class="empty-state">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="8" r="7"/>
                  <polyline points="8.21 13.89 7 23 12 20 17 23 15.79 13.88"/>
                </svg>
                <p>暂无成就，点击上方按钮添加</p>
              </div>
            </div>
          </div>

          <!-- 导入/导出 -->
          <div class="config-section">
            <h4 class="section-title">导入/导出</h4>
            <div class="import-export-actions">
              <button class="btn-secondary" @click="openSteamDB">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>
                  <polyline points="15 3 21 3 21 9"/>
                  <line x1="10" y1="14" x2="21" y2="3"/>
                </svg>
                打开 SteamDB
              </button>
              <button class="btn-secondary" @click="importFromFile">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                  <polyline points="7 10 12 15 17 10"/>
                  <line x1="12" y1="15" x2="12" y2="3"/>
                </svg>
                从文件导入
              </button>
              <button class="btn-secondary" @click="exportConfig">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                  <polyline points="17 8 12 3 7 8"/>
                  <line x1="12" y1="3" x2="12" y2="15"/>
                </svg>
                导出配置
              </button>
            </div>
            <p class="config-hint">
              提示：在 SteamDB 上找到游戏后，复制成就数据到 JSON 文件，然后使用"从文件导入"功能
            </p>
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
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import type { AchievementsConfig, Achievement } from '../../../src/types/steam-config.types'

const props = defineProps<{
  gamePath: string
  gameId: string
}>()

const emit = defineEmits<{
  close: []
  saved: []
}>()

const config = ref<AchievementsConfig>({
  enabled: true,
  showNotifications: true,
  achievements: []
})

const expandedIndex = ref<number | null>(null)

function toggleExpand(index: number) {
  expandedIndex.value = expandedIndex.value === index ? null : index
}

function addAchievement() {
  const newAchievement: Achievement = {
    name: '',
    displayName: '',
    description: '',
    hidden: false,
    icon: undefined,
    iconGray: undefined
  }
  config.value.achievements.push(newAchievement)
  expandedIndex.value = config.value.achievements.length - 1
}

function removeAchievement(index: number) {
  if (confirm('确定要删除这个成就吗？')) {
    config.value.achievements.splice(index, 1)
    if (expandedIndex.value === index) {
      expandedIndex.value = null
    }
  }
}

async function selectIcon(achievementIndex: number, type: 'icon' | 'iconGray') {
  try {
    const selected = await open({
      filters: [{
        name: '图片文件',
        extensions: ['png', 'jpg', 'jpeg', 'bmp']
      }],
      title: '选择成就图标'
    })
    
    if (selected && typeof selected === 'string') {
      config.value.achievements[achievementIndex][type] = selected
    }
  } catch (error) {
    // 选择失败时静默处理
  }
}

async function openSteamDB() {
  // 打开 SteamDB 网站，让用户手动查找成就数据
  const appId = props.gameId || prompt('请输入 Steam AppID：')
  if (appId) {
    const url = `https://steamdb.info/app/${appId}/stats/`
    await invoke('open_external_link', { url })
  }
}

async function importFromFile() {
  try {
    const selected = await open({
      filters: [{
        name: 'JSON 文件',
        extensions: ['json']
      }],
      title: '选择成就配置文件'
    })
    
    if (selected && typeof selected === 'string') {
      const result = await invoke<{
        success: boolean
        achievements?: Achievement[]
        message?: string
      }>('import_achievements_from_file', {
        filePath: selected
      })

      if (result.success && result.achievements) {
        config.value.achievements = [...config.value.achievements, ...result.achievements]
        alert(`成功导入 ${result.achievements.length} 个成就！`)
      } else {
        alert(`导入失败: ${result.message || '未知错误'}`)
      }
    }
  } catch (error) {
    alert(`导入失败: ${error}`)
  }
}

async function exportConfig() {
  try {
    const result = await invoke<{
      success: boolean
      data?: string
      message?: string
    }>('export_achievements_config', {
      gamePath: props.gamePath
    })

    if (result.success && result.data) {
      // 复制到剪贴板
      await navigator.clipboard.writeText(result.data)
      alert('配置已复制到剪贴板！')
    } else {
      alert(`导出失败: ${result.message || '未知错误'}`)
    }
  } catch (error) {
    alert(`导出失败: ${error}`)
  }
}

async function saveConfig() {
  // 验证数据
  const invalidAchievements = config.value.achievements.filter(a => !a.name || !a.displayName)
  if (invalidAchievements.length > 0) {
    alert('请为所有成就填写ID和显示名称')
    return
  }

  try {
    const result = await invoke<{ success: boolean; message: string }>('save_achievements_config', {
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

async function loadConfig() {
  try {
    const result = await invoke<{ 
      exists: boolean
      config?: AchievementsConfig 
    }>('load_achievements_config', {
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
  max-width: 800px;
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

/* 配置区域 */
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

/* 开关样式 */
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

/* 复选框样式 */
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
}

/* 按钮样式 */
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
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
}

.btn-secondary:hover {
  background-color: var(--steam-border);
}

.btn-secondary svg {
  width: 14px;
  height: 14px;
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

/* 成就列表 */
.achievements-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.achievement-card {
  background-color: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 10px;
  overflow: hidden;
  transition: all 0.15s ease;
}

.achievement-card:hover {
  border-color: var(--steam-border);
}

.achievement-card.expanded {
  border-color: var(--steam-accent-blue);
}

.achievement-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.achievement-header:hover {
  background-color: var(--steam-bg-tertiary);
}

.achievement-icon {
  width: 40px;
  height: 40px;
  border-radius: 8px;
  background-color: var(--steam-bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  overflow: hidden;
}

.achievement-icon svg {
  width: 24px;
  height: 24px;
  color: var(--steam-text-secondary);
}

.achievement-icon img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.achievement-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
}

.achievement-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.achievement-id {
  font-size: 12px;
  color: var(--steam-text-secondary);
  font-family: monospace;
}

.achievement-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.badge {
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 500;
}

.badge.hidden {
  background-color: rgba(245, 158, 11, 0.1);
  color: #f59e0b;
}

.achievement-details {
  padding: 16px;
  border-top: 1px solid var(--steam-border);
  background-color: var(--steam-bg-primary);
}

/* 表单样式 */
.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
  margin-bottom: 16px;
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
.form-group textarea {
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
.form-group textarea:focus {
  border-color: var(--steam-accent-blue);
}

.form-group textarea {
  resize: vertical;
  font-family: inherit;
}

.file-input-group {
  display: flex;
  gap: 8px;
}

.file-input-group input {
  flex: 1;
}

.btn-browse,
.btn-clear {
  padding: 10px 14px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
  white-space: nowrap;
}

.btn-browse:hover,
.btn-clear:hover {
  background-color: var(--steam-border);
}

/* 空状态 */
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

/* 导入/导出 */
.import-export-actions {
  display: flex;
  gap: 12px;
}

/* 响应式 */
@media (max-width: 600px) {
  .form-row {
    grid-template-columns: 1fr;
  }
  
  .section-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  
  .import-export-actions {
    flex-direction: column;
  }
}
</style>
