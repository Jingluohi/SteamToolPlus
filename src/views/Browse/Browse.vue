<template>
  <!--
    Browse.vue - 浏览页面
    程序主界面，显示游戏库浏览视图
    使用分页加载，每页16张卡片，控制内存占用
  -->
  <div class="browse-page">
    <!-- 页面标题 -->
    <div class="page-header">
      <div class="header-left">
        <h1 class="page-title">浏览</h1>
        <p class="page-desc">这些是经过验证的游戏，更多游戏从【游戏】->【本体下载】用清单文件下载本体，去【游戏】->【免Steam补丁】打启动补丁</p>
      </div>
      <!-- 搜索栏移到标题右侧 -->
      <div class="search-section">
        <div class="search-box">
          <svg class="search-icon" viewBox="0 0 24 24" fill="currentColor">
            <path d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>
          </svg>
          <input 
            v-model="searchKeyword"
            type="text"
            class="search-input"
            placeholder="搜索游戏..."
            @input="handleSearch"
          />
          <!-- 清除按钮 -->
          <button 
            v-if="searchKeyword"
            class="clear-btn"
            @click="clearSearch"
            title="清除搜索"
          >
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
            </svg>
          </button>
        </div>
      </div>
    </div>
    
    <div class="browse-content">
      <!-- 全部游戏网格 -->
      <section class="browse-section">
        <div class="section-header">
          <span class="game-count">共 {{ filteredGames.length }} 款游戏</span>
          <span v-if="totalPages > 1" class="page-info">第 {{ currentPage }} / {{ totalPages }} 页</span>
        </div>
        
        <!-- 加载状态 -->
        <div v-if="loading" class="loading-state">
          <div class="spinner"></div>
          <span>加载中...</span>
        </div>
        
        <!-- 空状态 -->
        <div v-else-if="filteredGames.length === 0" class="empty-state">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M21 19V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2zM8.5 13.5l2.5 3.01L14.5 12l4.5 6H5l3.5-4.5z"/>
          </svg>
          <p>暂无游戏</p>
        </div>
        
        <!-- 分页游戏网格 -->
        <template v-else>
          <div class="games-grid">
            <GameCard
              v-for="game in paginatedGames"
              :key="game.game_id"
              :game="game"
              @click="handleGameClick(game)"
            />
          </div>
          
          <!-- 分页器 -->
          <div v-if="totalPages > 1" class="pagination">
            <!-- 第一页 -->
            <button 
              class="page-btn" 
              :disabled="currentPage === 1"
              @click="goToPage(1)"
              title="第一页"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="11 18 5 12 11 6"></polyline>
                <polyline points="18 18 12 12 18 6"></polyline>
              </svg>
            </button>
            
            <!-- 上一页 -->
            <button 
              class="page-btn" 
              :disabled="currentPage === 1"
              @click="goToPage(currentPage - 1)"
              title="上一页"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="15 18 9 12 15 6"></polyline>
              </svg>
            </button>
            
            <div class="page-numbers">
              <template v-for="(page, index) in visiblePageNumbers" :key="index">
                <!-- 省略号 -->
                <span v-if="page === -1" class="page-ellipsis">...</span>
                <!-- 页码按钮 -->
                <button
                  v-else
                  class="page-number"
                  :class="{ active: page === currentPage }"
                  @click="goToPage(page)"
                >
                  {{ page }}
                </button>
              </template>
            </div>
            
            <!-- 下一页 -->
            <button 
              class="page-btn" 
              :disabled="currentPage === totalPages"
              @click="goToPage(currentPage + 1)"
              title="下一页"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="9 18 15 12 9 6"></polyline>
              </svg>
            </button>
            
            <!-- 最后一页 -->
            <button
              class="page-btn"
              :disabled="currentPage === totalPages"
              @click="goToPage(totalPages)"
              title="最后一页"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="13 18 19 12 13 6"></polyline>
                <polyline points="6 18 12 12 6 6"></polyline>
              </svg>
            </button>

            <!-- 跳转到指定页 -->
            <div class="page-jump">
              <span class="jump-label">跳转到</span>
              <input
                v-model.number="jumpPageInput"
                type="number"
                class="jump-input"
                :min="1"
                :max="totalPages"
                @keyup.enter="handleJumpToPage"
              />
              <span class="jump-label">页</span>
              <button
                class="page-btn jump-btn"
                @click="handleJumpToPage"
                :disabled="!isValidJumpPage"
              >
                跳转
              </button>
            </div>
          </div>
        </template>
      </section>
    </div>

    <!-- 游戏详情弹窗 -->
    <GameDetailModal
      :visible="showGameDetail"
      :game-config="selectedGame"
      @close="showGameDetail = false"
      @launch="handleLaunchGame"
      @apply-patch="handleApplyPatch"
      @select-path="handleSelectGamePath"
    />
  </div>
