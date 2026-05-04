<template>
  <div class="library-page">
    <!-- 左侧游戏列表 - 玻璃态侧边栏 -->
    <div class="library-sidebar">
      <!-- 搜索框 - 毛玻璃效果 -->
      <div class="sidebar-header">
        <div class="search-box">
          <svg class="search-icon" viewBox="0 0 24 24" fill="currentColor">
            <path d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>
          </svg>
          <input
            v-model="searchKeyword"
            type="text"
            placeholder="搜索游戏..."
            @input="handleSearch"
          />
        </div>
        
        <!-- 导入按钮 -->
        <button class="import-btn" @click="showImportDialog = true" title="导入本地游戏">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
          </svg>
        </button>
      </div>

      <!-- 游戏列表 - 超紧凑卡片设计 -->
      <div class="games-list-container" v-if="filteredGames.length > 0">
        <!-- 收藏游戏区域 -->
        <div v-if="favoriteGames.length > 0" class="list-section">
          <div class="section-title">
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z"/>
            </svg>
            收藏
          </div>
          <div class="games-list">
            <div
              v-for="game in favoriteGames"
              :key="game.game_id"
              class="game-list-item"
              :class="{ active: selectedGame?.game_id === game.game_id }"
              @click="selectGame(game)"
              @contextmenu.prevent="showContextMenu($event, game)"
            >
              <div class="game-thumb">
                <img
                  v-if="gameCovers[game.game_id]"
                  :src="gameCovers[game.game_id]"
                  alt=""
                />
                <div v-else class="thumb-placeholder">
                  {{ game.chinese_name.slice(0, 1) }}
                </div>
              </div>
              <div class="game-info">
                <span class="game-name">{{ game.chinese_name }}</span>
              </div>
              <div class="favorite-indicator">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z"/>
                </svg>
              </div>
            </div>
          </div>
        </div>

        <!-- 全部游戏区域 -->
        <div class="list-section" v-if="nonFavoriteGames.length > 0">
          <div class="section-title">
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M21 6H3c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-10 7H8v3H6v-3H3v-2h3V8h2v3h3v2zm4.5 2c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm4-3c-.83 0-1.5-.67-1.5-1.5S18.67 9 19.5 9s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z"/>
            </svg>
            全部游戏
          </div>
          <div class="games-list">
            <div
              v-for="game in nonFavoriteGames"
              :key="game.game_id"
              class="game-list-item"
              :class="{ active: selectedGame?.game_id === game.game_id }"
              @click="selectGame(game)"
              @contextmenu.prevent="showContextMenu($event, game)"
            >
              <div class="game-thumb">
                <img
                  v-if="gameCovers[game.game_id]"
                  :src="gameCovers[game.game_id]"
                  alt=""
                />
                <div v-else class="thumb-placeholder">
                  {{ game.chinese_name.slice(0, 1) }}
                </div>
              </div>
              <div class="game-info">
                <span class="game-name">{{ game.chinese_name }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 空状态 - 当没有游戏时显示 -->
      <div v-else class="empty-sidebar">
        <div class="empty-illustration">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M21 6H3c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-10 7H8v3H6v-3H3v-2h3V8h2v3h3v2zm4.5 2c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm4-3c-.83 0-1.5-.67-1.5-1.5S18.67 9 19.5 9s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z"/>
          </svg>
        </div>
        <p class="empty-title">暂无游戏</p>
        <p class="empty-subtitle">导入本地游戏或前往浏览页面</p>
      </div>

      <!-- 右键菜单 -->
      <div v-if="contextMenu.visible" class="context-menu" :style="{ top: contextMenu.y + 'px', left: contextMenu.x + 'px' }">
        <div class="context-menu-item" @click="toggleFavorite(contextMenu.game)">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z"/>
          </svg>
          {{ contextMenu.game?.is_favorite ? '取消收藏' : '收藏游戏' }}
        </div>
        <div class="context-menu-divider"></div>
        <div class="context-menu-item" @click="editGameFromContext(contextMenu.game)">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
          </svg>
          编辑游戏
        </div>
        <div class="context-menu-item" @click="browseLocalFiles(contextMenu.game)">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm0 12H4V8h16v10z"/>
          </svg>
          浏览本地文件
        </div>
        <div class="context-menu-divider"></div>
        <div class="context-menu-item danger" @click="uninstallGame(contextMenu.game)">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
          </svg>
          卸载
        </div>
      </div>
    </div>

    <!-- 右侧游戏详情 -->
    <div class="library-content" v-if="selectedGame">
      <!-- 游戏封面大图区域 -->
      <div class="game-hero" :class="{ 'no-image': !selectedGame?.cover_path }">
        <img
          v-if="selectedGame?.cover_path"
          :src="gameCovers[selectedGame.game_id] || selectedGame.cover_path"
          class="hero-image"
          alt=""
        />
        <!-- 内容层 -->
        <div class="hero-overlay">
          <div class="hero-content">
            <!-- 平台标签 -->
            <div class="platform-tags">
              <span class="platform-tag pc">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M20 18c1.1 0 1.99-.9 1.99-2L22 6c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2H0v2h24v-2h-4zM4 6h16v10H4V6z"/>
                </svg>
                Windows
              </span>
              <span class="platform-tag date" v-if="selectedGame.added_date">
                {{ formatDate(selectedGame.added_date) }} 添加
              </span>
            </div>
            
            <!-- 游戏标题 -->
            <h1 class="hero-title">{{ selectedGame.chinese_name }}</h1>
            <p class="hero-subtitle">{{ selectedGame.game_name }}</p>
            
            <!-- 快捷操作按钮 -->
            <div class="hero-actions">
              <button 
                v-if="!isGameRunning || runningGameId !== selectedGame.game_id"
                class="play-btn-large" 
                @click="launchGame" 
                :disabled="!selectedGame.exe_path"
              >
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M8 5v14l11-7z"/>
                </svg>
                <span>开始游戏</span>
              </button>
              <button 
                v-else
                class="close-btn-large" 
                @click="closeGame"
              >
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/>
                </svg>
                <span>停止游戏</span>
              </button>
              
              <button class="action-btn" @click="editGame" title="编辑">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
                </svg>
              </button>
              
              <button 
                class="action-btn" 
                :class="{ active: selectedGame.is_favorite }"
                @click="toggleFavorite(selectedGame)" 
                title="收藏"
              >
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z"/>
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 游戏信息卡片区域 -->
      <div class="game-info-section">
        <div class="info-grid">
          <!-- Steam ID 卡片 -->
          <div class="info-card">
            <div class="info-card-bg"></div>
            <div class="info-icon steam">
              <svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z"/>
              </svg>
            </div>
            <div class="info-content">
              <span class="info-label">Steam Game ID</span>
              <span class="info-value id">{{ selectedGame.steam_game_id || selectedGame.game_id }}</span>
            </div>
          </div>

          <!-- 安装路径卡片 -->
          <div class="info-card">
            <div class="info-card-bg"></div>
            <div class="info-icon folder">
              <svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z"/>
              </svg>
            </div>
            <div class="info-content">
              <span class="info-label">安装路径</span>
              <span class="info-value path" :title="selectedGame.install_path">{{ selectedGame.install_path || '未设置' }}</span>
            </div>
          </div>

          <!-- 最后运行卡片 -->
          <div class="info-card">
            <div class="info-card-bg"></div>
            <div class="info-icon time">
              <svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm.5-13H11v6l5.25 3.15.75-1.23-4.5-2.67z"/>
              </svg>
            </div>
            <div class="info-content">
              <span class="info-label">最后运行</span>
              <span class="info-value">{{ selectedGame.last_played ? formatDate(selectedGame.last_played) : '从未运行' }}</span>
            </div>
          </div>

          <!-- 游戏时长卡片 -->
          <div class="info-card">
            <div class="info-card-bg"></div>
            <div class="info-icon clock">
              <svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.5 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm.5-13H11v6l5.2 3.2.8-1.3-4.5-2.7z"/>
              </svg>
            </div>
            <div class="info-content">
              <span class="info-label">游戏时长</span>
              <span class="info-value">
                {{ formatPlayTime(selectedGame.play_time) }}
                <span v-if="isGameRunning && runningGameId === selectedGame.game_id" class="current-session">
                  +{{ formatDuration(currentPlayTime) }}
                </span>
              </span>
            </div>
          </div>
        </div>

        <!-- 额外信息区域 -->
        <div class="extra-info">
          <div class="info-block">
            <h4>游戏信息</h4>
            <div class="info-row">
              <span class="info-key">游戏类型</span>
              <span class="info-val">{{ getGameTypeLabel(selectedGame.game_type) }}</span>
            </div>
            <div class="info-row">
              <span class="info-key">主程序</span>
              <span class="info-val path" :title="selectedGame.exe_path">{{ selectedGame.exe_path ? selectedGame.exe_path.split(/[\\/]/).pop() : '未设置' }}</span>
            </div>
            <div class="info-row" v-if="selectedGame.save_path">
              <span class="info-key">存档路径</span>
              <span class="info-val path" :title="selectedGame.save_path">{{ selectedGame.save_path }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 未选择游戏时的默认内容 -->
    <div class="library-content empty" v-else>
      <div class="empty-state">
        <div class="empty-illustration large">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M21 6H3c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-10 7H8v3H6v-3H3v-2h3V8h2v3h3v2zm4.5 2c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm4-3c-.83 0-1.5-.67-1.5-1.5S18.67 9 19.5 9s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z"/>
          </svg>
        </div>
        <p class="empty-title">从左侧选择游戏</p>
        <p class="empty-subtitle">或前往浏览页面下载新游戏</p>
        <button class="browse-btn" @click="goToBrowse">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z"/>
          </svg>
          前往浏览
        </button>
      </div>
    </div>

    <!-- 导入游戏对话框 - 使用 GameFormDialog 复用组件 -->
    <GameFormDialog
      v-model="importForm"
      :visible="showImportDialog"
      title="导入本地游戏"
      confirm-text="导入"
      :auto-fill-name="true"
      @close="showImportDialog = false"
      @confirm="confirmImport"
    />

    <!-- 编辑游戏对话框 - 使用 GameFormDialog 复用组件 -->
    <GameFormDialog
      v-model="editForm"
      :visible="showEditDialog"
      title="编辑游戏信息"
      confirm-text="保存"
      @close="showEditDialog = false"
      @confirm="confirmEdit"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import {
  getAllGamesData,
  importCustomGame,
  launchGameWithTracking,
  closeGameProcess,
  updateGameData,
  upsertGameData,
  toggleGameFavorite,
  checkGameProcessStatus,
  formatPlayTime,
  formatDate,
  type GameData
} from '../../api/gameData.api'
import { getCachedCoverImage, getCachedLibraryImage } from '../../services/imageCache.service'
import GameFormDialog from '../../components/game/GameFormDialog.vue'
import type { GameFormData } from '../../components/game/GameFormDialog.vue'

const router = useRouter()

// 状态
const games = ref<GameData[]>([])
const gameCovers = ref<Record<string, string>>({})
const searchKeyword = ref('')
const selectedGame = ref<GameData | null>(null)
const showImportDialog = ref(false)
const showEditDialog = ref(false)

// 右键菜单状态
const contextMenu = ref({
  visible: false,
  x: 0,
  y: 0,
  game: null as GameData | null
})

// 游戏进程状态
const gameProcessPid = ref<number | null>(null)
const isGameRunning = ref(false)
const runningGameId = ref<string | null>(null)
const gameStartTime = ref<Date | null>(null)
const currentPlayTime = ref(0)
let gameMonitorInterval: number | null = null
let displayFrameId: number | null = null

// 导入表单
const importForm = ref({
  game_name: '',
  chinese_name: '',
  install_path: '',
  exe_path: '',
  save_path: '',
  cover_path: '',
  steam_game_id: ''
})

// 编辑表单
const editForm = ref({
  game_name: '',
  chinese_name: '',
  install_path: '',
  exe_path: '',
  save_path: '',
  cover_path: '',
  steam_game_id: ''
})

const filteredGames = computed(() => {
  // 只显示已安装的游戏
  const installedGames = games.value.filter(game => game.is_installed)
  if (!searchKeyword.value) return installedGames
  const keyword = searchKeyword.value.toLowerCase()
  return installedGames.filter(game =>
    game.chinese_name.toLowerCase().includes(keyword) ||
    game.game_name.toLowerCase().includes(keyword) ||
    game.game_id.includes(keyword)
  )
})

const favoriteGames = computed(() => {
  return filteredGames.value.filter(g => g.is_favorite)
})

const nonFavoriteGames = computed(() => {
  return filteredGames.value.filter(g => !g.is_favorite)
})

const getGameTypeLabel = (type: string): string => {
  const labels: Record<string, string> = {
    'downloaded': '已下载',
    'imported': '已导入'
  }
  return labels[type] || type
}

// 显示右键菜单
const showContextMenu = (event: MouseEvent, game: GameData) => {
  contextMenu.value = {
    visible: true,
    x: event.clientX - 50, // 向左移动50px
    y: event.clientY,
    game
  }
}

// 隐藏右键菜单
const hideContextMenu = () => {
  contextMenu.value.visible = false
}

// 切换收藏状态
const toggleFavorite = async (game: GameData | null) => {
  if (!game) return
  try {
    await toggleGameFavorite(game.game_id)
    hideContextMenu()
    await loadGames()
    // 更新选中游戏的状态
    if (selectedGame.value && selectedGame.value.game_id === game.game_id) {
      const updated = games.value.find(g => g.game_id === game.game_id)
      if (updated) selectedGame.value = updated
    }
  } catch (error) {
    alert('操作失败: ' + error)
  }
}

// 从右键菜单编辑游戏
const editGameFromContext = (game: GameData | null) => {
  hideContextMenu()
  if (!game) return
  selectedGame.value = game
  editGame()
}

// 浏览本地文件
const browseLocalFiles = async (game: GameData | null) => {
  hideContextMenu()
  if (!game) return
  try {
    // 调用后端命令使用 explorer.exe 打开游戏目录
    await invoke('open_game_directory', { path: game.install_path })
  } catch (error) {
    alert('打开游戏目录失败: ' + error)
  }
}

// 卸载游戏（只删除游戏文件，保留游戏数据）
const uninstallGame = async (game: GameData | null) => {
  hideContextMenu()
  if (!game) return

  // 确认对话框
  const confirmed = confirm(`确定要卸载 "${game.chinese_name}" 吗？\n\n这将删除游戏目录：\n${game.install_path}\n\n游戏数据（配置、游玩时长等）将保留，下次下载或导入时可继续使用。`)
  if (!confirmed) return

  try {
    // 调用后端删除游戏目录（保留存档）
    await invoke('delete_game_directory', {
      path: game.install_path,
      savePath: game.save_path
    })

    // 更新游戏状态为未安装，但保留游戏数据
    // 同时更新 download_status 为 ''，这样游戏详情页会显示为未下载
    const updatedGame: GameData = {
      ...game,
      is_installed: false,
      install_path: '',
      exe_path: '',
      download_status: '',
      download_progress: 0
    }
    await upsertGameData(updatedGame)

    // 重新加载游戏列表
    await loadGames()

    // 更新选中游戏的状态
    if (selectedGame.value?.game_id === game.game_id) {
      const updated = games.value.find(g => g.game_id === game.game_id)
      if (updated) selectedGame.value = updated
    }

    alert('游戏已卸载，数据已保留')
  } catch (error) {
    alert('卸载游戏失败: ' + error)
  }
}

// 点击其他地方隐藏右键菜单
onMounted(() => {
  document.addEventListener('click', hideContextMenu)
})

onUnmounted(() => {
  document.removeEventListener('click', hideContextMenu)
})

// 加载游戏数据
const loadGames = async () => {
  try {
    games.value = await getAllGamesData()
    await loadGameCovers()
    // 如果有选中的游戏，更新其数据
    if (selectedGame.value) {
      const updated = games.value.find(g => g.game_id === selectedGame.value!.game_id)
      if (updated) selectedGame.value = updated
    }
  } catch (error) {
    // 加载游戏数据失败时静默处理
  }
}

// 加载游戏封面 - 使用全局缓存服务，避免资源竞争
const loadGameCovers = async () => {
  for (const game of games.value) {
    try {
      // 1. 优先使用 steam_game_id 查找图片，如果没有则使用 game_id
      // 注意：steam_game_id 可能是 null 或 undefined，需要处理
      const imageId = (game.steam_game_id && game.steam_game_id !== 'null' && String(game.steam_game_id) !== 'null')
        ? String(game.steam_game_id)
        : game.game_id

      // 先从默认路径加载 resources/pic/库/[imageId].xxx
      const libraryUrl = await getCachedLibraryImage(imageId)

      if (libraryUrl) {
        gameCovers.value[game.game_id] = libraryUrl
        continue
      }

      // 2. 库目录不存在，尝试使用 Game_Cover 目录的图片
      const coverUrl = await getCachedCoverImage(imageId)

      if (coverUrl) {
        gameCovers.value[game.game_id] = coverUrl
        continue
      }

      // 3. 默认路径都不存在，使用用户设置的封面路径
      if (game.cover_path) {
        gameCovers.value[game.game_id] = game.cover_path
        continue
      }

      // 4. 都没有，标记为无封面
    } catch (err) {
      // 加载封面失败时静默处理
    }
  }
}

// 选择游戏
const selectGame = (game: GameData) => {
  selectedGame.value = game
}

// 搜索
const handleSearch = () => {}

// 启动游戏
const launchGame = async () => {
  if (!selectedGame.value) return
  try {
    const pid = await launchGameWithTracking(selectedGame.value.game_id)
    gameProcessPid.value = pid
    isGameRunning.value = true
    runningGameId.value = selectedGame.value.game_id
    gameStartTime.value = new Date()
    currentPlayTime.value = 0
    startGameMonitoring()
    await loadGames()
    if (selectedGame.value) {
      const updated = games.value.find(g => g.game_id === selectedGame.value!.game_id)
      if (updated) selectedGame.value = updated
    }
  } catch (error) {
    alert('启动游戏失败: ' + error)
  }
}

// 监控游戏状态 - 定期检查进程是否还在运行
const startGameMonitoring = () => {
  if (gameMonitorInterval) clearInterval(gameMonitorInterval)
  
  // 使用 requestAnimationFrame 进行高效的UI更新（仅更新显示，不执行逻辑）
  let lastDisplayUpdate = 0
  const updateDisplay = (timestamp: number) => {
    if (!isGameRunning.value || !gameStartTime.value) return
    
    // 每500ms更新一次显示（足够流畅且性能开销极小）
    if (timestamp - lastDisplayUpdate >= 500) {
      const elapsedSecs = Math.floor((Date.now() - gameStartTime.value.getTime()) / 1000)
      currentPlayTime.value = elapsedSecs
      lastDisplayUpdate = timestamp
    }
    
    displayFrameId = requestAnimationFrame(updateDisplay)
  }
  
  // 启动显示更新循环
  let displayFrameId = requestAnimationFrame(updateDisplay)
  
  // 每10秒检查一次进程状态（降低频率减少性能影响）
  gameMonitorInterval = window.setInterval(async () => {
    if (!isGameRunning.value || !runningGameId.value) return
    
    try {
      const elapsedSecs = Math.floor((Date.now() - gameStartTime.value!.getTime()) / 1000)
      const [isRunning, minutes] = await checkGameProcessStatus(
        runningGameId.value,
        elapsedSecs
      )
      
      if (!isRunning) {
        // 游戏进程已结束，停止监控并保存时长
        cancelAnimationFrame(displayFrameId)
        await stopGameMonitoring()
        // 重新加载游戏数据以显示更新后的时长
        await loadGames()
      }
    } catch (error) {
      // 检查进程状态失败时静默处理
    }
  }, 10000) // 每10秒检查一次
}

const stopGameMonitoring = async (savePlayTime: boolean = true) => {
  if (gameMonitorInterval) {
    clearInterval(gameMonitorInterval)
    gameMonitorInterval = null
  }
  
  // 取消动画帧
  if (displayFrameId) {
    cancelAnimationFrame(displayFrameId)
    displayFrameId = null
  }
  
  // 如果需要保存游玩时长
    if (savePlayTime && runningGameId.value && gameStartTime.value) {
      const elapsedSecs = Math.floor((Date.now() - gameStartTime.value.getTime()) / 1000)
      const minutes = Math.floor(elapsedSecs / 60)

      if (minutes > 0) {
        try {
          // 调用后端保存游玩时长
          await invoke('update_game_play_time', {
            game_id: runningGameId.value,
            additional_minutes: minutes
          })
        } catch (error) {
          // 保存失败时静默处理
        }
      }
    }
  
  gameProcessPid.value = null
  isGameRunning.value = false
  runningGameId.value = null
  gameStartTime.value = null
  currentPlayTime.value = 0
  await loadGames()
  if (selectedGame.value) {
    const updated = games.value.find(g => g.game_id === selectedGame.value!.game_id)
    if (updated) selectedGame.value = updated
  }
}

// 关闭游戏
const closeGame = async () => {
  if (!gameProcessPid.value) return
  const confirmed = confirm('确定要关闭游戏吗？\n\n注意：关闭游戏可能会导致未保存的进度丢失！')
  if (!confirmed) return
  try {
    await closeGameProcess(gameProcessPid.value)
    // 停止监控并保存时长
    await stopGameMonitoring(true)
  } catch (error) {
    alert('关闭游戏失败: ' + error)
  }
}

// 跳转
const goToGameDetail = () => {
  if (selectedGame.value && selectedGame.value.game_type === 'downloaded') {
    router.push(`/game/${selectedGame.value.game_id}`)
  }
}

const goToBrowse = () => {
  router.push('/')
}

// 编辑游戏
const editGame = () => {
  if (!selectedGame.value) return
  editForm.value = {
    game_name: selectedGame.value.game_name,
    chinese_name: selectedGame.value.chinese_name,
    install_path: selectedGame.value.install_path,
    exe_path: selectedGame.value.exe_path,
    save_path: selectedGame.value.save_path || '',
    cover_path: selectedGame.value.cover_path || '',
    steam_game_id: selectedGame.value.steam_game_id || ''
  }
  showEditDialog.value = true
}

const confirmEdit = async () => {
  if (!selectedGame.value) return
  try {
    await updateGameData(selectedGame.value.game_id, {
      game_name: editForm.value.game_name,
      chinese_name: editForm.value.chinese_name,
      install_path: editForm.value.install_path,
      exe_path: editForm.value.exe_path,
      save_path: editForm.value.save_path || undefined,
      cover_path: editForm.value.cover_path || undefined,
      steam_game_id: editForm.value.steam_game_id || undefined
    })
    showEditDialog.value = false
    await loadGames()
    if (selectedGame.value) {
      const updated = games.value.find(g => g.game_id === selectedGame.value!.game_id)
      if (updated) selectedGame.value = updated
    }
    alert('游戏信息更新成功！')
  } catch (error) {
    alert('更新游戏失败: ' + error)
  }
}

const confirmImport = async () => {
  try {
    await importCustomGame(
      importForm.value.game_name,
      importForm.value.chinese_name,
      importForm.value.install_path,
      importForm.value.exe_path,
      importForm.value.save_path || undefined,
      importForm.value.cover_path || undefined,
      importForm.value.steam_game_id || undefined
    )
    showImportDialog.value = false
    importForm.value = { game_name: '', chinese_name: '', install_path: '', exe_path: '', save_path: '', cover_path: '', steam_game_id: '' }
    await loadGames()
    alert('游戏导入成功！')
  } catch (error) {
    alert('导入游戏失败: ' + error)
  }
}

const handleImageError = (e: Event) => {
  const img = e.target as HTMLImageElement
  img.style.display = 'none'
}

const formatDuration = (seconds: number): string => {
  if (seconds < 60) return `${seconds}秒`
  else if (seconds < 3600) {
    const mins = Math.floor(seconds / 60)
    const secs = seconds % 60
    return secs > 0 ? `${mins}分${secs}秒` : `${mins}分`
  } else {
    const hours = Math.floor(seconds / 3600)
    const mins = Math.floor((seconds % 3600) / 60)
    return mins > 0 ? `${hours}小时${mins}分` : `${hours}小时`
  }
}

onMounted(() => loadGames())
onUnmounted(() => { if (gameMonitorInterval) clearInterval(gameMonitorInterval) })
</script>

<style scoped>
/* ============================================
   库页面整体布局
   ============================================ */
.library-page {
  display: flex;
  width: 100%;
  height: 100%;
  position: relative;
  overflow: hidden;
  /* 确保库页面内容在标题栏下方 */
  margin-top: 0;
}

/* ============================================
   左侧边栏 - 玻璃态效果
   ============================================ */
.library-sidebar {
  width: 280px;
  min-width: 280px;
  background: rgba(79, 79, 79, 0.65);
  backdrop-filter: blur(25px) saturate(180%);
  border-right: 1px solid var(--steam-border);
  display: flex;
  flex-direction: column;
  z-index: 10;
  position: relative;
}

/* 侧边栏头部 - 搜索和导入按钮 */
.sidebar-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  border-bottom: 1px solid var(--steam-border);
  background: var(--steam-bg-secondary);
}

.search-box {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 10px;
  background: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  transition: all 0.2s ease;
}

.search-box:focus-within {
  background: var(--steam-bg-hover);
  border-color: var(--steam-accent-blue);
  box-shadow: 0 0 0 2px rgba(var(--steam-accent-blue-rgb), 0.1);
}

.search-box .search-icon {
  width: 14px;
  height: 14px;
  color: var(--steam-text-muted);
  flex-shrink: 0;
}

.search-box input {
  flex: 1;
  background: transparent;
  border: none;
  color: var(--steam-text-primary);
  font-size: 12px;
  outline: none;
}

.search-box input::placeholder {
  color: var(--steam-text-muted);
}

.import-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 34px;
  height: 34px;
  background: rgba(var(--steam-accent-green-rgb), 0.2);
  border: 1px solid rgba(var(--steam-accent-green-rgb), 0.3);
  border-radius: 8px;
  color: var(--steam-accent-green);
  cursor: pointer;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.import-btn:hover {
  background: rgba(var(--steam-accent-green-rgb), 0.3);
  border-color: rgba(var(--steam-accent-green-rgb), 0.5);
  transform: translateY(-1px);
}

.import-btn svg {
  width: 16px;
  height: 16px;
}

/* 游戏列表容器 */
.games-list-container {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

/* 列表分区 */
.list-section {
  margin-bottom: 12px;
}

.section-title {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 10px;
  font-size: 12px;
  font-weight: 600;
  color: var(--steam-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.section-title svg {
  width: 12px;
  height: 12px;
}

/* 游戏列表 */
.games-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

/* 游戏列表项 - 超紧凑卡片设计 (高度约为原来的1/3) */
.game-list-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 8px;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s ease;
  position: relative;
  background: var(--steam-bg-secondary);
  border: 1px solid transparent;
  height: 28px; /* 固定高度，约为原来的1/3 */
}

.game-list-item:hover {
  background: var(--steam-bg-hover);
  border-color: var(--steam-border);
}

.game-list-item.active {
  background: var(--steam-bg-tertiary);
  border-color: var(--steam-accent-blue);
}

.game-list-item.active::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 16px;
  background: var(--steam-accent-blue);
  border-radius: 0 2px 2px 0;
}

/* 游戏缩略图 - 超小尺寸 */
.game-thumb {
  width: 18px;
  height: 18px;
  border-radius: 3px;
  overflow: hidden;
  flex-shrink: 0;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.4);
  background: var(--steam-bg-tertiary);
}

.game-thumb img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.thumb-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: 600;
  color: var(--steam-text-muted);
  background: var(--steam-bg-quaternary);
}

