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
  const extensionConfig = computed(() => config.value?.extension)
  const securityConfig = computed(() => config.value?.security)

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
      console.error('加载配置失败:', err)
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
      console.error('保存配置失败:', err)
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
    extensionConfig,
    securityConfig,
    // Actions
    loadConfig,
    saveConfigData
  }
})
