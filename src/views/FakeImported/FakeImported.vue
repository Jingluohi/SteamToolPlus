<template>
  <!--
    FakeImported.vue - 假入库游戏管理页面
    显示通过本程序入库到 Steam 的游戏列表，支持安全删除
  -->
  <div class="fake-imported-page">
    <!-- 页面标题 -->
    <div class="page-header">
      <div class="header-left">
        <h1 class="page-title">假入库游戏</h1>
        <p class="page-desc">管理通过本程序入库到 Steam 的游戏</p>
      </div>
    </div>

    <div class="fake-imported-content">
      <!-- 加载状态 -->
      <div v-if="loading" class="loading-state">
        <div class="spinner"></div>
        <span>加载中...</span>
      </div>

      <!-- 空状态 -->
      <div v-else-if="games.length === 0" class="empty-state">
        <svg viewBox="0 0 24 24" fill="currentColor">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
        </svg>
        <p>暂无假入库游戏</p>
        <p class="empty-hint">通过【清单入库】或【游戏详情页】入库 Steam 的游戏会显示在这里</p>
      </div>

      <!-- 游戏列表 -->
      <template v-else>
        <div class="section-header">
          <span class="game-count">共 {{ games.length }} 款假入库游戏</span>
          <button class="refresh-btn" @click="loadGames" title="刷新列表">
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
            </svg>
            刷新
          </button>
        </div>

        <div class="games-grid">
          <div
            v-for="game in games"
            :key="game.appId"
            class="fake-game-card"
          >
            <div class="card-image-wrapper">
              <img
                v-if="game.coverUrl"
                :src="game.coverUrl"
                :alt="game.displayName"
                class="card-cover-image"
                @error="game.coverUrl = ''"
              />
              <div v-else class="card-placeholder" :style="getPlaceholderStyle(game.appId)">
                <span class="placeholder-text">{{ game.appId.toString().slice(0, 4) }}</span>
              </div>
            </div>

            <div class="card-info">
              <h3 class="game-name" :title="game.displayName">{{ game.displayName }}</h3>
              <span class="game-id">AppID: {{ game.appId }}</span>
              <span v-if="game.inConfig" class="config-badge">在配置中</span>
              <span v-else class="custom-badge">自定义</span>
            </div>

            <button
              class="delete-btn"
              :disabled="deletingId === game.appId"
              @click="handleDelete(game)"
            >
              <svg v-if="deletingId === game.appId" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
              </svg>
              <svg v-else viewBox="0 0 24 24" fill="currentColor">
                <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
              </svg>
              {{ deletingId === game.appId ? '删除中...' : '删除入库' }}
            </button>
          </div>
        </div>
      </template>
    </div>

    <!-- 确认删除弹窗 -->
    <BaseModal
      v-model="showDeleteModal"
      title="确认删除入库游戏"
      confirm-text="确认删除"
      :confirm-disabled="isConfirmDeleting"
      @confirm="confirmDelete"
      @close="cancelDelete"
    >
      <template #body>
        <div class="delete-confirm-content">
          <p>确定要删除以下入库游戏吗？</p>
          <div class="confirm-game-info">
            <strong>{{ selectedGame?.displayName }}</strong>
            <span>AppID: {{ selectedGame?.appId }}</span>
          </div>
          <p class="warning-text">
            删除后将清理 OpenSteamTool 写入的 Lua 和 manifest 文件，该游戏会从 Steam 库和我的程序中消失。
          </p>
          <p class="notice-text">
            如果检测到正版购买记录（appmanifest_*.acf），将拒绝删除以保护正版游戏。
          </p>
        </div>
      </template>
    </BaseModal>
  </div>
</template>

<script setup lang="ts">
/**
 * FakeImported.vue - 假入库游戏管理页面
 */

import { ref, onMounted } from 'vue'
import { invoke, convertFileSrc } from '@tauri-apps/api/core'
import BaseModal from '../../components/common/BaseModal.vue'

/**
 * 假入库游戏信息
 */
interface FakeImportedGame {
  appId: number
  gameId: string
  displayName: string
  inConfig: boolean
  coverPath?: string
  coverUrl?: string
}

// ==================== 状态 ====================

/** 游戏列表 */
const games = ref<FakeImportedGame[]>([])
/** 是否正在加载 */
const loading = ref(false)
/** 当前正在删除的游戏 AppID */
const deletingId = ref<number | null>(null)
/** 是否显示删除确认弹窗 */
const showDeleteModal = ref(false)
/** 选中的游戏 */
const selectedGame = ref<FakeImportedGame | null>(null)
/** 是否正在确认删除中 */
const isConfirmDeleting = ref(false)

// ==================== 方法 ====================

/**
 * 加载假入库游戏列表
 */
async function loadGames() {
  loading.value = true
  try {
    const result = await invoke<FakeImportedGame[]>('get_fake_imported_games_command')
    games.value = result.map(game => ({
      ...game,
      coverUrl: game.coverPath ? convertFileSrc(game.coverPath) : undefined
    }))
  } catch (error) {
    alert('加载假入库游戏失败: ' + error)
  } finally {
    loading.value = false
  }
}

/**
 * 生成占位符样式
 */
