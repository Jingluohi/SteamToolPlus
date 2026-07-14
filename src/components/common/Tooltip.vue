<template>
  <!-- 
    Tooltip.vue - 提示框组件
    鼠标悬停时显示提示文本
  -->
  <div class="tooltip-wrapper">
    <!-- 触发元素 -->
    <slot />
    
    <!-- 提示内容 -->
    <div 
      class="tooltip-content" 
      :class="[`tooltip-${position}`]"
    >
      {{ text }}
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * Tooltip.vue - 提示框组件
 * 鼠标悬停显示提示文本，支持多个方向
 */

/**
 * 组件属性定义
 */
interface Props {
  /** 提示文本内容 */
  text: string
  /** 提示框位置 */
  position?: 'top' | 'bottom' | 'left' | 'right'
}

withDefaults(defineProps<Props>(), {
  position: 'top'
})
</script>

<style scoped>
.tooltip-wrapper {
  position: relative;
  display: inline-block;
}

.tooltip-content {
  /* 默认隐藏 */
  visibility: hidden;
  opacity: 0;
  
  /* 定位基础 */
  position: absolute;
  z-index: 1000;
  
  /* 样式 */
  padding: 8px 12px;
  background: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 12px;
  line-height: 1.5;
  border-radius: 4px;
  border: 1px solid var(--steam-border);
  box-shadow: var(--shadow-steam);
  
  /* 最大宽度限制 */
  max-width: 280px;
  word-wrap: break-word;
  
  /* 过渡动画 */
  transition: opacity 0.15s ease-out, visibility 0.15s ease-out;
  
  /* 禁止选中 */
  user-select: none;
  
  /* 默认指针事件 */
  pointer-events: none;
}

/* 悬停时显示 */
.tooltip-wrapper:hover .tooltip-content {
  visibility: visible;
  opacity: 1;
}

/* 位置样式 - top */
.tooltip-top {
  bottom: calc(100% + 8px);
  left: 50%;
  transform: translateX(-50%);
}

/* 位置样式 - bottom */
.tooltip-bottom {
  top: calc(100% + 8px);
  left: 50%;
  transform: translateX(-50%);
}

/* 位置样式 - left */
.tooltip-left {
  right: calc(100% + 8px);
  top: 50%;
  transform: translateY(-50%);
}

/* 位置样式 - right */
.tooltip-right {
  left: calc(100% + 8px);
  top: 50%;
  transform: translateY(-50%);
}

/* 小箭头指示器 */
.tooltip-content::before {
  content: '';
  position: absolute;
  width: 8px;
  height: 8px;
  background: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  transform: rotate(45deg);
}

/* top 位置的箭头 */
.tooltip-top::before {
  bottom: -5px;
  left: 50%;
  margin-left: -4px;
  border-top: none;
  border-left: none;
}

/* bottom 位置的箭头 */
.tooltip-bottom::before {
  top: -5px;
  left: 50%;
  margin-left: -4px;
  border-bottom: none;
  border-right: none;
}

/* left 位置的箭头 */
.tooltip-left::before {
  right: -5px;
  top: 50%;
  margin-top: -4px;
  border-left: none;
  border-bottom: none;
}

/* right 位置的箭头 */
.tooltip-right::before {
  left: -5px;
  top: 50%;
  margin-top: -4px;
  border-right: none;
  border-top: none;
}
</style>