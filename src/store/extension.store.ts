/**
 * extension.store.ts - 扩展系统状态管理
 * 使用Pinia实现扩展相关的全局状态
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Extension, ExtensionLoadStatus } from '../types/extension.types'
import * as extensionApi from '../api/extension.api'

/**
 * 扩展Store
 */
export const useExtensionStore = defineStore('extension', () => {
  // ==================== State ====================
  const extensions = ref<Extension[]>([])
  const selectedExtension = ref<Extension | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  // ==================== Getters ====================
  const enabledExtensions = computed(() => extensions.value.filter(e => e.enabled))
  const disabledExtensions = computed(() => extensions.value.filter(e => !e.enabled))
  const totalExtensions = computed(() => extensions.value.length)
  const enabledCount = computed(() => enabledExtensions.value.length)

  // ==================== Actions ====================

  /**
   * 加载扩展列表
   */
  async function loadExtensions() {
    loading.value = true
    error.value = null
    try {
      const response = await extensionApi.getExtensions()
      // 后端返回的是 ExtensionListResponse，需要取 extensions 数组
      extensions.value = Array.isArray(response) ? response : response.extensions || []
    } catch (err) {
      error.value = err instanceof Error ? err.message : '加载扩展列表失败'
      console.error('加载扩展列表失败:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * 选择扩展
   */
  function selectExtension(extension: Extension | null) {
    selectedExtension.value = extension
  }

  /**
   * 根据ID获取扩展
   */
  function getExtensionById(id: string): Extension | undefined {
    return extensions.value.find(e => e.manifest.id === id)
  }

  /**
   * 安装扩展
   * @param packagePath - 扩展包文件路径
   */
  async function installExtension(packagePath: string): Promise<void> {
    loading.value = true
    error.value = null
    try {
      const newExtension = await extensionApi.installExtension(packagePath)
      extensions.value.push(newExtension)
    } catch (err) {
      error.value = err instanceof Error ? err.message : '安装扩展失败'
      console.error('安装扩展失败:', err)
      throw err
    } finally {
      loading.value = false
    }
  }

  /**
   * 卸载扩展
   * @param extensionId - 扩展ID
   */
  async function uninstallExtension(extensionId: string): Promise<void> {
    loading.value = true
    error.value = null
    try {
      await extensionApi.uninstallExtension(extensionId)
      // 从列表中移除
      const index = extensions.value.findIndex(e => e.manifest.id === extensionId)
      if (index > -1) {
        extensions.value.splice(index, 1)
      }
      // 如果当前选中的是被卸载的扩展，清空选中状态
      if (selectedExtension.value?.manifest.id === extensionId) {
        selectedExtension.value = null
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : '卸载扩展失败'
      console.error('卸载扩展失败:', err)
      throw err
    } finally {
      loading.value = false
    }
  }

  /**
   * 启用/禁用扩展
   * @param extensionId - 扩展ID
   * @param enabled - 是否启用
   */
  async function toggleExtension(extensionId: string, enabled: boolean): Promise<void> {
    error.value = null
    try {
      // 调用后端API，返回更新后的扩展信息
      const updatedExtension = await extensionApi.toggleExtension(extensionId, enabled)

      // 更新本地状态
      const index = extensions.value.findIndex(e => e.manifest.id === extensionId)
      if (index > -1) {
        extensions.value[index] = updatedExtension
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : '切换扩展状态失败'
      console.error('切换扩展状态失败:', err)
      throw err
    }
  }

  /**
   * 重新加载扩展
   * @param extensionId - 扩展ID
   */
  async function reloadExtension(extensionId: string): Promise<void> {
    loading.value = true
    error.value = null
    try {
      // 更新本地状态为加载中
      const extension = extensions.value.find(e => e.manifest.id === extensionId)
      if (extension) {
        extension.loadStatus = 'loading' as ExtensionLoadStatus
      }

      // 调用后端API重新加载
      const updatedExtension = await extensionApi.reloadExtension(extensionId)

      // 更新本地数据
      const index = extensions.value.findIndex(e => e.manifest.id === extensionId)
      if (index > -1) {
        extensions.value[index] = updatedExtension
      }
    } catch (err) {
      // 标记为加载失败
      const extension = extensions.value.find(e => e.manifest.id === extensionId)
      if (extension) {
        extension.loadStatus = 'failed' as ExtensionLoadStatus
        extension.error = err instanceof Error ? err.message : '重新加载失败'
      }
      error.value = err instanceof Error ? err.message : '重新加载扩展失败'
      console.error('重新加载扩展失败:', err)
      throw err
    } finally {
      loading.value = false
    }
  }

  // 返回所有状态和方法
  return {
    // State
    extensions,
    selectedExtension,
    loading,
    error,
    // Getters
    enabledExtensions,
    disabledExtensions,
    totalExtensions,
    enabledCount,
    // Actions
    loadExtensions,
    selectExtension,
    getExtensionById,
    installExtension,
    uninstallExtension,
    toggleExtension,
    reloadExtension
  }
})
