<template>
  <!-- 
    Personalization.vue - 个性化设置页面
    用于配置应用程序的背景、主题等个性化设置
  -->
  <div class="personalization-page">
    <div class="page-header">
      <h1 class="page-title">个性化</h1>
      <p class="page-desc">配置应用程序的外观和背景</p>
    </div>
    
    <div class="personalization-content">
      <!-- 主题设置 -->
      <section class="settings-section">
        <h2 class="section-title">
          主题
        </h2>
        
        <div class="setting-item">
          <div class="setting-info">
            <h3 class="setting-name">主题模式</h3>
            <p class="setting-desc">选择应用程序的主题模式</p>
          </div>
          <div class="setting-control">
            <Dropdown
              v-model="settings.theme"
              :options="themeOptions"
              @change="handleThemeChange"
            />
          </div>
        </div>
      </section>
      
      <!-- 背景设置 -->
      <section class="settings-section">
        <h2 class="section-title">
          背景
        </h2>
        <BackgroundSettings @refresh="handleBackgroundRefresh" />
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * Personalization.vue - 个性化设置页面
 * 用于配置应用程序的外观和背景
 */

import { ref, onMounted } from 'vue'
import { useConfigStore } from '../../store/config.store'
import { useThemeStore } from '../../store/theme.store'
import Dropdown from '../../components/common/Dropdown.vue'
import BackgroundSettings from '../../components/background/BackgroundSettings.vue'

// Store
const configStore = useConfigStore()
const themeStore = useThemeStore()

// 本地设置状态
const settings = ref({
  theme: 'auto'
})

// 主题选项
const themeOptions = [
  { value: 'dark', label: '深色' },
  { value: 'light', label: '浅色' },
  { value: 'auto', label: '跟随系统' }
]

// 生命周期
onMounted(async () => {
  // 确保配置已加载
  if (!configStore.config) {
    await configStore.loadConfig()
  }
  loadSettings()
})

// 加载设置
function loadSettings() {
  const config = configStore.config
  if (config) {
    settings.value.theme = config.theme.mode
  }
}

// 处理主题变更
async function handleThemeChange(value: string) {
  themeStore.setThemeMode(value as 'dark' | 'light' | 'auto')
  
  // 立即保存主题设置到配置文件
  const currentConfig = configStore.config
  if (currentConfig) {
    const updateData = {
      theme: {
        mode: value as 'dark' | 'light' | 'auto',
        followSystem: value === 'auto',
        customVars: currentConfig.theme.customVars || {}
      }
    }
    try {
      await configStore.updateConfig(updateData)
    } catch (err) {
      // 保存失败时静默处理
    }
  }
}

// 处理背景刷新
function handleBackgroundRefresh() {
  // 背景设置组件通知刷新，可以在这里添加全局刷新逻辑
}
</script>

<style scoped>
.personalization-page {
  height: 100%;
  overflow-y: auto;
  padding: 20px;
  background: var(--steam-bg-secondary);
}

.page-header {
  margin-bottom: 20px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--steam-border);
}

.page-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin-bottom: 4px;
}

.page-desc {
  font-size: 13px;
  color: var(--steam-text-secondary);
}

.personalization-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
  width: 100%;
}

.settings-section {
  background: rgba(var(--steam-bg-primary-rgb), 0.8);
  border-radius: 12px;
  padding: 16px;
  border: 1px solid var(--steam-border);
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  position: relative;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin-bottom: 12px;
  padding-bottom: 10px;
  border-bottom: 1px solid var(--steam-border);
  display: flex;
  align-items: center;
  gap: 8px;
}

.setting-item {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: 12px 0;
  border-bottom: 1px solid rgba(255,255,255,0.05);
  gap: 16px;
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-info {
  flex: 1;
  min-width: 0;
  padding-right: 16px;
}

.setting-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
  margin-bottom: 4px;
  line-height: 1.4;
}

.setting-desc {
  font-size: 12px;
  color: var(--steam-text-muted);
  line-height: 1.5;
  word-wrap: break-word;
}

.setting-control {
  min-width: 140px;
  display: flex;
  justify-content: flex-end;
  align-items: center;
  padding-top: 2px;
}
</style>
