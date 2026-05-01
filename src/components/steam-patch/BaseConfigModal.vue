<template>
  <!--
    BaseConfigModal.vue - Steam补丁配置基础弹窗组件
    所有配置弹窗的基座组件，提供统一的布局、样式和交互
  -->
  <div class="modal-overlay" @click="handleClose">
    <div class="modal-content" @click.stop>
      <!-- 弹窗头部 -->
      <div class="modal-header">
        <div class="header-icon" :class="iconClass" :style="iconStyle">
          <slot name="icon">
            <!-- 默认图标 -->
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="3"/>
              <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
            </svg>
          </slot>
        </div>
        <h3>{{ title }}</h3>
        <button class="close-btn" @click="handleClose">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <!-- 弹窗内容区域 -->
      <div class="modal-body">
        <slot name="body"></slot>
      </div>

      <!-- 弹窗底部 -->
      <div class="modal-footer">
        <button class="btn-secondary" @click="handleClose">{{ cancelText }}</button>
        <button
          class="btn-primary"
          :disabled="saveDisabled"
          @click="handleSave"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="20 6 9 17 4 12"/>
          </svg>
          {{ saveText }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * BaseConfigModal.vue - Steam补丁配置基础弹窗组件
 * 提供统一的弹窗布局、头部图标、底部按钮等基础功能
 */

import { computed } from 'vue'

/**
 * 组件属性定义
 */
interface Props {
  /** 弹窗标题 */
  title: string
  /** 图标样式类名 */
  iconClass?: string
  /** 图标颜色（用于动态设置背景色） */
  iconColor?: string
  /** 是否禁用保存按钮 */
  saveDisabled?: boolean
  /** 取消按钮文本 */
  cancelText?: string
  /** 保存按钮文本 */
  saveText?: string
}

const props = withDefaults(defineProps<Props>(), {
  iconClass: '',
  iconColor: '#64748b',
  saveDisabled: false,
  cancelText: '取消',
  saveText: '保存配置'
})

/**
 * 组件事件定义
 */
const emit = defineEmits<{
  /** 关闭弹窗 */
  (e: 'close'): void
  /** 保存配置 */
  (e: 'save'): void
}>()

/**
 * 计算图标容器的动态样式
 * 根据传入的iconColor生成对应的背景色（10%透明度）
 */
const iconStyle = computed(() => ({
  backgroundColor: `${props.iconColor}1A`, // 10%透明度
  color: props.iconColor
}))

/**
 * 处理关闭事件
 * 触发close事件通知父组件关闭弹窗
 */
function handleClose() {
  emit('close')
}

/**
 * 处理保存事件
 * 触发save事件通知父组件执行保存操作
 */
function handleSave() {
  emit('save')
}
</script>

<style scoped>
/* 弹窗遮罩层 - 全屏半透明黑色背景 */
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

/* 弹窗内容容器 */
.modal-content {
  background-color: var(--steam-bg-primary);
  border-radius: 12px;
  border: 1px solid var(--steam-border);
  width: 90%;
  max-width: 600px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
}

/* 弹窗头部区域 */
.modal-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 20px;
  border-bottom: 1px solid var(--steam-border);
  flex-shrink: 0;
}

/* 头部图标容器 */
.header-icon {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.header-icon svg {
  width: 24px;
  height: 24px;
}

/* 弹窗标题 */
.modal-header h3 {
  flex: 1;
  font-size: 18px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0;
}

/* 关闭按钮 */
.close-btn {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 8px;
  background-color: transparent;
  color: var(--steam-text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
  flex-shrink: 0;
}

.close-btn:hover {
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
}

.close-btn svg {
  width: 18px;
  height: 18px;
}

/* 弹窗内容区域 - 可滚动 */
.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

/* 弹窗底部区域 */
.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid var(--steam-border);
  flex-shrink: 0;
}

/* 主要按钮样式 */
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

.btn-primary:hover:not(:disabled) {
  background-color: var(--steam-accent-hover);
}

.btn-primary:disabled {
  background-color: var(--steam-text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

.btn-primary svg {
  width: 16px;
  height: 16px;
}

/* 次要按钮样式 */
.btn-secondary {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
}

.btn-secondary:hover {
  background-color: var(--steam-border);
}
</style>
