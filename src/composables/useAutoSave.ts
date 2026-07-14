/**
 * useAutoSave composable
 * 统一处理延迟保存逻辑，避免组件间重复的 setTimeout 代码
 */

import { ref, onUnmounted } from 'vue'

/**
 * 自动保存配置
 */
interface AutoSaveOptions {
  /** 保存延迟时间（毫秒），默认 300ms */
  delay?: number
  /** 保存回调函数 */
  onSave: () => Promise<void> | void
  /** 保存前的校验函数（可选） */
  onValidate?: () => boolean
}

/**
 * 自动保存 composable
 * 提供延迟保存功能，避免频繁触发保存操作
 */
export function useAutoSave(options: AutoSaveOptions) {
  const { delay = 300, onSave, onValidate } = options

  // 保存定时器
  let saveTimer: ReturnType<typeof setTimeout> | null = null
  // 是否正在保存
  const isSaving = ref(false)
  // 是否有待保存的更改
  const hasPendingChanges = ref(false)

  /**
   * 取消待执行的保存
   */
  function cancelPending() {
    if (saveTimer) {
      clearTimeout(saveTimer)
      saveTimer = null
    }
  }

  /**
   * 立即执行保存
   */
  async function saveNow(): Promise<void> {
    cancelPending()
    
    // 校验
    if (onValidate && !onValidate()) {
      return
    }

    isSaving.value = true
    try {
      await onSave()
      hasPendingChanges.value = false
    } finally {
      isSaving.value = false
    }
  }

  /**
   * 触发延迟保存
   * 如果已有待保存的更改，重置定时器
   */
  function triggerSave(): void {
    hasPendingChanges.value = true
    cancelPending()
    
    saveTimer = setTimeout(async () => {
      await saveNow()
    }, delay)
  }

  /**
   * 强制保存（用于组件卸载前）
   */
  async function forceSave(): Promise<void> {
    if (hasPendingChanges.value) {
      cancelPending()
      await saveNow()
    }
  }

  // 组件卸载时清理定时器
  onUnmounted(() => {
    cancelPending()
  })

  return {
    /** 是否正在保存 */
    isSaving,
    /** 是否有待保存的更改 */
    hasPendingChanges,
    /** 触发延迟保存 */
    triggerSave,
    /** 立即保存 */
    saveNow,
    /** 强制保存（用于组件卸载前） */
    forceSave,
    /** 取消待执行的保存 */
    cancelPending
  }
}

/**
 * 创建防抖保存函数
 * 用于简单的防抖场景，不需要完整的状态管理
 */
export function useDebouncedSave(saveFn: () => Promise<void> | void, delay = 300) {
  let timer: ReturnType<typeof setTimeout> | null = null

  function debouncedSave() {
    if (timer) {
      clearTimeout(timer)
    }
    timer = setTimeout(async () => {
      await saveFn()
      timer = null
    }, delay)
  }

  function cancel() {
    if (timer) {
      clearTimeout(timer)
      timer = null
    }
  }

  onUnmounted(cancel)

  return {
    debouncedSave,
    cancel
  }
}