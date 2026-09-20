<template>
  <!-- 
    DownloadProgress.vue - 下载进度组件
    显示游戏下载的总体进度和各个Depot的详细进度
    可复用于任何需要显示下载进度的界面
  -->
  <div class="download-progress-container">
    <!-- 总体进度 -->
    <div class="overall-progress">
      <div class="progress-header">
        <div class="progress-head-left">
          <span class="progress-title">下载进度</span>
          <!-- 实时速度 + 剩余时间（标题右侧；下载完成后隐藏，交由状态徽章提示） -->
          <div v-if="speedInfoVisible" class="progress-meta">
            <span class="meta-item speed">
              <svg class="meta-icon" viewBox="0 0 24 24" fill="currentColor">
                <path d="M13.5.67s.74 2.65.74 4.8c0 2.06-1.35 3.73-3.41 3.73-2.07 0-3.63-1.67-3.63-3.73l.03-.36C5.21 7.51 4 10.62 4 14c0 4.42 3.58 8 8 8s8-3.58 8-8C20 8.61 17.41 3.8 13.5.67zM11.71 19c-1.78 0-3.22-1.4-3.22-3.14 0-1.62 1.05-2.76 2.81-3.12 1.77-.36 3.6-1.21 4.62-2.58.39 1.29.59 2.65.59 4.04 0 2.65-2.15 4.8-4.8 4.8z"/>
              </svg>
              {{ formatSpeed(speed) }}
            </span>
            <span class="meta-item eta">{{ remainingText }}</span>
          </div>
        </div>
        <span class="progress-percentage">{{ progress.overallPercentage }}%</span>
      </div>
      <div class="progress-bar-bg">
        <div
          class="progress-bar-fill"
          :style="{ width: `${progress.overallPercentage}%` }"
          :class="{ 'complete': progress.isComplete }"
        />
      </div>
      <div class="progress-stats">
        <span class="stats-item">
          <svg class="stats-icon" viewBox="0 0 24 24" fill="currentColor">
            <path d="M3 3h18v18H3V3zm16 16V5H5v14h14zM7 7h4v4H7V7zm0 6h4v4H7v-4zm6-6h4v4h-4V7zm0 6h4v4h-4v-4z"/>
          </svg>
          已完成: {{ progress.completedDepots }} / {{ progress.totalDepots }}
        </span>
        <span v-if="progress.isComplete" class="stats-item success">
          <svg class="stats-icon" viewBox="0 0 24 24" fill="currentColor">
            <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
          </svg>
          下载完成
        </span>
        <span v-else-if="hasPausedDepot()" class="stats-item paused">
          <svg class="stats-icon" viewBox="0 0 24 24" fill="currentColor">
            <path d="M6 4h4v16H6V4zm8 0h4v16h-4V4z"/>
          </svg>
          已暂停
        </span>
        <span v-else-if="isMonitoring" class="stats-item downloading">
          <svg class="stats-icon spin" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 4V1L8 5l4 4V6c3.31 0 6 2.69 6 6 0 1.01-.25 1.97-.7 2.8l1.46 1.46C19.54 15.03 20 13.57 20 12c0-4.42-3.58-8-8-8zm0 14c-3.31 0-6-2.69-6-6 0-1.01.25-1.97.7-2.8L5.24 7.74C4.46 8.97 4 10.43 4 12c0 4.42 3.58 8 8 8v3l4-4-4-4v3z"/>
          </svg>
          下载中
        </span>
      </div>
    </div>

    <!-- Depot 详情列表（紧凑布局：ID/速度/徽章/百分比合并一行） -->
    <div v-if="progress.depots.length > 0" class="depots-list">
      <div
        v-for="depot in progress.depots"
        :key="depot.depotId"
        class="depot-item"
        :class="{
          'is-completed': depot.status === 'completed',
          'is-error': depot.status === 'error',
          'is-paused': isPausedDepot(depot)
        }"
      >
        <!-- 第一行：Depot ID + 徽章 + 百分比 -->
        <div class="depot-main">
          <span class="depot-id">Depot {{ depot.depotId }}</span>
          <span class="depot-badge" :class="badgeClass(depot)">
            {{ badgeText(depot) }}
          </span>
          <span class="depot-percentage" :class="depot.status === 'error' ? (isPausedDepot(depot) ? 'is-paused' : 'is-error') : ''">
            {{ depot.status === 'error' ? (isPausedDepot(depot) ? '已暂停' : '已中断') : `${depot.percentage}%` }}
          </span>
        </div>
        <!-- 进度条 -->
        <div class="depot-progress-bar">
          <div
            class="depot-progress-fill"
            :style="{ width: `${depot.percentage}%` }"
            :class="{
              'is-complete': depot.status === 'completed',
              'is-error': depot.status === 'error'
            }"
          />
        </div>
        <!-- 说明行：下载中的当前文件 / 错误日志（单行省略，hover 查完整内容） -->
        <div v-if="depot.message" class="depot-detail" :title="depot.message">
          {{ depot.message }}
        </div>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-else-if="isMonitoring" class="empty-state">
      <svg class="empty-icon spin" viewBox="0 0 24 24" fill="currentColor">
        <path d="M12 4V1L8 5l4 4V6c3.31 0 6 2.69 6 6 0 1.01-.25 1.97-.7 2.8l1.46 1.46C19.54 15.03 20 13.57 20 12c0-4.42-3.58-8-8-8zm0 14c-3.31 0-6-2.69-6-6 0-1.01.25-1.97.7-2.8L5.24 7.74C4.46 8.97 4 10.43 4 12c0 4.42 3.58 8 8 8v3l4-4-4-4v3z"/>
      </svg>
      <span>等待下载开始</span>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * DownloadProgress.vue - 下载进度组件
 * 显示游戏下载的总体进度和各个Depot的详细进度
 */

