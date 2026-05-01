<template>
  <!-- 
    ExtensionManager.vue - 扩展管理页面
    用于管理应用程序的扩展
  -->
  <div class="extension-page">
    <div class="page-header">
      <h1 class="page-title">管理扩展</h1>
      <p class="page-desc">安装和管理扩展功能</p>
    </div>
    
    <div class="extension-content">
      <!-- 工具栏 -->
      <div class="toolbar">
        <div class="toolbar-left">
          <SearchBox 
            v-model="searchKeyword"
            placeholder="搜索扩展..."
            @search="handleSearch"
          />
        </div>
        <div class="toolbar-right">
          <Button variant="primary" @click="showInstallModal = true">
            <template #icon>
              <svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
              </svg>
            </template>
            安装扩展
          </Button>
        </div>
      </div>
      
      <!-- 扩展列表 -->
      <div v-if="loading" class="loading-state">
        <div class="spinner" />
        <span>加载中...</span>
      </div>
      
      <div v-else-if="filteredExtensions.length === 0" class="empty-state">
        <svg viewBox="0 0 24 24" fill="currentColor">
          <path d="M20 6h-4V4c0-1.11-.89-2-2-2h-4c-1.11 0-2 .89-2 2v2H4c-1.11 0-1.99.89-1.99 2L2 19c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2zm-6 0h-4V4h4v2z"/>
        </svg>
        <p>暂无扩展</p>
        <p class="hint">点击"安装扩展"按钮添加扩展功能</p>
      </div>
      
      <div v-else class="extension-list">
        <div 
          v-for="ext in filteredExtensions" 
          :key="ext.manifest.id"
          class="extension-card"
        >
          <!-- 扩展图标 -->
          <div class="ext-icon">
            <img 
              v-if="ext.manifest.icon"
              :src="`${ext.path}/${ext.manifest.icon}`"
              :alt="ext.manifest.name"
            />
            <svg v-else viewBox="0 0 24 24" fill="currentColor">
              <path d="M20 6h-4V4c0-1.11-.89-2-2-2h-4c-1.11 0-2 .89-2 2v2H4c-1.11 0-1.99.89-1.99 2L2 19c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2zm-6 0h-4V4h4v2z"/>
            </svg>
          </div>
          
          <!-- 扩展信息 -->
          <div class="ext-info">
            <div class="ext-header">
              <h4 class="ext-name">{{ ext.manifest.name }}</h4>
              <span class="ext-version">v{{ ext.manifest.version }}</span>
            </div>
            <p class="ext-desc">{{ ext.manifest.description }}</p>
            <div class="ext-meta">
              <span class="ext-author">作者: {{ ext.manifest.author }}</span>
              <span class="ext-status" :class="ext.loadStatus">
                {{ getStatusText(ext.loadStatus) }}
              </span>
            </div>
          </div>
          
          <!-- 扩展操作 -->
          <div class="ext-actions">
            <Toggle 
              :model-value="ext.enabled"
              @update:model-value="(v) => toggleExtension(ext.manifest.id, v)"
            />
            <Button 
              variant="ghost" 
              size="sm"
              @click="viewDetails(ext)"
            >
              详情
            </Button>
            <Button 
              variant="ghost" 
              size="sm"
              @click="reloadExtension(ext.manifest.id)"
            >
              重载
            </Button>
            <Button 
              variant="ghost" 
              size="sm"
              class="delete-btn"
              @click="uninstallExtension(ext.manifest.id)"
            >
              卸载
            </Button>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 安装扩展弹窗 -->
    <InstallExtensionModal
      v-model="showInstallModal"
      @confirm="handleInstall"
    />
  </div>
</template>

<script setup lang="ts">
/**
 * ExtensionManager.vue - 扩展管理页面
 * 用于管理应用程序的扩展
 */

import { ref, computed, onMounted } from 'vue'
import { useExtensionStore } from '../../store/extension.store'
import type { Extension, ExtensionLoadStatus } from '../../types'
import Button from '../../components/common/Button.vue'
import SearchBox from '../../components/common/SearchBox.vue'
import Toggle from '../GlobalSettings/components/Toggle.vue'
import InstallExtensionModal from './components/InstallExtensionModal.vue'

// Store
const extensionStore = useExtensionStore()

// 本地状态
const searchKeyword = ref('')
const showInstallModal = ref(false)

