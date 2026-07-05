<template>
  <div class="controller-config-panel">
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
          <span class="guide-label">控制器定义文件</span>
          <span class="guide-value">steam_settings/controller/ACTION_SET.txt</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">数字按键格式</span>
          <span class="guide-value">ACTION_NAME=BUTTON_NAME，如 jump=A</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">模拟按键格式</span>
          <span class="guide-value">ACTION_NAME=ANALOG_NAME=input source mode</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">配置文件</span>
          <span class="guide-value">configs.controller.ini</span>
        </div>
      </div>
      <div class="guide-example">
        <div class="example-title">数字按键示例：</div>
        <pre class="example-code">jump=A
attack=X,B
pause=START</pre>
      </div>
      <div class="guide-example">
        <div class="example-title">有效数字按键名称：</div>
        <pre class="example-code">DUP, DDOWN, DLEFT, DRIGHT
START, BACK, LSTICK, RSTICK
LBUMPER, RBUMPER, A, B, X, Y
DLTRIGGER, DRTRIGGER</pre>
      </div>
      <div class="guide-example">
        <div class="example-title">有效模拟按键名称：</div>
        <pre class="example-code">LTRIGGER, RTRIGGER, LJOY, RJOY, DPAD</pre>
      </div>
      <p class="guide-tip">提示：非 XInput 控制器需使用工具转换为 XInput 模拟器</p>
    </div>

    <!-- 启用开关 -->
    <div class="config-section">
      <label class="toggle-label">
        <input v-model="config.enabled" type="checkbox" class="toggle-input" />
        <span class="toggle-slider"></span>
        <span class="toggle-text">启用 XInput 控制器支持</span>
      </label>
    </div>

    <template v-if="config.enabled">
      <!-- 控制器类型 -->
      <div class="config-group">
        <label>控制器类型</label>
        <select v-model="config.controllerType" class="config-select">
          <option value="xbox">Xbox</option>
          <option value="playstation">PlayStation</option>
          <option value="nintendo">Nintendo</option>
          <option value="generic">通用</option>
        </select>
      </div>

      <!-- 按键绑定 -->
      <div class="config-group">
        <label>按键绑定（可选）</label>
        <textarea
          v-model="bindingsText"
          class="config-textarea"
          rows="5"
          placeholder="每行一个绑定:&#10;jump=A&#10;attack=X,B&#10;pause=START"
        ></textarea>
        <p class="field-hint">格式: ACTION_NAME=BUTTON_NAME，每行一个</p>
      </div>

      <!-- 死区设置 -->
      <h4 class="section-title">死区设置</h4>
      <div class="config-group">
        <label>左摇杆死区</label>
        <input v-model.number="config.deadzone.leftStick" type="range" min="0" max="1" step="0.01" class="config-slider" />
        <span class="slider-value">{{ (config.deadzone.leftStick * 100).toFixed(0) }}%</span>
      </div>
      <div class="config-group">
        <label>右摇杆死区</label>
        <input v-model.number="config.deadzone.rightStick" type="range" min="0" max="1" step="0.01" class="config-slider" />
        <span class="slider-value">{{ (config.deadzone.rightStick * 100).toFixed(0) }}%</span>
      </div>
      <div class="config-group">
        <label>左扳机死区</label>
        <input v-model.number="config.deadzone.leftTrigger" type="range" min="0" max="1" step="0.01" class="config-slider" />
        <span class="slider-value">{{ (config.deadzone.leftTrigger * 100).toFixed(0) }}%</span>
      </div>
      <div class="config-group">
        <label>右扳机死区</label>
        <input v-model.number="config.deadzone.rightTrigger" type="range" min="0" max="1" step="0.01" class="config-slider" />
        <span class="slider-value">{{ (config.deadzone.rightTrigger * 100).toFixed(0) }}%</span>
      </div>

      <!-- 震动设置 -->
      <h4 class="section-title">震动设置</h4>
      <div class="config-group">
        <label class="checkbox-label">
          <input v-model="config.rumble.enabled" type="checkbox" />
          <span>启用手柄震动</span>
        </label>
      </div>
      <div class="config-group">
        <label>震动强度</label>
        <input v-model.number="config.rumble.intensity" type="range" min="0" max="1" step="0.05" class="config-slider" />
        <span class="slider-value">{{ (config.rumble.intensity * 100).toFixed(0) }}%</span>
      </div>

      <!-- 自定义图标 -->
      <h4 class="section-title">自定义图标</h4>
      <template v-if="config.customGlyphs">
        <div class="config-group">
          <label class="checkbox-label">
            <input v-model="config.customGlyphs.enabled" type="checkbox" />
            <span>使用自定义图标</span>
          </label>
        </div>
        <div class="config-group">
          <label>图标目录路径</label>
          <input
            v-model="config.customGlyphs.path"
            type="text"
            class="config-input"
            placeholder="例如：glyphs/"
          />
          <p class="field-hint">包含自定义控制器图标文件的目录路径</p>
        </div>
      </template>
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
        <span>控制器配置已保存成功！</span>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
/**
 * ControllerConfigPanel.vue - 控制器配置统一 Panel
 * 供单独弹窗和完整配置管理器复用
 */

