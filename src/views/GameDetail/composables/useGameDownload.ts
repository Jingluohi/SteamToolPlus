/**
 * useGameDownload composable
 * 游戏下载功能的状态和方法管理
 */

import { ref, computed, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { DownloadProgress as DownloadProgressType } from '../../../types/download.types'
import type { GameData } from '../../../api/gameData.api'

/**
 * 下载日志条目类型
 */
interface DownloadLogEntry {
  time: string
  message: string
  type: 'info' | 'success' | 'error' | 'warning'
}

/**
 * 游戏下载 composable
 * @param gameId 游戏ID
 * @param manifestFolderPath 清单文件夹路径
 * @param downloadPath 下载路径
 */
export function useGameDownload(
  gameId: string,
  manifestFolderPath: string,
  downloadPath: string
) {
  // ==================== 下载状态 ====================

  /** 是否正在下载 */
  const isDownloading = ref(false)

  /** 是否正在验证 */
  const isVerifying = ref(false)

  /** 下载进度监控状态 */
  const isMonitoring = ref(false)

  /** 下载进度 */
  const downloadProgress = ref<DownloadProgressType>({
    totalDepots: 0,
    completedDepots: 0,
    overallPercentage: 0,
    depots: [],
    isComplete: false
  })

  /** 下载日志 */
  const downloadLogs = ref<DownloadLogEntry[]>([])

  /** 进度监控定时器 */
  let monitorInterval: number | null = null

  // ==================== 清单源选择状态 ====================

  /** 清单源选择模式 */
  const downloadManifestSourceMode = ref<'7z' | 'folder'>('7z')

  /** 已选择的清单源路径 */
  const selectedDownloadManifestPath = ref('')

  /** 是否正在准备清单 */
  const isPreparingDownloadManifest = ref(false)

  // ==================== 计算属性 ====================

  /**
   * 是否可以开始下载
   * 条件：清单文件夹已找到、下载路径已设置、不在下载中
   */
  const canDownload = computed(() => {
    return manifestFolderPath !== '' && downloadPath !== '' && !isDownloading.value
  })

  // ==================== 方法 ====================

  /**
   * 添加下载日志
   * @param message 日志消息
   * @param type 日志类型
   */
  function addDownloadLog(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
    const now = new Date()
    const time = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:${now.getSeconds().toString().padStart(2, '0')}`
    downloadLogs.value.push({ time, message, type })

    // 限制日志数量，防止内存占用过大
    if (downloadLogs.value.length > 100) {
      downloadLogs.value = downloadLogs.value.slice(-80)
    }
  }

  /**
   * 清空下载日志
   */
  function clearDownloadLogs() {
    downloadLogs.value = []
  }

  /**
   * 解析进度文件名获取depot ID和百分比
   * 文件名格式: "{百分比}% - {depotId}.json"
   * @param fileName 文件名
   */
  function parseProgressFileName(fileName: string): { depotId: string; percentage: number } | null {
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
   * 扫描进度文件
   * 读取下载进度JSON文件，更新各depot的下载进度
   */
  async function scanProgressFiles(): Promise<void> {
    try {
      // 获取指定游戏的进度文件
      const progressFiles = await invoke<Array<{ name: string; path: string }>>('get_download_progress_files', {
        gameId: gameId
      })

      // 更新每个depot的进度
      const updatedDepots = [...downloadProgress.value.depots]

      for (const file of progressFiles) {
        const parsed = parseProgressFileName(file.name)
        if (parsed) {
          // 读取文件内容获取已下载文件数量
          const fileContent = await invoke<Record<string, string[]>>('read_json_file', {
            filePath: file.path
          }).catch(() => ({}))

          const downloadedFiles = Object.keys(fileContent).length

          // 查找对应的depot并更新
          const depotIndex = updatedDepots.findIndex(d => d.depotId === parsed.depotId)
          if (depotIndex !== -1) {
            updatedDepots[depotIndex] = {
              depotId: parsed.depotId,
              percentage: parsed.percentage,
              downloadedFiles,
              totalFiles: downloadedFiles,
              status: parsed.percentage >= 100 ? 'completed' : 'downloading'
            }
          } else {
            // 如果depot不在列表中，添加它
            updatedDepots.push({
              depotId: parsed.depotId,
              percentage: parsed.percentage,
              downloadedFiles,
              totalFiles: downloadedFiles,
              status: parsed.percentage >= 100 ? 'completed' : 'downloading'
            })
          }
        }
      }

      // 计算总体进度
      const completedDepots = updatedDepots.filter(d => d.status === 'completed').length
      const overallPercentage = updatedDepots.length > 0
        ? Math.round(updatedDepots.reduce((sum, d) => sum + d.percentage, 0) / updatedDepots.length)
        : 0

      downloadProgress.value = {
        totalDepots: downloadProgress.value.totalDepots || updatedDepots.length,
        completedDepots,
        overallPercentage,
        depots: updatedDepots,
        isComplete: updatedDepots.length > 0 && updatedDepots.every(d => d.status === 'completed')
      }

      // 如果下载完成，停止监控
      if (downloadProgress.value.isComplete) {
        stopProgressMonitoring()
        addDownloadLog('所有depot下载完成！', 'success')
      }
    } catch {
      // 扫描进度文件失败时静默处理
    }
  }

  /**
   * 开始监控下载进度
   * 定期扫描进度文件，更新下载状态
   */
  async function startProgressMonitoring(): Promise<void> {
    if (isMonitoring.value) return

    isMonitoring.value = true

    // 首先读取manifest文件夹，获取所有depot ID
    try {
      const result = await invoke<{
        jsonFiles: string[]
        vdfFiles: string[]
        manifestFiles: string[]
      }>('read_manifest_folder', { folderPath: manifestFolderPath })

      // 从manifest文件名中提取depot ID
      // 文件名格式: "{depotId}_{version}.manifest"
      const depotIds = result.manifestFiles.map(filePath => {
        // 使用正则匹配路径分隔符后的文件名
        const match = filePath.match(/[\\/](\d+)_\d+\.manifest$/)
        return match ? match[1] : null
      }).filter(id => id !== null) as string[]

      // 初始化所有depot的进度状态
      downloadProgress.value.totalDepots = depotIds.length
      downloadProgress.value.depots = depotIds.map(depotId => ({
        depotId,
        percentage: 0,
        downloadedFiles: 0,
        totalFiles: 0,
        status: 'pending' as const
      }))

      addDownloadLog(`检测到 ${depotIds.length} 个depot`, 'info')
    } catch (error) {
      addDownloadLog(`读取manifest文件夹失败: ${error}`, 'error')
    }

    // 立即扫描一次
    await scanProgressFiles()

    // 设置定时扫描，每秒扫描一次
    monitorInterval = window.setInterval(() => {
      scanProgressFiles()
    }, 1000)
  }

  /**
   * 停止监控下载进度
   */
  function stopProgressMonitoring(): void {
    isMonitoring.value = false
    if (monitorInterval) {
      clearInterval(monitorInterval)
      monitorInterval = null
    }
  }

  /**
   * 开始下载游戏
   * @param _gameData 游戏数据（预留，用于保存到game.json）
   * @param gameName 游戏名称
   * @returns 下载结果
   */
  async function startDownload(_gameData: GameData, gameName: string): Promise<{ success: boolean; message: string }> {
    // 检查是否有清单文件夹
    if (!manifestFolderPath) {
      return { success: false, message: '未找到清单文件夹，请先下载清单文件' }
    }

    // 检查下载路径
    if (!downloadPath) {
      return { success: false, message: '请先选择下载路径' }
    }

    isDownloading.value = true
    clearDownloadLogs()

    addDownloadLog('开始下载游戏...', 'info')
    addDownloadLog(`游戏: ${gameName}`, 'info')
    addDownloadLog(`清单路径: ${manifestFolderPath}`, 'info')
    addDownloadLog(`下载路径: ${downloadPath}`, 'info')

    try {
      // 调用Rust后端执行下载命令
      const result = await invoke<{
        success: boolean
        message: string
      }>('start_game_download', {
        manifestPath: manifestFolderPath,
        downloadPath: downloadPath,
        gameId: gameId
      })

      if (result.success) {
        addDownloadLog('下载命令已启动', 'success')
        addDownloadLog(result.message, 'info')
        // 启动进度监控
        await startProgressMonitoring()
      } else {
        addDownloadLog(`下载启动失败: ${result.message}`, 'error')
      }

      return result
    } catch (error) {
      addDownloadLog(`下载出错: ${error}`, 'error')
      return { success: false, message: String(error) }
    } finally {
      isDownloading.value = false
    }
  }

  /**
   * 停止下载
   * 终止下载进程并停止进度监控
   */
  async function stopDownload(): Promise<void> {
    try {
      addDownloadLog('正在停止下载...', 'info')
      await invoke('stop_download', {
        gameId: gameId
      })
      addDownloadLog('下载已停止', 'success')

      // 停止进度监控
      stopProgressMonitoring()

      isDownloading.value = false
    } catch (error) {
      addDownloadLog(`停止下载失败: ${error}`, 'error')
    }
  }

  /**
   * 验证游戏完整性
   * 使用与下载相同的参数重新运行，验证并补全缺失文件
   * @param existingDownloadPath 已存在的下载路径
   * @param gameName 游戏名称
   */
  async function verifyIntegrity(existingDownloadPath: string, gameName: string): Promise<void> {
    if (!manifestFolderPath) {
      addDownloadLog('未找到清单文件夹', 'error')
      return
    }

    if (!existingDownloadPath) {
      addDownloadLog('未找到下载路径', 'error')
      return
    }

    isVerifying.value = true
    clearDownloadLogs()

    addDownloadLog('开始验证游戏完整性...', 'info')
    addDownloadLog(`游戏: ${gameName}`, 'info')
    addDownloadLog(`清单路径: ${manifestFolderPath}`, 'info')
    addDownloadLog(`下载路径: ${existingDownloadPath}`, 'info')

    try {
      // 调用与下载相同的命令，会自动验证并补全
      const result = await invoke<{
        success: boolean
        message: string
      }>('start_game_download', {
        manifestPath: manifestFolderPath,
        downloadPath: existingDownloadPath,
        gameId: gameId
      })

      if (result.success) {
        addDownloadLog('验证命令已启动', 'success')
        addDownloadLog(result.message, 'info')
        // 启动进度监控
        await startProgressMonitoring()
      } else {
        addDownloadLog(`验证启动失败: ${result.message}`, 'error')
      }
    } catch (error) {
      addDownloadLog(`验证出错: ${error}`, 'error')
    } finally {
      isVerifying.value = false
    }
  }

  /**
   * 选择 7z 压缩包作为清单源
   * 解压到 resources/manifest/{game_id}/ 目录
   * @returns 解压后的目标文件夹路径（成功时）或 null（失败时）
   */
  async function selectManifestArchive(): Promise<string | null> {
    try {
      // 打开文件选择对话框（需要在外部调用）
      // 这里只处理解压逻辑
      isPreparingDownloadManifest.value = true

      addDownloadLog(`正在解压清单压缩包...`, 'info')
      const targetFolder = await invoke<string>('extract_manifest_archive', {
        archivePath: selectedDownloadManifestPath.value,
        gameId: gameId
      })

      addDownloadLog(`清单已解压到: ${targetFolder}`, 'success')

      // 记录手动导入的清单游戏ID到缓存
      try {
        await invoke('add_imported_manifest_game_id', {
          gameId: gameId
        })
      } catch {
        // 清单导入缓存失败不影响主流程
      }

      return targetFolder
    } catch (error) {
      addDownloadLog(`解压清单压缩包失败: ${error}`, 'error')
      return null
    } finally {
      isPreparingDownloadManifest.value = false
    }
  }

  /**
   * 选择文件夹作为清单源
   * 复制到 resources/manifest/{game_id}/ 目录
   * @returns 复制后的目标文件夹路径（成功时）或 null（失败时）
   */
  async function selectManifestFolder(): Promise<string | null> {
    try {
      isPreparingDownloadManifest.value = true

      addDownloadLog(`正在复制清单文件夹...`, 'info')
      const targetFolder = await invoke<string>('copy_folder_to_manifest', {
        sourcePath: selectedDownloadManifestPath.value,
        gameId: gameId
      })

      addDownloadLog(`清单已复制到: ${targetFolder}`, 'success')
      return targetFolder
    } catch (error) {
      addDownloadLog(`复制清单文件夹失败: ${error}`, 'error')
      return null
    } finally {
      isPreparingDownloadManifest.value = false
    }
  }

  /**
   * 获取下载按钮文本
   * @param existingGameData 已存在的游戏数据
   */
  function getDownloadButtonText(existingGameData: GameData | null): string {
    if (existingGameData?.download_status === 'downloading') {
      return '下载中...'
    }
    if (isDownloading.value) {
      return '下载中...'
    }
    if (existingGameData?.download_status === 'completed') {
      return '已下载'
    }
    return '开始下载'
  }

  // 组件卸载时清理定时器
  onUnmounted(() => {
    stopProgressMonitoring()
  })

  return {
    // 下载状态
    isDownloading,
    isVerifying,
    isMonitoring,
    downloadProgress,
    downloadLogs,

    // 清单源选择状态
    downloadManifestSourceMode,
    selectedDownloadManifestPath,
    isPreparingDownloadManifest,

    // 计算属性
    canDownload,

    // 方法
    addDownloadLog,
    clearDownloadLogs,
    startDownload,
    stopDownload,
    verifyIntegrity,
    startProgressMonitoring,
    stopProgressMonitoring,
    scanProgressFiles,
    selectManifestArchive,
    selectManifestFolder,
    getDownloadButtonText
  }
}