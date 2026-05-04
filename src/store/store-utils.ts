/**
 * store-utils.ts - Store工具函数
 * 提供通用的store action包装器和状态管理工具
 */

import { ref, type Ref } from 'vue'

/**
 * 创建带加载状态和错误处理的store action
 * @returns 包装器函数和状态
 */
export function createAsyncState() {
  const loading = ref(false)
  const error = ref<string | null>(null)

  /**
   * 包装异步action
   * @param action - 要执行的异步函数
   * @param context - 错误上下文
   * @returns 执行结果或undefined
   */
  async function execute<T>(
    action: () => Promise<T>,
    context: string
  ): Promise<T | undefined> {
    loading.value = true
    error.value = null

    try {
      return await action()
    } catch (err) {
      error.value = err instanceof Error ? err.message : `${context}失败`
      return undefined
    } finally {
      loading.value = false
    }
  }

  return {
    loading,
    error,
    execute
  }
}

/**
 * 创建列表状态的store
 * @returns 列表状态和操作方法
 */
export function createListState<T>() {
  const items = ref<T[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  /**
   * 设置列表数据
   */
  function setItems(newItems: T[]) {
    items.value = newItems
  }

  /**
   * 添加单个项目
   */
  function addItem(item: T) {
    items.value.push(item)
  }

  /**
   * 移除单个项目
   */
  function removeItem(predicate: (item: T) => boolean) {
    const index = items.value.findIndex(predicate)
    if (index !== -1) {
      items.value.splice(index, 1)
    }
  }

  /**
   * 更新单个项目
   */
  function updateItem(predicate: (item: T) => boolean, updater: (item: T) => T) {
    const index = items.value.findIndex(predicate)
    if (index !== -1) {
      items.value[index] = updater(items.value[index])
    }
  }

  return {
    items,
    loading,
    error,
    setItems,
    addItem,
    removeItem,
    updateItem
  }
}

/**
   * 创建单个实体状态的store
 * @param defaultValue - 默认值
 * @returns 实体状态和操作方法
 */
export function createEntityState<T>(defaultValue: T) {
  const entity = ref<T>(defaultValue)
  const loading = ref(false)
  const error = ref<string | null>(null)

  /**
   * 设置实体数据
   */
  function setEntity(newEntity: T) {
    entity.value = newEntity
  }

  /**
   * 重置为默认值
   */
  function reset() {
    entity.value = defaultValue
  }

  return {
    entity,
    loading,
    error,
    setEntity,
    reset
  }
}

/**
 * 简化版的异步action包装器
 * 适用于不需要loading状态的场景
 * @param action - 异步函数
 * @param errorMessage - 错误消息
 */
export async function safeAsync<T>(
  action: () => Promise<T>,
  errorMessage: string
): Promise<T | undefined> {
  try {
    return await action()
  } catch (err) {
    return undefined
  }
}