function getPlaceholderStyle(appId: number) {
  const hue = appId % 360
  return {
    backgroundColor: `hsl(${hue}, 60%, 40%)`
  }
}

/**
 * 处理删除按钮点击
 */
function handleDelete(game: FakeImportedGame) {
  selectedGame.value = game
  showDeleteModal.value = true
}

/**
 * 取消删除
 */
function cancelDelete() {
  selectedGame.value = null
  showDeleteModal.value = false
}

/**
 * 确认删除
 */
async function confirmDelete() {
  if (!selectedGame.value) return

  isConfirmDeleting.value = true
  deletingId.value = selectedGame.value.appId

  try {
    const result = await invoke<{
      success: boolean
      message: string
      hasRealOwnership?: boolean
      deletedFiles?: string[]
    }>('remove_fake_imported_game_command', {
      appId: selectedGame.value.appId,
      steamPath: null
    })

    if (result.success) {
      alert('删除成功: ' + result.message)
      await loadGames()
    } else {
      alert('删除失败: ' + result.message)
    }
  } catch (error) {
    alert('删除失败: ' + error)
  } finally {
    isConfirmDeleting.value = false
    deletingId.value = null
    selectedGame.value = null
    showDeleteModal.value = false
  }
}

// ==================== 生命周期 ====================

onMounted(() => {
  loadGames()
})
</script>

<style scoped>
.fake-imported-page {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 16px;
  box-sizing: border-box;
  overflow: hidden;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 16px;
  flex-shrink: 0;
}

.header-left {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.page-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0;
}

.page-desc {
  font-size: 13px;
  color: var(--steam-text-secondary);
  margin: 0;
}

.fake-imported-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.loading-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  color: var(--steam-text-secondary);
  font-size: 14px;
}

.spinner {
  width: 32px;
  height: 32px;
  border: 3px solid var(--steam-border);
  border-top-color: var(--steam-accent-blue);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 8px;
  color: var(--steam-text-secondary);
}

.empty-state svg {
  width: 64px;
  height: 64px;
  opacity: 0.5;
}

.empty-state p {
  margin: 0;
  font-size: 16px;
}

.empty-hint {
  font-size: 13px;
  color: var(--steam-text-muted);
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
  flex-shrink: 0;
}

.game-count {
  font-size: 14px;
  color: var(--steam-text-secondary);
}

.refresh-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
  border: 1px solid var(--steam-border);
  border-radius: 4px;
  font-size: 13px;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.refresh-btn:hover {
  background-color: var(--steam-bg-hover);
}

.refresh-btn svg {
  width: 16px;
  height: 16px;
}

.games-grid {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 12px;
  overflow-y: auto;
  padding-right: 8px;
}

.fake-game-card {
  display: flex;
  flex-direction: column;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  overflow: hidden;
  border: 1px solid var(--steam-border);
  transition: transform 0.15s ease, box-shadow 0.15s ease;
}

.fake-game-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

.card-image-wrapper {
  position: relative;
  width: 100%;
  aspect-ratio: 460 / 215;
  overflow: hidden;
}

.card-cover-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  object-position: center;
  display: block;
}

.card-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.placeholder-text {
  font-size: 24px;
  font-weight: 700;
  color: rgba(255, 255, 255, 0.8);
}

.card-info {
  padding: 10px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
}

.game-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.game-id {
  font-size: 12px;
  color: var(--steam-text-secondary);
}

.config-badge,
.custom-badge {
  display: inline-flex;
  align-items: center;
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 11px;
  width: fit-content;
}

.config-badge {
  background-color: rgba(16, 185, 129, 0.15);
  color: #10b981;
}

.custom-badge {
  background-color: rgba(59, 130, 246, 0.15);
  color: #3b82f6;
}

.delete-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px;
  margin: 0 10px 10px 10px;
  background-color: rgba(239, 68, 68, 0.15);
  color: #ef4444;
  border: 1px solid rgba(239, 68, 68, 0.3);
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.delete-btn:hover:not(:disabled) {
  background-color: rgba(239, 68, 68, 0.25);
}

.delete-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.delete-btn svg {
  width: 14px;
  height: 14px;
}

.loading-icon {
  animation: spin 1s linear infinite;
}

/* 删除确认弹窗内容 */
.delete-confirm-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 8px 0;
}

.delete-confirm-content p {
  margin: 0;
  font-size: 14px;
  color: var(--steam-text-primary);
}

.confirm-game-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 10px;
  background-color: var(--steam-bg-secondary);
  border-radius: 6px;
  border-left: 3px solid var(--steam-accent-blue);
}

.confirm-game-info strong {
  font-size: 15px;
  color: var(--steam-text-primary);
}

.confirm-game-info span {
  font-size: 13px;
  color: var(--steam-text-secondary);
}

.warning-text {
  color: #f59e0b !important;
  font-size: 13px !important;
}

.notice-text {
  color: var(--steam-text-muted) !important;
  font-size: 12px !important;
}

@media (max-width: 1200px) {
  .games-grid {
    grid-template-columns: repeat(4, 1fr);
  }
}

@media (max-width: 900px) {
  .games-grid {
    grid-template-columns: repeat(3, 1fr);
  }
}

@media (max-width: 600px) {
  .games-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}
</style>