<template>
  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <div class="header-icon controller">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="2" y="8" width="20" height="12" rx="2"/>
            <path d="M6 12h4"/>
            <path d="M8 10v4"/>
            <circle cx="16" cy="13" r="1"/>
            <circle cx="18" cy="11" r="1"/>
          </svg>
        </div>
        <h3>控制器支持配置</h3>
        <button class="close-btn" @click="$emit('close')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <div class="modal-body">
        <!-- 启用控制器支持 -->
        <div class="config-item">
          <label class="toggle-label">
            <input type="checkbox" v-model="config.enabled" class="toggle-input" />
            <span class="toggle-slider"></span>
            <span class="toggle-text">启用 XInput 控制器支持</span>
          </label>
        </div>

        <!-- 控制器说明 -->
        <div class="info-box">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <line x1="12" y1="16" x2="12" y2="12"/>
            <line x1="12" y1="8" x2="12.01" y2="8"/>
          </svg>
          <div class="info-content">
            <p>仅支持 XInput 控制器（Xbox 360/One 手柄）。</p>
            <p>如果您的控制器不是 XInput，请使用 DS4Windows 或类似工具进行转换。</p>
          </div>
        </div>

        <!-- Action Set 配置 -->
        <div v-if="config.enabled" class="action-sets-section">
          <div class="section-header">
            <label class="config-label">Action Sets（操作集）</label>
            <button class="action-btn" @click="addActionSet">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="12" y1="5" x2="12" y2="19"/>
                <line x1="5" y1="12" x2="19" y2="12"/>
              </svg>
              添加 Action Set
            </button>
          </div>

          <!-- Action Set 列表 -->
          <div class="action-sets-list">
            <div
              v-for="(actionSet, setIndex) in config.actionSets"
              :key="setIndex"
              class="action-set-card"
            >
              <div class="action-set-header">
                <input
                  type="text"
                  v-model="actionSet.name"
                  class="action-set-name"
                  placeholder="Action Set 名称（如: InGameControls）"
                />
                <button class="remove-btn" @click="removeActionSet(setIndex)">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="18" y1="6" x2="6" y2="18"/>
                    <line x1="6" y1="6" x2="18" y2="18"/>
                  </svg>
                </button>
              </div>

              <!-- 数字动作（按钮） -->
              <div class="actions-section">
                <div class="actions-header">
                  <span class="actions-title">数字动作（按钮）</span>
                  <button class="add-action-btn" @click="addDigitalAction(setIndex)">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <line x1="12" y1="5" x2="12" y2="19"/>
                      <line x1="5" y1="12" x2="19" y2="12"/>
                    </svg>
                  </button>
                </div>
                <div class="actions-list">
                  <div
                    v-for="(action, actionIndex) in actionSet.digitalActions"
                    :key="actionIndex"
                    class="action-row"
                  >
                    <input
                      type="text"
                      v-model="action.name"
                      class="action-name"
                      placeholder="动作名称"
                    />
                    <span class="action-equals">=</span>
                    <select v-model="action.button" class="action-button">
                      <option value="">选择按钮</option>
                      <optgroup label="主要按钮">
                        <option value="A">A</option>
                        <option value="B">B</option>
                        <option value="X">X</option>
                        <option value="Y">Y</option>
                      </optgroup>
                      <optgroup label="肩键和扳机">
                        <option value="LBUMPER">LB（左肩键）</option>
                        <option value="RBUMPER">RB（右肩键）</option>
                        <option value="LTRIGGER">LT（左扳机）</option>
                        <option value="RTRIGGER">RT（右扳机）</option>
                      </optgroup>
                      <optgroup label="摇杆">
                        <option value="LSTICK">左摇杆按下</option>
                        <option value="RSTICK">右摇杆按下</option>
                        <option value="LJOY">左摇杆</option>
                        <option value="RJOY">右摇杆</option>
                      </optgroup>
                      <optgroup label="方向键">
                        <option value="DUP">方向上</option>
                        <option value="DDOWN">方向下</option>
                        <option value="DLEFT">方向左</option>
                        <option value="DRIGHT">方向右</option>
                        <option value="DPAD">方向键（模拟）</option>
                      </optgroup>
                      <optgroup label="功能键">
                        <option value="START">Start</option>
                        <option value="BACK">Back/Select</option>
                      </optgroup>
                    </select>
                    <button class="remove-action-btn" @click="removeDigitalAction(setIndex, actionIndex)">
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <line x1="18" y1="6" x2="6" y2="18"/>
                        <line x1="6" y1="6" x2="18" y2="18"/>
                      </svg>
                    </button>
                  </div>
                </div>
              </div>

              <!-- 模拟动作 -->
              <div class="actions-section">
                <div class="actions-header">
                  <span class="actions-title">模拟动作（摇杆/扳机）</span>
                  <button class="add-action-btn" @click="addAnalogAction(setIndex)">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <line x1="12" y1="5" x2="12" y2="19"/>
                      <line x1="5" y1="12" x2="19" y2="12"/>
                    </svg>
                  </button>
                </div>
                <div class="actions-list">
                  <div
                    v-for="(action, actionIndex) in actionSet.analogActions"
                    :key="actionIndex"
                    class="action-row"
                  >
                    <input
                      type="text"
                      v-model="action.name"
                      class="action-name"
                      placeholder="动作名称"
                    />
                    <span class="action-equals">=</span>
                    <select v-model="action.analog" class="action-analog">
                      <option value="">选择输入源</option>
                      <option value="LTRIGGER">左扳机</option>
                      <option value="RTRIGGER">右扳机</option>
                      <option value="LJOY">左摇杆</option>
                      <option value="RJOY">右摇杆</option>
                      <option value="DPAD">方向键</option>
                    </select>
                    <select v-model="action.mode" class="action-mode">
                      <option value="">模式</option>
                      <option value="joystick_move">摇杆移动</option>
                      <option value="joystick_camera">摇杆视角</option>
                      <option value="trigger">扳机</option>
                      <option value="absolute_mouse">绝对鼠标</option>
                    </select>
                    <button class="remove-action-btn" @click="removeAnalogAction(setIndex, actionIndex)">
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <line x1="18" y1="6" x2="6" y2="18"/>
                        <line x1="6" y1="6" x2="18" y2="18"/>
                      </svg>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <p v-if="config.actionSets.length === 0" class="empty-hint">
            暂无 Action Set，点击上方按钮添加
          </p>
        </div>

        <!-- 按钮图标注释 -->
        <div v-if="config.enabled" class="config-group">
          <label class="config-label">按钮图标注释</label>
          <p class="config-desc">自定义游戏中显示的按钮图标（可选）</p>
          <button class="file-btn" @click="selectGlyphsFolder">
            {{ config.glyphsFolder ? '更改图标注释文件夹' : '选择图标注释文件夹' }}
          </button>
          <p v-if="config.glyphsFolder" class="file-path">{{ config.glyphsFolder }}</p>
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

