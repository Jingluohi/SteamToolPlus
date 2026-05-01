/**
 * download.api.ts - 下载管理相关的API调用
 */

import { invoke } from '@tauri-apps/api/core'
import type { DownloadTask } from '../types/download.types'

/**
 * 获取下载任务列表
 */
export async function getDownloads(): Promise<DownloadTask[]> {
  return invoke<DownloadTask[]>('get_downloads')
}

/**
 * 创建下载任务
 */
export async function createDownload(url: string, savePath: string): Promise<DownloadTask> {
  return invoke<DownloadTask>('create_download', { url, savePath })
}

/**
 * 暂停下载
 */
export async function pauseDownload(taskId: string): Promise<void> {
  return invoke<void>('pause_download', { taskId })
}

/**
 * 恢复下载
 */
export async function resumeDownload(taskId: string): Promise<void> {
  return invoke<void>('resume_download', { taskId })
}

/**
 * 取消下载
 */
export async function cancelDownload(taskId: string): Promise<void> {
  return invoke<void>('cancel_download', { taskId })
}
