/**
 * useGameImport composable
 * 入库Steam相关的状态和方法
 */

import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { useConfigStore } from '../../../store/config.store'
import type { GameConfigData } from '../../../types'

/**
 * 扫描文件夹并转换文件（vdf转lua）
 * @param folderPath 文件夹路径
 */
async function scanAndConvertFolder(folderPath: string): Promise<{
  hasLua: boolean
  hasVdf: boolean
  hasManifest: boolean
}> {
  try {
    const result = await invoke<{
      hasLua: boolean
      hasVdf: boolean
      hasManifest: boolean
    }>('scan_manifest_folder', { folderPath })
    return result
  } catch (error) {
    console.error('扫描清单文件夹失败:', error)
    return { hasLua: false, hasVdf: false, hasManifest: false }
  }
}

/**
 * 入库Steam相关逻辑
 */
export function useGameImport() {
  const configStore = useConfigStore()

  // ==================== 入库状态 ====================

  /** 是否正在使用OpenSteamTool入库 */
  const isImportingWithOpenSteamTool = ref(false)

  /** 是否正在准备导入（解压或扫描中） */
  const isPreparingImport = ref(false)

  /** OpenSteamTool高级选项 - 锁定版本 */
  const lockVersion = ref(false)

  /** OpenSteamTool高级选项 - 访问令牌 */
  const accessToken = ref('')

  /** OpenSteamTool高级选项 - 成就SteamID */
  const statsSteamId = ref('')

  /** 自定义清单源模式 */
  const importSourceMode = ref<'7z' | 'folder'>('7z')

  /** 选中的导入路径 */
  const selectedImportPath = ref('')

  /** 选中的导入临时路径（7z解压后的路径） */
  const selectedImportTempPath = ref('')

  // ==================== 外部状态引用 ====================
  // 这些状态需要从外部传入或通过参数获取

  /** hasLua 状态的引用（需要从外部设置） */
  let hasLuaRef = ref(false)

  // ==================== 计算属性 ====================

  /** 自定义清单源是否准备就绪 */
  const importSourceReady = computed(() => {
    if (!selectedImportPath.value) return false
    // 7z模式需要已经解压到临时目录并检测到lua
    if (importSourceMode.value === '7z' && !selectedImportTempPath.value) return false
    return hasLuaRef.value
  })

  // ==================== 方法 ====================

  /**
   * 初始化状态引用
   * @param hasLua hasLua状态的ref
   */
  function initRefs(hasLua: { value: boolean }) {
    hasLuaRef = hasLua as any
  }

  /**
   * 设置hasLua引用（用于响应式更新）
   */
  function setHasLuaRef(ref: { value: boolean }) {
    hasLuaRef = ref as any
  }

  /**
   * 选择7z压缩包作为清单源
   */
  async function selectImportArchive(
    _checkGameManifest: () => Promise<void>
  ): Promise<void> {
    try {
      const selected = await open({
        multiple: false,
        filters: [
          { name: '7z压缩包', extensions: ['7z'] }
        ],
        title: '选择清单7z压缩包'
      })

      if (!selected || typeof selected !== 'string') {
        return
      }

      selectedImportPath.value = selected
      selectedImportTempPath.value = ''
      isPreparingImport.value = true

      try {
        // 解压到临时目录
        const tempFolder = await invoke<string>('extract_archive', {
          archivePath: selected
        })

        selectedImportTempPath.value = tempFolder

        // 扫描并转换
        const result = await scanAndConvertFolder(tempFolder)
        hasLuaRef.value = result.hasLua

        if (!result.hasLua) {
          alert('未在压缩包中找到vdf或lua文件，无法入库')
        }
      } catch (error) {
        alert(`解压失败: ${error}`)
        selectedImportPath.value = ''
        selectedImportTempPath.value = ''
      } finally {
        isPreparingImport.value = false
      }
    } catch (error) {
      alert(`选择文件失败: ${error}`)
    }
  }

  /**
   * 选择lua所在文件夹作为清单源
   */
  async function selectImportFolder(
    _checkGameManifest: () => Promise<void>
  ): Promise<void> {
    try {
      const selected = await open({
        directory: true,
        title: '选择包含lua/vdf文件的文件夹'
      })

      if (!selected || typeof selected !== 'string') {
        return
      }

      selectedImportPath.value = selected
      selectedImportTempPath.value = ''
      isPreparingImport.value = true

      try {
        // 扫描并转换
        const result = await scanAndConvertFolder(selected)
        hasLuaRef.value = result.hasLua

        if (!result.hasLua) {
          alert('未在文件夹中找到vdf或lua文件，无法入库')
        }
      } catch (error) {
        alert(`扫描失败: ${error}`)
        selectedImportPath.value = ''
      } finally {
        isPreparingImport.value = false
      }
    } catch (error) {
      alert(`选择文件夹失败: ${error}`)
    }
  }

  /**
   * 清除自定义清单源选择
   */
  function clearImportSource(
    checkGameManifest: () => Promise<void>
  ): void {
    selectedImportPath.value = ''
    selectedImportTempPath.value = ''
    // 重新检查内置清单
    checkGameManifest()
  }

  /**
   * 使用OpenSteamTool内核导入游戏到Steam
   */
  async function importWithOpenSteamTool(
    game: GameConfigData | undefined,
    gameId: string,
    _hasLua: boolean,
    importSourceReady: boolean
  ): Promise<void> {
    if (isImportingWithOpenSteamTool.value) return

    if (!game) {
      alert('游戏数据未加载')
      return
    }

    // 解析AppID
    const appId = parseInt(game.game_id, 10)
    if (isNaN(appId) || appId <= 0) {
      alert('游戏ID无效，无法作为Steam AppID使用')
      return
    }

    // 检查Steam路径
    let steamPath = configStore.config?.gameDirs?.steamPath
    if (!steamPath) {
      const selected = await open({
        directory: true,
        title: '请选择Steam安装目录'
      })

      if (!selected) {
        return
      }

      steamPath = selected
      await configStore.updateConfig({
        gameDirs: {
          steamPath: selected,
          coversPath: configStore.config?.gameDirs?.coversPath || 'data/covers'
        }
      })
    }

    // 高级模式确认
    const advancedMode = configStore.config?.opensteamtool?.advancedMode ?? false
    const hotReload = configStore.config?.opensteamtool?.hotReload ?? true
    if (advancedMode) {
      const confirmAdvanced = confirm(
        '高级模式已启用，将写入Windows注册表。\n\n' +
        '这通常用于Denuvo/在线游戏，但也可能带来风险。\n\n' +
        '是否继续？'
      )
      if (!confirmAdvanced) {
        return
      }
    }

    isImportingWithOpenSteamTool.value = true

    try {
      let result: {
        success: boolean
        message: string
        kernelInstalled: boolean
        luaWritten: boolean
        manifestCopied: number
        steamRestarted: boolean
        advancedEnabled: boolean
      }

      if (importSourceReady) {
        // 使用自定义清单源
        const folderPath = importSourceMode.value === '7z'
          ? selectedImportTempPath.value
          : selectedImportPath.value

        result = await invoke<{
          success: boolean
          message: string
          kernelInstalled: boolean
          luaWritten: boolean
          manifestCopied: number
          steamRestarted: boolean
          advancedEnabled: boolean
        }>('import_manifest_with_opensteamtool', {
          steamPath: steamPath,
          folderPath: folderPath,
          gameName: game.chinese_name || game.game_name || gameId,
          appId: appId,
          advancedMode: advancedMode,
          hotReload: hotReload,
          lockVersion: lockVersion.value,
          accessToken: accessToken.value || undefined,
          statsSteamId: statsSteamId.value || undefined
        })
      } else {
        // 使用内置清单
        result = await invoke<{
          success: boolean
          message: string
          kernelInstalled: boolean
          luaWritten: boolean
          manifestCopied: number
          steamRestarted: boolean
          advancedEnabled: boolean
        }>('import_game_with_opensteamtool', {
          steamPath: steamPath,
          gameId: gameId,
          gameName: game.chinese_name || game.game_name || gameId,
          appId: appId,
          advancedMode: advancedMode,
          hotReload: hotReload,
          lockVersion: lockVersion.value,
          accessToken: accessToken.value || undefined,
          statsSteamId: statsSteamId.value || undefined
        })
      }

      if (result.success) {
        const message =
          `OpenSteamTool入库成功！\n\n` +
          `游戏: ${game.chinese_name || game.game_name}\n` +
          `AppID: ${appId}\n` +
          `内核DLL: ${result.kernelInstalled ? '已安装' : '未安装'}\n` +
          `Lua文件: ${result.luaWritten ? '已写入' : '未写入'}\n` +
          `Manifest文件: ${result.manifestCopied}个\n` +
          `Steam: ${result.steamRestarted ? '已重启' : '未重启'}\n` +
          `${result.advancedEnabled ? '高级模式: 已启用（写入注册表）' : ''}`
        alert(message)
      } else {
        alert(`OpenSteamTool入库失败: ${result.message}`)
      }
    } catch (error) {
      alert(`OpenSteamTool入库失败: ${error}`)
    } finally {
      isImportingWithOpenSteamTool.value = false
    }
  }

  /**
   * 重启Steam
   */
  async function restartSteam(): Promise<void> {
    try {
      const result = await invoke<{
        success: boolean
        message: string
      }>('restart_steam')

      if (result.success) {
        alert('Steam重启成功！')
      } else {
        alert(`重启Steam失败: ${result.message}`)
      }
    } catch (error) {
      alert(`重启Steam失败: ${error}`)
    }
  }

  return {
    // 状态
    isImportingWithOpenSteamTool,
    isPreparingImport,
    lockVersion,
    accessToken,
    statsSteamId,
    importSourceMode,
    selectedImportPath,
    selectedImportTempPath,
    importSourceReady,

    // 方法
    initRefs,
    setHasLuaRef,
    selectImportArchive,
    selectImportFolder,
    clearImportSource,
    importWithOpenSteamTool,
    restartSteam
  }
}