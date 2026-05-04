/**
 * api-wrapper.ts - API请求包装器
 * 统一处理所有Tauri IPC调用的错误处理和响应格式化
 */

import { invoke } from '@tauri-apps/api/core'
import { handleError } from '../utils/error-handler'

/**
 * API响应格式
 */
export interface ApiResponse<T = unknown> {
  success: boolean
  data?: T
  error?: string
}

/**
 * 调用Tauri命令的统一包装函数
 * @param command - Tauri命令名
 * @param args - 命令参数
 * @returns Promise<T>
 */
export async function invokeCommand<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<T> {
  try {
    return await invoke<T>(command, args)
  } catch (error) {
    handleError(error, `API.${command}`, { silent: true })
    throw error
  }
}

/**
 * 调用Tauri命令并返回标准化响应
 * @param command - Tauri命令名
 * @param args - 命令参数
 * @returns Promise<ApiResponse<T>>
 */
export async function invokeApi<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<ApiResponse<T>> {
  try {
    const data = await invoke<T>(command, args)
    return { success: true, data }
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error)
    return { success: false, error: errorMessage }
  }
}

/**
 * 批量调用多个API命令
 * @param commands - 命令数组
 * @returns Promise<ApiResponse<unknown>[]>
 */
export async function batchInvoke(
  commands: Array<{ command: string; args?: Record<string, unknown> }>
): Promise<ApiResponse<unknown>[]> {
  return Promise.all(
    commands.map(({ command, args }) => invokeApi(command, args))
  )
}

/**
 * 带重试机制的API调用
 * @param command - Tauri命令名
 * @param args - 命令参数
 * @param retries - 重试次数
 * @param delay - 重试间隔(ms)
 * @returns Promise<T>
 */
export async function invokeWithRetry<T>(
  command: string,
  args?: Record<string, unknown>,
  retries = 3,
  delay = 1000
): Promise<T> {
  let lastError: unknown

  for (let i = 0; i < retries; i++) {
    try {
      return await invoke<T>(command, args)
    } catch (error) {
      lastError = error
      if (i < retries - 1) {
        await new Promise(resolve => setTimeout(resolve, delay))
      }
    }
  }

  throw lastError
}
