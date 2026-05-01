<template>
  <!-- 
    Dropdown.vue - 下拉菜单组件
    通用下拉选择器
  -->
  <div 
    class="dropdown"
    :class="{ open: isOpen }"
    v-click-outside="close"
  >
    <!-- 触发按钮 -->
    <button 
      class="dropdown-trigger"
      @click="toggle"
    >
      <span class="trigger-text">{{ displayText }}</span>
      <svg class="trigger-icon" viewBox="0 0 24 24" fill="currentColor">
        <path d="M7 10l5 5 5-5z"/>
      </svg>
    </button>
    
    <!-- 下拉菜单 -->
    <Transition name="dropdown">
      <div v-show="isOpen" class="dropdown-menu">
        <div class="dropdown-header" v-if="title">{{ title }}</div>
        <div class="dropdown-items">
          <button
            v-for="option in options"
            :key="option.value"
            class="dropdown-item"
            :class="{ active: modelValue === option.value }"
            @click="select(option)"
          >
            <span class="item-text">{{ option.label }}</span>
            <svg 
              v-if="modelValue === option.value" 
              class="check-icon" 
              viewBox="0 0 24 24" 
              fill="currentColor"
            >
              <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
            </svg>
          </button>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
/**
 * Dropdown.vue - 下拉菜单组件
 * 通用下拉选择器组件
 */

import { ref, computed } from 'vue'

/**
 * 选项接口
 */
export interface DropdownOption {
  /** 选项值 */
  value: string
  /** 显示文本 */
  label: string
}

/**
 * 组件属性定义
 */
interface Props {
  /** 选项列表 */
  options: DropdownOption[]
  /** 当前选中值 */
  modelValue: string
  /** 占位符 */
  placeholder?: string
  /** 标题 */
  title?: string
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: '请选择'
})

/**
 * 组件事件定义
 */
const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'change', value: string): void
}>()

// 是否打开
const isOpen = ref(false)

// 显示文本
const displayText = computed(() => {
  const option = props.options.find(o => o.value === props.modelValue)
  return option?.label || props.placeholder
})

/**
 * 切换下拉菜单
 */
function toggle() {
  isOpen.value = !isOpen.value
}

/**
 * 关闭下拉菜单
 */
function close() {
  isOpen.value = false
}

/**
 * 选择选项
 */
function select(option: DropdownOption) {
  emit('update:modelValue', option.value)
  emit('change', option.value)
  close()
}

// 点击外部指令
const vClickOutside = {
  mounted(el: HTMLElement, binding: { value: () => void }) {
    const handler = (e: MouseEvent) => {
      if (!el.contains(e.target as Node)) {
        binding.value()
      }
    }
    document.addEventListener('click', handler)
    ;(el as any)._clickOutside = handler
  },
  unmounted(el: HTMLElement) {
    const handler = (el as any)._clickOutside
    if (handler) {
      document.removeEventListener('click', handler)
    }
  }
}
</script>

<style scoped>
.dropdown {
  position: relative;
  display: inline-block;
}

.dropdown-trigger {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0 12px;
  height: 32px;
  background: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border);
  border-radius: 4px;
  color: var(--steam-text-primary);
  font-size: 14px;
  transition: border-color 0.15s ease-out, background 0.15s ease-out;
}

.dropdown-trigger:hover {
  border-color: var(--steam-border-light);
}

.dropdown.open .dropdown-trigger {
  border-color: var(--steam-accent-blue);
}

.trigger-text {
  flex: 1;
}

.trigger-icon {
  width: 16px;
  height: 16px;
  color: var(--steam-text-secondary);
  transition: transform 0.2s ease-out;
}

.dropdown.open .trigger-icon {
  transform: rotate(180deg);
}

/* 下拉菜单 */
.dropdown-menu {
  position: absolute;
  top: 100%;
  left: 0;
  min-width: 100%;
  margin-top: 4px;
  background: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  box-shadow: var(--shadow-steam);
  z-index: 1000;
  overflow: hidden;
}

.dropdown-header {
  padding: 8px 12px;
  font-size: 12px;
  font-weight: 600;
  color: var(--steam-text-muted);
  text-transform: uppercase;
  border-bottom: 1px solid var(--steam-border);
}

.dropdown-items {
  max-height: 240px;
  overflow-y: auto;
}

.dropdown-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: 10px 12px;
  font-size: 14px;
  color: var(--steam-text-primary);
  text-align: left;
  transition: background 0.15s ease-out;
}

.dropdown-item:hover {
  background: var(--steam-accent-hover);
}

.dropdown-item.active {
  color: var(--steam-accent-blue);
}

.item-text {
  flex: 1;
}

.check-icon {
  width: 16px;
  height: 16px;
  margin-left: 8px;
}

/* 动画 */
.dropdown-enter-active,
.dropdown-leave-active {
  transition: opacity 0.2s ease-out, transform 0.2s ease-out;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}
</style>
