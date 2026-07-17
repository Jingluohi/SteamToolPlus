<template>
  <!--
    SliderToggle.vue - 左右滑块开关组件
    左标签 = modelValue 为 false 时的状态
    右标签 = modelValue 为 true 时的状态
  -->
  <div class="slider-toggle-wrapper">
    <span
      class="slider-toggle-label"
      :class="{ active: !modelValue, inactive: modelValue }"
    >
      {{ leftLabel }}
    </span>
    <button
      class="slider-toggle"
      :class="{ checked: modelValue }"
      :aria-checked="modelValue"
      role="switch"
      @click="toggle"
    >
      <span class="slider-toggle-track">
        <span class="slider-toggle-thumb" />
      </span>
    </button>
    <span
      class="slider-toggle-label"
      :class="{ active: modelValue, inactive: !modelValue }"
    >
      {{ rightLabel }}
    </span>
  </div>
</template>

<script setup lang="ts">
/**
 * SliderToggle.vue - 左右滑块开关组件
 */

/**
 * 组件属性定义
 */
interface Props {
  /** 当前值：false=左侧，true=右侧 */
  modelValue: boolean
  /** 左侧标签 */
  leftLabel: string
  /** 右侧标签 */
  rightLabel: string
}

const props = defineProps<Props>()

/**
 * 组件事件定义
 */
const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
}>()

/**
 * 切换开关状态
 */
function toggle() {
  emit('update:modelValue', !props.modelValue)
}
</script>

<style scoped>
.slider-toggle-wrapper {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  user-select: none;
}

.slider-toggle-label {
  font-size: 13px;
  transition: color 0.2s ease;
}

.slider-toggle-label.active {
  color: var(--steam-text-primary);
  font-weight: 500;
}

.slider-toggle-label.inactive {
  color: var(--steam-text-muted);
}

.slider-toggle {
  width: 48px;
  height: 26px;
  padding: 2px;
  background: transparent;
  border: none;
  cursor: pointer;
  flex-shrink: 0;
}

.slider-toggle-track {
  display: block;
  width: 100%;
  height: 100%;
  background: var(--steam-bg-tertiary);
  border-radius: 13px;
  position: relative;
  transition: background 0.2s ease-out;
  border: 1px solid var(--steam-border);
}

.slider-toggle-thumb {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 18px;
  height: 18px;
  background: white;
  border-radius: 50%;
  transition: transform 0.2s ease-out;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.slider-toggle.checked .slider-toggle-track {
  background: var(--steam-accent-blue);
  border-color: var(--steam-accent-blue);
}

.slider-toggle.checked .slider-toggle-thumb {
  transform: translateX(22px);
}
</style>
