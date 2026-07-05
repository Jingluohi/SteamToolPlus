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

        <!-- 配置内容区：全部使用共享 Panel -->
        <div class="config-content">
          <div v-if="activeTab === 'items'" class="tab-panel">
            <ItemsConfigPanel
              ref="itemsConfigPanelRef"
              :game-path="props.gamePath"
              @saved="handleItemsSaved"
            />
          </div>

          <div v-if="activeTab === 'mods'" class="tab-panel">
            <ModsConfigPanel
              ref="modsConfigPanelRef"
              :game-path="props.gamePath"
              @saved="handleModsSaved"
            />
          </div>

          <div v-if="activeTab === 'leaderboards'" class="tab-panel">
            <LeaderboardsConfigPanel
              ref="leaderboardsConfigPanelRef"
              :game-path="props.gamePath"
              @saved="handleLeaderboardsSaved"
            />
          </div>

          <div v-if="activeTab === 'controller'" class="tab-panel">
            <ControllerConfigPanel
              ref="controllerConfigPanelRef"
              :game-path="props.gamePath"
              @saved="handleControllerSaved"
            />
          </div>

          <div v-if="activeTab === 'other'" class="tab-panel">
            <OtherConfigPanel
              ref="otherConfigPanelRef"
              :game-path="props.gamePath"
              @saved="handleOtherSaved"
            />
          </div>

          <div v-if="activeTab === 'coldclient'" class="tab-panel">
            <ColdClientLoaderConfigPanel
              ref="coldClientLoaderConfigPanelRef"
              :game-path="props.gamePath"
              @saved="handleColdClientSaved"
            />
          </div>

          <div v-if="activeTab === 'lobby'" class="tab-panel">
            <LobbyConnectConfigPanel
              ref="lobbyConnectConfigPanelRef"
              :game-path="props.gamePath"
              @saved="handleLobbySaved"
            />
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
/**
 * AdvancedConfig.vue - 高级配置管理
 * 现在作为各共享 Panel 的容器，所有具体逻辑复用 Panel 组件
 * 实现「完整配置管理器」与单独窗口共用同一套数据模型和保存/加载逻辑
 */

import { ref, onMounted, onUnmounted } from 'vue'
import { CONFIG_EVENTS } from '../../constants/config-events'
import ItemsConfigPanel from './panels/ItemsConfigPanel.vue'
import ModsConfigPanel from './panels/ModsConfigPanel.vue'
import LeaderboardsConfigPanel from './panels/LeaderboardsConfigPanel.vue'
import ControllerConfigPanel from './panels/ControllerConfigPanel.vue'
import ColdClientLoaderConfigPanel from './panels/ColdClientLoaderConfigPanel.vue'
import LobbyConnectConfigPanel from './panels/LobbyConnectConfigPanel.vue'
import OtherConfigPanel from './panels/OtherConfigPanel.vue'

const props = defineProps<{
  gamePath: string
  gameId: string
}>()

const emit = defineEmits<{
  close: []
  saved: []
}>()

const activeTab = ref('items')

// Panel 组件引用
const itemsConfigPanelRef = ref<InstanceType<typeof ItemsConfigPanel> | null>(null)
const modsConfigPanelRef = ref<InstanceType<typeof ModsConfigPanel> | null>(null)
const leaderboardsConfigPanelRef = ref<InstanceType<typeof LeaderboardsConfigPanel> | null>(null)
const controllerConfigPanelRef = ref<InstanceType<typeof ControllerConfigPanel> | null>(null)
const coldClientLoaderConfigPanelRef = ref<InstanceType<typeof ColdClientLoaderConfigPanel> | null>(null)
const lobbyConnectConfigPanelRef = ref<InstanceType<typeof LobbyConnectConfigPanel> | null>(null)
const otherConfigPanelRef = ref<InstanceType<typeof OtherConfigPanel> | null>(null)