/* 游戏信息 */
.game-info {
  display: flex;
  flex-direction: column;
  gap: 0;
  flex: 1;
  min-width: 0;
  justify-content: center;
}

.game-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  line-height: 1.3;
}

.game-list-item.active .game-name {
  color: var(--steam-accent-blue);
}

.game-meta {
  font-size: 12px;
  color: var(--steam-text-muted);
}

/* 收藏指示器 */
.favorite-indicator {
  flex-shrink: 0;
}

.favorite-indicator svg {
  width: 12px;
  height: 12px;
  color: #ffd700;
  filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.5));
}

/* 右键菜单 */
.context-menu {
  position: fixed;
  background: var(--steam-bg-primary);
  backdrop-filter: blur(30px) saturate(200%);
  border: 1px solid var(--steam-border);
  border-radius: 10px;
  padding: 6px;
  z-index: 1000;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
  min-width: 150px;
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  color: var(--steam-text-primary);
  transition: all 0.15s ease;
}

.context-menu-item:hover {
  background: var(--steam-bg-hover);
}

.context-menu-item.danger {
  color: var(--steam-accent-red);
}

.context-menu-item.danger:hover {
  background: rgba(255, 77, 79, 0.1);
}

.context-menu-item svg {
  width: 16px;
  height: 16px;
  color: var(--steam-accent-gold);
}

