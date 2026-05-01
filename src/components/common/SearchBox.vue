<template>
  <!-- 
    SearchBox.vue - 搜索框组件
    支持实时搜索、清除按钮
  -->
  <div class="search-box" :class="{ focused: isFocused }">
    <svg class="search-icon" viewBox="0 0 24 24" fill="currentColor">
      <path d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>
    </svg>
    
    <input
      ref="inputRef"
      v-model="searchText"
      type="text"
      :placeholder="placeholder"
      class="search-input"
      @focus="isFocused = true"
      @blur="isFocused = false"
      @input="handleInput"
      @keydown.enter="handleSubmit"
    />
    
    <button 
      v-if="searchText"
      class="clear-btn"
      @click="clearSearch"
    >
      <svg viewBox="0 0 24 24" fill="currentColor">
        <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
      </svg>
    </button>
  </div>
</template>

<script setup lang="ts">
/**
 * SearchBox.vue - 搜索框组件
 * 实现实时搜索功能
 */

import { ref, watch } from 'vue'

/**
 * 组件属性定义
 */
interface Props {
  /** 占位符文本 */
  placeholder?: string
  /** 初始值 */
  modelValue?: string
  /** 防抖延迟（毫秒） */
  debounce?: number
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: '搜索...',
  modelValue: '',
  debounce: 300
})

/**
 * 组件事件定义
 */
const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'search', value: string): void
  (e: 'submit', value: string): void
}>()

// 内部状态
const searchText = ref(props.modelValue)
const isFocused = ref(false)
const inputRef = ref<HTMLInputElement>()

// 防抖定时器
let debounceTimer: ReturnType<typeof setTimeout> | null = null

/**
 * 监听属性变化
 */
watch(() => props.modelValue, (newValue) => {
  searchText.value = newValue
})

/**
 * 监听输入变化
 */
watch(searchText, (newValue) => {
  emit('update:modelValue', newValue)
})

/**
 * 处理输入事件（防抖）
 */
function handleInput() {
  if (debounceTimer) {
    clearTimeout(debounceTimer)
  }
  
  debounceTimer = setTimeout(() => {
    emit('search', searchText.value)
  }, props.debounce)
}

/**
 * 处理提交事件
 */
function handleSubmit() {
  emit('submit', searchText.value)
}

/**
 * 清除搜索
 */
function clearSearch() {
  searchText.value = ''
  emit('search', '')
  emit('update:modelValue', '')
  inputRef.value?.focus()
}

/**
 * 聚焦输入框
 */
function focus() {
  inputRef.value?.focus()
}

defineExpose({
  focus,
  clear: clearSearch
})
</script>

<style scoped>
.search-box {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0 12px;
  height: 36px;
  background: var(--steam-bg-tertiary);
  border: 1px solid transparent;
  border-radius: 8px;
  transition: border-color 0.15s ease-out, background 0.15s ease-out;
}

.search-box.focused {
  border-color: var(--steam-accent-blue);
  background: var(--steam-bg-secondary);
}

.search-icon {
  width: 18px;
  height: 18px;
  color: var(--steam-text-muted);
  flex-shrink: 0;
}

.search-input {
  flex: 1;
  height: 100%;
  background: transparent;
  border: none;
  color: var(--steam-text-primary);
  font-size: 14px;
  outline: none;
}

.search-input::placeholder {
  color: var(--steam-text-muted);
}

.clear-btn {
  width: 18px;
  height: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--steam-text-muted);
  border-radius: 50%;
  flex-shrink: 0;
  transition: color 0.15s ease-out, background 0.15s ease-out;
}

.clear-btn:hover {
  color: var(--steam-text-primary);
  background: rgba(255, 255, 255, 0.1);
}

.clear-btn svg {
  width: 14px;
  height: 14px;
}
</style>
