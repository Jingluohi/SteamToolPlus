/**
 * usePatch composable
 * 补丁应用相关的状态和方法管理
 */

import { ref, type Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import type { GameConfigData } from '../../../types'
import { getPatchSourcePath } from '../../../types'

/**
 * 补丁应用结果类型
 */
interface PatchApplyResult {
  success: boolean
  backed_up_files: string[]
  copied_files: string[]
  errors: string[]
}

/**
 * 补丁标签类型（从 GameConfigData 中提取）
 */
interface PatchTag {
  id: string
  name: string
  patchType: number
  patchPath: string
  downloadUrls?: { source: string; url: string; pwd?: string | null }[]
}

/**
 * 补丁相关状态和方法
 * @param game 游戏配置数据
 * @param gamePath 游戏路径
 */
export function usePatch(
  game: Ref<GameConfigData | undefined>,
  gamePath: Ref<string>
) {
  // ==================== 补丁状态 ====================

  /** 是否正在应用补丁 */
  const applyingPatch = ref(false)

  /** 补丁应用结果 */
  const patchResult = ref<PatchApplyResult | null>(null)

  /** 补丁说明缓存（按补丁类型存储） */
  const patchReadmes = ref<Map<number, string>>(new Map())

  /** 本地补丁文件存在状态缓存 */
  const patchLocalStatus = ref<Map<number, boolean>>(new Map())

  // ==================== 补丁方法 ====================

  /**
   * 检查本地补丁文件是否存在
   */
  const checkPatchLocalStatus = async () => {
    if (!game.value) return

    for (const tag of game.value.tags) {
      try {
        const patchPath = getPatchSourcePath(tag, game.value.game_id)
        const result = await invoke<string>('check_patch_file_exists', {
          patchSourcePath: patchPath
        })
        patchLocalStatus.value.set(tag.patch_type, result !== '')
      } catch (error) {
        patchLocalStatus.value.set(tag.patch_type, false)
      }
    }
  }

  /**
   * 获取本地补丁存在状态
   */
  const isPatchLocalExists = (patchType: number): boolean => {
    return patchLocalStatus.value.get(patchType) || false
  }

  /**
   * 应用补丁（使用本地补丁路径）
   */
  const applyPatch = async (tab: PatchTag) => {
    if (!gamePath.value) {
      alert('请先选择游戏路径')
      return
    }

    applyingPatch.value = true
    patchResult.value = null

    try {
      const result = await invoke<PatchApplyResult>('apply_patch', {
        patchSourcePath: tab.patchPath,
        gamePath: gamePath.value
      })

      patchResult.value = result
      
      if (result.success) {
        alert('补丁应用成功！')
      } else {
        alert('补丁应用完成，但有一些错误，请查看详情')
      }
    } catch (error) {
      alert('应用补丁失败: ' + error)
      patchResult.value = {
        success: false,
        backed_up_files: [],
        copied_files: [],
        errors: [String(error)]
      }
    } finally {
      applyingPatch.value = false
    }
  }

  /**
   * 选择本地补丁文件并直接应用
   * @param _tab 补丁标签（用于接口一致性，实际从用户选择的文件应用）
   */
  const selectAndApplyPatch = async (_tab: PatchTag) => {
    if (!gamePath.value) {
      alert('请先选择游戏路径')
      return
    }

    try {
      // 打开文件选择对话框
      const selected = await open({
        title: '选择补丁压缩包文件',
        filters: [{
          name: '7z压缩包',
          extensions: ['7z']
        }],
        multiple: false
      })

      if (!selected) {
        return // 用户取消了选择
      }

      const archivePath = Array.isArray(selected) ? selected[0] : selected

      // 确认应用
      const confirmApply = confirm(`确定要将补丁应用到游戏目录吗？\n\n补丁文件: ${archivePath}\n游戏路径: ${gamePath.value}\n\n应用前会自动备份原有文件。`)
      if (!confirmApply) {
        return
      }

      applyingPatch.value = true
      patchResult.value = null

      // 调用后端命令直接应用补丁
      const result = await invoke<PatchApplyResult>('apply_patch_from_file', {
        archivePath: archivePath,
        gamePath: gamePath.value
      })

      patchResult.value = result

      if (result.success) {
        alert('补丁应用成功！')
      } else {
        alert('补丁应用完成，但有一些错误，请查看详情')
      }
    } catch (error) {
      alert('应用补丁失败: ' + error)
      patchResult.value = {
        success: false,
        backed_up_files: [],
        copied_files: [],
        errors: [String(error)]
      }
    } finally {
      applyingPatch.value = false
    }
  }

  /**
   * 加载所有补丁说明
   */
  const loadPatchReadmes = async () => {
    if (!game.value) return

    for (const tag of game.value.tags) {
      try {
        const readme = await invoke<string>('get_patch_readme', {
          gameId: game.value.game_id,
          patchType: tag.patch_type
        })
        if (readme) {
          patchReadmes.value.set(tag.patch_type, readme)
        }
      } catch (error) {
        // 加载失败时忽略
      }
    }
  }

  /**
   * 获取补丁说明
   */
  const getPatchReadme = (patchType: number): string => {
    return patchReadmes.value.get(patchType) || ''
  }

  /**
   * 打开虚拟化环境配置教程视频
   */
  const openVirtualizationTutorial = async () => {
    try {
      await invoke('open_virtualization_tutorial')
    } catch (error) {
      alert('打开视频失败: ' + error)
    }
  }

  /**
   * 打开下载链接
   */
  const openDownloadUrl = async (url: string) => {
    try {
      await invoke('open_external_link', { url })
    } catch (error) {
      alert('打开下载链接失败: ' + error)
    }
  }

  /**
   * 获取下载源名称
   */
  const getDownloadSourceName = (source: string): string => {
    const sourceNames: Record<string, string> = {
      'lanzou': '蓝奏云',
      'quark': '夸克网盘',
      'baidu': '百度网盘',
      'xunlei': '迅雷网盘',
      '123': '123云盘'
    }
    return sourceNames[source] || source
  }

  /**
   * 清除补丁应用结果
   */
  const clearPatchResult = () => {
    patchResult.value = null
  }

  // ==================== 返回 ====================

  return {
    // 状态
    applyingPatch,
    patchResult,
    patchReadmes,
    patchLocalStatus,
    
    // 方法
    checkPatchLocalStatus,
    isPatchLocalExists,
    applyPatch,
    selectAndApplyPatch,
    loadPatchReadmes,
    getPatchReadme,
    openVirtualizationTutorial,
    openDownloadUrl,
    getDownloadSourceName,
    clearPatchResult
  }
}