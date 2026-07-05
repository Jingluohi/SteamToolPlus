<template>
  <div class="app-config-panel">
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
          <span class="guide-label">应用配置文件</span>
          <span class="guide-value">configs.app.ini</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">分支名称</span>
          <span class="guide-value">public（默认）或其他分支名</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">DLC ID</span>
          <span class="guide-value">纯数字，如 123456</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">DLC 名称（可选）</span>
          <span class="guide-value">appid=DLC名称，如 367680=RimWorld - Royalty</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">用途</span>
          <span class="guide-value">控制应用版本分支、DLC 解锁策略、Steam Input 和云存档</span>
        </div>
      </div>
      <div class="guide-example">
        <div class="example-title">DLC 列表示例（支持带名称）：</div>
        <pre class="example-code">367680=RimWorld - Name in Game Access
990430=RimWorld - Soundtrack
1149640=RimWorld - Royalty
1392840=RimWorld - Ideology</pre>
      </div>
    </div>

    <!-- 分支名称 -->
    <div class="config-group">
      <label class="config-label">分支名称</label>
      <input
        v-model="config.branchName"
        type="text"
        class="config-input"
        placeholder="public"
      />
      <p class="field-hint">Steam 应用分支名称，默认为 public。部分游戏有 beta 分支可填写对应名称</p>
    </div>

    <div class="config-group">
      <label class="checkbox-label">
        <input v-model="config.isBetaBranch" type="checkbox" />
        <span>当前为 Beta 分支</span>
      </label>
      <p class="field-hint">启用后标记当前分支为 Beta 分支</p>
    </div>

    <!-- DLC 模式选择 -->
    <div class="config-group">
      <label class="config-label">DLC 解锁模式</label>
      <div class="dlc-mode-selector">
        <label class="dlc-mode-option">
          <input
            v-model="config.dlcs.unlockAll"
            type="radio"
            name="dlcMode"
            :value="true"
          />
          <span class="dlc-mode-label">解锁所有 DLC</span>
        </label>
        <label class="dlc-mode-option">
          <input
            v-model="config.dlcs.unlockAll"
            type="radio"
            name="dlcMode"
            :value="false"
          />
          <span class="dlc-mode-label">手动指定 DLC</span>
        </label>
      </div>
      <p class="field-hint">部分游戏不适用"解锁所有"，可手动指定需要解锁的 DLC ID</p>
    </div>

    <div v-if="!config.dlcs.unlockAll" class="config-group">
      <label class="config-label">DLC 列表</label>
      <p class="config-desc">输入要解锁的 DLC ID，支持带名称（每行一个）</p>
      <textarea
        v-model="config.dlcs.customList"
        class="config-textarea"
        rows="6"
        placeholder="格式1（纯ID）:&#10;123456&#10;789012&#10;&#10;格式2（带名称）:&#10;367680=RimWorld - Royalty&#10;1392840=RimWorld - Ideology"
      ></textarea>
      <p class="field-hint">每行一个 DLC ID，支持 "appid=DLC名称" 格式（名称仅用于显示）</p>
    </div>

    <!-- Steam Input 控制器配置 -->
    <div class="config-section">
      <h4>Steam Input 控制器</h4>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.controller.steamInput" type="checkbox" />
          <span>启用 Steam Input</span>
        </label>
        <p class="field-hint">启用后，模拟器会模拟 Steam Input API，让游戏识别手柄输入</p>
      </div>
      <div class="form-group">
        <label>控制器类型</label>
        <select v-model="config.controller.type">
          <option value="">不指定</option>
          <option value="XBOX360">Xbox 360</option>
          <option value="PS4">PS4</option>
          <option value="PS5">PS5</option>
          <option value="SWITCH">Switch</option>
        </select>
        <p class="field-hint">指定后，游戏会认为连接的是该类型控制器</p>
      </div>
    </div>

    <!-- 云存档配置 -->
    <div class="config-section">
      <h4>云存档</h4>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.cloudSaves.enabled" type="checkbox" />
          <span>启用云存档</span>
        </label>
        <p class="field-hint">启用后，模拟器会模拟 Steam 云存档功能，支持存档同步</p>
      </div>
      <div class="form-row">
        <label class="checkbox-label">
          <input v-model="config.cloudSaves.createDefaultDir" type="checkbox" />
          <span>自动创建默认目录</span>
        </label>
        <label class="checkbox-label">
          <input v-model="config.cloudSaves.createSpecificDirs" type="checkbox" />
          <span>自动创建特定目录</span>
        </label>
      </div>
      <p class="field-hint">自动创建目录可避免游戏因找不到存档目录而报错</p>
    </div>

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
        <span>应用配置已保存成功！</span>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