import type { DownloadProgress } from '../../types/download.types'
import { computed } from 'vue'
import { formatSpeed, formatRemaining } from '../../utils/downloadFormat'

/**
 * 组件属性定义
 */
interface Props {
  /** 下载进度数据 */
  progress: DownloadProgress
  /** 是否正在监控 */
  isMonitoring: boolean
  /** 实时下载速度（字节/秒；展示在"下载进度"标题右侧） */
  speed?: number
  /** 本次下载已下载字节数（引擎全局字节模型；供计算剩余时间） */
  doneBytes?: number
  /** 本次下载总字节数（引擎全局字节模型；供计算剩余时间） */
  totalBytes?: number
}

const props = withDefaults(defineProps<Props>(), {
  speed: 0,
  doneBytes: 0,
  totalBytes: 0
})

// ==================== 头部速度 / 剩余时间 ====================

/**
 * 头部速度信息是否可见：有有效速度采样、且整体未完成。
 * 完成/暂停后速度归零由 store 复位，此处随 speed 自动隐藏。
 */
const speedInfoVisible = computed(
  () => props.speed > 0 && props.doneBytes > 0 && !props.progress.isComplete
)

/** 剩余时间文本（按当前速度估算；速度未就绪或无剩余字节时返回空串） */
const remainingText = computed(() =>
  formatRemaining(props.speed, props.doneBytes, props.totalBytes)
)

// ==================== 暂停/失败判定 ====================

/** 该 depot 是否因用户暂停而中断（消息中含取消/暂停语义） */
function isPausedDepot(d: { status: string; message?: string }): boolean {
  return d.status === 'error' && /cancel|取消|暂停/i.test(d.message || '')
}

/** 是否存在处于暂停态的 depot（整体区联动："已暂停"代替"下载中"） */
function hasPausedDepot(): boolean {
  return props.progress.depots.some(
    (d) => d.status === 'error' && /cancel|取消|暂停/i.test(d.message || '')
  )
}

// ==================== 徽章与速度 ====================

/** 状态徽章文案（完成/已暂停/失败/下载中/等待） */
function badgeText(d: { status: string; message?: string }): string {
  if (d.status === 'completed') return '完成'
  if (d.status === 'error') return isPausedDepot(d) ? '已暂停' : '失败'
  if (d.status === 'downloading') return '下载中'
  return '等待'
}

/** 状态徽章配色 class */
function badgeClass(d: { status: string; message?: string }): string {
  if (d.status === 'completed') return 'completed'
  if (d.status === 'error') return isPausedDepot(d) ? 'paused' : 'error'
  if (d.status === 'downloading') return 'downloading'
  return 'pending'
}

/** 字节/秒 → 速度文本（十进制 MB/s，与下载列表页共用 utils/downloadFormat） */
// formatSpeed 由 utils/downloadFormat 导入，统一各页面速度显示逻辑
</script>

<style scoped>
.download-progress-container {
  background-color: var(--steam-bg-secondary);
  border-radius: 10px;
  padding: 12px 14px;
  border: 1px solid var(--steam-border);
}

/* 总体进度 */
.overall-progress {
  margin-bottom: 12px;
}

.progress-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

/* 标题 + 速度/剩余时间 组合（速度紧随标题右侧，百分比最右） */
.progress-head-left {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
}

.progress-meta {
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
}

.meta-item {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 13px;
  font-variant-numeric: tabular-nums;
  white-space: nowrap;
}

.meta-item.speed {
  color: var(--steam-accent-blue);
}

.meta-item.eta {
  color: var(--steam-text-secondary);
}

.meta-icon {
  width: 15px;
  height: 15px;
  flex-shrink: 0;
}

.progress-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.progress-percentage {
  font-size: 20px;
  font-weight: 700;
  color: var(--steam-accent-blue);
}

.progress-bar-bg {
  height: 12px;
  background-color: var(--steam-bg-tertiary);
  border-radius: 6px;
  overflow: hidden;
  margin-bottom: 12px;
}

