<template>
  <div class="coldclient-config-panel">
    <!-- 使用说明 -->
    <div class="usage-guide">
      <div class="guide-header">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"/>
          <line x1="12" y1="16" x2="12" y2="12"/>
          <line x1="12" y1="8" x2="12.01" y2="8"/>
        </svg>
        <span>格式说明</span>
      </div>
      <div class="guide-content">
        <div class="guide-item">
          <span class="guide-label">配置文件</span>
          <span class="guide-value">ColdClientLoader.ini（与 steamclient_loader.exe 同目录）</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">核心作用</span>
          <span class="guide-value">保留原 steam_api(64).dll，注入模拟 steamclient(64).dll</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">启动参数</span>
          <span class="guide-value">ExeCommandLine，如 -windowed -novid</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">额外DLL</span>
          <span class="guide-value">DllsToInjectFolder 指定包含待注入 DLL 的文件夹</span>
        </div>
      </div>
      <div class="guide-example">
        <div class="example-title">ColdClientLoader.ini 示例：</div>
        <pre class="example-code">[SteamClient]
Exe=game.exe
AppId=480
SteamClientDll=steamclient.dll
SteamClient64Dll=steamclient64.dll

[Injection]
ForceInjectSteamClient=1
ForceInjectGameOverlayRenderer=1
DllsToInjectFolder=extra_dlls</pre>
      </div>
      <p class="guide-tip">提示：用于绕过 Steam DRM，实现免 Steam 启动游戏</p>
    </div>

    <!-- 启用开关 -->
    <div class="config-group">
      <label class="toggle-label">
        <input v-model="config.enabled" type="checkbox" class="toggle-input" />
        <span class="toggle-slider"></span>
        <span class="toggle-text">启用 ColdClientLoader</span>
      </label>
    </div>

    <template v-if="config.enabled">
      <!-- SteamClient 配置 -->
      <div class="form-section">
        <h4 class="section-title">SteamClient</h4>

        <div class="form-group">
          <label>游戏可执行文件路径（Exe）</label>
          <input v-model="config.exe" type="text" placeholder="game.exe 或完整路径" />
        </div>

        <div class="form-group">
          <label>游戏运行目录（ExeRunDir，可选）</label>
          <input v-model="config.exeRunDir" type="text" placeholder="留空则自动设为 exe 所在目录" />
        </div>

        <div class="form-group">
          <label>启动参数（ExeCommandLine）</label>
          <input v-model="config.exeCommandLine" type="text" placeholder="例如：-windowed -novid" />
        </div>

        <div class="form-group">
          <label>AppID（留空则读取 steam_appid.txt）</label>
          <input v-model="config.appId" type="text" placeholder="例如：480" />
        </div>

        <div class="form-group">
          <label>32 位 SteamClient DLL 路径</label>
          <input v-model="config.steamClientDll" type="text" placeholder="steamclient.dll" />
        </div>

        <div class="form-group">
          <label>64 位 SteamClient64 DLL 路径</label>
          <input v-model="config.steamClient64Dll" type="text" placeholder="steamclient64.dll" />
        </div>
      </div>

      <!-- Injection 配置 -->
      <div class="form-section">
        <h4 class="section-title">Injection</h4>

        <div class="form-group checkbox-group">
          <label class="checkbox-label">
            <input v-model="config.forceInjectSteamClient" type="checkbox" />
            <span>强制注入 steamclient(64).dll</span>
          </label>
        </div>

        <div class="form-group checkbox-group">
          <label class="checkbox-label">
            <input v-model="config.forceInjectGameOverlayRenderer" type="checkbox" />
            <span>强制注入 GameOverlayRenderer(64).dll</span>
          </label>
        </div>

        <div class="form-group">
          <label>额外 DLL 注入文件夹（DllsToInjectFolder）</label>
          <input v-model="config.dllsToInjectFolder" type="text" placeholder="extra_dlls" />
        </div>

        <div class="form-group checkbox-group">
          <label class="checkbox-label">
            <input v-model="config.ignoreInjectionError" type="checkbox" />
            <span>忽略 DLL 注入失败错误</span>
          </label>
        </div>

        <div class="form-group checkbox-group">
          <label class="checkbox-label">
            <input v-model="config.ignoreLoaderArchDifference" type="checkbox" />
            <span>忽略 loader 与程序架构差异</span>
          </label>
        </div>
      </div>

      <!-- Persistence 配置 -->
      <div class="form-section">
        <h4 class="section-title">Persistence</h4>

        <div class="form-group">
          <label>持久化模式（Mode）</label>
          <select v-model.number="config.persistenceMode">
            <option :value="0">0 - 关闭</option>
            <option :value="1">1 - 启动 exe 并挂起等待确认</option>
            <option :value="2">2 - 仅设置环境，需手动运行 exe</option>
          </select>
        </div>
      </div>

      <!-- Debug 配置 -->
      <div class="form-section">
        <h4 class="section-title">Debug</h4>

        <div class="form-group checkbox-group">
          <label class="checkbox-label">
            <input v-model="config.resumeByDebugger" type="checkbox" />
            <span>通过调试器恢复主线程（ResumeByDebugger）</span>
          </label>
        </div>
      </div>
    </template>

    <!-- 保存按钮 -->
    <div class="panel-actions">
      <button class="btn-primary" @click="saveConfig">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="20 6 9 17 4 12"/>
        </svg>
        保存配置
      </button>
    </div>

    <!-- 保存成功提示 -->
    <transition name="toast">
      <div v-if="showToast" class="toast-success">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
          <polyline points="22 4 12 14.01 9 11.01"/>
        </svg>
        <span>ColdClientLoader 配置已保存成功！</span>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
