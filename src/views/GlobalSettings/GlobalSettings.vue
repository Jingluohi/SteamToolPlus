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
              <Button variant="secondary" size="sm" @click="selectSteamPath">
                浏览
              </Button>
            </div>
          </div>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <h3 class="setting-name">游戏默认下载路径</h3>
            <p class="setting-desc">设置游戏下载的默认保存路径，留空则使用系统默认盘符的SteamGame文件夹</p>
          </div>
          <div class="setting-control">
            <div class="input-with-btn">
              <input
                v-model="settings.defaultDownloadPath"
                type="text"
                class="form-input"
                placeholder="选择游戏默认下载路径"
                readonly
              />
              <Button variant="secondary" size="sm" @click="selectDefaultDownloadPath">
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

      <!-- OpenSteamTool内核设置 -->
      <section class="settings-section">
        <h2 class="section-title">
          OpenSteamTool内核
        </h2>

        <div class="setting-item">
          <div class="setting-info">
            <h3 class="setting-name">内核状态</h3>
            <p class="setting-desc">OpenSteamTool内核DLL是否已安装到Steam目录，首次使用会自动安装</p>
          </div>
          <div class="setting-control">
            <span class="status-badge" :class="kernelStatus.installed ? 'success' : 'warning'">
              {{ kernelStatus.checking ? '检测中...' : (kernelStatus.installed ? '已安装' : '未安装') }}
            </span>
          </div>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <h3 class="setting-name">Steam运行状态</h3>
            <p class="setting-desc">检测Steam客户端是否正在运行，用于决定入库时是否需要重启</p>
          </div>
          <div class="setting-control">
            <span class="status-badge" :class="steamRunningStatus.running ? 'success' : 'warning'">
              {{ steamRunningStatus.checking ? '检测中...' : (steamRunningStatus.running ? '运行中' : '未运行') }}
            </span>
          </div>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <h3 class="setting-name">安装/卸载内核</h3>
            <p class="setting-desc">手动安装或卸载OpenSteamTool内核DLL到Steam目录</p>
          </div>
          <div class="setting-control">
            <Button
              v-if="!kernelStatus.installed"
              variant="primary"
              size="sm"
              :loading="kernelStatus.operating"
              :disabled="!settings.steamPath || kernelStatus.checking"
              @click="installKernel"
            >
              安装内核
            </Button>
            <Button
              v-else
              variant="danger"
              size="sm"
              :loading="kernelStatus.operating"
              :disabled="!settings.steamPath"
              @click="uninstallKernel"
            >
              卸载内核
            </Button>
          </div>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <h3 class="setting-name">热加载入库</h3>
            <p class="setting-desc">开启后入库时不重启Steam，依赖OpenSteamTool的文件监视自动加载新游戏</p>
          </div>
          <div class="setting-control">
            <Toggle v-model="settings.openSteamToolHotReload" />
          </div>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <h3 class="setting-name">高级模式</h3>
            <p class="setting-desc">启用后会写入Windows注册表，通常用于Denuvo/在线游戏（需要二次确认）</p>
          </div>
          <div class="setting-control">
            <Toggle v-model="settings.openSteamToolAdvancedMode" />
          </div>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <h3 class="setting-name">生成 opensteamtool.toml</h3>
            <p class="setting-desc">在Steam根目录生成OpenSteamTool默认配置文件，使用wudrm作为国内友好的manifest源</p>
          </div>
          <div class="setting-control">
            <Button
              variant="secondary"
              size="sm"
              :disabled="!settings.steamPath"
              @click="generateOpenSteamToolConfig"
            >
              生成配置
            </Button>
          </div>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <h3 class="setting-name">清理SteamTools残留</h3>
            <p class="setting-desc">清理旧版SteamTools可能残留的DLL、stplug-in目录和注册表项</p>
          </div>
          <div class="setting-control">
            <Button
              variant="danger"
              size="sm"
              :disabled="!settings.steamPath"
              @click="cleanSteamToolsResiduals"
            >
              清理残留
            </Button>
          </div>
        </div>
      </section>

      <!-- 清除缓存 -->
      <section class="settings-section">
        <h2 class="section-title">
          清除缓存
        </h2>

        <div class="setting-item">
          <div class="setting-info">
            <h3 class="setting-name">导入过的清单文件</h3>
            <p class="setting-desc">
              清理通过下载界面手动导入的清单文件缓存，将删除 resources/manifest 下对应的文件夹
            </p>
          </div>
          <div class="setting-control">
            <Button
              variant="danger"
              size="sm"
              :loading="cacheClearStatus.loading"
              @click="clearImportedManifestCache"
            >
              清除
            </Button>
          </div>
        </div>
      </section>

      <!-- 操作按钮 -->
      <div class="settings-actions">
        <Button variant="ghost" size="sm" @click="resetSettings">
          恢复默认
        </Button>
        <Button variant="primary" size="sm" @click="saveSettings">
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

