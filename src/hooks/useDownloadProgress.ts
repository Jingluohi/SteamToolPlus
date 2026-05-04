/**
 * 下载进度监控 Hook
 * 用于监控 ddv20.exe 生成的进度文件，实时显示下载进度
 * 独立于组件，可在多个界面复用
 */

import { ref, computed, onUnmounted } from 'vue'
import type { DownloadProgress, DepotProgress } from '../types'
import {
  getDownloadProgressFiles,
  readJsonFile,
  readManifestFolder,
  checkAndCleanupCompletedDownloads
} from '../api/download.api'

/**
 * 使用下载进度监控
 * @param manifestPath 清单文件夹路径
 * @param appId 游戏ID
 */
export function useDownloadProgress(manifestPath: string, appId: string) {
  // 进度数据
  const progress = ref<DownloadProgress>({
    totalDepots: 0,
    completedDepots: 0,
    overallPercentage: 0,
    depots: [],
    isComplete: false
  })

  // 是否正在监控
  const isMonitoring = ref(false)

  // 监控定时器
  let monitorInterval: number | null = null

  /**
   * 解析进度文件名获取depot ID和百分比
   * 文件名格式: "{百分比}% - {depotId}.json"
   */
  const parseProgressFileName = (fileName: string): { depotId: string; percentage: number } | null => {
    const match = fileName.match(/^(\d+)%\s*-\s*(\d+)\.json$/)
    if (match) {
      return {
        percentage: parseInt(match[1], 10),
        depotId: match[2]
      }
    }
    return null
  }

  /**
   * 获取depot总数（通过读取manifest文件夹中的.manifest文件数量）
   */
  const getTotalDepots = async (): Promise<number> => {
    try {
      const result = await readManifestFolder(manifestPath)
      return result.manifestFiles.length
    } catch (error) {
      return 0
    }
  }

  /**
   * 扫描进度文件
   */
  const scanProgressFiles = async () => {
    try {
      // 获取程序根目录下的进度文件
      const progressFiles = await getDownloadProgressFiles()

      const depotProgressMap = new Map<string, DepotProgress>()

      // 解析每个进度文件
      for (const file of progressFiles) {
        const parsed = parseProgressFileName(file.name)
        if (parsed) {
          // 读取文件内容获取已下载文件数量
          const fileContent = await readJsonFile(file.path).catch(() => ({}))

          const downloadedFiles = Object.keys(fileContent).length

          depotProgressMap.set(parsed.depotId, {
            depotId: parsed.depotId,
            percentage: parsed.percentage,
            downloadedFiles,
            totalFiles: downloadedFiles, // 总数会在后续更新
            status: parsed.percentage >= 100 ? 'completed' : 'downloading'
          })
        }
      }

      // 更新进度数据
      const depots = Array.from(depotProgressMap.values())
      const completedDepots = depots.filter(d => d.status === 'completed').length
      const overallPercentage = depots.length > 0
        ? Math.round(depots.reduce((sum, d) => sum + d.percentage, 0) / depots.length)
        : 0

      const isComplete = depots.length > 0 && depots.every(d => d.status === 'completed')

      progress.value = {
        totalDepots: Math.max(depots.length, progress.value.totalDepots),
        completedDepots,
        overallPercentage,
        depots,
        isComplete
      }

      // 如果下载完成，自动静默清理进度文件
      if (isComplete && depots.length > 0) {
        try {
          await checkAndCleanupCompletedDownloads()
        } catch (_cleanupError) {
          // 静默处理清理错误
        }
      }
    } catch (error) {
      // 扫描进度文件失败时静默处理
    }
  }

  /**
   * 开始监控下载进度
   */
  const startMonitoring = async () => {
    if (isMonitoring.value) return

    // 获取depot总数
    const totalDepots = await getTotalDepots()
    progress.value.totalDepots = totalDepots

    isMonitoring.value = true

    // 立即扫描一次
    await scanProgressFiles()

    // 设置定时扫描
    monitorInterval = window.setInterval(() => {
      scanProgressFiles()
    }, 1000) // 每秒扫描一次
  }

  /**
   * 停止监控下载进度
   */
  const stopMonitoring = () => {
    isMonitoring.value = false
    if (monitorInterval) {
      clearInterval(monitorInterval)
      monitorInterval = null
    }
  }

  /**
   * 重置进度
   */
  const resetProgress = () => {
    stopMonitoring()
    progress.value = {
      totalDepots: 0,
      completedDepots: 0,
      overallPercentage: 0,
      depots: [],
      isComplete: false
    }
  }

  // 组件卸载时清理
  onUnmounted(() => {
    stopMonitoring()
  })

  return {
    progress: computed(() => progress.value),
    isMonitoring: computed(() => isMonitoring.value),
    startMonitoring,
    stopMonitoring,
    resetProgress
  }
}
