<template>
  <!-- 
    Toggle.vue - 开关组件
    用于设置页面的布尔值切换
  -->
  <button
    class="toggle"
    :class="{ checked: modelValue }"
    @click="toggle"
  >
    <span class="toggle-track">
      <span class="toggle-thumb" />
    </span>
  </button>
</template>

<script setup lang="ts">
/**
 * Toggle.vue - 开关组件
 */

/**
 * 组件属性定义
 */
interface Props {
  modelValue: boolean
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
.toggle {
  width: 44px;
  height: 24px;
  padding: 2px;
  background: transparent;
  border: none;
  cursor: pointer;
}

.toggle-track {
  display: block;
  width: 100%;
  height: 100%;
  background: var(--steam-bg-tertiary);
  border-radius: 12px;
  position: relative;
  transition: background 0.2s ease-out;
}

.toggle-thumb {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 16px;
  height: 16px;
  background: white;
  border-radius: 50%;
  transition: transform 0.2s ease-out;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.toggle.checked .toggle-track {
  background: var(--steam-accent-blue);
}

.toggle.checked .toggle-thumb {
  transform: translateX(20px);
}
</style>