import { ref, onMounted, nextTick, watch } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { useConfigStore } from '../../store/config.store'
import Button from '../../components/common/Button.vue'
import Toggle from './components/Toggle.vue'

// Store
const configStore = useConfigStore()

// 本地设置状态
const settings = ref({
  steamPath: '',
  defaultDownloadPath: '',
  startMinimizedToTray: false,
  hideToTrayOnClose: true,
  openSteamToolAdvancedMode: false,
  openSteamToolHotReload: true
})

// OpenSteamTool内核状态
const kernelStatus = ref({
  installed: false,
  checking: false,
  operating: false
})

// Steam运行状态
const steamRunningStatus = ref({
  running: false,
  checking: false
})

// 缓存清除状态
const cacheClearStatus = ref({
  loading: false
})

// 监听Steam路径变化，重新检测内核状态和Steam运行状态
watch(() => settings.value.steamPath, async (newPath, oldPath) => {
  if (newPath && newPath !== oldPath) {
    await checkKernelStatus()
    await checkSteamRunning()
  }
})

// 生命周期
onMounted(async () => {
  // 确保配置已加载
  if (!configStore.config) {
    await configStore.loadConfig()
  }
  // 使用 nextTick 确保响应式数据已更新
  nextTick(async () => {
    loadSettings()
    await checkKernelStatus()
    await checkSteamRunning()
  })
})

// 加载设置
function loadSettings() {
  const config = configStore.config
  if (config) {
    settings.value.steamPath = config.gameDirs.steamPath || ''
    settings.value.defaultDownloadPath = config.gameDirs.defaultDownloadPath || ''
    settings.value.startMinimizedToTray = config.launch.startMinimizedToTray ?? false
    settings.value.hideToTrayOnClose = config.launch.hideToTrayOnClose ?? true
    settings.value.openSteamToolAdvancedMode = config.opensteamtool?.advancedMode ?? false
    settings.value.openSteamToolHotReload = config.opensteamtool?.hotReload ?? true
  }
}

// 检查OpenSteamTool内核安装状态
async function checkKernelStatus() {
  if (!settings.value.steamPath) {
    kernelStatus.value.installed = false
    return
  }

  kernelStatus.value.checking = true
  try {
    const result = await invoke<{ installed: boolean }>('check_opensteamtool_kernel_installed', {
      steamPath: settings.value.steamPath
    })
    kernelStatus.value.installed = result.installed
  } catch {
    kernelStatus.value.installed = false
  } finally {
    kernelStatus.value.checking = false
  }
}

// 检查Steam是否正在运行
async function checkSteamRunning() {
  steamRunningStatus.value.checking = true
  try {
    const result = await invoke<{ running: boolean }>('check_steam_running')
    steamRunningStatus.value.running = result.running
  } catch {
    steamRunningStatus.value.running = false
  } finally {
    steamRunningStatus.value.checking = false
  }
}

// 安装OpenSteamTool内核
async function installKernel() {
  if (!settings.value.steamPath) {
    alert('请先配置Steam路径')
    return
  }

  kernelStatus.value.operating = true
  try {
    const result = await invoke<{ success: boolean; message: string }>('install_opensteamtool_kernel', {
      steamPath: settings.value.steamPath
    })
    if (result.success) {
      kernelStatus.value.installed = true
      alert(result.message)
    } else {
      alert(result.message)
    }
  } catch (error) {
    alert(`安装内核失败: ${error}`)
  } finally {
    kernelStatus.value.operating = false
  }
}

// 卸载OpenSteamTool内核
async function uninstallKernel() {
  if (!settings.value.steamPath) {
    alert('请先配置Steam路径')
    return
  }

  const confirmUninstall = confirm('确定要卸载OpenSteamTool内核吗？\n\n这将移除Steam目录中的内核DLL并尝试恢复原始文件。')
  if (!confirmUninstall) {
    return
  }

  kernelStatus.value.operating = true
  try {
    const result = await invoke<{ success: boolean; message: string }>('uninstall_opensteamtool_kernel', {
      steamPath: settings.value.steamPath
    })
    if (result.success) {
      kernelStatus.value.installed = false
      alert(result.message)
    } else {
      alert(result.message)
    }
  } catch (error) {
    alert(`卸载内核失败: ${error}`)
  } finally {
    kernelStatus.value.operating = false
  }
}