// 计算属性
const loading = computed(() => extensionStore.loading)
const filteredExtensions = computed(() => {
  if (!searchKeyword.value) return extensionStore.extensions
  
  const keyword = searchKeyword.value.toLowerCase()
  return extensionStore.extensions.filter(ext => 
    ext.manifest.name.toLowerCase().includes(keyword) ||
    ext.manifest.description.toLowerCase().includes(keyword)
  )
})

// 生命周期
onMounted(() => {
  extensionStore.loadExtensions()
})

// 获取状态文本
function getStatusText(status: ExtensionLoadStatus): string {
  const statusMap: Record<ExtensionLoadStatus, string> = {
    unloaded: '未加载',
    loading: '加载中',
    loaded: '已加载',
    failed: '加载失败',
    disabled: '已禁用'
  }
  return statusMap[status]
}

// 搜索
function handleSearch(keyword: string) {
  searchKeyword.value = keyword
}

// 切换扩展状态
async function toggleExtension(id: string, enabled: boolean) {
  try {
    await extensionStore.toggleExtension(id, enabled)
  } catch (err) {
    console.error('切换扩展状态失败:', err)
    alert('切换扩展状态失败')
  }
}

// 重新加载扩展
async function reloadExtension(id: string) {
  try {
    await extensionStore.reloadExtension(id)
    alert('扩展已重新加载')
  } catch (err) {
    console.error('重新加载扩展失败:', err)
    alert('重新加载扩展失败')
  }
}

// 卸载扩展
async function uninstallExtension(id: string) {
  if (!confirm('确定要卸载此扩展吗？')) return
  
  try {
    await extensionStore.uninstallExtension(id)
  } catch (err) {
    console.error('卸载扩展失败:', err)
    alert('卸载扩展失败')
  }
}

// 查看详情
function viewDetails(ext: Extension) {
  console.log('查看扩展详情:', ext.manifest.name)
  // 可以实现详情弹窗
}

// 安装扩展
async function handleInstall(packagePath: string) {
  try {
    await extensionStore.installExtension({ packagePath })
    showInstallModal.value = false
    alert('扩展安装成功')
  } catch (err) {
    console.error('安装扩展失败:', err)
    alert('安装扩展失败: ' + (err instanceof Error ? err.message : '未知错误'))
  }
}
</script>

<style scoped>
.extension-page {
  height: 100%;
  overflow-y: auto;
  padding: 24px;
  background: var(--steam-bg-secondary);
}

.page-header {
  margin-bottom: 32px;
}

.page-title {
  font-size: 28px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin-bottom: 8px;
}

.page-desc {
  font-size: 14px;
  color: var(--steam-text-secondary);
}

.extension-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.toolbar-left {
  flex: 1;
  max-width: 300px;
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  padding: 60px;
  color: var(--steam-text-secondary);
}

.spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--steam-border);
  border-top-color: var(--steam-accent-blue);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 60px;
  color: var(--steam-text-secondary);
}

.empty-state svg {
  width: 64px;
  height: 64px;
  opacity: 0.5;
}

.hint {
  font-size: 12px;
  color: var(--steam-text-muted);
}

/* 扩展列表 */
.extension-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.extension-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  background: var(--steam-bg-primary);
  border-radius: 8px;
}

.ext-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--steam-bg-tertiary);
  border-radius: 8px;
  color: var(--steam-text-secondary);
  overflow: hidden;
}

.ext-icon img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.ext-icon svg {
  width: 24px;
  height: 24px;
}

.ext-info {
  flex: 1;
  min-width: 0;
}

.ext-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.ext-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

.ext-version {
  font-size: 12px;
  color: var(--steam-text-muted);
}

.ext-desc {
  font-size: 13px;
  color: var(--steam-text-secondary);
  margin-bottom: 8px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.ext-meta {
  display: flex;
  align-items: center;
  gap: 12px;
}

.ext-author {
  font-size: 12px;
  color: var(--steam-text-muted);
}

.ext-status {
  padding: 2px 8px;
  font-size: 11px;
  border-radius: 4px;
}

.ext-status.loaded {
  background: rgba(102, 192, 244, 0.2);
  color: var(--steam-accent-green);
}

.ext-status.disabled {
  background: rgba(143, 152, 160, 0.2);
  color: var(--steam-text-secondary);
}

.ext-status.failed {
  background: rgba(232, 17, 35, 0.2);
  color: #ff4d4f;
}

.ext-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.delete-btn {
  color: var(--steam-text-secondary);
}

.delete-btn:hover {
  color: #ff4d4f;
}
</style>
