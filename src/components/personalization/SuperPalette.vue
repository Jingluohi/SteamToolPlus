<template>
  <!--
    SuperPalette.vue - 超级调色盘主组件
    整合实时预览、预设选择、任意颜色调节与配置保存
  -->
  <div class="super-palette">
    <div class="palette-layout">
      <!-- 左侧：实时预览与操作按钮 -->
      <div class="palette-left">
        <PalettePreview />
        <div class="palette-actions">
          <button class="action-btn reset" @click="handleResetAll">
            恢复默认
          </button>
          <button class="action-btn save" @click="handleSaveNow">
            保存到配置
          </button>
        </div>
        <p class="save-hint">颜色变化会实时预览，并自动保存到配置文件</p>
      </div>

      <!-- 右侧：预设与自定义颜色 -->
      <div class="palette-right">
        <ColorPresetPanel
          :custom-vars="localCustomVars"
          @select="handlePresetSelect"
        />

        <div class="variables-section">
          <h3 class="variables-title">自定义颜色</h3>
          <ColorVariableGroup
            v-for="group in COLOR_VARIABLE_GROUPS"
            :key="group.id"
            :title="group.title"
            :variables="group.variables"
            :custom-vars="localCustomVars"
            @update="handleVariableUpdate"
            @reset="handleVariableReset"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * SuperPalette.vue - 超级调色盘主组件
 */

import { ref, onMounted, watch } from 'vue'
import { useThemeStore } from '../../store/theme.store'
import { useConfigStore } from '../../store/config.store'
import PalettePreview from './PalettePreview.vue'
import ColorPresetPanel from './ColorPresetPanel.vue'
import ColorVariableGroup from './ColorVariableGroup.vue'
import { COLOR_VARIABLE_GROUPS, type ColorPreset } from './palette.config'

const themeStore = useThemeStore()
const configStore = useConfigStore()

/** 本地自定义变量副本，用于驱动 UI */
const localCustomVars = ref<Record<string, string>>({})

/** 自动保存防抖计时器 */
let saveTimer: ReturnType<typeof setTimeout> | null = null

/** 组件挂载时同步 themeStore 的自定义变量 */
onMounted(() => {
  localCustomVars.value = { ...themeStore.customVars }
})

/**
 * 用户选择预设主题
 * 将预设颜色批量应用到 themeStore 与本地状态
 */
function handlePresetSelect(preset: ColorPreset) {
  localCustomVars.value = { ...preset.colors }
  themeStore.setCustomVars(localCustomVars.value)
  scheduleSave()
}

/**
 * 用户修改单个颜色变量
 */
function handleVariableUpdate(key: string, value: string) {
  localCustomVars.value[key] = value
  themeStore.setCustomVar(key, value)
  scheduleSave()
}

/**
 * 用户恢复单个颜色变量为默认
 */
function handleVariableReset(key: string) {
  delete localCustomVars.value[key]
  themeStore.removeCustomVar(key)
  scheduleSave()
}

/**
 * 恢复所有颜色变量为默认
 */
function handleResetAll() {
  localCustomVars.value = {}
  themeStore.resetCustomVars()
  scheduleSave()
}

/**
 * 立即保存到配置文件
 */
async function handleSaveNow() {
  await saveToConfig()
}

/**
 * 将当前自定义变量保存到应用配置
 * 颜色变化会触发防抖自动保存
 */
async function saveToConfig() {
  const currentConfig = configStore.config
  if (!currentConfig) return
  const updateData = {
    theme: {
      mode: currentConfig.theme.mode,
      followSystem: currentConfig.theme.followSystem,
      customVars: { ...localCustomVars.value }
    }
  }
  try {
    await configStore.updateConfig(updateData)
  } catch {
    // 保存失败时静默处理，不影响用户操作
  }
}

/**
 * 防抖调度保存，避免颜色选择器拖动时频繁写盘
 */
function scheduleSave() {
  if (saveTimer) {
    clearTimeout(saveTimer)
  }
  saveTimer = setTimeout(() => {
    saveToConfig()
  }, 300)
}

/**
 * 监听本地自定义变量变化，保持与 themeStore 同步
 * 主要用于外部（如 TitleBar 切换主题）可能重置 customVars 的场景
 */
watch(
  () => themeStore.customVars,
  (newVars) => {
    localCustomVars.value = { ...newVars }
  },
  { deep: true }
)
</script>

<style scoped>
.super-palette {
  width: 100%;
}

.palette-layout {
  display: grid;
  grid-template-columns: 280px 1fr;
  gap: 20px;
  align-items: flex-start;
}

.palette-left {
  display: flex;
  flex-direction: column;
  gap: 12px;
  position: sticky;
  top: 0;
}

.palette-actions {
  display: flex;
  gap: 10px;
}

.action-btn {
  flex: 1;
  height: 34px;
  border-radius: var(--radius-md);
  font-size: 13px;
  font-weight: 500;
  transition: all var(--transition-fast);
  border: 1px solid var(--steam-border);
}

.action-btn.reset {
  background: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
}

.action-btn.reset:hover {
  background: var(--steam-bg-hover);
}

.action-btn.save {
  background: var(--steam-accent-blue);
  color: white;
  border-color: var(--steam-accent-blue);
}

.action-btn.save:hover {
  background: var(--steam-accent-blue-hover);
}

.save-hint {
  font-size: 12px;
  color: var(--steam-text-muted);
  text-align: center;
  line-height: 1.4;
}

.palette-right {
  display: flex;
  flex-direction: column;
  gap: 20px;
  min-width: 0;
}

.variables-section {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.variables-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

@media (max-width: 900px) {
  .palette-layout {
    grid-template-columns: 1fr;
  }

  .palette-left {
    position: static;
  }
}
</style>
