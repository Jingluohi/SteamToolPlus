<template>
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
          <svg class="stats-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
            <line x1="3" y1="9" x2="21" y2="9"/>
            <line x1="9" y1="21" x2="9" y2="9"/>
          </svg>
          已完成: {{ progress.completedDepots }} / {{ progress.totalDepots }}
        </span>
        <span v-if="progress.isComplete" class="stats-item success">
          <svg class="stats-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
            <polyline points="22 4 12 14.01 9 11.01"/>
          </svg>
          下载完成
        </span>
        <span v-else-if="isMonitoring" class="stats-item downloading">
          <svg class="stats-icon spin" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
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
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="20 6 9 17 4 12"/>
            </svg>
            完成
          </span>
          <span v-else class="status-badge downloading">
            <svg class="spin" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
            </svg>
            下载中
          </span>
        </div>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-else-if="isMonitoring" class="empty-state">
      <svg class="empty-icon spin" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
      </svg>
      <span>正在等待下载开始...</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { DownloadProgress } from '../../composables/useDownloadProgress'

interface Props {
  progress: DownloadProgress
  isMonitoring: boolean
}

defineProps<Props>()
</script>

<style scoped>
.download-progress-container {
  background-color: var(--bg-secondary);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
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
  color: var(--text-primary);
}

.progress-percentage {
  font-size: 20px;
  font-weight: 700;
  color: var(--accent-color);
}

.progress-bar-bg {
  height: 12px;
  background-color: var(--bg-primary);
  border-radius: 6px;
  overflow: hidden;
  margin-bottom: 12px;
}

.progress-bar-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--accent-color), var(--accent-hover));
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
  background: linear-gradient(90deg, #10b981, #34d399);
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
  color: var(--text-secondary);
}

.stats-item.success {
  color: #10b981;
}

.stats-item.downloading {
  color: var(--accent-color);
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
  background: var(--bg-primary);
  border-radius: 3px;
}

.depots-list::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 3px;
}

.depot-item {
  background-color: var(--bg-primary);
  border-radius: 8px;
  padding: 12px;
  border: 1px solid var(--border-color);
  transition: all 0.2s ease;
}

.depot-item:hover {
  border-color: var(--accent-color);
}

.depot-item.completed {
  border-color: rgba(16, 185, 129, 0.3);
  background-color: rgba(16, 185, 129, 0.05);
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
  color: var(--text-primary);
  font-family: 'Courier New', monospace;
}

.depot-percentage {
  font-size: 13px;
  font-weight: 600;
  color: var(--accent-color);
}

.depot-progress-bar {
  height: 6px;
  background-color: var(--bg-secondary);
  border-radius: 3px;
  overflow: hidden;
  margin-bottom: 8px;
}

.depot-progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--accent-color), var(--accent-hover));
  border-radius: 3px;
  transition: width 0.3s ease;
}

.depot-progress-fill.complete {
  background: linear-gradient(90deg, #10b981, #34d399);
}

.depot-files {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.files-count {
  font-size: 12px;
  color: var(--text-secondary);
}

.status-badge {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 500;
}

.status-badge.completed {
  background-color: rgba(16, 185, 129, 0.1);
  color: #10b981;
}

.status-badge.downloading {
  background-color: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
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
  color: var(--text-secondary);
}

.empty-icon {
  width: 32px;
  height: 32px;
  color: var(--accent-color);
}

.empty-icon.spin {
  animation: spin 1s linear infinite;
}
</style>