.context-menu-item.danger svg {
  color: var(--steam-accent-red);
}

.context-menu-divider {
  height: 1px;
  background: var(--steam-border);
  margin: 6px 0;
}

/* 空侧边栏 */
.empty-sidebar {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 30px 20px;
  text-align: center;
}

.empty-illustration {
  width: 60px;
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--steam-bg-tertiary);
  border-radius: 16px;
  margin-bottom: 16px;
  border: 1px solid var(--steam-border);
}

.empty-illustration svg {
  width: 28px;
  height: 28px;
  color: var(--steam-text-muted);
}

.empty-title {
  margin: 0 0 4px 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.empty-subtitle {
  margin: 0;
  font-size: 12px;
  color: var(--steam-text-muted);
}

/* ============================================
   右侧内容区
   ============================================ */
.library-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  position: relative;
  z-index: 10;
  overflow: hidden;
}

.library-content.empty {
  align-items: center;
  justify-content: center;
}

/* ============================================
   游戏封面大图 - 沉浸式Hero区域
   ============================================ */
.game-hero {
  position: relative;
  height: 420px;
  flex-shrink: 0;
  overflow: hidden;
  background: rgba(89, 89, 89, 0.7);
}

.hero-image {
  width: 100%;
  height: 100%;
  object-fit: contain;
  object-position: center top;
}



