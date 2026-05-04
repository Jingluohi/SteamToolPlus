/**
 * 主题状态管理
 * 使用Pinia实现主题相关的全局状态
 */

import { defineStore } from 'pinia'
import { ref, computed, watch } from 'vue'
import type { ThemeConfig } from '../types'

/**
 * 主题模式类型
 */
type ThemeMode = 'dark' | 'light' | 'auto'

/**
 * 检测系统主题是否为深色
 */
function detectSystemTheme(): boolean {
  if (typeof window === 'undefined') return true
  return window.matchMedia('(prefers-color-scheme: dark)').matches
}

/**
 * 主题Store
 */
export const useThemeStore = defineStore('theme', () => {
  // ==================== State ====================
  /** 当前主题模式 */
  const themeMode = ref<ThemeMode>('auto')
  /** 是否跟随系统主题 */
  const followSystem = ref(true)
  /** 系统主题是否为深色 - 初始化时立即检测 */
  const systemIsDark = ref(detectSystemTheme())

  // ==================== Getters ====================
  /** 当前是否为深色主题 */
  const isDark = computed(() => {
    if (themeMode.value === 'auto') {
      return systemIsDark.value
    }
    return themeMode.value === 'dark'
  })

  /** 当前主题名称 */
  const currentTheme = computed(() => {
    return isDark.value ? 'dark' : 'light'
  })

  // ==================== Actions ====================
  /**
   * 初始化主题
   */
  function initTheme() {
    // 检测系统主题
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    systemIsDark.value = mediaQuery.matches

    // 监听系统主题变化
    mediaQuery.addEventListener('change', (e) => {
      systemIsDark.value = e.matches
      if (followSystem.value) {
        applyTheme()
      }
    })

    // 应用主题
    applyTheme()
  }

  /**
   * 应用主题到DOM
   */
  function applyTheme() {
    const html = document.documentElement
    const body = document.body

    if (isDark.value) {
      html.setAttribute('data-theme', 'dark')
      body.classList.remove('light-theme')
      body.classList.add('dark-theme')
    } else {
      html.setAttribute('data-theme', 'light')
      body.classList.remove('dark-theme')
      body.classList.add('light-theme')
    }
  }

  /**
   * 设置主题模式
   */
  function setThemeMode(mode: ThemeMode) {
    themeMode.value = mode
    if (mode !== 'auto') {
      followSystem.value = false
    }
    applyTheme()
  }

  /**
   * 切换深色/浅色主题
   */
  function toggleTheme() {
    if (isDark.value) {
      setThemeMode('light')
    } else {
      setThemeMode('dark')
    }
  }

  /**
   * 设置是否跟随系统主题
   */
  function setFollowSystem(follow: boolean) {
    followSystem.value = follow
    if (follow) {
      themeMode.value = 'auto'
    }
    applyTheme()
  }

  /**
   * 从配置加载主题设置
   */
  function loadFromConfig(config: ThemeConfig) {
    themeMode.value = config.mode as ThemeMode
    followSystem.value = config.followSystem
    applyTheme()
  }

  /**
   * 导出主题配置
   */
  function exportConfig(): ThemeConfig {
    return {
      mode: themeMode.value,
      followSystem: followSystem.value,
      customVars: {}
    }
  }

  // 监听主题变化并应用
  watch(isDark, () => {
    applyTheme()
  })

  // 返回所有状态和方法
  return {
    // State
    themeMode,
    followSystem,
    systemIsDark,
    // Getters
    isDark,
    currentTheme,
    // Actions
    initTheme,
    applyTheme,
    setThemeMode,
    toggleTheme,
    setFollowSystem,
    loadFromConfig,
    exportConfig
  }
})