.progress-bar-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--steam-accent-blue), #2a9fff);
  border-radius: 6px;
  transition: width 0.3s ease;
  position: relative;
  overflow: hidden;
}

.progress-bar-fill::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(255, 255, 255, 0.3),
    transparent
  );
  animation: shimmer 2s infinite;
}

.progress-bar-fill.complete {
  background: linear-gradient(90deg, var(--steam-accent-green), #45d163);
}

@keyframes shimmer {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}

.progress-stats {
  display: flex;
  gap: 20px;
  flex-wrap: wrap;
}

.stats-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--steam-text-secondary);
}

.stats-item.success {
  color: var(--steam-accent-green);
}

.stats-item.downloading {
  color: var(--steam-accent-blue);
}

/* 暂停态（橙色警示，区别于失败红） */
.stats-item.paused {
  color: var(--steam-warning);
}

.stats-icon {
  width: 16px;
  height: 16px;
}

.stats-icon.spin {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

/* Depot 列表（紧凑） */
.depots-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
  max-height: 300px;
  overflow-y: auto;
  padding-right: 4px;
}

.depots-list::-webkit-scrollbar {
  width: 6px;
}

.depots-list::-webkit-scrollbar-track {
  background: var(--steam-bg-tertiary);
  border-radius: 3px;
}

.depots-list::-webkit-scrollbar-thumb {
  background: var(--steam-border);
  border-radius: 3px;
}

.depot-item {
  background-color: var(--steam-bg-tertiary);
  border-radius: 6px;
  padding: 6px 10px;
  border: 1px solid var(--steam-border);
  transition: all 0.2s ease;
}

.depot-item:hover {
  border-color: var(--steam-accent-blue);
}

.depot-item.is-completed {
  border-color: rgba(76, 175, 80, 0.3);
  background-color: rgba(76, 175, 80, 0.05);
}

.depot-item.is-error {
  border-color: var(--steam-error);
  background-color: color-mix(in srgb, var(--steam-error-rgb) 6%, transparent);
}

.depot-item.is-paused {
  border-color: var(--steam-warning);
  background-color: color-mix(in srgb, var(--steam-warning-rgb) 6%, transparent);
}

/* 第一行：Depot ID + 速度 + 徽章（靠右） + 百分比 */
.depot-main {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.depot-id {
  flex-shrink: 0;
  font-size: 12px;
  font-weight: 600;
  color: var(--steam-text-primary);
  font-family: 'Courier New', monospace;
  line-height: 1.4;
}

/* 状态徽章：右推靠右，与百分比相邻 */
.depot-badge {
  margin-left: auto;
  flex-shrink: 0;
  display: inline-flex;
  align-items: center;
  padding: 0 8px;
  height: 18px;
  border-radius: 9px;
  font-size: 11px;
  font-weight: 500;
  line-height: 1;
}

.depot-badge.completed {
  background-color: rgba(76, 175, 80, 0.12);
  color: var(--steam-accent-green);
}

.depot-badge.downloading {
  background-color: rgba(27, 159, 255, 0.12);
  color: var(--steam-accent-blue);
}

.depot-badge.error {
  background-color: color-mix(in srgb, var(--steam-error-rgb) 12%, transparent);
  color: var(--steam-error);
}

.depot-badge.paused {
  background-color: rgba(217, 119, 6, 0.14);
  color: var(--steam-warning);
}

.depot-badge.pending {
  background-color: rgba(255, 255, 255, 0.06);
  color: var(--steam-text-subtle);
}

.depot-percentage {
  flex-shrink: 0;
  min-width: 44px;
  text-align: right;
  font-size: 12px;
  font-weight: 700;
  color: var(--steam-accent-blue);
  font-variant-numeric: tabular-nums;
  line-height: 1.4;
}

.depot-percentage.is-error {
  color: var(--steam-error);
}

.depot-percentage.is-paused {
  color: var(--steam-warning);
}

.depot-progress-bar {
  height: 5px;
  background-color: var(--steam-bg-secondary);
  border-radius: 3px;
  overflow: hidden;
  margin-bottom: 3px;
}

.depot-progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--steam-accent-blue), #2a9fff);
  border-radius: 3px;
  transition: width 0.3s ease;
}

.depot-progress-fill.is-complete {
  background: linear-gradient(90deg, var(--steam-accent-green), #45d163);
}

.depot-progress-fill.is-error {
  background: linear-gradient(90deg, var(--steam-error), var(--steam-error-hover));
}

/* 说明行：下载中的当前文件 / 错误日志（单行省略，hover 查完整内容） */
.depot-detail {
  font-size: 11px;
  line-height: 1.5;
  color: var(--steam-text-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 空状态 */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px;
  gap: 12px;
  color: var(--steam-text-secondary);
}

.empty-icon {
  width: 32px;
  height: 32px;
  color: var(--steam-accent-blue);
}

.empty-icon.spin {
  animation: spin 1s linear infinite;
}
</style>
