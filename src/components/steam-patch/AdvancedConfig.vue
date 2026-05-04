<template>
  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-content advanced-config" @click.stop>
      <div class="modal-header">
        <div class="header-icon advanced">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="3"/>
            <path d="M12 1v6m0 6v6m4.22-10.22l4.24-4.24M6.34 17.66l-4.24 4.24M23 12h-6m-6 0H1m20.24 4.24l-4.24-4.24M6.34 6.34L2.1 2.1"/>
          </svg>
        </div>
        <h3>高级配置管理</h3>
        <button class="close-btn" @click="$emit('close')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <div class="modal-body">
        <!-- 配置标签页 -->
        <div class="config-tabs">
          <button 
            v-for="tab in tabs" 
            :key="tab.id"
            class="tab-btn"
            :class="{ active: activeTab === tab.id }"
            @click="activeTab = tab.id"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" v-html="tab.icon"/>
            <span>{{ tab.name }}</span>
          </button>
        </div>

        <!-- 配置内容区 -->
        <div class="config-content">
          <!-- 物品配置 -->
          <div v-if="activeTab === 'items'" class="tab-panel">
            <div class="panel-header">
              <label class="toggle-label">
                <input v-model="configs.items.enabled" type="checkbox" class="toggle-input" />
                <span class="toggle-slider"></span>
                <span class="toggle-text">启用物品系统</span>
              </label>
            </div>
            <div v-if="configs.items.enabled" class="panel-body">
              <div class="section-header">
                <h4>物品定义</h4>
                <button class="btn-add" @click="addItem">添加物品</button>
              </div>
              <div class="list-container">
                <div v-for="(item, index) in configs.items.itemDefinitions" :key="index" class="list-item">
                  <input v-model="item.id" placeholder="物品ID" />
                  <input v-model="item.name" placeholder="物品名称" />
                  <input v-model="item.maxStackSize" type="number" placeholder="最大堆叠" />
                  <button class="btn-icon" @click="removeItem(index)">×</button>
                </div>
              </div>
            </div>
          </div>

          <!-- 模组配置 -->
          <div v-if="activeTab === 'mods'" class="tab-panel">
            <div class="panel-header">
              <label class="toggle-label">
                <input v-model="configs.mods.enabled" type="checkbox" class="toggle-input" />
                <span class="toggle-slider"></span>
                <span class="toggle-text">启用创意工坊模组</span>
              </label>
            </div>
            <div v-if="configs.mods.enabled" class="panel-body">
              <div class="section-header">
                <h4>已订阅模组</h4>
                <button class="btn-add" @click="addMod">添加模组</button>
              </div>
              <div class="list-container">
                <div v-for="(mod, index) in configs.mods.subscribedMods" :key="index" class="list-item">
                  <input v-model="mod.publishedFileId" placeholder="模组ID" />
                  <input v-model="mod.title" placeholder="模组标题" />
                  <select v-model="mod.visibility">
                    <option value="public">公开</option>
                    <option value="friends">好友</option>
                    <option value="private">私有</option>
                  </select>
                  <button class="btn-icon" @click="removeMod(index)">×</button>
                </div>
              </div>
            </div>
          </div>

          <!-- 排行榜配置 -->
          <div v-if="activeTab === 'leaderboards'" class="tab-panel">
            <div class="panel-header">
              <label class="toggle-label">
                <input v-model="configs.leaderboards.enabled" type="checkbox" class="toggle-input" />
                <span class="toggle-slider"></span>
                <span class="toggle-text">启用排行榜</span>
              </label>
            </div>
            <div v-if="configs.leaderboards.enabled" class="panel-body">
              <div class="section-header">
                <h4>排行榜列表</h4>
                <button class="btn-add" @click="addLeaderboard">添加排行榜</button>
              </div>
              <div class="list-container">
                <div v-for="(lb, index) in configs.leaderboards.leaderboards" :key="index" class="list-item">
                  <input v-model="lb.name" placeholder="排行榜ID" />
                  <input v-model="lb.displayName" placeholder="显示名称" />
                  <select v-model="lb.sortMethod">
                    <option value="asc">升序</option>
                    <option value="desc">降序</option>
                  </select>
                  <button class="btn-icon" @click="removeLeaderboard(index)">×</button>
                </div>
              </div>
            </div>
          </div>

          <!-- 控制器配置 -->
          <div v-if="activeTab === 'controller'" class="tab-panel">
            <div class="panel-header">
              <label class="toggle-label">
                <input v-model="configs.controller.enabled" type="checkbox" class="toggle-input" />
                <span class="toggle-slider"></span>
                <span class="toggle-text">启用控制器支持</span>
              </label>
            </div>
            <div v-if="configs.controller.enabled" class="panel-body">
              <div class="form-row">
                <div class="form-group">
                  <label>控制器类型</label>
                  <select v-model="configs.controller.controllerType">
                    <option value="xbox">Xbox</option>
                    <option value="playstation">PlayStation</option>
                    <option value="nintendo">Nintendo</option>
                    <option value="generic">通用</option>
                  </select>
                </div>
              </div>
              <div class="form-row">
                <div class="form-group">
                  <label>左摇杆死区</label>
                  <input v-model.number="configs.controller.deadzone.leftStick" type="number" step="0.01" min="0" max="1" />
                </div>
                <div class="form-group">
                  <label>右摇杆死区</label>
                  <input v-model.number="configs.controller.deadzone.rightStick" type="number" step="0.01" min="0" max="1" />
                </div>
              </div>
              <div class="form-row">
                <div class="form-group">
                  <label>震动强度</label>
                  <input v-model.number="configs.controller.rumble.intensity" type="number" step="0.1" min="0" max="1" />
                </div>
                <div class="form-group">
                  <label class="checkbox-label">
                    <input v-model="configs.controller.rumble.enabled" type="checkbox" />
                    <span>启用震动</span>
                  </label>
                </div>
              </div>
            </div>
          </div>

          <!-- 其他配置 -->
          <div v-if="activeTab === 'other'" class="tab-panel">
            <div class="panel-body">
              <!-- 已安装应用ID -->
              <div class="config-section">
                <h4>已安装应用ID</h4>
                <textarea 
                  v-model="otherConfigs.installedAppIds" 
                  rows="3"
                  placeholder="每行一个AppID"
                ></textarea>
              </div>

              <!-- 订阅群组 -->
              <div class="config-section">
                <h4>订阅群组ID</h4>
                <textarea 
                  v-model="otherConfigs.subscribedGroups" 
                  rows="3"
                  placeholder="每行一个群组ID"
                ></textarea>
              </div>

              <!-- 购买密钥 -->
              <div class="config-section">
                <h4>CD密钥</h4>
                <textarea 
                  v-model="otherConfigs.purchasedKeys" 
                  rows="3"
                  placeholder="每行一个密钥"
                ></textarea>
              </div>

              <!-- 支持语言 -->
              <div class="config-section">
                <h4>支持语言</h4>
                <div class="checkbox-group">
                  <label v-for="lang in availableLanguages" :key="lang.code" class="checkbox-label">
                    <input 
                      type="checkbox" 
                      :value="lang.code"
                      v-model="selectedLanguages"
                    />
                    <span>{{ lang.name }}</span>
                  </label>
                </div>
              </div>
            </div>
          </div>

          <!-- ColdClientLoader 配置 -->
          <div v-if="activeTab === 'coldclient'" class="tab-panel">
            <div class="panel-header">
              <label class="toggle-label">
                <input v-model="configs.coldClientLoader.enabled" type="checkbox" class="toggle-input" />
                <span class="toggle-slider"></span>
                <span class="toggle-text">启用 ColdClientLoader</span>
              </label>
            </div>
            <div v-if="configs.coldClientLoader.enabled" class="panel-body">
              <div class="form-row">
                <div class="form-group">
                  <label>注入模式</label>
                  <select v-model="configs.coldClientLoader.injectionMode">
                    <option value="direct">直接注入</option>
                    <option value="loader">使用加载器</option>
                  </select>
                </div>
              </div>
              <div class="form-group">
                <label>启动参数</label>
                <input v-model="configs.coldClientLoader.launchArgs" placeholder="游戏启动参数" />
              </div>
              <div class="form-group">
                <label>额外DLL列表（每行一个）</label>
                <textarea 
                  v-model="coldClientDlls" 
                  rows="4"
                  placeholder="例如：extra.dll&#10;plugin.dll"
                ></textarea>
              </div>
            </div>
          </div>

          <!-- Lobby Connect 配置 -->
          <div v-if="activeTab === 'lobby'" class="tab-panel">
            <div class="panel-header">
              <label class="toggle-label">
                <input v-model="configs.lobbyConnect.enabled" type="checkbox" class="toggle-input" />
                <span class="toggle-slider"></span>
                <span class="toggle-text">启用 Lobby Connect</span>
              </label>
            </div>
            <div v-if="configs.lobbyConnect.enabled" class="panel-body">
              <div class="form-row">
                <div class="form-group">
                  <label class="checkbox-label">
                    <input v-model="configs.lobbyConnect.autoJoin" type="checkbox" />
                    <span>自动加入大厅</span>
                  </label>
                </div>
              </div>
              <div class="form-group">
                <label>目标大厅ID</label>
                <input v-model="configs.lobbyConnect.targetLobbyId" placeholder="输入大厅ID" />
              </div>
              <div class="form-group">
                <label>连接密码（可选）</label>
                <input v-model="configs.lobbyConnect.password" type="password" placeholder="大厅密码" />
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn-secondary" @click="$emit('close')">取消</button>
        <button class="btn-primary" @click="saveAllConfigs">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="20 6 9 17 4 12"/>
          </svg>
          保存所有配置
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { 
  ItemsConfig, 
  ModsConfig, 
  LeaderboardsConfig, 
  ControllerConfig,
  ColdClientLoaderConfig,
  LobbyConnectConfig,
  ItemDefinition,
  WorkshopMod,
  Leaderboard
} from '../../../src/types/steam-config.types'

