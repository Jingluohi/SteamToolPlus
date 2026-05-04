<template>
  <!--
    GameFormDialog.vue - 游戏表单对话框组件
    复用导入和编辑游戏的表单逻辑
  -->
  <div v-if="visible" class="dialog-overlay" @click.self="handleClose">
    <div class="dialog">
      <div class="dialog-header">
        <h3>{{ title }}</h3>
        <button class="close-btn" @click="handleClose">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
          </svg>
        </button>
      </div>
      <div class="dialog-body">
        <div class="form-group">
          <label>游戏名称（英文）</label>
          <input v-model="form.game_name" type="text" placeholder="例如: Elden Ring" />
        </div>
        <div class="form-group">
          <label>游戏中文名</label>
          <input v-model="form.chinese_name" type="text" placeholder="例如: 艾尔登法环" />
        </div>
        <div class="form-group">
          <label>游戏安装目录</label>
          <div class="path-input-group">
            <input v-model="form.install_path" type="text" readonly placeholder="请选择游戏安装目录..." />
            <button @click="selectPath('install_path', true)">浏览</button>
          </div>
        </div>
        <div class="form-group">
          <label>游戏主程序 (exe)</label>
          <div class="path-input-group">
            <input v-model="form.exe_path" type="text" readonly placeholder="请选择游戏主程序..." />
            <button @click="selectExePath">浏览</button>
          </div>
        </div>
        <div class="form-group">
          <label>游戏存档目录（可选）</label>
          <div class="path-input-group">
            <input v-model="form.save_path" type="text" readonly placeholder="请选择存档目录..." />
            <button @click="selectPath('save_path', true)">浏览</button>
          </div>
        </div>
        <div class="form-group">
          <label>游戏封面（可选）</label>
          <div class="path-input-group">
            <input v-model="form.cover_path" type="text" readonly placeholder="请选择封面图片..." />
            <button @click="selectPath('cover_path', false, [{ name: '图片文件', extensions: ['jpg', 'jpeg', 'png', 'webp'] }])">浏览</button>
          </div>
        </div>
        <div class="form-group">
          <label>Steam游戏ID（可选）</label>
          <input v-model="form.steam_game_id" type="text" placeholder="例如: 1245620" />
        </div>
      </div>
      <div class="dialog-footer">
        <button class="cancel-btn" @click="handleClose">取消</button>
        <button class="confirm-btn" @click="handleConfirm" :disabled="!canConfirm">{{ confirmText }}</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * GameFormDialog.vue - 游戏表单对话框组件
 * 统一导入和编辑游戏的表单逻辑，减少代码重复
 */

import { computed, watch } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'

/**
 * 表单数据接口
 */
export interface GameFormData {
  game_name: string
  chinese_name: string
  install_path: string
  exe_path: string
  save_path: string
  cover_path: string
  steam_game_id: string
}

/**
 * 组件属性定义
 */
interface Props {
  /** 是否显示对话框 */
  visible: boolean
  /** 对话框标题 */
  title: string
  /** 确认按钮文字 */
  confirmText: string
  /** 表单数据 */
  modelValue: GameFormData
  /** 是否允许自动填充游戏名（导入时有用） */
  autoFillName?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  autoFillName: false
})

/**
 * 组件事件定义
 */
const emit = defineEmits<{
  (e: 'update:modelValue', value: GameFormData): void
  (e: 'close'): void
  (e: 'confirm', form: GameFormData): void
}>()

// 计算属性：是否可以确认
const canConfirm = computed(() => {
  return props.modelValue.game_name &&
         props.modelValue.chinese_name &&
         props.modelValue.install_path &&
         props.modelValue.exe_path
})

// 监听表单变化，自动填充游戏名
watch(() => props.modelValue.exe_path, (newPath) => {
  if (props.autoFillName && newPath) {
    const pathParts = newPath.split(/[\\/]/)
    const exeName = pathParts[pathParts.length - 1] || ''
    const gameName = exeName.replace('.exe', '')
    if (!props.modelValue.game_name) {
      updateForm('game_name', gameName)
    }
    if (!props.modelValue.chinese_name) {
      updateForm('chinese_name', gameName)
    }
  }
})

