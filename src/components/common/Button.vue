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
/* 统一按钮设计系统：与全局 .ui-btn 完全一致的外观与反馈
   令牌（圆角/字重/尺寸档/过渡/光晕）统一来自 main.css 的 :root 自定义属性 */
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  font-family: inherit;
  font-weight: var(--btn-fw);
  border: 1px solid transparent;
  border-radius: var(--btn-radius);
  cursor: pointer;
  transition: var(--btn-transition);
  white-space: nowrap;
  user-select: none;
}

/* 悬停上浮 + 按下回弹 */
.btn:hover:not(:disabled) {
  transform: translateY(-1px);
}

.btn:active:not(:disabled) {
  transform: translateY(0) scale(0.97);
}

.btn:disabled {
  opacity: 0.45;
  cursor: not-allowed;
  transform: none !important;
  box-shadow: none !important;
  filter: grayscale(0.4);
}

/* 尺寸档 */
.btn-sm {
  height: var(--btn-h-sm);
  padding: 0 var(--btn-px-sm);
  font-size: var(--btn-fs-sm);
}

.btn-md {
  height: var(--btn-h-md);
  padding: 0 var(--btn-px-md);
  font-size: var(--btn-fs-md);
}

.btn-lg {
  height: var(--btn-h-lg);
  padding: 0 var(--btn-px-lg);
  font-size: var(--btn-fs-lg);
}

/* 变体 */
.btn-primary {
  background: var(--steam-accent-blue);
  border-color: transparent;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--steam-accent-blue-hover);
  box-shadow: var(--btn-glow-primary);
}

/* 次级：描边玻璃（与“更新游戏”按钮同质感，悬停映射品牌蓝） */
.btn-secondary {
  background: var(--btn-outline-bg);
  border-color: var(--glass-border-strong);
  color: var(--steam-text-primary);
}

.btn-secondary:hover:not(:disabled) {
  background: var(--btn-outline-bg-hover);
  border-color: var(--steam-accent-blue);
  color: var(--steam-accent-blue);
}

.btn-ghost {
  background: transparent;
  border-color: transparent;
  color: var(--steam-text-secondary);
}

.btn-ghost:hover:not(:disabled) {
  background: var(--steam-accent-hover);
  color: var(--steam-text-primary);
}

.btn-danger {
  background: var(--steam-error);
  border-color: transparent;
  color: white;
}

.btn-danger:hover:not(:disabled) {
  background: var(--steam-error-hover);
  box-shadow: var(--btn-glow-danger);
}

.btn-success {
  background: var(--steam-accent-green);
  border-color: transparent;
  color: white;
}

.btn-success:hover:not(:disabled) {
  background: var(--steam-accent-green-hover);
  box-shadow: var(--btn-glow-success);
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
  width: var(--btn-ic-md);
  height: var(--btn-ic-md);
}

.btn-sm .btn-icon svg {
  width: var(--btn-ic-sm);
  height: var(--btn-ic-sm);
}

.btn-lg .btn-icon svg {
  width: var(--btn-ic-lg);
  height: var(--btn-ic-lg);
}

/* 加载动画 */
.btn-spinner {
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-spinner svg {
  width: var(--btn-ic-md);
  height: var(--btn-ic-md);
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