// 生成 opensteamtool.toml 配置文件
async function generateOpenSteamToolConfig() {
  if (!settings.value.steamPath) {
    alert('请先配置Steam路径')
    return
  }

  try {
    const result = await invoke<{ success: boolean; path: string; message: string }>('generate_opensteamtool_config', {
      steamPath: settings.value.steamPath
    })
    if (result.success) {
      alert(`${result.message}\n路径: ${result.path}`)
    } else {
      alert(result.message)
    }
  } catch (error) {
    alert(`生成配置失败: ${error}`)
  }
}

// 清理 SteamTools 残留
async function cleanSteamToolsResiduals() {
  if (!settings.value.steamPath) {
    alert('请先配置Steam路径')
    return
  }

  const confirmClean = confirm('确定要清理SteamTools残留吗？\n\n这将清理Steam目录中可能残留的SteamTools DLL、stplug-in目录和相关注册表项。')
  if (!confirmClean) {
    return
  }

  try {
    const result = await invoke<{
      success: boolean
      removedFiles: string[]
      removedDirs: string[]
      removedRegistryKeys: string[]
      message: string
    }>('clean_steamtools_residuals_command', {
      steamPath: settings.value.steamPath
    })

    const details = []
    if (result.removedFiles.length > 0) {
      details.push(`文件: ${result.removedFiles.length} 个`)
    }
    if (result.removedDirs.length > 0) {
      details.push(`目录: ${result.removedDirs.length} 个`)
    }
    if (result.removedRegistryKeys.length > 0) {
      details.push(`注册表项: ${result.removedRegistryKeys.length} 个`)
    }

    const detailText = details.length > 0 ? `\n\n清理内容:\n${details.join('\n')}` : ''
    alert(`${result.message}${detailText}`)
  } catch (error) {
    alert(`清理失败: ${error}`)
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

// 选择游戏默认下载路径
async function selectDefaultDownloadPath() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择游戏默认下载路径'
    })

    if (selected && typeof selected === 'string') {
      settings.value.defaultDownloadPath = selected
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
    const defaultDownloadPathValue = settings.value.defaultDownloadPath?.trim() || ''

    const updateData = {
      gameDirs: {
        steamPath: steamPathValue !== '' ? steamPathValue : currentConfig.gameDirs.steamPath,
        defaultDownloadPath: defaultDownloadPathValue !== '' ? defaultDownloadPathValue : undefined,
        coversPath: currentConfig.gameDirs.coversPath || 'data/covers'
      },
      launch: {
        startMinimizedToTray: settings.value.startMinimizedToTray,
        hideToTrayOnClose: settings.value.hideToTrayOnClose,
        verifyBeforeLaunch: currentConfig.launch.verifyBeforeLaunch || false,
        manifestImportInitialized: currentConfig.launch.manifestImportInitialized || false
      },
      opensteamtool: {
        kernelInstalled: currentConfig.opensteamtool?.kernelInstalled || kernelStatus.value.installed,
        advancedMode: settings.value.openSteamToolAdvancedMode,
        hotReload: settings.value.openSteamToolHotReload
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

// 清除导入过的清单文件缓存
async function clearImportedManifestCache() {
  const confirmClear = confirm('确定要清除导入过的清单文件缓存吗？\n\n这将删除 resources/manifest 下所有手动导入的清单文件夹，且无法恢复。')
  if (!confirmClear) {
    return
  }

  cacheClearStatus.value.loading = true
  try {
    const deletedCount = await invoke<number>('clear_imported_manifest_cache')
    alert(`已清除 ${deletedCount} 个导入过的清单文件夹`)
  } catch (error) {
    alert(`清除缓存失败: ${error}`)
  } finally {
    cacheClearStatus.value.loading = false
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

/* 状态标签 */
.status-badge {
  font-size: 12px;
  font-weight: 500;
  padding: 4px 8px;
  border-radius: 4px;
  display: inline-block;
}

.status-badge.success {
  color: var(--steam-accent-green);
  background: rgba(76, 175, 80, 0.1);
}

.status-badge.warning {
  color: var(--steam-accent-orange);
  background: rgba(255, 152, 0, 0.1);
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