</template>

<script setup lang="ts">
/**
 * Browse.vue - 浏览页面
 * 程序主界面，从resources/games_config.json加载游戏配置
 * 使用分页加载，每页20张卡片，控制内存占用
 */

import { ref, computed, onMounted, watch } from 'vue'
import { useRouter, onBeforeRouteLeave } from 'vue-router'
import type { GameConfigData } from '../../types'
import { loadGamesConfigFromFile } from '../../api/game.api'
import { PATCH_TYPE_MAP } from '../../types'
import GameCard from '../../components/game/GameCard.vue'
import GameDetailModal from '../../components/game/GameDetailModal.vue'

// 路由
const router = useRouter()

// 分页配置
const PAGE_SIZE = 16 // 每页16张卡片

// 状态
const loading = ref(false)
const gamesConfig = ref<GameConfigData[]>([])
const searchKeyword = ref('')
const currentPage = ref(1)
const showGameDetail = ref(false)
const selectedGame = ref<GameConfigData | null>(null)
const jumpPageInput = ref<number | null>(null)

// 计算属性：跳转页码是否有效
const isValidJumpPage = computed(() => {
  if (jumpPageInput.value === null) return false
  const page = Number(jumpPageInput.value)
  return page >= 1 && page <= totalPages.value && page !== currentPage.value
})

// 计算属性
const filteredGames = computed(() => {
  let result = gamesConfig.value
  
  // 搜索筛选
  if (searchKeyword.value) {
    const keyword = searchKeyword.value.toLowerCase()
    result = result.filter(game => 
      game.chinese_name.toLowerCase().includes(keyword) ||
      game.game_name.toLowerCase().includes(keyword) ||
      game.game_id.includes(keyword)
    )
  }
  
  return result
})

// 总页数
const totalPages = computed(() => Math.ceil(filteredGames.value.length / PAGE_SIZE))

// 当前页的游戏
const paginatedGames = computed(() => {
  const start = (currentPage.value - 1) * PAGE_SIZE
  const end = start + PAGE_SIZE
  return filteredGames.value.slice(start, end)
})

// 可见的页码
// 显示逻辑：最多5个页码 [1] [...] [current-1] [current] [current+1] [...] [total]
const visiblePageNumbers = computed(() => {
  const pages: number[] = []
  const total = totalPages.value
  const current = currentPage.value
  
  // 如果总页数较少，直接显示所有页码
  if (total <= 5) {
    for (let i = 1; i <= total; i++) {
      pages.push(i)
    }
    return pages
  }
  
  // 始终显示第1页
  pages.push(1)
  
  // 计算中间部分的起始和结束（以当前页为中心，最多3个页码）
  let middleStart = Math.max(2, current - 1)
  let middleEnd = Math.min(total - 1, current + 1)
  
  // 调整中间部分，确保最多显示3个页码
  if (current <= 2) {
    // 靠近开头，显示 2, 3
    middleStart = 2
    middleEnd = 3
  } else if (current >= total - 1) {
    // 靠近结尾，显示 倒数第2, 倒数第3
    middleStart = total - 2
    middleEnd = total - 1
  }
  
  // 添加省略号（如果中间部分与第1页不连续）
  if (middleStart > 2) {
    pages.push(-1) // -1 表示省略号
  }
  
  // 添加中间页码
  for (let i = middleStart; i <= middleEnd; i++) {
    if (i > 1 && i < total) { // 避免重复添加第1页和最后1页
      pages.push(i)
    }
  }
  
  // 添加省略号（如果中间部分与最后1页不连续）
  if (middleEnd < total - 1) {
    pages.push(-1) // -1 表示省略号
  }
  
  // 始终显示最后1页
  pages.push(total)
  
  return pages
})

