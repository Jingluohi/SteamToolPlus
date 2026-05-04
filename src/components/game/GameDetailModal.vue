<template>
  <!-- 
    GameDetailModal.vue - 游戏详情弹窗组件
    显示游戏详细信息、补丁选项、下载链接等
    仿照旧版Steam Tool Plus的游戏详情界面
  -->
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="visible" class="game-detail-modal" @click.self="handleClose">
        <div class="modal-content">
          <!-- 关闭按钮 -->
          <button class="close-btn" @click="handleClose">
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
            </svg>
          </button>

          <!-- 顶部横幅区域 -->
          <div class="modal-header">
            <div class="header-bg" :style="headerBgStyle">
              <div class="header-gradient"></div>
            </div>
            <div class="header-content">
              <!-- 游戏封面 -->
              <div class="cover-container">
                <img 
                  v-if="coverImage"
                  :src="coverImage" 
                  :alt="gameConfig?.chinese_name || gameConfig?.game_name"
                  class="cover-image"
                />
                <div v-else class="cover-placeholder">
                  <svg viewBox="0 0 24 24" fill="currentColor">
                    <path d="M21 19V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2zM8.5 13.5l2.5 3.01L14.5 12l4.5 6H5l3.5-4.5z"/>
                  </svg>
                </div>
              </div>

              <!-- 游戏信息 -->
              <div class="game-info">
                <h1 class="game-title">{{ gameConfig?.chinese_name || gameConfig?.game_name }}</h1>
                <p class="game-subtitle">{{ gameConfig?.game_name }}</p>
                <div class="game-meta">
                  <span class="game-id">游戏ID: {{ gameConfig?.game_id }}</span>
                  <span v-if="gameConfig?.downloadable" class="downloadable-badge">可下载</span>
                </div>
              </div>
            </div>
          </div>

          <!-- 内容区域 -->
          <div class="modal-body">
            <!-- 左侧：补丁选项 -->
            <div class="body-left">
              <h2 class="section-title">补丁选项</h2>
              
              <div v-if="gameConfig?.tags && gameConfig.tags.length > 0" class="patch-list">
                <div 
                  v-for="(tag, index) in gameConfig.tags" 
                  :key="index"
                  class="patch-item"
                  :class="{ active: selectedPatch === index }"
                  @click="selectedPatch = index"
                >
                  <div class="patch-header">
                    <span class="patch-type">{{ getPatchTypeName(tag.patch_type) }}</span>
                    <span v-if="tag.download_urls && tag.download_urls.length > 0" class="has-download">有下载链接</span>
                  </div>
                  <p class="patch-description">{{ getPatchTypeDescription(tag.patch_type) }}</p>
                  
                  <!-- 多网盘下载按钮组 -->
                  <div v-if="tag.download_urls && tag.download_urls.length > 0" class="download-section">
                    <p class="download-section-title">下载补丁：</p>
                    <div class="download-buttons">
                      <div
                        v-for="(item, dlIndex) in tag.download_urls"
                        :key="dlIndex"
                        class="download-btn-wrapper"
                      >
                        <button
                          class="action-btn download-btn"
                          @click="openDownloadUrl(item.url)"
                        >
                          <svg viewBox="0 0 24 24" fill="currentColor">
                            <path d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z"/>
                          </svg>
                          {{ getDownloadSourceName(item.source) }}
                        </button>
                        <p v-if="item.pwd || item.source === 'lanzou'" class="pwd-hint">
                          提取码: {{ item.pwd || '1234' }}
                        </p>
                      </div>
                    </div>
                  </div>
                  
                  <div class="patch-actions">
                    <button 
                      class="action-btn apply-btn"
                      @click="applyPatch(tag)"
                    >
                      <svg viewBox="0 0 24 24" fill="currentColor">
                        <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
                      </svg>
                      应用补丁
                    </button>
                    <!-- D加密虚拟机类型显示虚拟化环境配置教程按钮 -->
                    <button 
                      v-if="tag.patch_type === 3"
                      class="action-btn tutorial-btn"
                      @click="openVirtualizationTutorial"
                    >
                      <svg viewBox="0 0 24 24" fill="currentColor">
                        <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
                      </svg>
                      虚拟化环境配置教程
                    </button>
                  </div>
                </div>
              </div>

              <div v-else class="no-patches">
                <p>该游戏暂无可用补丁</p>
              </div>
            </div>

            <!-- 右侧：游戏信息 -->
            <div class="body-right">
              <h2 class="section-title">游戏信息</h2>
              
              <div class="info-list">
                <div class="info-item">
                  <span class="info-label">游戏名称</span>
                  <span class="info-value">{{ gameConfig?.chinese_name || gameConfig?.game_name }}</span>
                </div>
                <div class="info-item">
                  <span class="info-label">英文名称</span>
                  <span class="info-value">{{ gameConfig?.game_name }}</span>
                </div>
                <div class="info-item">
                  <span class="info-label">游戏ID</span>
                  <span class="info-value">{{ gameConfig?.game_id }}</span>
                </div>
                <div class="info-item">
                  <span class="info-label">下载支持</span>
                  <span class="info-value">{{ gameConfig?.downloadable ? '支持' : '不支持' }}</span>
                </div>
                <div class="info-item">
                  <span class="info-label">补丁数量</span>
                  <span class="info-value">{{ gameConfig?.tags?.length || 0 }} 个</span>
                </div>
              </div>

              <!-- 操作按钮 -->
              <div class="action-buttons">
                <button class="primary-btn" @click="handleLaunchGame">
                  <svg viewBox="0 0 24 24" fill="currentColor">
                    <path d="M8 5v14l11-7z"/>
                  </svg>
                  启动游戏
                </button>
                <button class="secondary-btn" @click="handleSelectGamePath">
                  <svg viewBox="0 0 24 24" fill="currentColor">
                    <path d="M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm0 12H4V8h16v10z"/>
                  </svg>
                  选择游戏路径
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
/**
 * GameDetailModal.vue - 游戏详情弹窗组件
 * 展示游戏详细信息和补丁管理功能
 */

