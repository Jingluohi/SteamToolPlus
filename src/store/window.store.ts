/**
 * window.store.ts - 窗口状态管理
 * 使用Pinia实现窗口相关的全局状态
 */

import { defineStore } from 'pinia'
import { ref } from 'vue'
import * as windowApi from '../api/window.api'

/**
 * 窗口Store
 */
export const useWindowStore = defineStore('window', () => {
  // ==================== State ====================
  const isMaximized = ref(false)
  const isFullscreen = ref(false)
  const width = ref(1600)
  const height = ref(900)

  // ==================== Actions ====================

  /**
   * 最小化窗口
   */
  async function minimize() {
    try {
      await windowApi.minimizeWindow()
    } catch (err) {
      console.error('最小化窗口失败:', err)
    }
  }

  /**
   * 最大化/还原窗口
   */
  async function maximize() {
    try {
      await windowApi.maximizeWindow()
      isMaximized.value = !isMaximized.value
    } catch (err) {
      console.error('最大化窗口失败:', err)
    }
  }

  /**
   * 关闭窗口
   */
  async function close() {
    try {
      await windowApi.closeWindow()
    } catch (err) {
      console.error('关闭窗口失败:', err)
    }
  }

  /**
   * 切换全屏模式
   */
  async function toggleFullscreen() {
    try {
      await windowApi.toggleFullscreen()
      isFullscreen.value = !isFullscreen.value
    } catch (err) {
      console.error('切换全屏失败:', err)
    }
  }

  /**
   * 打开帮助窗口
   */
  async function openHelp() {
    try {
      await windowApi.openHelpWindow()
    } catch (err) {
      console.error('打开帮助窗口失败:', err)
    }
  }

  /**
   * 关闭帮助窗口
   */
  async function closeHelp() {
    try {
      await windowApi.closeHelpWindow()
    } catch (err) {
      console.error('关闭帮助窗口失败:', err)
    }
  }

  // 返回所有状态和方法
  return {
    // State
    isMaximized,
    isFullscreen,
    width,
    height,
    // Actions
    minimize,
    maximize,
    close,
    toggleFullscreen,
    openHelp,
    closeHelp
  }
})