const tabs = [
  { id: 'items', name: '物品', icon: '<rect x="3" y="3" width="18" height="18" rx="2"/><path d="M12 8v8m-4-4h8"/>' },
  { id: 'mods', name: '模组', icon: '<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><polyline points="3.27 6.96 12 12.01 20.73 6.96"/><line x1="12" y1="22.08" x2="12" y2="12"/>' },
  { id: 'leaderboards', name: '排行榜', icon: '<path d="M6 9H4.5a2.5 2.5 0 0 1 0-5H6"/><path d="M18 9h1.5a2.5 2.5 0 0 0 0-5H18"/><path d="M4 22h16"/><path d="M10 14.66V17c0 .55-.47.98-.97 1.21C7.85 18.75 7 20.24 7 22"/><path d="M14 14.66V17c0 .55.47.98.97 1.21C16.15 18.75 17 20.24 17 22"/><path d="M18 2H6v7a6 6 0 0 0 12 0V2Z"/>' },
  { id: 'controller', name: '控制器', icon: '<rect x="2" y="8" width="20" height="12" rx="2"/><path d="M6 12h.01"/><path d="M18 12h.01"/><path d="M8 12h.01"/><path d="M16 12h.01"/>' },
  { id: 'other', name: '其他', icon: '<circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/>' },
  { id: 'coldclient', name: 'ColdClient', icon: '<path d="m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z"/>' },
  { id: 'lobby', name: 'Lobby', icon: '<path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/>' },
]

// 各 Panel 保存后的处理（Panel 自己会显示提示并广播事件）
function handleItemsSaved() { /* Panel 已处理 */ }
function handleModsSaved() { /* Panel 已处理 */ }
function handleLeaderboardsSaved() { /* Panel 已处理 */ }
function handleControllerSaved() { /* Panel 已处理 */ }
function handleColdClientSaved() { /* Panel 已处理 */ }
function handleLobbySaved() { /* Panel 已处理 */ }
function handleOtherSaved() { /* Panel 已处理 */ }

/**
 * 保存所有配置
 * 并发调用所有 Panel 的 save()，由 Panel 自行广播同步事件
 */
async function saveAllConfigs() {
  try {
    const promises = [
      itemsConfigPanelRef.value?.save(),
      modsConfigPanelRef.value?.save(),
      leaderboardsConfigPanelRef.value?.save(),
      controllerConfigPanelRef.value?.save(),
      coldClientLoaderConfigPanelRef.value?.save(),
      lobbyConnectConfigPanelRef.value?.save(),
      otherConfigPanelRef.value?.save(),
    ].filter(Boolean) as Promise<any>[]

    await Promise.all(promises)

    emit('saved')
    emit('close')
  } catch (error) {
    alert(`保存失败: ${error}`)
  }
}

// 同步事件处理器：当其它窗口保存配置且 gamePath 匹配时，刷新对应 Panel
function createSyncHandler(panelRef: { value: { load: () => void | Promise<void> } | null }) {
  return (event: Event) => {
    const customEvent = event as CustomEvent<{ gamePath?: string }>
    if (customEvent.detail?.gamePath === props.gamePath) {
      panelRef.value?.load()
    }
  }
}

const handlers: { event: string; handler: (e: Event) => void }[] = [
  { event: CONFIG_EVENTS.ITEMS_SAVED, handler: createSyncHandler(itemsConfigPanelRef) },
  { event: CONFIG_EVENTS.MODS_SAVED, handler: createSyncHandler(modsConfigPanelRef) },
  { event: CONFIG_EVENTS.LEADERBOARDS_SAVED, handler: createSyncHandler(leaderboardsConfigPanelRef) },
  { event: CONFIG_EVENTS.CONTROLLER_SAVED, handler: createSyncHandler(controllerConfigPanelRef) },
  { event: CONFIG_EVENTS.COLDCLIENT_SAVED, handler: createSyncHandler(coldClientLoaderConfigPanelRef) },
  { event: CONFIG_EVENTS.LOBBY_SAVED, handler: createSyncHandler(lobbyConnectConfigPanelRef) },
  { event: CONFIG_EVENTS.OTHER_SAVED, handler: createSyncHandler(otherConfigPanelRef) },
]

onMounted(() => {
  handlers.forEach(({ event, handler }) => {
    window.addEventListener(event, handler)
  })
})

onUnmounted(() => {
  handlers.forEach(({ event, handler }) => {
    window.removeEventListener(event, handler)
  })
})
</script>

<style scoped>
.modal-overlay {
  backdrop-filter: var(--config-modal-backdrop);
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
  background-color: var(--config-modal-bg);
  backdrop-filter: var(--config-modal-backdrop);
  border-radius: 12px;
  border: 1px solid var(--config-modal-border);
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

/* 按钮 */
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

@media (max-width: 768px) {
  .config-tabs {
    flex-wrap: wrap;
  }
}
</style>