/**
 * ColdClientLoaderConfigPanel.vue - ColdClientLoader 配置统一 Panel
 * 供单独弹窗和完整配置管理器复用
 */

import { shallowReactive, ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { CONFIG_EVENTS } from '../../../constants/config-events'
import type { ColdClientLoaderConfig } from '../../../types/steam-config.types'

const props = defineProps<{
  gamePath: string
}>()

const emit = defineEmits<{
  saved: []
}>()

const showToast = ref(false)

/**
 * ColdClientLoader 配置对象
 * 与 Rust ColdClientLoaderConfig 结构体一致，字段对应 ColdClientLoader.ini。
 * 使用 shallowReactive 减少深层响应式代理开销。
 */
const config = shallowReactive<ColdClientLoaderConfig>({
  enabled: false,
  exe: '',
  exeRunDir: '',
  exeCommandLine: '',
  appId: '',
  steamClientDll: 'steamclient.dll',
  steamClient64Dll: 'steamclient64.dll',
  forceInjectSteamClient: true,
  forceInjectGameOverlayRenderer: true,
  dllsToInjectFolder: '',
  ignoreInjectionError: true,
  ignoreLoaderArchDifference: false,
  persistenceMode: 0,
  resumeByDebugger: false,
})

/**
 * 保存配置
 */
async function saveConfig() {
  try {
    const result = await invoke<{ success: boolean; message: string }>('save_coldclient_config', {
      gamePath: props.gamePath,
      config,
    })

    if (result.success) {
      showToast.value = true
      setTimeout(() => {
        showToast.value = false
      }, 3000)
      emit('saved')
      // 广播 ColdClientLoader 配置已保存事件，通知完整配置管理器等其它窗口刷新
      window.dispatchEvent(new CustomEvent(CONFIG_EVENTS.COLDCLIENT_SAVED, {
        detail: { gamePath: props.gamePath }
      }))
    } else {
      alert(`保存失败: ${result.message}`)
    }
  } catch (error) {
    alert(`保存失败: ${error}`)
  }
}

/**
 * 加载现有配置
 */
async function loadConfig() {
  try {
    const result = await invoke<{
      exists: boolean
      config?: ColdClientLoaderConfig
    }>('load_coldclient_config', {
      gamePath: props.gamePath,
    })

    if (result.exists && result.config) {
      const cfg = result.config
      config.enabled = cfg.enabled ?? false
      config.exe = cfg.exe || ''
      config.exeRunDir = cfg.exeRunDir || ''
      config.exeCommandLine = cfg.exeCommandLine || ''
      config.appId = cfg.appId || ''
      config.steamClientDll = cfg.steamClientDll || 'steamclient.dll'
      config.steamClient64Dll = cfg.steamClient64Dll || 'steamclient64.dll'
      config.forceInjectSteamClient = cfg.forceInjectSteamClient ?? true
      config.forceInjectGameOverlayRenderer = cfg.forceInjectGameOverlayRenderer ?? true
      config.dllsToInjectFolder = cfg.dllsToInjectFolder || ''
      config.ignoreInjectionError = cfg.ignoreInjectionError ?? true
      config.ignoreLoaderArchDifference = cfg.ignoreLoaderArchDifference ?? false
      config.persistenceMode = cfg.persistenceMode ?? 0
      config.resumeByDebugger = cfg.resumeByDebugger ?? false
    }
  } catch (error) {
    // 加载失败时使用默认值
  }
}

/**
 * 统一配置保存事件处理器：仅当事件携带的 gamePath 与当前 Panel 匹配时重新加载
 */
function onConfigSavedEvent(e: Event) {
  const customEvent = e as CustomEvent<{ gamePath?: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    loadConfig()
  }
}

onMounted(() => {
  loadConfig()
  // 监听 ColdClientLoader 配置保存事件，与完整配置管理器实时同步
  window.addEventListener(CONFIG_EVENTS.COLDCLIENT_SAVED, onConfigSavedEvent)
})

onUnmounted(() => {
  window.removeEventListener(CONFIG_EVENTS.COLDCLIENT_SAVED, onConfigSavedEvent)
})

defineExpose({
  load: loadConfig,
  save: saveConfig
})
</script>

<style scoped>
.coldclient-config-panel {
  width: 100%;
}

.config-group {
  margin-bottom: 20px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 16px;
}

.form-group label {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

.form-group input,
.form-group select,
.form-group textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 13px;
  outline: none;
  box-sizing: border-box;
}

.form-group input:focus,
.form-group select:focus,
.form-group textarea:focus {
  border-color: var(--steam-accent-blue);
}

.form-section {
  margin-bottom: 20px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--steam-border);
}