// 生命周期
onMounted(() => {
  loadGames()
  
  // 恢复页码
  const savedPage = sessionStorage.getItem('browse_current_page')
  if (savedPage) {
    currentPage.value = parseInt(savedPage, 10)
  }
  
  // 恢复搜索关键词
  const savedKeyword = sessionStorage.getItem('browse_search_keyword')
  if (savedKeyword) {
    searchKeyword.value = savedKeyword
  }
})

// 监听搜索变化，重置到第一页
watch(searchKeyword, () => {
  currentPage.value = 1
})

// 在离开路由前保存页码和搜索关键词
onBeforeRouteLeave((to, from, next) => {
  sessionStorage.setItem('browse_current_page', String(currentPage.value))
  sessionStorage.setItem('browse_search_keyword', searchKeyword.value)
  next()
})

// 方法
async function loadGames() {
  loading.value = true
  try {
    const config = await loadGamesConfigFromFile()
    gamesConfig.value = config
  } catch (err) {
    // 加载游戏配置失败时静默处理
  } finally {
    loading.value = false
  }
}

function handleSearch() {
  // 搜索通过computed自动处理，会触发watch重置页码
}

function clearSearch() {
  searchKeyword.value = ''
  // 清空后会触发watch，自动重置到第一页
}

function handleGameClick(game: GameConfigData) {
  // 跳转到游戏详情页面
  router.push(`/game/${game.game_id}`)
}

function goToPage(page: number) {
  if (page >= 1 && page <= totalPages.value && page !== currentPage.value) {
    currentPage.value = page
    // 清空跳转输入框
    jumpPageInput.value = null
    // 滚动到顶部
    const gridElement = document.querySelector('.games-grid')
    if (gridElement) {
      gridElement.scrollIntoView({ behavior: 'smooth', block: 'start' })
    }
  }
}

// 处理跳转到指定页
function handleJumpToPage() {
  if (isValidJumpPage.value && jumpPageInput.value !== null) {
    goToPage(Number(jumpPageInput.value))
  }
}

function handleLaunchGame(gameId: string) {
  // 启动游戏功能待实现
}

function handleApplyPatch(tag: any) {
  // 应用补丁功能待实现
}

function handleSelectGamePath(gameId: string) {
  // 选择游戏路径功能待实现
}
</script>

<style scoped>
.browse-page {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 16px 24px;
  overflow: hidden;
}

.page-header {
  margin-bottom: 4px;
  flex-shrink: 0;
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 24px;
}

.header-left {
  flex: 1;
}

.page-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--steam-text-primary);
  margin: 0;
}

.page-desc {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
  margin: 0;
}

/* 搜索栏 */
.search-section {
  flex-shrink: 0;
  width: 400px;
  margin-top: 3px;
}

.search-box {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 16px;
  background: rgba(102, 192, 244, 0.1);
  backdrop-filter: blur(10px);
  border-radius: 8px;
  border: 1px solid var(--steam-input-border);
  transition: border-color 0.2s ease;
}

.search-box:focus-within {
  border-color: var(--steam-accent-blue);
  background: rgba(102, 192, 244, 0.15);
}

.search-icon {
  width: 20px;
  height: 20px;
  color: var(--steam-text-muted);
}

.search-input {
  flex: 1;
  background: transparent;
  border: none;
  color: var(--steam-text-primary);
  font-size: 14px;
  outline: none;
}

.search-input::placeholder {
  color: var(--steam-text-muted);
}

/* 清除按钮 */
.clear-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  padding: 0;
  background: transparent;
  border: none;
  color: var(--steam-text-muted);
  cursor: pointer;
  transition: color 0.2s ease;
  flex-shrink: 0;
}