/**
 * AppConfigPanel.vue - 应用配置 Panel
 * 供 DlcConfig.vue 单独弹窗和 CompleteConfigManager.vue 完整管理器复用
 * 统一加载、保存、默认值和同步逻辑
 */

import { ref, shallowReactive, reactive, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { CONFIG_EVENTS } from '../../../constants/config-events'

const props = defineProps<{
  gamePath: string
}>()

const emit = defineEmits<{
  saved: []
}>()

/**
 * 应用配置对象
 * 外层使用 shallowReactive 减少深层代理开销；
 * 嵌套对象（dlcs / controller / cloudSaves）使用 reactive，
 * 确保 v-model 在表单字段上仍能正常响应。
 */
const config = shallowReactive({
  branchName: 'public',
  isBetaBranch: false,
  dlcs: reactive({
    unlockAll: true,
    customList: ''      // 支持 "appid" 或 "appid=DLC Name" 格式
  }),
  controller: reactive({
    steamInput: false,
    type: ''
  }),
  cloudSaves: reactive({
    enabled: false,
    createDefaultDir: false,
    createSpecificDirs: false
  })
})

const showToast = ref(false)

/**
 * 规范化 DLC 列表文本
 * 支持格式: "123456" 或 "123456=DLC Name"
 * 保留原始 "appid=Name" 格式，仅去除空行和首尾空格，过滤非数字开头的行
 */
function normalizeDlcList(text: string): string {
  return text
    .split('\n')
    .map(line => line.trim())
    .filter(line => line && /^\d/.test(line))
    .join('\n')
}

// 加载现有的应用配置
async function loadAppConfig() {
  try {
    const result = await invoke<{ exists: boolean; config?: any }>('load_app_config', {
      gamePath: props.gamePath
    })

    if (result.exists && result.config) {
      const app = result.config

      // 分支
      config.branchName = app.branch_name || 'public'
      config.isBetaBranch = app.is_beta_branch || false

      // DLC
      config.dlcs.unlockAll = app.dlcs?.unlock_all ?? true
      if (app.dlcs?.individual_dlcs && app.dlcs.individual_dlcs.length > 0) {
        config.dlcs.customList = app.dlcs.individual_dlcs
          .filter((d: any) => d.enabled)
          .map((d: any) => {
            // 如果名称是默认生成的 "DLC {appid}" 或等于 app_id，只显示 appid
            if (d.name && d.name !== `DLC ${d.app_id}` && d.name !== d.app_id) {
              return `${d.app_id}=${d.name}`
            }
            return d.app_id
          })
          .join('\n')
      } else {
        config.dlcs.customList = ''
      }

      // Steam Input 控制器
      if (app.controller) {
        config.controller.steamInput = app.controller.steam_input ?? false
        config.controller.type = app.controller.type ?? ''
      }

      // 云存档
      if (app.cloud_saves) {
        config.cloudSaves.enabled = app.cloud_saves.enabled ?? false
        config.cloudSaves.createDefaultDir = app.cloud_saves.create_default_dir ?? false
        config.cloudSaves.createSpecificDirs = app.cloud_saves.create_specific_dirs ?? false
      }
    }
  } catch (error) {
    // 加载失败时使用默认值
  }
}

/**
 * 统一配置保存事件处理器
 * 仅当事件携带的 gamePath 与当前 Panel 匹配时重新加载配置
 */
function onConfigSavedEvent(e: Event) {
  const customEvent = e as CustomEvent<{ gamePath?: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    loadAppConfig()
  }
}

onMounted(() => {
  loadAppConfig()
  // 监听应用配置保存事件，与完整配置管理器/其它单独窗口实时同步
  window.addEventListener(CONFIG_EVENTS.APP_SAVED, onConfigSavedEvent)
})

onUnmounted(() => {
  window.removeEventListener(CONFIG_EVENTS.APP_SAVED, onConfigSavedEvent)
})

/**
 * 保存配置
 * 保留 DLC 列表中的 "appid=Name" 格式发送到后端
 */
async function saveConfig() {
  try {
    const result = await invoke<{ success: boolean; message: string }>('save_app_config', {
      gamePath: props.gamePath,
      config: {
        branchName: config.branchName || 'public',
        isBetaBranch: config.isBetaBranch,
        controller: {
          steamInput: config.controller.steamInput,
          type: config.controller.type
        },
        cloudSaves: {
          enabled: config.cloudSaves.enabled,
          createDefaultDir: config.cloudSaves.createDefaultDir,
          createSpecificDirs: config.cloudSaves.createSpecificDirs
        },
        dlcs: {
          unlockAll: config.dlcs.unlockAll,
          dlcList: normalizeDlcList(config.dlcs.customList)
        }
      }
    })

    if (result.success) {
      showToast.value = true
      setTimeout(() => {
        showToast.value = false
      }, 3000)
      emit('saved')
      // 广播应用配置已保存事件，通知完整配置管理器等其它窗口刷新
      window.dispatchEvent(new CustomEvent(CONFIG_EVENTS.APP_SAVED, {
        detail: { gamePath: props.gamePath }
      }))
    } else {
      alert(`保存失败: ${result.message}`)
    }
  } catch (error) {
    alert(`保存失败: ${error}`)
  }
}

// 暴露方法供父组件调用
defineExpose({
  load: loadAppConfig,
  save: saveConfig
})
</script>

<style scoped>
.app-config-panel {
  position: relative;
}

.config-group {
  margin-bottom: 20px;
}

.config-section {
  margin-bottom: 24px;
}

.config-section h4 {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 14px 0;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--steam-border);
}