import { ref, computed, watch } from 'vue'
import { open as openShell } from '@tauri-apps/plugin-shell'
import type { GameConfigData, GameTagConfig } from '../../types'
import { getCachedCoverImage } from '../../services/imageCache.service'
import { PATCH_TYPE_MAP } from '../../types'

/**
 * 组件属性定义
 */
interface Props {
  /** 是否显示弹窗 */
  visible: boolean
  /** 游戏配置数据 */
  gameConfig: GameConfigData | null
}

const props = withDefaults(defineProps<Props>(), {
  visible: false,
  gameConfig: null
})

/**
 * 组件事件定义
 */
const emit = defineEmits<{
  /** 关闭弹窗 */
  (e: 'close'): void
  /** 启动游戏 */
  (e: 'launch', gameId: string): void
  /** 应用补丁 */
  (e: 'apply-patch', tag: GameTagConfig): void
  /** 选择游戏路径 */
  (e: 'select-path', gameId: string): void
}>()

// 选中的补丁索引
const selectedPatch = ref(0)
// 封面图片
const coverImage = ref('')

// 头部背景样式
const headerBgStyle = computed(() => {
  if (coverImage.value) {
    return {
      backgroundImage: `url(${coverImage.value})`,
      backgroundSize: 'cover',
      backgroundPosition: 'center'
    }
  }
  return {}
})

/**
 * 监听游戏配置变化，加载封面图片
 * 使用全局缓存服务，避免资源竞争
 */
watch(() => props.gameConfig, async (newConfig) => {
  if (newConfig?.game_id) {
    try {
      coverImage.value = await getCachedCoverImage(newConfig.game_id)
    } catch (error) {
      coverImage.value = ''
    }
  } else {
    coverImage.value = ''
  }
  // 重置选中的补丁
  selectedPatch.value = 0
}, { immediate: true })

/**
 * 获取补丁类型名称
 */
function getPatchTypeName(patchType: number): string {
  return PATCH_TYPE_MAP[patchType]?.name || '未知补丁'
}

/**
 * 获取补丁类型描述
 */
function getPatchTypeDescription(patchType: number): string {
  return PATCH_TYPE_MAP[patchType]?.description || ''
}