.hero-overlay {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 40px 48px 50px;
}

.hero-content {
  max-width: 800px;
}

/* 平台标签 */
.platform-tags {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 16px;
}

.platform-tag {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: rgba(99, 99, 99, 0.6);
  backdrop-filter: blur(10px) saturate(180%);
  border-radius: 20px;
  font-size: 12px;
  font-weight: 500;
  border: 1px solid var(--steam-border);
}

.platform-tag.pc {
  color: var(--steam-text-primary);
}

.platform-tag.date {
  color: var(--steam-text-muted);
  background: rgba(89, 89, 89, 0.6);
}

.platform-tag svg {
  width: 12px;
  height: 12px;
}

/* 游戏标题 */
.hero-title {
  font-size: 20px;
  font-weight: 800;
  color: var(--steam-text-primary);
  margin: 0 0 8px 0;
  letter-spacing: -0.5px;
  line-height: 1.1;
}

.hero-subtitle {
  font-size: 16px;
  color: var(--steam-text-secondary);
  margin: 0 0 24px 0;
  font-weight: 400;
}

/* Hero操作按钮 */
.hero-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.play-btn-large {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 14px 32px;
  background: var(--steam-accent-green);
  color: white;
  border: none;
  border-radius: 12px;
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 4px 20px rgba(107, 142, 35, 0.4);
}

