<template>
  <!--
    ColorVariableRow.vue - 单个颜色变量调节行
    提供颜色预览、HEX/RGBA 文本输入、原生颜色选择器与恢复默认按钮
  -->
  <div class="color-variable-row">
    <span class="variable-label">{{ label }}</span>
    <div class="variable-controls">
      <!-- 颜色预览方块 -->
      <div class="color-preview" :style="{ background: displayValue }"></div>

      <!-- HEX / RGBA 文本输入 -->
      <input
        type="text"
        class="color-input"
        :value="displayValue"
        @change="handleInputChange"
      />

      <!-- 原生颜色选择器（支持 HEX，RGBA 会回退到解析后的 HEX） -->
      <label class="color-picker-wrapper" :title="`选择 ${label}`">
        <input
          type="color"
          class="color-picker"
          :value="hexValue"
          @input="handlePickerChange"
        />
        <svg class="picker-icon" viewBox="0 0 24 24" fill="currentColor">
          <path d="M12 3c-4.97 0-9 4.03-9 9s4.03 9 9 9c.83 0 1.5-.67 1.5-1.5 0-.39-.15-.74-.39-1.01-.23-.26-.38-.61-.38-.99 0-.83.67-1.5 1.5-1.5H16c2.76 0 5-2.24 5-5 0-4.42-4.03-8-9-8zm-5.5 9c-.83 0-1.5-.67-1.5-1.5S5.67 9 6.5 9 8 9.67 8 10.5 7.33 12 6.5 12zm3-4C8.67 8 8 7.33 8 6.5S8.67 5 9.5 5s1.5.67 1.5 1.5S10.33 8 9.5 8zm5 0c-.83 0-1.5-.67-1.5-1.5S13.67 5 14.5 5s1.5.67 1.5 1.5S15.33 8 14.5 8zm3 4c-.83 0-1.5-.67-1.5-1.5S16.67 9 17.5 9s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z"/>
        </svg>
      </label>

      <!-- 恢复默认按钮 -->
      <button
        v-if="hasCustomValue"
        class="reset-btn"
        title="恢复默认"
        @click="handleReset"
      >
        <svg viewBox="0 0 24 24" fill="currentColor">
          <path d="M12 5V1L7 6l5 5V7c3.31 0 6 2.69 6 6s-2.69 6-6 6-6-2.69-6-6H4c0 4.42 3.58 8 8 8s8-3.58 8-8-3.58-8-8-8z"/>
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * ColorVariableRow.vue - 单个颜色变量调节行
 */

import { computed } from 'vue'

const props = defineProps<{
  /** CSS 变量名 */
  variableKey: string
  /** 变量显示名称 */
  label: string
  /** 当前主题下的默认值 */
  defaultValue: string
  /** 用户自定义值，undefined 表示未自定义 */
  modelValue: string | undefined
}>()

const emit = defineEmits<{
  /** 用户修改颜色 */
  (e: 'update', key: string, value: string): void
  /** 用户恢复默认 */
  (e: 'reset', key: string): void
}>()

/** 实际显示的颜色值（自定义优先，否则用默认值） */
const displayValue = computed(() => props.modelValue ?? props.defaultValue)

/** 是否存在用户自定义值 */
const hasCustomValue = computed(() => props.modelValue !== undefined)

/**
 * 将当前颜色值转换为 HEX，供原生 <input type="color"> 使用
 * 原生颜色选择器不支持 RGBA，因此 RGBA 会被解析为不透明的近似 HEX
 */
const hexValue = computed(() => {
  const value = displayValue.value.trim()
  if (value.startsWith('#')) {
    return value.slice(0, 7)
  }
  const rgbaMatch = value.match(/rgba?\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)/)
  if (rgbaMatch) {
    const r = parseInt(rgbaMatch[1], 10)
    const g = parseInt(rgbaMatch[2], 10)
    const b = parseInt(rgbaMatch[3], 10)
    return '#' + [r, g, b].map((v) => v.toString(16).padStart(2, '0')).join('')
  }
  return '#000000'
})

/** 文本输入框失车时触发 */
function handleInputChange(event: Event) {
  const target = event.target as HTMLInputElement
  emit('update', props.variableKey, target.value)
}

/** 颜色选择器变化时触发 */
function handlePickerChange(event: Event) {
  const target = event.target as HTMLInputElement
  emit('update', props.variableKey, target.value)
}

/** 恢复默认 */
function handleReset() {
  emit('reset', props.variableKey)
}
</script>

<style scoped>
.color-variable-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid var(--steam-border);
  gap: 12px;
}

.color-variable-row:last-child {
  border-bottom: none;
}

.variable-label {
  font-size: 13px;
  color: var(--steam-text-secondary);
  flex: 1;
  min-width: 0;
}

.variable-controls {
  display: flex;
  align-items: center;
  gap: 8px;
}

.color-preview {
  width: 22px;
  height: 22px;
  border-radius: 4px;
  border: 1px solid var(--steam-border);
  flex-shrink: 0;
}

.color-input {
  width: 110px;
  height: 28px;
  padding: 0 8px;
  border-radius: 6px;
  background: var(--steam-input-bg);
  border: 1px solid var(--steam-input-border);
  color: var(--steam-text-primary);
  font-size: 12px;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
  transition: border-color var(--transition-fast);
}

.color-input:focus {
  border-color: var(--steam-accent-blue);
}

.color-picker-wrapper {
  position: relative;
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  background: var(--steam-input-bg);
  border: 1px solid var(--steam-input-border);
  color: var(--steam-text-secondary);
  cursor: pointer;
  transition: all var(--transition-fast);
  flex-shrink: 0;
}

.color-picker-wrapper:hover {
  border-color: var(--steam-accent-blue);
  color: var(--steam-text-primary);
}

.color-picker {
  position: absolute;
  inset: 0;
  opacity: 0;
  width: 100%;
  height: 100%;
  cursor: pointer;
}

.picker-icon {
  width: 16px;
  height: 16px;
  pointer-events: none;
}

.reset-btn {
  width: 26px;
  height: 26px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  color: var(--steam-text-muted);
  transition: all var(--transition-fast);
  flex-shrink: 0;
}

.reset-btn:hover {
  background: var(--steam-error);
  color: white;
}

.reset-btn svg {
  width: 14px;
  height: 14px;
}
</style>