interface DigitalAction {
  name: string
  button: string
}

interface AnalogAction {
  name: string
  analog: string
  mode: string
}

interface ActionSet {
  name: string
  digitalActions: DigitalAction[]
  analogActions: AnalogAction[]
}

interface ControllerConfig {
  enabled: boolean
  actionSets: ActionSet[]
  glyphsFolder: string
}

// ============================================
// 响应式状态
// ============================================

const config = ref<ControllerConfig>({
  enabled: true,
  actionSets: [],
  glyphsFolder: ''
})

// ============================================
// 方法
// ============================================

/**
 * 添加 Action Set
 */
const addActionSet = () => {
  config.value.actionSets.push({
    name: '',
    digitalActions: [],
    analogActions: []
  })
}

/**
 * 移除 Action Set
 */
const removeActionSet = (index: number) => {
  config.value.actionSets.splice(index, 1)
}

/**
 * 添加数字动作
 */
const addDigitalAction = (setIndex: number) => {
  config.value.actionSets[setIndex].digitalActions.push({
    name: '',
    button: ''
  })
}

/**
 * 移除数字动作
 */
const removeDigitalAction = (setIndex: number, actionIndex: number) => {
  config.value.actionSets[setIndex].digitalActions.splice(actionIndex, 1)
}

/**
 * 添加模拟动作
 */