.play-btn-large:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 6px 24px rgba(107, 142, 35, 0.5);
}

.play-btn-large:disabled {
  background: var(--steam-bg-hover);
  cursor: not-allowed;
  box-shadow: none;
}

.play-btn-large svg {
  width: 20px;
  height: 20px;
}

.close-btn-large {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 14px 32px;
  background: var(--steam-accent-blue);
  color: white;
  border: none;
  border-radius: 12px;
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 4px 20px rgba(90, 169, 230, 0.4);
}

.close-btn-large:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 24px rgba(90, 169, 230, 0.5);
}

.close-btn-large svg {
  width: 20px;
  height: 20px;
}

.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  background: rgba(99, 99, 99, 0.6);
  backdrop-filter: blur(10px) saturate(180%);
  border: 1px solid var(--steam-border);
  border-radius: 12px;
  color: var(--steam-text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.action-btn:hover {
  background: var(--steam-bg-hover);
  border-color: var(--steam-border-light);
  transform: translateY(-2px);
}

.action-btn.active {
  background: rgba(255, 215, 0, 0.2);
  border-color: var(--steam-accent-gold);
  color: var(--steam-accent-gold);
}

.action-btn svg {
  width: 20px;
  height: 20px;
}

/* ============================================
   游戏信息区 - 玻璃态卡片
   ============================================ */
.game-info-section {
  flex: 1;
  padding: 32px 48px;
  overflow-y: auto;
  background: rgba(89, 89, 89, 0.6);
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 32px;
}

/* 信息卡片 - 玻璃态效果 */
.info-card {
  position: relative;
  display: flex;
  align-items: flex-start;
  gap: 14px;
  padding: 20px;
  background: rgba(99, 99, 99, 0.65);
  backdrop-filter: blur(18px) saturate(180%);
  border-radius: 14px;
  border: 1px solid var(--steam-border);
  transition: all 0.25s ease;
  overflow: hidden;
}

.info-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--steam-border-light), transparent);
}