const props = defineProps<{
  gamePath: string
  gameId: string
}>()

const emit = defineEmits<{
  close: []
  saved: []
}>()

const activeTab = ref('items')

const tabs = [
  { id: 'items', name: '物品', icon: '<rect x="3" y="3" width="18" height="18" rx="2"/><path d="M12 8v8m-4-4h8"/>' },
  { id: 'mods', name: '模组', icon: '<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><polyline points="3.27 6.96 12 12.01 20.73 6.96"/><line x1="12" y1="22.08" x2="12" y2="12"/>' },
  { id: 'leaderboards', name: '排行榜', icon: '<path d="M6 9H4.5a2.5 2.5 0 0 1 0-5H6"/><path d="M18 9h1.5a2.5 2.5 0 0 0 0-5H18"/><path d="M4 22h16"/><path d="M10 14.66V17c0 .55-.47.98-.97 1.21C7.85 18.75 7 20.24 7 22"/><path d="M14 14.66V17c0 .55.47.98.97 1.21C16.15 18.75 17 20.24 17 22"/><path d="M18 2H6v7a6 6 0 0 0 12 0V2Z"/>' },
  { id: 'controller', name: '控制器', icon: '<rect x="2" y="8" width="20" height="12" rx="2"/><path d="M6 12h.01"/><path d="M18 12h.01"/><path d="M8 12h.01"/><path d="M16 12h.01"/>' },
  { id: 'other', name: '其他', icon: '<circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/>' },
  { id: 'coldclient', name: 'ColdClient', icon: '<path d="m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z"/>' },
  { id: 'lobby', name: 'Lobby', icon: '<path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/>' },
]

