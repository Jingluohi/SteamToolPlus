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
        <span class="progress-title">下载进度</span>
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
        <span v-else-if="isMonitoring" class="stats-item downloading">
          <svg class="stats-icon spin" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 4V1L8 5l4 4V6c3.31 0 6 2.69 6 6 0 1.01-.25 1.97-.7 2.8l1.46 1.46C19.54 15.03 20 13.57 20 12c0-4.42-3.58-8-8-8zm0 14c-3.31 0-6-2.69-6-6 0-1.01.25-1.97.7-2.8L5.24 7.74C4.46 8.97 4 10.43 4 12c0 4.42 3.58 8 8 8v3l4-4-4-4v3z"/>
          </svg>
          下载中...
        </span>
      </div>
    </div>

    <!-- Depot 详情列表 -->
    <div v-if="progress.depots.length > 0" class="depots-list">
      <div
        v-for="depot in progress.depots"
        :key="depot.depotId"
        class="depot-item"
        :class="{ 'completed': depot.status === 'completed' }"
      >
        <div class="depot-header">
          <span class="depot-id">Depot {{ depot.depotId }}</span>
          <span class="depot-percentage">{{ depot.percentage }}%</span>
        </div>
        <div class="depot-progress-bar">
          <div
            class="depot-progress-fill"
            :style="{ width: `${depot.percentage}%` }"
            :class="{ 'complete': depot.status === 'completed' }"
          />
        </div>
        <div class="depot-files">
          <span class="files-count">已下载文件: {{ depot.downloadedFiles }}</span>
          <span v-if="depot.status === 'completed'" class="status-badge completed">
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
            </svg>
            完成
          </span>
          <span v-else class="status-badge downloading">
            <svg class="spin" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 4V1L8 5l4 4V6c3.31 0 6 2.69 6 6 0 1.01-.25 1.97-.7 2.8l1.46 1.46C19.54 15.03 20 13.57 20 12c0-4.42-3.58-8-8-8zm0 14c-3.31 0-6-2.69-6-6 0-1.01.25-1.97.7-2.8L5.24 7.74C4.46 8.97 4 10.43 4 12c0 4.42 3.58 8 8 8v3l4-4-4-4v3z"/>
            </svg>
            下载中
          </span>
        </div>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-else-if="isMonitoring" class="empty-state">
      <svg class="empty-icon spin" viewBox="0 0 24 24" fill="currentColor">
        <path d="M12 4V1L8 5l4 4V6c3.31 0 6 2.69 6 6 0 1.01-.25 1.97-.7 2.8l1.46 1.46C19.54 15.03 20 13.57 20 12c0-4.42-3.58-8-8-8zm0 14c-3.31 0-6-2.69-6-6 0-1.01.25-1.97.7-2.8L5.24 7.74C4.46 8.97 4 10.43 4 12c0 4.42 3.58 8 8 8v3l4-4-4-4v3z"/>
      </svg>
      <span>正在等待下载开始...</span>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * DownloadProgress.vue - 下载进度组件
 * 显示游戏下载的总体进度和各个Depot的详细进度
 */

import type { DownloadProgress } from '../../types'

/**
 * 组件属性定义
 */
interface Props {
  /** 下载进度数据 */
  progress: DownloadProgress
  /** 是否正在监控 */
  isMonitoring: boolean
}

defineProps<Props>()
</script>

<style scoped>
.download-progress-container {
  background-color: var(--steam-bg-secondary);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--steam-border);
}

/* 总体进度 */
.overall-progress {
  margin-bottom: 20px;
}

.progress-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
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

/* Depot 列表 */
.depots-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
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
  border-radius: 8px;
  padding: 12px;
  border: 1px solid var(--steam-border);
  transition: all 0.2s ease;
}

.depot-item:hover {
  border-color: var(--steam-accent-blue);
}

.depot-item.completed {
  border-color: rgba(76, 175, 80, 0.3);
  background-color: rgba(76, 175, 80, 0.05);
}

.depot-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.depot-id {
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-text-primary);
  font-family: 'Courier New', monospace;
}

.depot-percentage {
  font-size: 13px;
  font-weight: 600;
  color: var(--steam-accent-blue);
}

.depot-progress-bar {
  height: 6px;
  background-color: var(--steam-bg-secondary);
  border-radius: 3px;
  overflow: hidden;
  margin-bottom: 8px;
}

.depot-progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--steam-accent-blue), #2a9fff);
  border-radius: 3px;
  transition: width 0.3s ease;
}

.depot-progress-fill.complete {
  background: linear-gradient(90deg, var(--steam-accent-green), #45d163);
}

.depot-files {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.files-count {
  font-size: 12px;
  color: var(--steam-text-secondary);
}

.status-badge {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.status-badge.completed {
  background-color: rgba(76, 175, 80, 0.1);
  color: var(--steam-accent-green);
}

.status-badge.downloading {
  background-color: rgba(27, 159, 255, 0.1);
  color: var(--steam-accent-blue);
}

.status-badge svg {
  width: 12px;
  height: 12px;
}

.status-badge .spin {
  animation: spin 1s linear infinite;
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
