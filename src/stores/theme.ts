import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

// 主题类型定义
type Theme = 'light' | 'dark' | 'auto'

export const useThemeStore = defineStore('theme', () => {
  // 当前主题设置
  const theme = ref<Theme>('auto')
  // 系统是否处于深色模式
  const systemDark = ref(false)

  // 计算当前是否为深色模式
  const isDark = computed(() => {
    if (theme.value === 'auto') {
      return systemDark.value
    }
    return theme.value === 'dark'
  })

  // 初始化主题
  const initTheme = () => {
    // 监听系统主题变化
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    systemDark.value = mediaQuery.matches

    mediaQuery.addEventListener('change', (e) => {
      systemDark.value = e.matches
    })

    // 从本地存储读取主题设置
    const savedTheme = localStorage.getItem('theme') as Theme
    if (savedTheme && ['light', 'dark', 'auto'].includes(savedTheme)) {
      theme.value = savedTheme
    }
  }

  // 设置主题
  const setTheme = (newTheme: Theme) => {
    theme.value = newTheme
    localStorage.setItem('theme', newTheme)
  }

  // 切换主题
  const toggleTheme = () => {
    const themes: Theme[] = ['light', 'dark', 'auto']
    const currentIndex = themes.indexOf(theme.value)
    const nextIndex = (currentIndex + 1) % themes.length
    setTheme(themes[nextIndex])
  }

  return {
    theme,
    isDark,
    systemDark,
    initTheme,
    setTheme,
    toggleTheme
  }
})