.form-section:last-of-type {
  border-bottom: none;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 14px 0;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--steam-border);
}

.checkbox-group .checkbox-label {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

.checkbox-group .checkbox-label input[type="checkbox"] {
  width: 18px;
  height: 18px;
  cursor: pointer;
  accent-color: var(--steam-accent-blue);
  flex-shrink: 0;
}

.form-group textarea {
  font-family: 'Consolas', 'Courier New', monospace;
  resize: vertical;
}

.toggle-label {
  display: flex;
  align-items: center;
  gap: 12px;
  cursor: pointer;
}

.toggle-input {
  display: none;
}

.toggle-slider {
  width: 48px;
  height: 26px;
  background-color: var(--steam-border);
  border-radius: 13px;
  position: relative;
  transition: background-color 0.2s ease;
  flex-shrink: 0;
}

.toggle-slider::after {
  content: '';
  position: absolute;
  width: 22px;
  height: 22px;
  background-color: white;
  border-radius: 50%;
  top: 2px;
  left: 2px;
  transition: transform 0.2s ease;
}

.toggle-input:checked + .toggle-slider {
  background-color: var(--steam-accent-blue);
}

.toggle-input:checked + .toggle-slider::after {
  transform: translateX(22px);
}

.toggle-text {
  font-size: 14px;
  color: var(--steam-text-primary);
}

.btn-primary {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  background-color: var(--steam-accent-blue);
  color: white;
}

.btn-primary:hover {
  background-color: var(--steam-accent-hover);
}

.btn-primary svg {
  width: 16px;
  height: 16px;
}

.panel-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 24px;
  padding-top: 20px;
  border-top: 1px solid var(--steam-border);
}

/* 使用说明 */
.usage-guide {
  background-color: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 10px;
  padding: 16px 20px;
  margin-bottom: 20px;
}

.guide-header {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 14px;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-accent-blue);
}

.guide-header svg {
  width: 18px;
  height: 18px;
  flex-shrink: 0;
}

.guide-content {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-bottom: 16px;
}

.guide-item {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  font-size: 13px;
  line-height: 1.6;
}

.guide-item::before {
  content: '';
  display: block;
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background-color: var(--steam-accent-blue);
  flex-shrink: 0;
  margin-top: 7px;
}

.guide-label {
  color: var(--steam-text-secondary);
  white-space: nowrap;
  min-width: 100px;
  flex-shrink: 0;
}

.guide-value {
  color: var(--steam-text-primary);
  font-family: 'Consolas', 'Courier New', monospace;
  font-size: 12px;
  word-break: break-all;
}

.guide-example {
  background-color: var(--steam-bg-primary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  padding: 12px 14px;
  margin-bottom: 10px;
}

.guide-example:last-of-type {
  margin-bottom: 0;
}

.example-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin-bottom: 8px;
}

.example-code {
  font-size: 12px;
  color: var(--steam-text-primary);
  background-color: rgba(0, 0, 0, 0.2);
  padding: 10px 14px;
  border-radius: 6px;
  overflow-x: auto;
  line-height: 1.6;
  margin: 0;
  white-space: pre-wrap;
  word-break: break-all;
}

.guide-tip {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 8px 0 0 0;
}

/* 保存成功提示 */
.toast-success {
  position: fixed;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  background-color: #10b981;
  color: white;
  padding: 12px 24px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  font-weight: 500;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 9999;
}

.toast-success svg {
  width: 20px;
  height: 20px;
}

.toast-enter-active {
  animation: toast-in 0.3s ease;
}

.toast-leave-active {
  animation: toast-out 0.3s ease;
}

@keyframes toast-in {
  from {
    opacity: 0;
    transform: translateX(-50%) translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateX(-50%) translateY(0);
  }
}

@keyframes toast-out {
  from {
    opacity: 1;
    transform: translateX(-50%) translateY(0);
  }
  to {
    opacity: 0;
    transform: translateX(-50%) translateY(-20px);
  }
}
</style>