/**
 * 更新表单字段
 */
function updateForm<K extends keyof GameFormData>(key: K, value: GameFormData[K]) {
  emit('update:modelValue', { ...props.modelValue, [key]: value })
}

/**
 * 通用路径选择
 * @param field 表单字段名
 * @param isDirectory 是否选择目录
 * @param filters 文件过滤器（仅文件选择时有效）
 */
async function selectPath(field: keyof GameFormData, isDirectory: boolean, filters?: { name: string; extensions: string[] }[]) {
  const selected = await open({
    directory: isDirectory,
    title: getDialogTitle(field),
    filters: isDirectory ? undefined : filters
  })
  if (selected) {
    updateForm(field, selected as string)
  }
}

/**
 * 选择 exe 路径（特殊处理：自动填充游戏名）
 */
async function selectExePath() {
  const selected = await open({
    title: '选择游戏主程序',
    filters: [{ name: '可执行文件', extensions: ['exe'] }]
  })
  if (selected) {
    updateForm('exe_path', selected as string)
  }
}

/**
 * 获取对话框标题
 */
function getDialogTitle(field: keyof GameFormData): string {
  const titles: Record<string, string> = {
    install_path: '选择游戏安装目录',
    save_path: '选择游戏存档目录',
    cover_path: '选择游戏封面图片'
  }
  return titles[field] || '选择文件'
}

/**
 * 关闭对话框
 */
function handleClose() {
  emit('close')
}

/**
 * 确认提交
 */
function handleConfirm() {
  emit('confirm', props.modelValue)
}
</script>

<style scoped>
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(10px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: var(--steam-bg-primary);
  backdrop-filter: blur(30px) saturate(200%);
  border-radius: 16px;
  width: 500px;
  max-height: 85vh;
  overflow: hidden;
  box-shadow: 0 24px 64px rgba(0, 0, 0, 0.6);
  border: 1px solid var(--steam-border);
  display: flex;
  flex-direction: column;
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px;
  border-bottom: 1px solid var(--steam-border);
}

.dialog-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.close-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  color: var(--steam-text-muted);
  cursor: pointer;
  transition: all 0.2s ease;
}

.close-btn:hover {
  background: var(--steam-bg-hover);
  color: var(--steam-text-primary);
}

.close-btn svg {
  width: 18px;
  height: 18px;
}

.dialog-body {
  padding: 20px 24px;
  overflow-y: auto;
  flex: 1;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  margin-bottom: 6px;
  font-size: 12px;
  color: var(--steam-text-secondary);
  font-weight: 500;
}

.form-group input {
  width: 100%;
  padding: 10px 14px;
  background: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  color: var(--steam-text-primary);
  font-size: 13px;
  outline: none;
  transition: all 0.2s ease;
}

.form-group input:focus {
  border-color: var(--steam-accent-blue);
  background: var(--steam-bg-hover);
  box-shadow: 0 0 0 3px rgba(var(--steam-accent-blue-rgb), 0.1);
}

.path-input-group {
  display: flex;
  gap: 8px;
}

.path-input-group input {
  flex: 1;
}

.path-input-group button {
  padding: 10px 16px;
  background: var(--steam-bg-tertiary);
  color: var(--steam-text-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.path-input-group button:hover {
  background: var(--steam-bg-hover);
  border-color: var(--steam-border-light);
  color: var(--steam-text-primary);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px;
  border-top: 1px solid var(--steam-border);
}

.cancel-btn {
  padding: 10px 20px;
  background: transparent;
  color: var(--steam-text-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.cancel-btn:hover {
  background: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
  border-color: var(--steam-border-light);
}

.confirm-btn {
  padding: 10px 24px;
  background: var(--steam-accent-blue);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 4px 12px rgba(var(--steam-accent-blue-rgb), 0.3);
}

.confirm-btn:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 6px 16px rgba(var(--steam-accent-blue-rgb), 0.4);
}

.confirm-btn:disabled {
  background: var(--steam-bg-hover);
  cursor: not-allowed;
  box-shadow: none;
}
</style>