const addAnalogAction = (setIndex: number) => {
  config.value.actionSets[setIndex].analogActions.push({
    name: '',
    analog: '',
    mode: ''
  })
}

/**
 * 移除模拟动作
 */
const removeAnalogAction = (setIndex: number, actionIndex: number) => {
  config.value.actionSets[setIndex].analogActions.splice(actionIndex, 1)
}

/**
 * 选择图标注释文件夹
 */
const selectGlyphsFolder = async () => {
  try {
    const result = await invoke<string | null>('select_folder', {
      title: '选择按钮图标注释文件夹'
    })
    if (result) {
      config.value.glyphsFolder = result
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
    // 过滤掉空的 action sets 和 actions
    const validActionSets = config.value.actionSets
      .filter(set => set.name.trim() !== '')
      .map(set => ({
        ...set,
        digitalActions: set.digitalActions.filter(a => a.name.trim() !== '' && a.button !== ''),
        analogActions: set.analogActions.filter(a => a.name.trim() !== '' && a.analog !== '')
      }))

    const result = await invoke<{
      success: boolean
      message: string
    }>('save_controller_config', {
      gamePath: props.gamePath,
      config: {
        ...config.value,
        actionSets: validActionSets
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
      config?: ControllerConfig
    }>('load_controller_config', {
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

.header-icon.controller {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
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

/* Action Sets 区域 */
.action-sets-section {
  margin-bottom: 20px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
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

/* Action Set 卡片 */
.action-sets-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
  max-height: 400px;
  overflow-y: auto;
}

.action-set-card {
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 16px;
}

.action-set-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
}

.action-set-name {
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

.action-set-name:focus {
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

/* Actions 区域 */
.actions-section {
  margin-bottom: 16px;
}

.actions-section:last-child {
  margin-bottom: 0;
}

.actions-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
}

.actions-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary);
}

.add-action-btn {
  width: 24px;
  height: 24px;
  border: none;
  border-radius: 4px;
  background-color: var(--bg-secondary);
  color: var(--accent-color);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.add-action-btn:hover {
  background-color: var(--border-color);
}

.add-action-btn svg {
  width: 14px;
  height: 14px;
}

.actions-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.action-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.action-name {
  flex: 1;
  padding: 6px 10px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
}

.action-name:focus {
  border-color: var(--accent-color);
}

.action-equals {
  font-size: 13px;
  color: var(--text-secondary);
}

.action-button,
.action-analog,
.action-mode {
  padding: 6px 10px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 12px;
  outline: none;
  cursor: pointer;
}

.action-button:focus,
.action-analog:focus,
.action-mode:focus {
  border-color: var(--accent-color);
}

.action-button {
  width: 140px;
}

.action-analog {
  width: 120px;
}

.action-mode {
  width: 130px;
}

.remove-action-btn {
  width: 24px;
  height: 24px;
  border: none;
  border-radius: 4px;
  background-color: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.remove-action-btn:hover {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.remove-action-btn svg {
  width: 14px;
  height: 14px;
}

.empty-hint {
  text-align: center;
  padding: 40px;
  color: var(--text-secondary);
  font-size: 14px;
}

/* 文件按钮 */
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

.file-path {
  font-size: 11px;
  color: var(--text-secondary);
  font-family: 'Courier New', monospace;
  margin: 8px 0 0 0;
  word-break: break-all;
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