// 配置数据
const configs = ref({
  items: { enabled: false, itemDefinitions: [], initialItems: [] } as ItemsConfig,
  mods: { enabled: false, subscribedMods: [], autoUpdate: false } as ModsConfig,
  leaderboards: { enabled: false, leaderboards: [] } as LeaderboardsConfig,
  controller: {
    enabled: false,
    controllerType: 'xbox',
    bindings: [],
    deadzone: { leftStick: 0.1, rightStick: 0.1, leftTrigger: 0.1, rightTrigger: 0.1 },
    rumble: { enabled: true, intensity: 0.8 },
    customGlyphs: { enabled: false }
  } as ControllerConfig,
  coldClientLoader: { enabled: false, injectionMode: 'direct', extraDlls: [], launchArgs: '' } as ColdClientLoaderConfig,
  lobbyConnect: { enabled: false, autoJoin: false, targetLobbyId: '', password: '' } as LobbyConnectConfig,
})

// 其他配置
const otherConfigs = ref({
  installedAppIds: '',
  subscribedGroups: '',
  purchasedKeys: '',
})

const selectedLanguages = ref<string[]>([])

const coldClientDlls = computed({
  get: () => configs.value.coldClientLoader.extraDlls.join('\n'),
  set: (val: string) => {
    configs.value.coldClientLoader.extraDlls = val.split('\n').filter(s => s.trim())
  }
})

