<template>
  <!--
    BaseModal.vue - 通用弹窗基础组件
    整合所有弹窗的通用布局、样式和交互逻辑
    支持表单弹窗、信息弹窗等多种类型
  -->
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="modelValue" class="modal-overlay" @click="handleOverlayClick">
        <div class="modal-container" :class="containerClass" @click.stop>
          <!-- 弹窗头部 -->
          <div class="modal-header">
            <h3 class="modal-title">{{ title }}</h3>
            <button class="close-btn" @click="handleClose">
              <svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
              </svg>
            </button>
          </div>

          <!-- 弹窗内容 -->
          <div class="modal-body" :class="bodyClass">
            <slot name="body"></slot>
          </div>

          <!-- 弹窗底部 -->
          <div v-if="showFooter" class="modal-footer">
            <slot name="footer">
              <!-- 默认底部按钮 -->
              <Button v-if="showCancel" :variant="cancelVariant" @click="handleClose">
                {{ cancelText }}
              </Button>
              <Button
                v-if="showConfirm"
                :variant="confirmVariant"
                :disabled="confirmDisabled"
                @click="handleConfirm"
              >
                {{ confirmText }}
              </Button>
            </slot>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
/**
 * BaseModal.vue - 通用弹窗基础组件
 * 提供统一的弹窗布局、动画和交互控制
 * 所有业务弹窗都应基于此组件构建
 */

import { computed } from 'vue'
import Button from './Button.vue'

// ============================================
// 类型定义
// ============================================

type ButtonVariant = 'primary' | 'secondary' | 'ghost' | 'danger'

// ============================================
// Props 定义
// ============================================

interface Props {
  /** 控制弹窗显示/隐藏 */
  modelValue: boolean
  /** 弹窗标题 */
  title: string
  /** 弹窗宽度 */
  width?: string
  /** 是否显示底部区域 */
  showFooter?: boolean
  /** 是否显示取消按钮 */
  showCancel?: boolean
  /** 取消按钮文本 */
  cancelText?: string
  /** 取消按钮样式 */
  cancelVariant?: ButtonVariant
  /** 是否显示确认按钮 */
  showConfirm?: boolean
  /** 确认按钮文本 */
  confirmText?: string
  /** 确认按钮样式 */
  confirmVariant?: ButtonVariant
  /** 是否禁用确认按钮 */
  confirmDisabled?: boolean
  /** 点击遮罩层是否关闭 */
  closeOnOverlay?: boolean
  /** 自定义容器类名 */
  containerClass?: string
  /** 自定义内容区类名 */
  bodyClass?: string
}

const props = withDefaults(defineProps<Props>(), {
  width: '480px',
  showFooter: true,
  showCancel: true,
  cancelText: '取消',
  cancelVariant: 'ghost',
  showConfirm: true,
  confirmText: '确认',
  confirmVariant: 'primary',
  confirmDisabled: false,
  closeOnOverlay: true,
  containerClass: '',
  bodyClass: ''
})

// ============================================
// Emits 定义
// ============================================

const emit = defineEmits<{
  /** 更新显示状态 */
  (e: 'update:modelValue', value: boolean): void
  /** 关闭弹窗 */
  (e: 'close'): void
  /** 确认操作 */
  (e: 'confirm'): void
}>()

// ============================================
// 计算属性
// ============================================

/**
 * 容器动态样式
 * 根据传入的width设置弹窗宽度
 */
const containerStyle = computed(() => ({
  maxWidth: props.width
}))

// ============================================
// 方法
// ============================================

/**
 * 处理遮罩层点击
 * 根据closeOnOverlay配置决定是否关闭弹窗
 */
function handleOverlayClick() {
  if (props.closeOnOverlay) {
    handleClose()
  }
}

/**
 * 处理关闭操作
 * 触发事件通知父组件关闭弹窗
 */
function handleClose() {
  emit('update:modelValue', false)
  emit('close')
}

/**
 * 处理确认操作
 * 触发confirm事件通知父组件执行确认逻辑
 */
function handleConfirm() {
  emit('confirm')
}
</script>

<style scoped>
/* 弹窗遮罩层 */
.modal-overlay {
  position: fixed;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.7);
  z-index: 2000;
  padding: 24px;
}

/* 弹窗容器 */
.modal-container {
  width: 100%;
  max-width: v-bind('containerStyle.maxWidth');
  background: var(--steam-bg-secondary);
  border-radius: 12px;
  box-shadow: var(--shadow-steam-lg);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  max-height: 90vh;
}

/* 弹窗头部 */
.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--steam-border);
  flex-shrink: 0;
}

.modal-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0;
}

.close-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--steam-text-secondary);
  border-radius: 4px;
  transition: all 0.15s ease-out;
  background: transparent;
  border: none;
  cursor: pointer;
}

.close-btn:hover {
  color: var(--steam-text-primary);
  background: var(--steam-accent-hover);
}

.close-btn svg {
  width: 20px;
  height: 20px;
}

/* 弹窗内容区 */
.modal-body {
  padding: 20px;
  overflow-y: auto;
  flex: 1;
}

/* 弹窗底部 */
.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid var(--steam-border);
  flex-shrink: 0;
}

/* 弹窗动画 */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease-out;
}

.modal-enter-active .modal-container,
.modal-leave-active .modal-container {
  transition: transform 0.2s ease-out, opacity 0.2s ease-out;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .modal-container,
.modal-leave-to .modal-container {
  opacity: 0;
  transform: scale(0.95);
}
</style>
