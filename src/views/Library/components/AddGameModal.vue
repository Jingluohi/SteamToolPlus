<template>
  <!--
    AddGameModal.vue - 添加游戏弹窗
    使用 BaseModal 基础组件
  -->
  <BaseModal
    :model-value="modelValue"
    title="添加游戏"
    :confirm-disabled="!isValid"
    confirm-text="添加"
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
        <label class="form-label">可执行文件路径</label>
        <div class="input-with-btn">
          <input
            v-model="form.exePath"
            type="text"
            class="form-input"
            placeholder="选择游戏exe文件"
            readonly
          />
          <Button variant="secondary" @click="selectExeFile">浏览...</Button>
        </div>
      </div>

      <div class="form-group">
        <label class="form-label">封面图（可选）</label>
        <div class="input-with-btn">
          <input
            v-model="form.coverPath"
            type="text"
            class="form-input"
            placeholder="选择封面图片"
            readonly
          />
          <Button variant="secondary" @click="selectCoverFile">浏览...</Button>
        </div>
      </div>

      <div class="form-group">
        <label class="form-label">启动参数（可选）</label>
        <input
          v-model="form.launchParams"
          type="text"
          class="form-input"
          placeholder="输入启动参数"
        />
      </div>

      <div class="form-group">
        <label class="form-label">发行商（可选）</label>
        <input
          v-model="form.publisher"
          type="text"
          class="form-input"
          placeholder="输入发行商"
        />
      </div>

      <div class="form-group">
        <label class="form-label">标签（可选，用逗号分隔）</label>
        <input
          v-model="tagsInput"
          type="text"
          class="form-input"
          placeholder="例如：动作,冒险,RPG"
        />
      </div>
    </template>
  </BaseModal>
</template>

<script setup lang="ts">
/**
 * AddGameModal.vue - 添加游戏弹窗组件
 * 使用 BaseModal 基础组件，代码量减少约60%
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
  exePath: string
  coverPath: string
  launchParams: string
  publisher: string
}

interface ConfirmData {
  name: string
  exePath: string
  coverPath?: string
  launchParams?: string
  publisher?: string
  tags?: string[]
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
  exePath: '',
  coverPath: '',
  launchParams: '',
  publisher: ''
})

const tagsInput = ref('')

// ============================================
// 计算属性
// ============================================

/**
 * 表单是否有效
 * 游戏名称和可执行文件路径为必填项
 */
const isValid = computed(() => {
  return form.value.name.trim() && form.value.exePath.trim()
})

// ============================================
// 方法
// ============================================

/**
 * 选择exe文件
 * 打开文件选择对话框，自动提取文件名作为游戏名
 */
async function selectExeFile() {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: '可执行文件',
        extensions: ['exe']
      }]
    })

    if (selected && typeof selected === 'string') {
      form.value.exePath = selected

      // 自动提取文件名作为游戏名
      if (!form.value.name) {
        const fileName = selected.split(/[\\/]/).pop() || ''
        form.value.name = fileName.replace('.exe', '')
      }
    }
  } catch (err) {
    console.error('选择文件失败:', err)
  }
}

/**
 * 选择封面文件
 * 打开文件选择对话框选择图片文件
 */
async function selectCoverFile() {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: '图片文件',
        extensions: ['jpg', 'jpeg', 'png', 'webp']
      }]
    })

    if (selected && typeof selected === 'string') {
      form.value.coverPath = selected
    }
  } catch (err) {
    console.error('选择文件失败:', err)
  }
}

/**
 * 关闭弹窗
 * 重置表单并触发关闭事件
 */
function handleClose() {
  emit('update:modelValue', false)
  resetForm()
}

/**
 * 确认添加
 * 解析标签并触发确认事件
 */
function handleConfirm() {
  if (!isValid.value) return

  const tags = tagsInput.value
    .split(/[,，]/)
    .map(t => t.trim())
    .filter(t => t)

  emit('confirm', {
    name: form.value.name.trim(),
    exePath: form.value.exePath.trim(),
    coverPath: form.value.coverPath || undefined,
    launchParams: form.value.launchParams || undefined,
    publisher: form.value.publisher || undefined,
    tags: tags.length > 0 ? tags : undefined
  })

  handleClose()
}

/**
 * 重置表单
 * 将所有字段恢复到初始状态
 */
function resetForm() {
  form.value = {
    name: '',
    exePath: '',
    coverPath: '',
    launchParams: '',
    publisher: ''
  }
  tagsInput.value = ''
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
