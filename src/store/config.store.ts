/**
 * config.store.ts - 配置状态管理
 * 使用Pinia实现配置相关的全局状态
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { AppConfig } from '../types/config.types'
import * as configApi from '../api/config.api'

/**
 * 配置Store
 */
export const useConfigStore = defineStore('config', () => {
  // ==================== State ====================
  const config = ref<AppConfig | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  // ==================== Getters ====================
  const windowConfig = computed(() => config.value?.window)
  const themeConfig = computed(() => config.value?.theme)
  const gameDirsConfig = computed(() => config.value?.gameDirs)
  const launchConfig = computed(() => config.value?.launch)

  // ==================== Actions ====================

  /**
   * 加载配置
   */
  async function loadConfig() {
    loading.value = true
    error.value = null
    try {
      config.value = await configApi.getConfig()
    } catch (err) {
      error.value = err instanceof Error ? err.message : '加载配置失败'
    } finally {
      loading.value = false
    }
  }

  /**
   * 保存配置
   */
  async function saveConfigData(configData: AppConfig) {
    loading.value = true
    error.value = null
    try {
      await configApi.saveConfig(configData)
      config.value = configData
    } catch (err) {
      error.value = err instanceof Error ? err.message : '保存配置失败'
    } finally {
      loading.value = false
    }
  }

  /**
   * 更新配置（部分更新）
   */
  async function updateConfig(partialConfig: Partial<AppConfig>) {
    loading.value = true
    error.value = null
    try {
      if (!config.value) {
        throw new Error('配置未加载')
      }
      
      // 使用 API 更新配置，让后端合并
      const result = await configApi.updateConfig(partialConfig)
      config.value = result
    } catch (err) {
      error.value = err instanceof Error ? err.message : '更新配置失败'
      throw err
    } finally {
      loading.value = false
    }
  }

  /**
   * 重置配置到默认值
   */
  async function resetConfig() {
    loading.value = true
    error.value = null
    try {
      const defaultConfig = await configApi.resetConfig()
      config.value = defaultConfig
    } catch (err) {
      error.value = err instanceof Error ? err.message : '重置配置失败'
      throw err
    } finally {
      loading.value = false
    }
  }

  // 返回所有状态和方法
  return {
    // State
    config,
    loading,
    error,
    // Getters
    windowConfig,
    themeConfig,
    gameDirsConfig,
    launchConfig,
    // Actions
    loadConfig,
    saveConfigData,
    updateConfig,
    resetConfig
  }
})