const availableLanguages = [
  { code: 'schinese', name: '简体中文' },
  { code: 'tchinese', name: '繁体中文' },
  { code: 'english', name: '英语' },
  { code: 'japanese', name: '日语' },
  { code: 'korean', name: '韩语' },
  { code: 'russian', name: '俄语' },
  { code: 'german', name: '德语' },
  { code: 'french', name: '法语' },
  { code: 'spanish', name: '西班牙语' },
  { code: 'portuguese', name: '葡萄牙语' },
  { code: 'polish', name: '波兰语' },
  { code: 'turkish', name: '土耳其语' },
]

// 物品操作
function addItem() {
  configs.value.items.itemDefinitions.push({
    id: '',
    name: '',
    stackable: true,
    maxStackSize: 99
  } as ItemDefinition)
}

function removeItem(index: number) {
  configs.value.items.itemDefinitions.splice(index, 1)
}

// 模组操作
function addMod() {
  configs.value.mods.subscribedMods.push({
    publishedFileId: '',
    title: '',
    visibility: 'public',
    files: []
  } as WorkshopMod)
}

function removeMod(index: number) {
  configs.value.mods.subscribedMods.splice(index, 1)
}

// 排行榜操作
function addLeaderboard() {
  configs.value.leaderboards.leaderboards.push({
    name: '',
    displayName: '',
    sortMethod: 'desc',
    displayType: 'numeric',
    entries: []
  } as Leaderboard)
}

function removeLeaderboard(index: number) {
  configs.value.leaderboards.leaderboards.splice(index, 1)
}

// 保存所有配置
async function saveAllConfigs() {
  try {
    // 保存各个配置
    const promises = []

    if (configs.value.items.enabled) {
      promises.push(invoke('save_items_config', {
        gamePath: props.gamePath,
        config: configs.value.items
      }))
    }

    if (configs.value.mods.enabled) {
      promises.push(invoke('save_mods_config', {
        gamePath: props.gamePath,
        config: configs.value.mods
      }))
    }

    if (configs.value.leaderboards.enabled) {
      promises.push(invoke('save_leaderboards_config', {
        gamePath: props.gamePath,
        config: configs.value.leaderboards
      }))
    }

    promises.push(invoke('save_controller_config', {
      gamePath: props.gamePath,
      config: configs.value.controller
    }))

    // 保存其他配置
    promises.push(saveOtherConfigs())

    await Promise.all(promises)

    emit('saved')
    emit('close')
  } catch (error) {
    alert(`保存失败: ${error}`)
  }
}

