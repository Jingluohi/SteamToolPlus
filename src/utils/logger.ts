// 前端日志工具
// 将运行时错误记录到程序根目录下的 logs 目录

import { invoke } from '@tauri-apps/api/core'

/**
 * 日志级别
 */
export enum LogLevel {
  DEBUG = 'DEBUG',
  INFO = 'INFO',
  WARN = 'WARN',
  ERROR = 'ERROR'
}

/**
 * 写入日志到后端
 * @param level 日志级别
 * @param message 日志消息
 * @param source 日志来源（默认 'frontend'）
 */
export async function logToFile(level: LogLevel, message: string, source: string = 'frontend'): Promise<void> {
  try {
    await invoke('log_to_file', {
      level: level.toString(),
      message,
      source
    })
  } catch (err) {
    // 如果后端日志失败，输出到控制台
    console.error('日志记录失败:', err)
  }
}

/**
 * 记录调试日志
 */
export async function debugLog(message: string): Promise<void> {
  await logToFile(LogLevel.DEBUG, message)
}

/**
 * 记录信息日志
 */
export async function infoLog(message: string): Promise<void> {
  await logToFile(LogLevel.INFO, message)
}

/**
 * 记录警告日志
 */
export async function warnLog(message: string): Promise<void> {
  await logToFile(LogLevel.WARN, message)
}

/**
 * 记录错误日志
 */
export async function errorLog(message: string): Promise<void> {
  await logToFile(LogLevel.ERROR, message)
}

/**
 * 设置全局错误捕获
 * 自动捕获未处理的Promise错误和全局错误
 */
export function setupGlobalErrorHandler(): void {
  // 捕获未处理的Promise错误
  window.addEventListener('unhandledrejection', (event) => {
    const error = event.reason
    const message = error instanceof Error ? error.message : String(error)
    errorLog(`未处理的Promise错误: ${message}`)
  })

  // 捕获全局错误
  window.addEventListener('error', (event) => {
    const message = `${event.message} at ${event.filename}:${event.lineno}:${event.colno}`
    errorLog(`全局错误: ${message}`)
  })
}