/**
 * 获取下载来源显示名称
 * @param source 网盘来源标识
 */
function getDownloadSourceName(source: string): string {
  const sourceMap: Record<string, string> = {
    'baidu': '百度网盘',
    'thunder': '迅雷网盘',
    'lanzou': '蓝奏云',
    'other': '其他网盘'
  }
  return sourceMap[source] || source || '未知网盘'
}

/**
 * 打开下载链接 - 使用系统默认浏览器
 */
async function openDownloadUrl(url: string) {
  if (url) {
    try {
      // 使用 Tauri shell 插件在默认浏览器中打开链接
      await openShell(url)
    } catch (error) {
      // 打开链接失败时静默处理
    }
  }
}

/**
 * 应用补丁
 */
function applyPatch(tag: GameTagConfig) {
  emit('apply-patch', tag)
}

/**
 * 启动游戏
 */
function handleLaunchGame() {
  if (props.gameConfig?.game_id) {
    emit('launch', props.gameConfig.game_id)
  }
}

/**
 * 选择游戏路径
 */
function handleSelectGamePath() {
  if (props.gameConfig?.game_id) {
    emit('select-path', props.gameConfig.game_id)
  }
}

/**
 * 关闭弹窗
 */
function handleClose() {
  emit('close')
}

/**
 * 打开虚拟化环境配置教程视频
 * 使用系统默认应用打开 resources/D加密虚拟化（虚拟机）环境搭建教程.mp4
 */
async function openVirtualizationTutorial() {
  try {
    // 使用 Tauri 命令获取程序根目录并打开视频文件
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('open_virtualization_tutorial')
  } catch (error) {
    // 打开失败时静默处理
  }
}
</script>

<style scoped>
/* 弹窗遮罩 */
.game-detail-modal {
  position: fixed;
  inset: 0;
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(4px);
  padding: 40px;
}

/* 弹窗内容 */
.modal-content {
  position: relative;
  width: 100%;
  max-width: 1000px;
  max-height: 90vh;
  background: var(--steam-bg-primary);
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
  display: flex;
  flex-direction: column;
}

/* 关闭按钮 */
.close-btn {
  position: absolute;
  top: 16px;
  right: 16px;
  z-index: 10;
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.5);
  border-radius: 50%;
  color: white;
  transition: background 0.15s ease-out;
}

.close-btn:hover {
  background: rgba(255, 255, 255, 0.2);
}

.close-btn svg {
  width: 24px;
  height: 24px;
}

/* 头部区域 */
.modal-header {
  position: relative;
  height: 280px;
  flex-shrink: 0;
}

.header-bg {
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg, var(--steam-bg-secondary) 0%, var(--steam-bg-tertiary) 100%);
}

.header-gradient {
  position: absolute;
  inset: 0;
  background: linear-gradient(to bottom, rgba(0, 0, 0, 0.3) 0%, var(--steam-bg-primary) 100%);
}

.header-content {
  position: relative;
  height: 100%;
  display: flex;
  align-items: flex-end;
  gap: 24px;
  padding: 24px;
}

/* 封面 */
.cover-container {
  width: 200px;
  height: 100px;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
  flex-shrink: 0;
  background: var(--steam-bg-tertiary);
}

.cover-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.cover-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--steam-text-muted);
}

.cover-placeholder svg {
  width: 48px;
  height: 48px;
}

/* 游戏信息 */
.game-info {
  flex: 1;
  padding-bottom: 8px;
}

.game-title {
  font-size: 26px;
  font-weight: 700;
  color: white;
  margin-bottom: 8px;
}

.game-subtitle {
  font-size: 16px;
  color: rgba(255, 255, 255, 0.8);
  margin-bottom: 12px;
}

.game-meta {
  display: flex;
  align-items: center;
  gap: 16px;
}

.game-id {
  font-size: 13px;
  color: rgba(255, 255, 255, 0.6);
  font-family: monospace;
}

.downloadable-badge {
  padding: 4px 12px;
  background: var(--steam-accent-green);
  color: white;
  font-size: 12px;
  font-weight: 500;
  border-radius: 4px;
}