async function saveOtherConfigs() {
  // 保存 installed_app_ids.txt
  const appIds = otherConfigs.value.installedAppIds
    .split('\n')
    .map(s => s.trim())
    .filter(s => s)
  if (appIds.length > 0) {
    await invoke('save_installed_app_ids', {
      gamePath: props.gamePath,
      appIds
    })
  }

  // 保存 subscribed_groups.txt
  const groupIds = otherConfigs.value.subscribedGroups
    .split('\n')
    .map(s => s.trim())
    .filter(s => s)
  if (groupIds.length > 0) {
    await invoke('save_subscribed_groups', {
      gamePath: props.gamePath,
      groupIds
    })
  }

  // 保存 purchased_keys.txt (格式: appid=KEY)
  const keys = otherConfigs.value.purchasedKeys
    .split('\n')
    .map(s => s.trim())
    .filter(s => s && !s.startsWith('#'))
  if (keys.length > 0) {
    await invoke('save_purchased_keys', {
      gamePath: props.gamePath,
      keys
    })
  }

  // 保存 supported_languages.txt
  if (selectedLanguages.value.length > 0) {
    await invoke('save_supported_languages', {
      gamePath: props.gamePath,
      languages: selectedLanguages.value
    })
  }
}

async function loadConfigs() {
  try {
    // 加载各个配置
    const [itemsResult, modsResult, leaderboardsResult, controllerResult] = await Promise.all([
      invoke<{ exists: boolean; config?: ItemsConfig }>('load_items_config', { gamePath: props.gamePath }),
      invoke<{ exists: boolean; config?: ModsConfig }>('load_mods_config', { gamePath: props.gamePath }),
      invoke<{ exists: boolean; config?: LeaderboardsConfig }>('load_leaderboards_config', { gamePath: props.gamePath }),
      invoke<{ exists: boolean; config?: ControllerConfig }>('load_controller_config', { gamePath: props.gamePath }),
    ])

    if (itemsResult.exists && itemsResult.config) {
      configs.value.items = itemsResult.config
    }
    if (modsResult.exists && modsResult.config) {
      configs.value.mods = modsResult.config
    }
    if (leaderboardsResult.exists && leaderboardsResult.config) {
      configs.value.leaderboards = leaderboardsResult.config
    }
    if (controllerResult.exists && controllerResult.config) {
      configs.value.controller = controllerResult.config
    }

    // 加载其他配置
    await loadOtherConfigs()
  } catch (error) {
    // 加载失败时使用默认值
  }
}

async function loadOtherConfigs() {
  try {
    // 加载 installed_app_ids.txt
    const appIds = await invoke<string[]>('load_installed_app_ids', { gamePath: props.gamePath })
    if (appIds && appIds.length > 0) {
      otherConfigs.value.installedAppIds = appIds.join('\n')
    }

    // 加载 subscribed_groups.txt
    const groupIds = await invoke<string[]>('load_subscribed_groups', { gamePath: props.gamePath })
    if (groupIds && groupIds.length > 0) {
      otherConfigs.value.subscribedGroups = groupIds.join('\n')
    }

    // 加载 purchased_keys.txt
    const keys = await invoke<string[]>('load_purchased_keys', { gamePath: props.gamePath })
    if (keys && keys.length > 0) {
      otherConfigs.value.purchasedKeys = keys.join('\n')
    }

    // 加载 supported_languages.txt
    const languages = await invoke<string[]>('load_supported_languages', { gamePath: props.gamePath })
    if (languages && languages.length > 0) {
      selectedLanguages.value = languages
    }
  } catch (error) {
    // 加载失败时使用默认值
  }
}