.clear-btn:hover {
  color: var(--steam-text-primary);
}

.clear-btn svg {
  width: 16px;
  height: 16px;
}

.browse-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 0;
}

.browse-section {
  background: transparent;
  border-radius: 8px;
  padding: 16px 16px 8px 16px;
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 0;
  border: none;
  margin-top: -8px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 6px;
  flex-shrink: 0;
}

.game-count {
  font-size: 13px;
  color: var(--steam-text-muted);
}

.page-info {
  font-size: 13px;
  color: var(--steam-text-secondary);
}

/* 加载状态 */
.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px;
  gap: 16px;
  color: var(--steam-text-muted);
  flex: 1;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--steam-bg-tertiary);
  border-top-color: var(--steam-accent-blue);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

/* 空状态 */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px;
  gap: 16px;
  color: var(--steam-text-muted);
  flex: 1;
}

.empty-state svg {
  width: 64px;
  height: 64px;
  opacity: 0.5;
}

/* 游戏网格 */
.games-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(330px, 1fr));
  gap: 19px;
  overflow-y: auto;
  flex: 1;
  min-height: 0;
  padding-right: 8px;
}



/* 自定义滚动条 */
.games-grid::-webkit-scrollbar {
  width: 6px;
}

.games-grid::-webkit-scrollbar-track {
  background: transparent;
}

.games-grid::-webkit-scrollbar-thumb {
  background: var(--steam-border);
  border-radius: 3px;
}

.games-grid::-webkit-scrollbar-thumb:hover {
  background: var(--steam-text-secondary);
}

/* 分页器 */
.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 4px;
  padding: 4px 0;
  border-top: 1px solid var(--steam-border);
  flex-shrink: 0;
  margin-top: 8px;
}

.page-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  padding: 0;
  background: rgba(102, 192, 244, 0.1);
  backdrop-filter: blur(10px);
  border: 1px solid var(--steam-card-border);
  border-radius: 4px;
  color: var(--steam-text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.page-btn:hover:not(:disabled) {
  background: rgba(102, 192, 244, 0.2);
  border-color: var(--steam-accent-blue);
  color: var(--steam-text-primary);
}

.page-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.page-btn svg {
  width: 12px;
  height: 12px;
}

.page-numbers {
  display: flex;
  gap: 4px;
}

.page-number {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
  background: rgba(102, 192, 244, 0.1);
  backdrop-filter: blur(10px);
  border: 1px solid var(--steam-card-border);
  border-radius: 4px;
  color: var(--steam-text-secondary);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.page-number:hover {
  background: rgba(102, 192, 244, 0.2);
  border-color: var(--steam-accent-blue);
  color: var(--steam-text-primary);
}

.page-number.active {
  background: rgba(66, 133, 244, 0.8);
  border-color: var(--steam-accent-blue);
  color: white;
}

/* 省略号 */
.page-ellipsis {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--steam-text-muted);
  font-size: 13px;
  user-select: none;
  flex-shrink: 0;
}

/* 跳转到指定页 */
.page-jump {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-left: 12px;
  padding-left: 12px;
  border-left: 1px solid var(--steam-border);
}

.jump-label {
  font-size: 13px;
  color: var(--steam-text-secondary);
}

.jump-input {
  width: 50px;
  height: 32px;
  padding: 0 8px;
  background: rgba(102, 192, 244, 0.1);
  backdrop-filter: blur(10px);
  border: 1px solid var(--steam-card-border);
  border-radius: 4px;
  color: var(--steam-text-primary);
  font-size: 13px;
  text-align: center;
  outline: none;
  transition: all 0.2s ease;
}

.jump-input:focus {
  border-color: var(--steam-accent-blue);
  background: rgba(102, 192, 244, 0.15);
}

.jump-input::-webkit-inner-spin-button,
.jump-input::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

.jump-btn {
  width: auto;
  padding: 0 12px;
  font-size: 13px;
  white-space: nowrap;
}
</style>
