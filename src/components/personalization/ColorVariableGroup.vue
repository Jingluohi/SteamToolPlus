<template>
  <!--
    ColorVariableGroup.vue - 颜色变量分组
    提供折叠/展开功能，内部渲染一组 ColorVariableRow
  -->
  <div class="color-variable-group">
    <button class="group-header" @click="toggleExpanded">
      <span class="group-title">{{ title }}</span>
      <svg
        class="expand-icon"
        :class="{ expanded: isExpanded }"
        viewBox="0 0 24 24"
        fill="currentColor"
      >
        <path d="M7.41 8.59L12 13.17l4.59-4.58L18 10l-6 6-6-6 1.41-1.41z"/>
      </svg>
    </button>
    <div v-show="isExpanded" class="group-content">
      <ColorVariableRow
        v-for="variable in variables"
        :key="variable.key"
        :variable-key="variable.key"
        :label="variable.label"
        :default-value="themeStore.isDark ? variable.defaultDark : variable.defaultLight"
        :model-value="customVars[variable.key]"
        @update="handleUpdate"
        @reset="handleReset"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * ColorVariableGroup.vue - 颜色变量分组
 */

import { ref } from 'vue'
import { useThemeStore } from '../../store/theme.store'
import ColorVariableRow from './ColorVariableRow.vue'
import type { ColorVariable } from './palette.config'

const props = defineProps<{
  /** 分组标题 */
  title: string
  /** 分组内的变量列表 */
  variables: ColorVariable[]
  /** 当前所有自定义变量 */
  customVars: Record<string, string>
}>()

const emit = defineEmits<{
  /** 变量颜色更新 */
  (e: 'update', key: string, value: string): void
  /** 变量恢复默认 */
  (e: 'reset', key: string): void
}>()

const themeStore = useThemeStore()

/** 分组是否展开 */
const isExpanded = ref(true)

/** 切换展开状态 */
function toggleExpanded() {
  isExpanded.value = !isExpanded.value
}

/** 透传颜色更新事件 */
function handleUpdate(key: string, value: string) {
  emit('update', key, value)
}

/** 透传恢复默认事件 */
function handleReset(key: string) {
  emit('reset', key)
}
</script>

<style scoped>
.color-variable-group {
  border: 1px solid var(--steam-border);
  border-radius: var(--radius-md);
  overflow: hidden;
  background: rgba(var(--steam-bg-primary-rgb), 0.4);
}

.group-header {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 12px;
  background: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
  font-size: 14px;
  font-weight: 500;
  transition: background var(--transition-fast);
}

.group-header:hover {
  background: var(--steam-bg-hover);
}

.group-title {
  font-size: 14px;
}

.expand-icon {
  width: 18px;
  height: 18px;
  color: var(--steam-text-muted);
  transition: transform var(--transition-fast);
}

.expand-icon.expanded {
  transform: rotate(180deg);
}

.group-content {
  padding: 0 12px;
}
</style>
