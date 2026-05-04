/**
 * window.api.ts - 窗口管理相关的API调用
 */

import { invoke } from '@tauri-apps/api/core'

/**
 * 最小化窗口
 */
export async function minimizeWindow(): Promise<void> {
  return invoke<void>('minimize_window')
}

/**
 * 最大化/还原窗口
 */
export async function maximizeWindow(): Promise<void> {
  return invoke<void>('maximize_window')
}

/**
 * 关闭窗口
 */
export async function closeWindow(): Promise<void> {
  return invoke<void>('close_window')
}

/**
 * 打开帮助窗口
 */
export async function openHelpWindow(): Promise<void> {
  return invoke<void>('open_help_window')
}

/**
 * 关闭帮助窗口
 */
export async function closeHelpWindow(): Promise<void> {
  return invoke<void>('close_help_window')
}

/**
 * 切换全屏模式
 */
export async function toggleFullscreen(): Promise<void> {
  return invoke<void>('toggle_fullscreen')
}

/**
 * 窗口状态接口
 */
export interface WindowState {
  isMaximized: boolean
  isFullscreen: boolean
  width: number
  height: number
}

/**
 * 获取窗口状态
 */
export async function getWindowState(): Promise<WindowState | null> {
  try {
    return invoke<WindowState>('get_window_state')
  } catch (err) {
    return null
  }
}
