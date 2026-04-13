import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

// 游戏标签类型
export interface GameTag {
  patch_type: number
  patch_source_path: string
  download_url?: string  // 可选的网盘下载链接
}

// 游戏数据类型定义
export interface Game {
  id: string
  game_id: string
  game_name: string
  chinese_name: string
  downloadable: boolean
  tags: GameTag[]
  coverUrl: string
}

// patch_type 映射
// 0: 免Steam启动
// 1: 局域网联机
// 2: Steam联机
// 3: D加密虚拟机

// 分类映射
const categoryMap: Record<string, number> = {
  'all': -1,
  'no-steam': 0,
  'lan-multiplayer': 1,
  'steam-multiplayer': 2,
  'denuvo-vm': 3
}

// 获取分类标签名称
export const getCategoryName = (patchType: number): string => {
  const names: Record<number, string> = {
    0: '免 Steam 启动',
    1: '局域网联机',
    2: 'Steam 联机',
    3: 'D 加密虚拟机'
  }
  return names[patchType] || '未知分类'
}

// 获取分类标签颜色
export const getCategoryColor = (patchType: number): string => {
  const colors: Record<number, string> = {
    0: '#22c55e', // 绿色
    1: '#3b82f6', // 蓝色
    2: '#f59e0b', // 橙色
    3: '#ef4444'  // 红色
  }
  return colors[patchType] || '#6b7280'
}

// 缓存游戏封面图片
const coverCache = new Map<string, string>()

export const useGamesStore = defineStore('games', () => {
  // 游戏列表
  const games = ref<Game[]>([])
  // 搜索关键词
  const searchQuery = ref('')
  // 当前分类筛选
  const currentCategory = ref('all')
  // 加载状态
  const loading = ref(false)
  // 错误信息
  const error = ref<string | null>(null)

  // 根据分类和搜索词筛选游戏
  const filteredGames = computed(() => {
    let result = games.value

    // 按分类筛选 - 根据 patch_type
    if (currentCategory.value !== 'all') {
      const targetPatchType = categoryMap[currentCategory.value]
      result = result.filter(game =>
        game.tags.some(tag => tag.patch_type === targetPatchType)
      )
    }

    // 按搜索词筛选 - 支持中文名、英文名、Game ID
    if (searchQuery.value.trim()) {
      const query = searchQuery.value.toLowerCase()
      result = result.filter(game =>
        game.chinese_name.toLowerCase().includes(query) ||
        game.game_name.toLowerCase().includes(query) ||
        game.game_id.includes(query)
      )
    }

    return result
  })

  // 根据ID获取游戏
  const getGameById = (id: string): Game | undefined => {
    return games.value.find(game => game.game_id === id)
  }

  // 加载单个游戏封面图片
  const loadGameCover = async (gameId: string): Promise<string> => {
    // 检查缓存
    if (coverCache.has(gameId)) {
      return coverCache.get(gameId)!
    }

    try {
      // 通过 Tauri 后端读取游戏封面图片
      const coverDataUrl = await invoke<string>('get_game_cover', { gameId })
      // 缓存结果
      coverCache.set(gameId, coverDataUrl)
      return coverDataUrl
    } catch (err) {
      console.error(`加载游戏封面失败 (${gameId}):`, err)
      return ''
    }
  }

  // 加载游戏数据 - 从 resources/games_config.json
  const loadGames = async () => {
    loading.value = true
    error.value = null
    try {
      console.log('开始加载游戏数据...')
      // 通过 Tauri 后端读取 games_config.json 文件
      const jsonContent = await invoke<string>('read_games_config')
      console.log('读取到配置内容长度:', jsonContent.length)

      let data: any[]
      try {
        data = JSON.parse(jsonContent)
      } catch (parseErr) {
        console.error('JSON 解析失败:', parseErr)
        error.value = '配置文件格式错误'
        games.value = []
        return
      }

      console.log('解析到游戏数量:', data.length)

      // 转换数据格式
      games.value = data.map((item: any) => ({
        id: item.game_id,
        game_id: item.game_id,
        game_name: item.game_name,
        chinese_name: item.chinese_name,
        downloadable: item.downloadable,
        tags: (item.tags || []).map((tag: any) => ({
          patch_type: tag.patch_type,
          patch_source_path: tag.patch_source_path,
          download_url: tag.download_url
        })),
        coverUrl: '' // 初始为空，由组件懒加载
      }))

      console.log('游戏数据转换完成，封面将按需懒加载...')
      // 封面不再一次性加载，由 GameCard 组件按需加载
    } catch (err) {
      console.error('加载游戏数据失败:', err)
      error.value = String(err)
      games.value = []
    } finally {
      loading.value = false
    }
  }

  // 按需加载游戏封面 - 由组件调用
  const loadGameCoverOnDemand = async (gameId: string): Promise<string> => {
    const game = games.value.find(g => g.game_id === gameId)
    if (!game) return ''
    
    // 如果已经加载过，直接返回
    if (game.coverUrl) {
      return game.coverUrl
    }
    
    // 否则加载封面
    const coverUrl = await loadGameCover(gameId)
    game.coverUrl = coverUrl
    return coverUrl
  }

  // 设置搜索关键词
  const setSearchQuery = (query: string) => {
    searchQuery.value = query
  }

  // 设置当前分类
  const setCategory = (category: string) => {
    currentCategory.value = category
  }

  return {
    games,
    searchQuery,
    currentCategory,
    loading,
    error,
    filteredGames,
    getGameById,
    loadGames,
    loadGameCover,
    loadGameCoverOnDemand,
    setSearchQuery,
    setCategory
  }
})
