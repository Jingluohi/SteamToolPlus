<template>
  <!--
    InstallExtensionModal.vue - 安装扩展弹窗
    使用 BaseModal 基础组件
  -->
  <BaseModal
    :model-value="modelValue"
    title="安装扩展"
    :confirm-disabled="!packagePath"
    confirm-text="安装"
    @update:model-value="$emit('update:modelValue', $event)"
    @close="handleClose"
    @confirm="handleConfirm"
  >
    <template #body>
      <div class="form-group">
        <label class="form-label">选择扩展包</label>
        <p class="form-hint">支持 .7z 格式的扩展包</p>
        <div class="input-with-btn">
          <input
            v-model="packagePath"
            type="text"
            class="form-input"
            placeholder="选择扩展包文件"
            readonly
          />
          <Button variant="secondary" @click="selectPackage">浏览...</Button>
        </div>
      </div>

      <div class="info-box">
        <h4>扩展包格式要求：</h4>
        <ul>
          <li v-for="(item, index) in requirements" :key="index">{{ item }}</li>
        </ul>
      </div>
    </template>
  </BaseModal>
</template>

<script setup lang="ts">
/**
 * InstallExtensionModal.vue - 安装扩展弹窗组件
 * 使用 BaseModal 基础组件，代码量减少约60%
 */

import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import Button from '../../../components/common/Button.vue'
import BaseModal from '../../../components/common/BaseModal.vue'

// ============================================
// 常量定义
// ============================================

/**
 * 扩展包格式要求列表
 * 集中管理要求说明，便于维护
 */
const requirements = [
  '扩展包必须是 .7z 压缩格式',
  '必须包含 manifest.json 清单文件',
  '必须包含 index.js 入口文件',
  '可选包含 style.css 样式文件和 logo.png 图标'
] as const

// ============================================
// Props 和 Emits
// ============================================

const props = defineProps<{
  modelValue: boolean
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'confirm', packagePath: string): void
}>()

// ============================================
// 响应式状态
// ============================================

const packagePath = ref('')

// ============================================
// 方法
// ============================================

/**
 * 选择扩展包文件
 * 打开文件选择对话框，仅显示.7z文件
 */
async function selectPackage() {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: '扩展包',
        extensions: ['7z']
      }]
    })

    if (selected && typeof selected === 'string') {
      packagePath.value = selected
    }
  } catch (err) {
    console.error('选择文件失败:', err)
  }
}

/**
 * 关闭弹窗
 * 清空已选路径
 */
function handleClose() {
  emit('update:modelValue', false)
  packagePath.value = ''
}

/**
 * 确认安装
 * 触发确认事件
 */
function handleConfirm() {
  if (!packagePath.value) return

  emit('confirm', packagePath.value)
}
</script>

<style scoped>
/* 表单组 */
.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 20px;
}

/* 表单标签 */
.form-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-text-secondary);
}

/* 表单提示 */
.form-hint {
  font-size: 12px;
  color: var(--steam-text-muted);
  margin: 0;
}

/* 表单输入框 */
.form-input {
  height: 40px;
  padding: 0 12px;
  background: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border);
  border-radius: 4px;
  color: var(--steam-text-primary);
  font-size: 14px;
  transition: border-color 0.15s ease-out;
  width: 100%;
}

.form-input:focus {
  border-color: var(--steam-accent-blue);
  outline: none;
}

/* 带按钮的输入框布局 */
.input-with-btn {
  display: flex;
  gap: 8px;
}

.input-with-btn .form-input {
  flex: 1;
}

/* 信息框 */
.info-box {
  padding: 16px;
  background: var(--steam-bg-tertiary);
  border-radius: 8px;
}

.info-box h4 {
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-text-primary);
  margin: 0 0 8px 0;
}

.info-box ul {
  list-style: disc;
  padding-left: 20px;
  margin: 0;
}

.info-box li {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin-bottom: 4px;
}

.info-box li:last-child {
  margin-bottom: 0;
}
</style>
