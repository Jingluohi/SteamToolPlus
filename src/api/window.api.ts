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
 * 切换全屏模式
 */
export async function toggleFullscreen(): Promise<void> {
  return invoke<void>('toggle_fullscreen')
}
