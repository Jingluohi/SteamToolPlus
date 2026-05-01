<template>
  <!-- 
    GlobalSettings.vue - 全局设置页面
    用于配置应用程序的各项设置
  -->
  <div class="settings-page">
    <div class="page-header">
      <h1 class="page-title">全局设置</h1>
      <p class="page-desc">配置应用程序的各项参数</p>
    </div>
    
    <div class="settings-content">
      <!-- 通用设置 -->
      <section class="settings-section">
        <h2 class="section-title">通用</h2>
        
        <div class="setting-item">
          <div class="setting-info">
            <h3 class="setting-name">主题</h3>
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
      
      <!-- 游戏设置 -->
      <section class="settings-section">
        <h2 class="section-title">游戏</h2>

        <div class="setting-item">
          <div class="setting-info">
            <h3 class="setting-name">Steam路径</h3>
            <p class="setting-desc">Steam安装目录路径，用于导入已安装的Steam游戏</p>
          </div>
          <div class="setting-control">
            <div class="input-with-btn">
              <input
                v-model="settings.steamPath"
                type="text"
                class="form-input"
                placeholder="选择Steam安装路径"
                readonly
              />
              <Button variant="secondary" @click="selectSteamPath">
                浏览...
              </Button>
            </div>
          </div>
        </div>
      </section>
      
      <!-- 启动设置 -->
      <section class="settings-section">
        <h2 class="section-title">启动</h2>
        
        <div class="setting-item">
          <div class="setting-info">
            <h3 class="setting-name">启动后最小化到托盘</h3>
            <p class="setting-desc">程序启动后自动隐藏到系统托盘，双击托盘图标可打开</p>
          </div>
          <div class="setting-control">
            <Toggle v-model="settings.startMinimizedToTray" />
          </div>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <h3 class="setting-name">关闭程序后隐藏在托盘</h3>
            <p class="setting-desc">点击关闭按钮后程序隐藏到托盘继续运行，2秒后自动释放图片缓存</p>
          </div>
          <div class="setting-control">
            <Toggle v-model="settings.hideToTrayOnClose" />
          </div>
        </div>

      </section>
      
      <!-- 操作按钮 -->
      <div class="settings-actions">
        <Button variant="ghost" @click="resetSettings">
          恢复默认
        </Button>
        <Button variant="primary" @click="saveSettings">
          保存设置
        </Button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * GlobalSettings.vue - 全局设置页面
 * 用于配置应用程序的各项设置
 */

import { ref, onMounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { useConfigStore } from '../../store/config.store'
import { useThemeStore } from '../../store/theme.store'
import Button from '../../components/common/Button.vue'
import Dropdown from '../../components/common/Dropdown.vue'
import Toggle from './components/Toggle.vue'

// Store
const configStore = useConfigStore()
const themeStore = useThemeStore()

// 本地设置状态
const settings = ref({
  theme: 'auto',
  steamPath: '',
  startMinimizedToTray: false,
  hideToTrayOnClose: true
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
    settings.value.steamPath = config.gameDirs.steamPath || ''
    settings.value.startMinimizedToTray = config.launch.startMinimizedToTray ?? false
    settings.value.hideToTrayOnClose = config.launch.hideToTrayOnClose ?? true
  }
}

// 处理主题变更
function handleThemeChange(value: string) {
  themeStore.setThemeMode(value as 'dark' | 'light' | 'auto')
}

// 选择Steam路径
async function selectSteamPath() {
  try {
    const selected = await open({
      directory: true,
      multiple: false
    })
    
    if (selected && typeof selected === 'string') {
      settings.value.steamPath = selected
    }
  } catch (err) {
    console.error('选择路径失败:', err)
  }
}

// 保存设置
async function saveSettings() {
  try {
    // 先获取当前配置，然后合并更新
    const currentConfig = configStore.config
    if (!currentConfig) {
      alert('配置未加载，请稍后重试')
      return
    }

    const updateData = {
      theme: {
        mode: settings.value.theme as 'dark' | 'light' | 'auto',
        followSystem: settings.value.theme === 'auto',
        customVars: currentConfig.theme.customVars || {}
      },
      gameDirs: {
        steamPath: settings.value.steamPath && settings.value.steamPath.trim() !== ''
          ? settings.value.steamPath
          : currentConfig.gameDirs.steamPath,
        coversPath: currentConfig.gameDirs.coversPath || 'data/covers'
      },
      launch: {
        startMinimizedToTray: settings.value.startMinimizedToTray,
        hideToTrayOnClose: settings.value.hideToTrayOnClose,
        verifyBeforeLaunch: currentConfig.launch.verifyBeforeLaunch || false
      }
    }
    
    await configStore.updateConfig(updateData)
    alert('设置已保存')
  } catch (err) {
    console.error('保存设置失败:', err)
    alert('保存设置失败: ' + (err instanceof Error ? err.message : String(err)))
  }
}

// 重置设置
async function resetSettings() {
  if (confirm('确定要恢复默认设置吗？')) {
    await configStore.resetConfig()
    loadSettings()
  }
}
</script>

<style scoped>
.settings-page {
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

.settings-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
  width: 100%;
}

.settings-section {
  background: var(--steam-bg-primary);
  border-radius: 8px;
  padding: 20px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--steam-border);
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 0;
  border-bottom: 1px solid var(--steam-border);
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-info {
  flex: 1;
}

.setting-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
  margin-bottom: 4px;
}

.setting-desc {
  font-size: 12px;
  color: var(--steam-text-muted);
}

.setting-control {
  min-width: 200px;
  display: flex;
  justify-content: flex-end;
}

/* 输入框 */
.input-with-btn {
  display: flex;
  gap: 8px;
}

.form-input {
  width: 240px;
  height: 36px;
  padding: 0 12px;
  background: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border);
  border-radius: 4px;
  color: var(--steam-text-primary);
  font-size: 14px;
}

.form-input::placeholder {
  color: var(--steam-text-muted);
}

/* 操作按钮 */
.settings-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding-top: 16px;
}
</style>