onMounted(() => {
  loadConfigs()
})
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background-color: var(--steam-bg-primary);
  border-radius: 12px;
  border: 1px solid var(--steam-border);
  width: 90%;
  max-width: 900px;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
}

.modal-content.advanced-config {
  max-width: 1000px;
}

.modal-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 20px;
  border-bottom: 1px solid var(--steam-border);
  flex-shrink: 0;
}

.header-icon {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.header-icon.advanced {
  background-color: rgba(139, 92, 246, 0.1);
  color: #8b5cf6;
}

.header-icon svg {
  width: 24px;
  height: 24px;
}

.modal-header h3 {
  flex: 1;
  font-size: 18px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0;
}

.close-btn {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 8px;
  background-color: transparent;
  color: var(--steam-text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.close-btn:hover {
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
}

.close-btn svg {
  width: 18px;
  height: 18px;
}

.modal-body {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid var(--steam-border);
  flex-shrink: 0;
}

/* 标签页 */
.config-tabs {
  display: flex;
  gap: 4px;
  padding: 12px 16px;
  border-bottom: 1px solid var(--steam-border);
  overflow-x: auto;
  flex-shrink: 0;
}

.tab-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  border: none;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  background-color: transparent;
  color: var(--steam-text-secondary);
  white-space: nowrap;
}

.tab-btn:hover {
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
}

.tab-btn.active {
  background-color: var(--steam-accent-blue);
  color: white;
}

.tab-btn svg {
  width: 16px;
  height: 16px;
}

/* 配置内容 */
.config-content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.tab-panel {
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

.panel-header {
  margin-bottom: 20px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--steam-border);
}

.panel-body {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.section-header h4 {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0;
}

/* 开关样式 */
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
  font-weight: 500;
  color: var(--steam-text-primary);
}

/* 按钮 */
.btn-add {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  background-color: var(--steam-accent-blue);
  color: white;
}

.btn-add:hover {
  background-color: var(--steam-accent-hover);
}

.btn-icon {
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 6px;
  background-color: transparent;
  color: var(--steam-text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
  font-size: 18px;
}

.btn-icon:hover {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
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

.btn-secondary {
  padding: 10px 20px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
}

.btn-secondary:hover {
  background-color: var(--steam-border);
}

/* 列表容器 */
.list-container {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.list-item {
  display: flex;
  gap: 8px;
  align-items: center;
  padding: 12px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--steam-border);
}

.list-item input,
.list-item select {
  flex: 1;
  padding: 8px 10px;
  border: 1px solid var(--steam-border);
  border-radius: 6px;
  background-color: var(--steam-bg-primary);
  color: var(--steam-text-primary);
  font-size: 13px;
  outline: none;
}

.list-item input:focus,
.list-item select:focus {
  border-color: var(--steam-accent-blue);
}

/* 表单 */
.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-group label {
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

.form-group input,
.form-group select,
.form-group textarea {
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color 0.15s ease;
}

.form-group input:focus,
.form-group select:focus,
.form-group textarea:focus {
  border-color: var(--steam-accent-blue);
}

.form-group textarea {
  resize: vertical;
  font-family: inherit;
}

/* 复选框组 */
.checkbox-group {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 13px;
  color: var(--steam-text-primary);
}

.checkbox-label input[type="checkbox"] {
  width: 16px;
  height: 16px;
  accent-color: var(--steam-accent-blue);
}

/* 配置区域 */
.config-section {
  padding: 16px;
  background-color: var(--steam-bg-secondary);
  border-radius: 10px;
  border: 1px solid var(--steam-border);
}

.config-section h4 {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 12px 0;
}

@media (max-width: 768px) {
  .config-tabs {
    flex-wrap: wrap;
  }
  
  .form-row {
    grid-template-columns: 1fr;
  }
  
  .checkbox-group {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .list-item {
    flex-direction: column;
    align-items: stretch;
  }
}
</style>
