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
   * 初始化窗口状态
   */
  async function initWindow() {
    // 窗口状态初始化完成
    // 实际状态由后端事件驱动更新
    return Promise.resolve()
  }

  /**
   * 最小化窗口
   */
  async function minimize() {
    try {
      await windowApi.minimizeWindow()
    } catch (err) {
      // 最小化窗口失败时静默处理
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
      // 最大化窗口失败时静默处理
    }
  }

  /**
   * 关闭窗口
   */
  async function close() {
    try {
      await windowApi.closeWindow()
    } catch (err) {
      // 关闭窗口失败时静默处理
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
      // 切换全屏失败时静默处理
    }
  }

  /**
   * 打开帮助窗口
   */
  async function openHelp() {
    try {
      await windowApi.openHelpWindow()
    } catch (err) {
      // 打开帮助窗口失败时静默处理
    }
  }

  /**
   * 关闭帮助窗口
   */
  async function closeHelp() {
    try {
      await windowApi.closeHelpWindow()
    } catch (err) {
      // 关闭帮助窗口失败时静默处理
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
    initWindow,
    minimize,
    maximize,
    close,
    toggleFullscreen,
    openHelp,
    closeHelp
  }
})
