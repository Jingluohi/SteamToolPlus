<template>
  <!--
    ColorPresetPanel.vue - 预设主题分类面板
    仿照 QQ 超级调色盘的分类网格布局
  -->
  <div class="color-preset-panel">
    <!-- 隐藏的原生颜色选择器，用于「自选颜色」预设 -->
    <input
      ref="customColorInput"
      type="color"
      class="hidden-color-input"
      @input="handleCustomColorInput"
    />

    <div v-for="category in PRESET_CATEGORIES" :key="category.id" class="preset-category">
      <div class="category-header">
        <h3 class="category-title">{{ category.title }}</h3>
        <span class="category-subtitle">{{ category.subtitle }}</span>
      </div>
      <div class="preset-grid">
        <div
          v-for="preset in category.presets"
          :key="preset.id"
          class="preset-card"
          :class="{ active: activePresetId === preset.id }"
          @click="handleSelect(preset)"
        >
          <div class="preset-preview" :style="getPresetStyle(preset)">
            <svg v-if="isCustomPreset(preset)" class="custom-icon" viewBox="0 0 24 24" fill="currentColor">
              <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
            </svg>
          </div>
          <span class="preset-name">{{ preset.name }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * ColorPresetPanel.vue - 预设主题分类面板
 */

import { computed, ref } from 'vue'
import { PRESET_CATEGORIES, isCustomPreset, type ColorPreset } from './palette.config'

const props = defineProps<{
  /** 当前已应用的自定义变量 */
  customVars: Record<string, string>
}>()

const emit = defineEmits<{
  /** 用户选择某个预设 */
  (e: 'select', preset: ColorPreset): void
}>()

/** 隐藏颜色选择器引用 */
const customColorInput = ref<HTMLInputElement | null>(null)
/** 当前正在处理的「自选颜色」预设 */
const pendingCustomPreset = ref<ColorPreset | null>(null)

/**
 * 根据当前 customVars 匹配预设 ID
 * 若当前自定义值与某个非自选预设完全一致，则高亮该预设
 */
const activePresetId = computed(() => {
  for (const category of PRESET_CATEGORIES) {
    for (const preset of category.presets) {
      if (isCustomPreset(preset)) continue
      const presetKeys = Object.keys(preset.colors)
      const currentKeys = Object.keys(props.customVars)
      if (presetKeys.length !== currentKeys.length) continue
      const allMatch = presetKeys.every((key) => props.customVars[key] === preset.colors[key])
      if (allMatch) return preset.id
    }
  }
  return undefined
})

/**
 * 获取预设卡片的预览背景样式
 * 普通预设使用主题强调色到主背景色的渐变
 * 自选颜色预设使用彩虹渐变表示可自定义
 */
function getPresetStyle(preset: ColorPreset) {
  if (isCustomPreset(preset)) {
    return {
      background: 'linear-gradient(135deg, #ff6b6b 0%, #feca57 25%, #48dbfb 50%, #ff9ff3 75%, #54a0ff 100%)'
    }
  }
  const accent = preset.colors['--steam-accent-blue'] || '#60a5fa'
  const bg = preset.colors['--steam-bg-primary'] || '#363636'
  return {
    background: `linear-gradient(135deg, ${accent} 0%, ${bg} 100%)`
  }
}

/**
 * 处理预设卡片点击
 * 普通预设直接应用；自选颜色预设打开原生颜色选择器
 */
function handleSelect(preset: ColorPreset) {
  if (isCustomPreset(preset)) {
    pendingCustomPreset.value = preset
    customColorInput.value?.click()
    return
  }
  emit('select', preset)
}

/**
 * 原生颜色选择器输入完成后，生成只包含主题强调色的自定义预设并应用
 */
function handleCustomColorInput(event: Event) {
  const target = event.target as HTMLInputElement
  if (!pendingCustomPreset.value) return
  const customPreset: ColorPreset = {
    ...pendingCustomPreset.value,
    colors: { '--steam-accent-blue': target.value }
  }
  emit('select', customPreset)
}
</script>

<style scoped>
.color-preset-panel {
  display: flex;
  flex-direction: column;
  gap: 20px;
  position: relative;
}

.hidden-color-input {
  position: absolute;
  visibility: hidden;
  width: 0;
  height: 0;
  pointer-events: none;
}

.preset-category {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.category-header {
  display: flex;
  align-items: baseline;
  gap: 10px;
}

.category-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.category-subtitle {
  font-size: 12px;
  color: var(--steam-text-muted);
}

.preset-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(90px, 1fr));
  gap: 10px;
}

.preset-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  cursor: pointer;
  padding: 6px;
  border-radius: var(--radius-md);
  border: 1px solid transparent;
  transition: all var(--transition-fast);
}

.preset-card:hover {
  background: var(--steam-accent-hover);
}

.preset-card.active {
  border-color: var(--steam-accent-blue);
  background: rgba(var(--steam-accent-blue-rgb), 0.15);
}

.preset-preview {
  width: 100%;
  aspect-ratio: 1;
  border-radius: var(--radius-md);
  border: 1px solid var(--steam-border);
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.custom-icon {
  width: 20px;
  height: 20px;
  color: white;
  filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.4));
}

.preset-name {
  font-size: 12px;
  color: var(--steam-text-secondary);
  text-align: center;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100%;
}
</style>
