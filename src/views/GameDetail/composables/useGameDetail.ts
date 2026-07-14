/**
 * useGameDetail composable
 * 游戏详情页共享状态管理
 */

import { ref, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import type { GameConfigData } from '../../../types'
import { loadGamesConfigFromFile } from '../../../api/game.api'
import { getCachedCoverImage } from '../../../services/imageCache.service'
import { 
  getGameData,
  type GameData 
} from '../../../api/gameData.api'

/**
 * 游戏详情页共享状态
 */
export function useGameDetail() {
  const route = useRoute()
  const router = useRouter()

  // ==================== 核心状态 ====================
  
  /** 游戏ID */
  const gameId = computed(() => route.params.id as string)
  
  /** 游戏配置列表 */
  const gamesConfig = ref<GameConfigData[]>([])
  
  /** 当前游戏配置 */
  const game = computed(() => gamesConfig.value.find(g => g.game_id === gameId.value))
  
  /** 已存在的游戏数据（从game.json加载） */
  const existingGameData = ref<GameData | null>(null)
  
  /** 封面图片 */
  const coverImage = ref('')

  // ==================== 路径状态 ====================
  
  /** 下载路径 */
  const downloadPath = ref('')
  
  /** 游戏路径 */
  const gamePath = ref('')
  
  /** 清单文件夹路径（自动检测） */
  const manifestFolderPath = ref('')

  // ==================== 清单检测状态 ====================
  
  /** 清单文件夹检测状态 */
  const manifestCheckStatus = ref<'checking' | 'found' | 'not_found'>('checking')
  
  /** manifest是否存在 */
  const manifestExists = ref(false)
  
  /** lua文件是否存在 */
  const hasLua = ref(false)
  
  /** vdf文件是否存在 */
  const hasVdf = ref(false)
  
  /** manifest文件是否存在 */
  const hasManifest = ref(false)

  // ==================== UI状态 ====================
  
  /** 当前选中的标签页 */
  const currentTab = ref('')
  
  /** 修改器本地文件内容 */
  const trainerContent = ref<string>('')

  // ==================== 方法 ====================

  /**
   * 加载游戏配置
   */
  async function loadGamesConfig() {
    try {
      gamesConfig.value = await loadGamesConfigFromFile()
    } catch (error) {
      console.error('加载游戏配置失败:', error)
    }
  }

  /**
   * 加载游戏数据（从game.json）
   */
  async function loadExistingGameData() {
    if (!gameId.value) return
    try {
      const data = await getGameData(gameId.value)
      existingGameData.value = data
    } catch {
      existingGameData.value = null
    }
  }

  /**
   * 加载封面图片
   */
  async function loadCoverImage() {
    if (!gameId.value) return
    try {
      coverImage.value = await getCachedCoverImage(gameId.value)
    } catch {
      coverImage.value = ''
    }
  }

  /**
   * 根据游戏数据设置默认标签页
   */
  function setDefaultTab() {
    if (game.value?.downloadable) {
      currentTab.value = 'download'
    } else if (game.value?.has_extract_play === true) {
      currentTab.value = 'extract-play'
    } else if (game.value?.tags && game.value.tags.length > 0) {
      currentTab.value = `patch-${game.value.tags[0].patch_type}`
    } else {
      currentTab.value = ''
    }
  }

  /**
   * 返回上一页
   */
  function goBack() {
    router.back()
  }

  /**
   * 初始化游戏详情页
   */
  async function initialize() {
    await loadGamesConfig()
    await loadExistingGameData()
    await loadCoverImage()
    setDefaultTab()
  }

  return {
    // 核心状态
    gameId,
    gamesConfig,
    game,
    existingGameData,
    coverImage,
    
    // 路径状态
    downloadPath,
    gamePath,
    manifestFolderPath,
    
    // 清单检测状态
    manifestCheckStatus,
    manifestExists,
    hasLua,
    hasVdf,
    hasManifest,
    
    // UI状态
    currentTab,
    trainerContent,
    
    // 方法
    initialize,
    goBack,
    setDefaultTab,
    loadExistingGameData
  }
}