.config-label {
  display: block;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin-bottom: 8px;
}

.config-desc {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 0 0 12px 0;
}

.config-input,
select {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 13px;
  outline: none;
}

.config-input:focus,
select:focus {
  border-color: var(--steam-accent-blue);
}

.config-textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 13px;
  font-family: 'Courier New', monospace;
  resize: vertical;
  outline: none;
}

.config-textarea:focus {
  border-color: var(--steam-accent-blue);
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  color: var(--steam-text-primary);
  font-size: 14px;
}

.checkbox-label input[type="checkbox"] {
  width: 18px;
  height: 18px;
  accent-color: var(--steam-accent-blue);
  cursor: pointer;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin-bottom: 8px;
}

.form-row {
  display: flex;
  gap: 16px;
  flex-wrap: wrap;
  margin-bottom: 12px;
}

.dlc-mode-selector {
  display: flex;
  gap: 16px;
  flex-wrap: wrap;
}

.dlc-mode-option {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  color: var(--steam-text-primary);
  font-size: 14px;
}

.dlc-mode-option input[type="radio"] {
  width: 18px;
  height: 18px;
  accent-color: var(--steam-accent-blue);
  cursor: pointer;
}

.dlc-mode-label {
  user-select: none;
}

.btn-primary {
  display: inline-flex;
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
  margin-top: 24px;
  padding-top: 16px;
  border-top: 1px solid var(--steam-border);
  display: flex;
  justify-content: flex-end;
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

.field-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 6px 0 0 0;
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