.info-card:hover {
  background: var(--steam-bg-hover);
  border-color: var(--steam-border-light);
  transform: translateY(-2px);
}

.info-card-bg {
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg, rgba(var(--steam-accent-blue-rgb), 0.03) 0%, transparent 50%);
  pointer-events: none;
}

.info-icon {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 10px;
  flex-shrink: 0;
}

.info-icon.steam {
  background: rgba(var(--steam-accent-blue-rgb), 0.15);
  color: var(--steam-accent-blue);
}

.info-icon.folder {
  background: rgba(var(--steam-accent-orange-rgb), 0.15);
  color: var(--steam-accent-orange);
}

.info-icon.time {
  background: rgba(var(--steam-accent-purple-rgb), 0.15);
  color: var(--steam-accent-purple);
}

.info-icon.clock {
  background: rgba(var(--steam-accent-green-rgb), 0.15);
  color: var(--steam-accent-green);
}

.info-icon svg {
  width: 20px;
  height: 20px;
}

.info-content {
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
  flex: 1;
}

.info-label {
  font-size: 12px;
  color: var(--steam-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  font-weight: 600;
}

.info-value {
  font-size: 13px;
  color: var(--steam-text-primary);
  font-weight: 500;
}

.info-value.id {
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  color: var(--steam-text-secondary);
}

.info-value.path {
  font-size: 12px;
  color: var(--steam-text-muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.current-session {
  color: var(--steam-accent-green);
  font-size: 12px;
  margin-left: 6px;
  font-weight: 500;
}

/* 额外信息区域 */
.extra-info {
  background: rgba(99, 99, 99, 0.6);
  border-radius: 14px;
  padding: 24px;
  border: 1px solid var(--steam-border);
  backdrop-filter: blur(12px) saturate(180%);
}

.info-block h4 {
  margin: 0 0 16px 0;
  font-size: 13px;
  font-weight: 600;
  color: var(--steam-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.info-row {
  display: flex;
  align-items: center;
  padding: 10px 0;
  border-bottom: 1px solid var(--steam-border);
}

.info-row:last-child {
  border-bottom: none;
}

.info-key {
  width: 100px;
  font-size: 12px;
  color: var(--steam-text-muted);
}

.info-val {
  flex: 1;
  font-size: 12px;
  color: var(--steam-text-primary);
}

.info-val.path {
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  color: var(--steam-text-muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* ============================================
   空状态
   ============================================ */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  color: var(--steam-text-muted);
  text-align: center;
}

.empty-illustration.large {
  width: 100px;
  height: 100px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--steam-bg-tertiary);
  border-radius: 28px;
  margin-bottom: 8px;
  border: 1px solid var(--steam-border);
}

.empty-illustration.large svg {
  width: 44px;
  height: 44px;
  color: var(--steam-text-muted);
}

.empty-state .empty-title {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.empty-state .empty-subtitle {
  margin: 0;
  font-size: 14px;
  color: var(--steam-text-muted);
}

.browse-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 8px;
  padding: 12px 28px;
  background: var(--steam-accent-blue);
  color: white;
  border: none;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 4px 16px rgba(90, 169, 230, 0.3);
}

.browse-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(90, 169, 230, 0.4);
}

.browse-btn svg {
  width: 18px;
  height: 18px;
}

/* 响应式调整 */
@media (max-width: 1200px) {
  .info-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .hero-title {
    font-size: 20px;
  }
}

@media (max-width: 900px) {
  .library-sidebar {
    width: 240px;
    min-width: 240px;
  }
  
  .hero-title {
    font-size: 20px;
  }
  
  .hero-overlay {
    padding: 30px 32px 40px;
  }
  
  .game-info-section {
    padding: 24px 32px;
  }
  
  .info-grid {
    grid-template-columns: 1fr;
  }
}
</style>