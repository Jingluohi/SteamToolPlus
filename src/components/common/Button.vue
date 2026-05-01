<template>
  <!-- 
    Button.vue - 按钮组件
    通用按钮组件，支持多种变体和尺寸
  -->
  <button
    class="btn"
    :class="[
      `btn-${variant}`,
      `btn-${size}`,
      { 'btn-loading': loading, 'btn-block': block }
    ]"
    :disabled="disabled || loading"
    @click="handleClick"
  >
    <!-- 加载状态 -->
    <span v-if="loading" class="btn-spinner">
      <svg viewBox="0 0 24 24" fill="none">
        <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" opacity="0.25"/>
        <path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="3" stroke-linecap="round"/>
      </svg>
    </span>
    
    <!-- 图标（左侧） -->
    <span v-if="icon && !loading" class="btn-icon btn-icon-left">
      <slot name="icon">
        <svg viewBox="0 0 24 24" fill="currentColor">
          <path :d="icon"/>
        </svg>
      </slot>
    </span>
    
    <!-- 文本内容 -->
    <span class="btn-content">
      <slot />
    </span>
    
    <!-- 图标（右侧） -->
    <span v-if="iconRight" class="btn-icon btn-icon-right">
      <slot name="icon-right">
        <svg viewBox="0 0 24 24" fill="currentColor">
          <path :d="iconRight"/>
        </svg>
      </slot>
    </span>
  </button>
</template>

<script setup lang="ts">
/**
 * Button.vue - 按钮组件
 * 通用按钮组件
 */

/**
 * 组件属性定义
 */
interface Props {
  /** 按钮变体 */
  variant?: 'primary' | 'secondary' | 'ghost' | 'danger'
  /** 按钮尺寸 */
  size?: 'sm' | 'md' | 'lg'
  /** 是否禁用 */
  disabled?: boolean
  /** 是否加载中 */
  loading?: boolean
  /** 是否块级显示 */
  block?: boolean
  /** 左侧图标路径 */
  icon?: string
  /** 右侧图标路径 */
  iconRight?: string
}

withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'md',
  disabled: false,
  loading: false,
  block: false
})

/**
 * 组件事件定义
 */
const emit = defineEmits<{
  (e: 'click', event: MouseEvent): void
}>()

/**
 * 处理点击事件
 */
function handleClick(event: MouseEvent) {
  emit('click', event)
}
</script>

<style scoped>
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  font-family: inherit;
  font-weight: 500;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s ease-out;
  white-space: nowrap;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* 尺寸 */
.btn-sm {
  padding: 6px 12px;
  font-size: 12px;
  height: 28px;
}

.btn-md {
  padding: 8px 16px;
  font-size: 14px;
  height: 36px;
}

.btn-lg {
  padding: 12px 24px;
  font-size: 16px;
  height: 44px;
}

/* 变体 */
.btn-primary {
  background: var(--steam-accent-blue);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--steam-accent-green);
}

.btn-secondary {
  background: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
}

.btn-secondary:hover:not(:disabled) {
  background: var(--steam-accent-hover);
}

.btn-ghost {
  background: transparent;
  color: var(--steam-text-primary);
}

.btn-ghost:hover:not(:disabled) {
  background: var(--steam-bg-tertiary);
}

.btn-danger {
  background: #e81123;
  color: white;
}

.btn-danger:hover:not(:disabled) {
  background: #ff4d4f;
}

/* 块级 */
.btn-block {
  width: 100%;
}

/* 图标 */
.btn-icon {
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-icon svg {
  width: 16px;
  height: 16px;
}

.btn-sm .btn-icon svg {
  width: 14px;
  height: 14px;
}

.btn-lg .btn-icon svg {
  width: 20px;
  height: 20px;
}

/* 加载动画 */
.btn-spinner {
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-spinner svg {
  width: 16px;
  height: 16px;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.btn-loading .btn-content {
  opacity: 0.7;
}
</style>
