<template>
  <!--
    NewDownloadModal.vue - 新建下载弹窗
    使用 BaseModal 基础组件
  -->
  <BaseModal
    :model-value="modelValue"
    title="新建下载"
    :confirm-disabled="!isValid"
    confirm-text="开始下载"
    @update:model-value="$emit('update:modelValue', $event)"
    @close="handleClose"
    @confirm="handleConfirm"
  >
    <template #body>
      <div class="form-group">
        <label class="form-label">游戏名称</label>
        <input
          v-model="form.name"
          type="text"
          class="form-input"
          placeholder="输入游戏名称"
        />
      </div>

      <div class="form-group">
        <label class="form-label">下载链接</label>
        <input
          v-model="form.url"
          type="text"
          class="form-input"
          placeholder="输入下载链接"
        />
      </div>

      <div class="form-group">
        <label class="form-label">保存路径</label>
        <div class="input-with-btn">
          <input
            v-model="form.savePath"
            type="text"
            class="form-input"
            placeholder="选择保存路径"
            readonly
          />
          <Button variant="secondary" @click="selectSavePath">浏览...</Button>
        </div>
      </div>
    </template>
  </BaseModal>
</template>

<script setup lang="ts">
/**
 * NewDownloadModal.vue - 新建下载弹窗组件
 * 使用 BaseModal 基础组件，代码量减少约65%
 */

import { ref, computed } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import Button from '../../../components/common/Button.vue'
import BaseModal from '../../../components/common/BaseModal.vue'

// ============================================
// 类型定义
// ============================================

interface FormData {
  name: string
  url: string
  savePath: string
}

interface ConfirmData {
  name: string
  url: string
  savePath?: string
}

// ============================================
// Props 和 Emits
// ============================================

const props = defineProps<{
  modelValue: boolean
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'confirm', data: ConfirmData): void
}>()

// ============================================
// 响应式状态
// ============================================

const form = ref<FormData>({
  name: '',
  url: '',
  savePath: ''
})

// ============================================
// 计算属性
// ============================================

/**
 * 表单是否有效
 * 游戏名称和下载链接为必填项
 */
const isValid = computed(() => {
  return form.value.name.trim() && form.value.url.trim()
})

// ============================================
// 方法
// ============================================

/**
 * 选择保存路径
 * 打开目录选择对话框
 */
async function selectSavePath() {
  try {
    const selected = await open({
      directory: true,
      multiple: false
    })

    if (selected && typeof selected === 'string') {
      form.value.savePath = selected
    }
  } catch (err) {
    console.error('选择路径失败:', err)
  }
}

/**
 * 关闭弹窗
 * 重置表单
 */
function handleClose() {
  emit('update:modelValue', false)
  resetForm()
}

/**
 * 确认下载
 * 触发确认事件
 */
function handleConfirm() {
  if (!isValid.value) return

  emit('confirm', {
    name: form.value.name.trim(),
    url: form.value.url.trim(),
    savePath: form.value.savePath || undefined
  })

  handleClose()
}

/**
 * 重置表单
 */
function resetForm() {
  form.value = {
    name: '',
    url: '',
    savePath: ''
  }
}
</script>

<style scoped>
/* 表单组 */
.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 16px;
}

.form-group:last-child {
  margin-bottom: 0;
}

/* 表单标签 */
.form-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-text-secondary);
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
</style>
