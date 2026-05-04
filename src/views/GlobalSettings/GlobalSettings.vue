<template>
  <!-- 
    GlobalSettings.vue - 全局设置页面
    用于配置应用程序的各项设置
  -->
  <div class="settings-page">
    <div class="page-header">
      <h1 class="page-title">全局设置</h1>
      <p class="page-desc">配置应用程序的基本参数</p>
    </div>
    
    <div class="settings-content">
      <!-- 游戏设置 -->
      <section class="settings-section">
        <h2 class="section-title">
          游戏
        </h2>

        <div class="setting-item">
          <div class="setting-info">
            <h3 class="setting-name">Steam路径</h3>
            <p class="setting-desc">Steam安装目录路径，用于导入已安装的Steam游戏和清单入库功能</p>
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
              <Button variant="secondary" size="small" @click="selectSteamPath">
                浏览
              </Button>
            </div>
          </div>
        </div>
      </section>
      
      <!-- 启动设置 -->
      <section class="settings-section">
        <h2 class="section-title">
          启动
        </h2>
        
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
        <Button variant="ghost" size="small" @click="resetSettings">
          恢复默认
        </Button>
        <Button variant="primary" size="small" @click="saveSettings">
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

import { ref, onMounted, nextTick } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { useConfigStore } from '../../store/config.store'
import Button from '../../components/common/Button.vue'
import Toggle from './components/Toggle.vue'

// Store
const configStore = useConfigStore()

// 本地设置状态
const settings = ref({
  steamPath: '',
  startMinimizedToTray: false,
  hideToTrayOnClose: true
})

// 生命周期
onMounted(async () => {
  // 确保配置已加载
  if (!configStore.config) {
    await configStore.loadConfig()
  }
  // 使用 nextTick 确保响应式数据已更新
  nextTick(() => {
    loadSettings()
  })
})

// 加载设置
function loadSettings() {
  const config = configStore.config
  if (config) {
    settings.value.steamPath = config.gameDirs.steamPath || ''
    settings.value.startMinimizedToTray = config.launch.startMinimizedToTray ?? false
    settings.value.hideToTrayOnClose = config.launch.hideToTrayOnClose ?? true
  }
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
    // 选择路径失败时静默处理
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

    // 确保 steamPath 有值
    const steamPathValue = settings.value.steamPath?.trim() || ''

    const updateData = {
      gameDirs: {
        steamPath: steamPathValue !== '' ? steamPathValue : currentConfig.gameDirs.steamPath,
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
  padding: 20px;
  background: var(--steam-bg-secondary);
}

.page-header {
  margin-bottom: 20px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--steam-border);
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin-bottom: 4px;
}

.page-desc {
  font-size: 13px;
  color: var(--steam-text-secondary);
}

.settings-content {
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

.section-icon {
  font-size: 16px;
  filter: grayscale(0.3);
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

/* 输入框 */
.input-with-btn {
  display: flex;
  gap: 8px;
  align-items: center;
}

.form-input {
  width: 200px;
  height: 32px;
  padding: 0 10px;
  background: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border);
  border-radius: 6px;
  color: var(--steam-text-primary);
  font-size: 13px;
  transition: border-color 0.15s ease;
}

.form-input:focus {
  outline: none;
  border-color: var(--steam-accent);
}

.form-input::placeholder {
  color: var(--steam-text-muted);
}

/* 操作按钮 */
.settings-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  padding-top: 12px;
  margin-top: 8px;
  border-top: 1px solid var(--steam-border);
}
</style>