/* 内容区域 */
.modal-body {
  flex: 1;
  display: flex;
  gap: 32px;
  padding: 24px;
  overflow-y: auto;
}

.body-left {
  flex: 1.5;
}

.body-right {
  flex: 1;
  min-width: 280px;
}

.section-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin-bottom: 16px;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--steam-border);
}

/* 补丁列表 */
.patch-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.patch-item {
  padding: 16px;
  background: var(--steam-bg-secondary);
  border-radius: 8px;
  border: 2px solid transparent;
  cursor: pointer;
  transition: all 0.15s ease-out;
}

.patch-item:hover {
  background: var(--steam-bg-tertiary);
}

.patch-item.active {
  border-color: var(--steam-accent-blue);
  background: rgba(27, 159, 255, 0.1);
}

.patch-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
}

.patch-type {
  font-size: 15px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.has-download {
  padding: 2px 8px;
  background: var(--steam-accent-blue);
  color: white;
  font-size: 12px;
  border-radius: 4px;
}

.patch-description {
  font-size: 13px;
  color: var(--steam-text-secondary);
  margin-bottom: 12px;
}

/* 下载区域样式 */
.download-section {
  margin-bottom: 12px;
}

.download-section-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-text-primary);
  margin: 0 0 8px 0;
}

.download-buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.download-btn-wrapper {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.pwd-hint {
  font-size: 11px;
  color: var(--steam-text-secondary);
  margin: 0;
}

.patch-actions {
  display: flex;
  gap: 8px;
}

.action-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  border-radius: 4px;
  font-size: 13px;
  font-weight: 500;
  transition: all 0.15s ease-out;
}

.action-btn svg {
  width: 16px;
  height: 16px;
}

.download-btn {
  background: var(--steam-accent-blue);
  color: white;
}

.download-btn:hover {
  background: #2a9fff;
}

.apply-btn {
  background: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
  border: 1px solid var(--steam-border);
}

.apply-btn:hover {
  background: var(--steam-bg-secondary);
}

/* 虚拟化环境配置教程按钮 */
.tutorial-btn {
  background: rgba(156, 39, 176, 0.2);
  color: #ce93d8;
  border: 1px solid rgba(156, 39, 176, 0.5);
}

.tutorial-btn:hover {
  background: rgba(156, 39, 176, 0.3);
  border-color: rgba(156, 39, 176, 0.7);
}

.no-patches {
  padding: 40px;
  text-align: center;
  color: var(--steam-text-muted);
}

/* 信息列表 */
.info-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 24px;
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 0;
  border-bottom: 1px solid var(--steam-border);
}

.info-item:last-child {
  border-bottom: none;
}

.info-label {
  font-size: 13px;
  color: var(--steam-text-secondary);
}

.info-value {
  font-size: 13px;
  color: var(--steam-text-primary);
  font-weight: 500;
}

/* 操作按钮 */
.action-buttons {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.primary-btn,
.secondary-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 14px 24px;
  border-radius: 8px;
  font-size: 15px;
  font-weight: 600;
  transition: all 0.15s ease-out;
}

.primary-btn {
  background: var(--steam-accent-green);
  color: white;
}

.primary-btn:hover {
  background: #45d163;
  transform: translateY(-1px);
}

.secondary-btn {
  background: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
  border: 1px solid var(--steam-border);
}

.secondary-btn:hover {
  background: var(--steam-bg-secondary);
}

.primary-btn svg,
.secondary-btn svg {
  width: 20px;
  height: 20px;
}

/* 弹窗动画 */
.modal-enter-active,
.modal-leave-active {
  transition: all 0.3s ease-out;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .modal-content,
.modal-leave-to .modal-content {
  transform: scale(0.95);
  opacity: 0;
}

/* 响应式 */
@media (max-width: 768px) {
  .game-detail-modal {
    padding: 16px;
  }

  .modal-header {
    height: auto;
    min-height: 200px;
  }

  .header-content {
    flex-direction: column;
    align-items: flex-start;
  }

  .cover-container {
    width: 160px;
    height: 80px;
  }

  .game-title {
    font-size: 24px;
  }

  .modal-body {
    flex-direction: column;
  }

  .body-right {
    min-width: auto;
  }
}
</style>
