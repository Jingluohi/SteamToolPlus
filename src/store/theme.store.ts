/**
 * 主题状态管理
 * 使用Pinia实现主题相关的全局状态
 */

import { defineStore } from 'pinia'
import { ref, computed, watch } from 'vue'
import type { ThemeConfig } from '../types'

/**
 * 主题模式类型
 * - dark / light / auto：使用背景图片
 * - black / white / auto-solid：使用纯色背景
 */
type ThemeMode = 'dark' | 'light' | 'black' | 'white' | 'auto' | 'auto-solid'

/** 纯色背景色值（非纯白/纯黑） */
const SOLID_BLACK_BG = '#28324a'
const SOLID_WHITE_BG = '#f2f2f2'

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
  /** 用户自定义主题 CSS 变量 */
  const customVars = ref<Record<string, string>>({})

  // ==================== Getters ====================
  /** 当前是否为深色主题（dark / black / auto / auto-solid 在系统深色时） */
  const isDark = computed(() => {
    const mode = themeMode.value
    if (mode === 'auto' || mode === 'auto-solid') {
      return systemIsDark.value
    }
    return mode === 'dark' || mode === 'black'
  })

  /** 当前是否为纯色背景模式 */
  const isSolid = computed(() => {
    return themeMode.value === 'black' || themeMode.value === 'white' || themeMode.value === 'auto-solid'
  })

  /** 当前是否为图片背景模式 */
  const isPicture = computed(() => !isSolid.value)

  /** 当前主题名称（仅 dark / light，用于 CSS 变量切换控件/文字颜色） */
  const currentTheme = computed(() => {
    return isDark.value ? 'dark' : 'light'
  })

  /** 当前纯色背景色 */
  const solidBgColor = computed(() => {
    return isDark.value ? SOLID_BLACK_BG : SOLID_WHITE_BG
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
   * data-theme 仅控制控件/文字颜色（dark/light），
   * data-bg-mode 区分图片模式与纯色模式。
   */
  function applyTheme() {
    const html = document.documentElement
    const body = document.body

    // 控件与文字颜色统一按 dark/light 处理
    if (isDark.value) {
      html.setAttribute('data-theme', 'dark')
      body.classList.remove('light-theme')
      body.classList.add('dark-theme')
    } else {
      html.setAttribute('data-theme', 'light')
      body.classList.remove('dark-theme')
      body.classList.add('light-theme')
    }

    // 标记当前是图片模式还是纯色模式
    html.setAttribute('data-bg-mode', isSolid.value ? 'solid' : 'picture')

    // 同步纯色背景色到 CSS 变量
    if (isSolid.value) {
      html.style.setProperty('--app-bg-color', solidBgColor.value)
    } else {
      // 图片模式下使用主题主背景色兜底
      html.style.setProperty('--app-bg-color', isDark.value ? '#363636' : '#f8f9fa')
    }

    // 应用用户自定义主题变量
    Object.entries(customVars.value).forEach(([key, value]) => {
      html.style.setProperty(key, value)
    })
  }

  /**
   * 设置主题模式
   */
  function setThemeMode(mode: ThemeMode) {
    themeMode.value = mode
    // auto 与 auto-solid 均视为跟随系统
    followSystem.value = mode === 'auto' || mode === 'auto-solid'
    applyTheme()
  }

  /**
   * 切换深色/浅色主题
   * 若当前为图片模式则切换为 dark/light；若为纯色模式则切换为 black/white。
   */
  function toggleTheme() {
    if (isSolid.value) {
      setThemeMode(isDark.value ? 'white' : 'black')
    } else {
      setThemeMode(isDark.value ? 'light' : 'dark')
    }
  }

  /**
   * 设置是否跟随系统主题
   * @param follow 是否跟随
   * @param solid 跟随系统时是否使用纯色背景
   */
  function setFollowSystem(follow: boolean, solid: boolean = false) {
    followSystem.value = follow
    if (follow) {
      themeMode.value = solid ? 'auto-solid' : 'auto'
    }
    applyTheme()
  }

  /**
   * 从配置加载主题设置
   * 兼容旧配置：若 mode 不在新枚举中则回退到 auto。
   */
  function loadFromConfig(config: ThemeConfig) {
    const validModes: ThemeMode[] = ['dark', 'light', 'black', 'white', 'auto', 'auto-solid']
    const mode = config.mode as ThemeMode
    themeMode.value = validModes.includes(mode) ? mode : 'auto'
    followSystem.value = themeMode.value === 'auto' || themeMode.value === 'auto-solid'
    customVars.value = config.customVars || {}
    applyTheme()
  }

  /**
   * 导出主题配置
   */
  function exportConfig(): ThemeConfig {
    return {
      mode: themeMode.value,
      followSystem: followSystem.value,
      customVars: { ...customVars.value }
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
    customVars,
    // Getters
    isDark,
    isSolid,
    isPicture,
    currentTheme,
    solidBgColor,
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