import { shallowReactive, reactive, ref, onMounted, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { ControllerConfig } from '../../../types/steam-config.types'
import { CONFIG_EVENTS } from '../../../constants/config-events'

const props = defineProps<{
  gamePath: string
}>()

const emit = defineEmits<{
  saved: []
}>()

const showToast = ref(false)

/**
 * 控制器配置对象
 * 字段名与 Rust 后端 ControllerConfig 结构一致。
 * 外层使用 shallowReactive 减少代理开销；
 * 嵌套对象使用 reactive，确保 v-model 在表单字段上仍能正常工作。
 */
const config = shallowReactive<ControllerConfig>({
  enabled: true,
  controllerType: 'xbox',
  bindings: [],
  deadzone: reactive({
    leftStick: 0.1,
    rightStick: 0.1,
    leftTrigger: 0.1,
    rightTrigger: 0.1,
  }),
  rumble: reactive({
    enabled: true,
    intensity: 0.8,
  }),
  customGlyphs: reactive({
    enabled: false,
    path: '',
  }),
})

/** 按键绑定文本（用于 textarea 编辑） */
const bindingsText = ref('')

/**
 * 将数组格式的 bindings 转换为文本
 */
function syncBindingsToText() {
  bindingsText.value = config.bindings
    .map((b) => `${b.action}=${b.button}`)
    .join('\n')
}

/**
 * 将文本格式的 bindings 转换为数组
 */
function syncTextToBindings() {
  const lines = bindingsText.value
    .split('\n')
    .map((l) => l.trim())
    .filter((l) => l && l.includes('='))

  config.bindings = lines.map((line) => {
    const idx = line.indexOf('=')
    return {
      action: line.slice(0, idx),
      button: line.slice(idx + 1),
    }
  })
}

/** 监听文本变化同步到数组 */
watch(bindingsText, syncTextToBindings)

/**
 * 保存配置
 */
async function saveConfig() {
  // 确保按键绑定文本已同步到数组
  syncTextToBindings()

  try {
    const result = await invoke<{ success: boolean; message: string }>('save_controller_config', {
      gamePath: props.gamePath,
      config,
    })

    if (result.success) {
      showToast.value = true
      setTimeout(() => {
        showToast.value = false
      }, 3000)
      emit('saved')
      // 广播控制器配置已保存事件，通知完整配置管理器等其它窗口刷新
      window.dispatchEvent(new CustomEvent(CONFIG_EVENTS.CONTROLLER_SAVED, {
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
      config?: ControllerConfig
    }>('load_controller_config', {
      gamePath: props.gamePath,
    })

    if (result.exists && result.config) {
      const cfg = result.config
      config.enabled = cfg.enabled ?? true
      config.controllerType = cfg.controllerType || 'xbox'

      // 加载死区：替换为新的 reactive 对象，保持 v-model 响应式
      if (cfg.deadzone) {
        config.deadzone = reactive({
          leftStick: cfg.deadzone.leftStick ?? 0.1,
          rightStick: cfg.deadzone.rightStick ?? 0.1,
          leftTrigger: cfg.deadzone.leftTrigger ?? 0.1,
          rightTrigger: cfg.deadzone.rightTrigger ?? 0.1,
        })
      }

      // 加载震动：替换为新的 reactive 对象
      if (cfg.rumble) {
        config.rumble = reactive({
          enabled: cfg.rumble.enabled ?? true,
          intensity: cfg.rumble.intensity ?? 0.8,
        })
      }

      // 加载自定义图标：替换为新的 reactive 对象
      if (cfg.customGlyphs) {
        config.customGlyphs = reactive({
          enabled: cfg.customGlyphs.enabled ?? false,
          path: cfg.customGlyphs.path || '',
        })
      } else {
        config.customGlyphs = reactive({
          enabled: false,
          path: '',
        })
      }

      // 加载按键绑定（包括空数组，确保外部清空后文本区同步）
      config.bindings = Array.isArray(cfg.bindings) ? cfg.bindings : []
      syncBindingsToText()
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
  // 监听控制器配置保存事件，与完整配置管理器实时同步
  window.addEventListener(CONFIG_EVENTS.CONTROLLER_SAVED, onConfigSavedEvent)
})

onUnmounted(() => {
  window.removeEventListener(CONFIG_EVENTS.CONTROLLER_SAVED, onConfigSavedEvent)
})

defineExpose({
  load: loadConfig,
  save: saveConfig
})
</script>

<style scoped>
.controller-config-panel {
  width: 100%;
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
  margin: 0 0 16px 0;
}

.config-group {
  margin-bottom: 20px;
}

.config-group > label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-text-primary);
  margin-bottom: 8px;
}

.field-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 4px 0 0 0;
}

.config-input,
.config-textarea,
.config-select {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color 0.15s ease;
  box-sizing: border-box;
}

.config-textarea {
  font-family: 'Consolas', 'Courier New', monospace;
  resize: vertical;
}

.config-input:focus,
.config-textarea:focus,
.config-select:focus {
  border-color: var(--steam-accent-blue);
}

/* 滑块样式 */
.config-group input[type="range"] {
  width: 100%;
  height: 6px;
  -webkit-appearance: none;
  appearance: none;
  background: var(--steam-border);
  border-radius: 3px;
  outline: none;
}

.config-group input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  background: var(--steam-accent-blue);
  border-radius: 50%;
  cursor: pointer;
}

.slider-value {
  font-size: 13px;
  color: var(--steam-text-secondary);
  display: block;
  text-align: right;
  margin-top: 4px;
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
  color: var(--steam-text-primary);
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
  display: flex;
  align-items: flex-start;
  gap: 8px;
  font-size: 12px;
  color: var(--steam-accent-blue);
  margin-top: 14px;
  line-height: 1.5;
  padding: 8px 12px;
  background-color: rgba(59, 130, 246, 0.08);
  border-radius: 6px;
}

.guide-tip::before {
  content: '';
  display: block;
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background-color: var(--steam-accent-blue);
  flex-shrink: 0;
  margin-top: 6px;
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
