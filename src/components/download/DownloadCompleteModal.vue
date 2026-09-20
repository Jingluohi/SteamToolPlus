<template>
  <!-- 下载完成提示弹窗（全局版）：任意界面下载完成时由 App 层弹出，
       提示前往注入补丁；点击某个补丁跳转到该游戏详情页并自动切到对应补丁标签 -->
  <BaseModal
    :model-value="visible"
    :title="modalTitle"
    width="40%"
    container-class="download-complete-modal"
    :show-confirm="false"
    :show-cancel="true"
    cancel-text="稍后再说"
    @update:model-value="close"
    @close="close"
  >
    <template #body>
      <div class="download-complete-modal-content">
        <p class="modal-main-hint">下载完成，请前往注入补丁，补丁应用成功才能游玩</p>
        <p v-if="installPath" class="modal-path-hint">
          已自动设置安装路径：<code>{{ installPath }}</code>
        </p>
        <p v-if="exePath && exePath !== installPath" class="modal-path-hint">
          已自动定位游戏 exe 路径：<code>{{ exePath }}</code>
        </p>
        <div class="patch-select-section">
          <p class="patch-select-title">补丁选择：</p>
          <div class="patch-select-buttons">
            <button
              v-for="tab in patchTabs"
              :key="tab.id"
              class="patch-select-btn ui-btn ui-btn-md"
              :style="{ backgroundColor: getCategoryColor(tab.patchType) }"
              @click="goToPatchTab(tab)"
            >
              {{ tab.name }}
            </button>
          </div>
        </div>
        <p v-if="patchTabs.length === 0" class="no-patch-hint">
          该游戏暂无补丁配置，请前往游戏库查看。
        </p>
      </div>
    </template>
  </BaseModal>
</template>

<script setup lang="ts">
/**
 * DownloadCompleteModal.vue - 全局「下载完成：提示打补丁」弹窗
 *
 * 由 App.vue 监听 depot-download-finished(success) 后按 gameId 唤起（visible=true）。
 * 弹窗内自行加载：
 * - 游戏库配置（games_config.json）→ 该游戏的补丁标签列表（无补丁则只提示）；
 * - game.json（get_game_data）→ 已设置的安装路径 / exe 路径。
 * 点击补丁按钮 → 跳转 /game/{id}?patch={tabId}，GameDetail 据此自动切到对应补丁标签。
 */

import { computed, ref, watch, onBeforeUnmount } from 'vue'
import { useRouter } from 'vue-router'
import BaseModal from '../common/BaseModal.vue'
import { loadGamesConfigFromFile } from '../../api/game.api'
import { getGameData } from '../../api/gameData.api'
import type { GameConfigData, GameTagConfig } from '../../types/game.types'
import { getCategoryName, getCategoryColor } from '../../constants/game'
import { getPatchSourcePath } from '../../types'
import { resolveGameName } from '../../store/downloadManager.store'

/** 弹窗显隐（v-model，由 App 层控制） */
const props = defineProps<{
  modelValue: boolean
  /** 刚下载完成的游戏 ID */
  gameId: string
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  /** 弹窗关闭（内部取消按钮） */
  (e: 'close'): void
}>()

const visible = computed({
  get: () => props.modelValue,
  set: (v: boolean) => emit('update:modelValue', v)
})

const router = useRouter()

/** 游戏库配置（懒加载一次，供查询 tags） */
let gamesConfigCache: GameConfigData[] | null = null

/** 当前游戏（来自游戏库配置） */
const game = ref<GameConfigData | null>(null)

/** 游戏显示名（中文 + 英文，异步解析） */
const gameName = ref('')

/** 弹窗标题：标明是哪个游戏下载完成 */
const modalTitle = computed(() => {
  const name = gameName.value || props.gameId
  return `下载完成 - ${name}`
})

/** 已设置的安装路径 / exe 目录（来自 game.json） */
const installPath = ref('')
const exePath = ref('')

/** 补丁标签列表（与游戏详情页口径一致：tags → patch 标签） */
const patchTabs = computed(() => {
  const g = game.value
  if (!g?.tags) return []
  return g.tags.map((tag: GameTagConfig) => ({
    id: `patch-${tag.patch_type}`,
    name: getCategoryName(tag.patch_type),
    patchType: tag.patch_type,
    patchPath: getPatchSourcePath(tag, g.game_id),
    downloadUrls: tag.download_urls || []
  }))
})

/** 弹窗打开时加载游戏数据（名称 / 配置 + game.json 路径），关闭时清理 */
watch(
  () => props.modelValue,
  async (open) => {
    if (!open || !props.gameId) return
    gameName.value = ''
    // 异步解析显示名（中文 + 英文），解析失败时标题回退为 gameId
    resolveGameName(props.gameId)
      .then((name) => {
        gameName.value = name
      })
      .catch(() => {
        gameName.value = ''
      })
    loadPaths()
  }
)

/** 加载安装路径与 exe 路径，并确保游戏库配置就绪（供 patchTabs 计算） */
async function loadPaths() {
  try {
    // 1. game.json：安装路径 / exe
    const data = await getGameData(props.gameId)
    if (data) {
      installPath.value = data.install_path || ''
      const exe = data.exe_path || ''
      exePath.value = exe ? exe.substring(0, exe.lastIndexOf('\\')) : ''
    }
  } catch {
    // 读取失败保持为空
  }
  // 2. 游戏库配置（懒加载一次）
  if (!gamesConfigCache) {
    try {
      gamesConfigCache = await loadGamesConfigFromFile()
      // 同步由 App 层经 gameId 变化触发的最近一次 game 查询
      const g = gamesConfigCache.find((x) => x.game_id === props.gameId) || null
      if (g) game.value = g
    } catch {
      gamesConfigCache = []
    }
  }
  const g = gamesConfigCache.find((x) => x.game_id === props.gameId) || null
  if (g) game.value = g
}

/** 跳转该补丁标签：先关闭弹窗，等关闭动画结束后再跳转，避免与详情页弹窗短暂同屏 */
function goToPatchTab(tab: { id: string }) {
  close()
  // 等待 BaseModal 的 0.2s 关闭动画完成后再跳转，
  // 否则详情页的补丁提示弹窗会与正在关闭的本弹窗重叠显示
  setTimeout(() => {
    router.push({ path: `/game/${props.gameId}`, query: { patch: tab.id } })
  }, 250)
}

/** 关闭弹窗（取消/覆盖点击） */
function close() {
  emit('update:modelValue', false)
}

onBeforeUnmount(() => {
  // 无额外资源需要释放
})
</script>

<style scoped>
/* 弹窗内容布局（与原游戏详情页弹窗一致） */
.download-complete-modal-content {
  display: flex;
  flex-direction: column;
  gap: 10px;
  justify-content: flex-start;
}

.modal-main-hint {
  margin: 0;
  font-size: 15px;
  color: var(--steam-text-primary);
  line-height: 1.3;
}

.modal-path-hint {
  margin: 0;
  font-size: 13px;
  color: var(--steam-text-secondary);
  word-break: break-all;
}

.modal-path-hint code {
  padding: 2px 6px;
  background-color: var(--steam-bg-tertiary);
  border-radius: 4px;
  font-family: 'Courier New', monospace;
  word-break: break-all;
}

.patch-select-section {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.patch-select-title {
  margin: 0;
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

.patch-select-buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  width: 100%;
}

.patch-select-btn {
  flex: 1 1 calc(50% - 3px);
  min-width: 100px;
  color: #fff;
  overflow: hidden;
  text-overflow: ellipsis;
}

.no-patch-hint {
  margin: 0;
  font-size: 13px;
  color: var(--steam-text-muted);
}

/* 弹窗宽度约束（BaseModal container-class 全局生效） */
:global(.download-complete-modal) {
  width: 40% !important;
  max-width: 40% !important;
  height: auto !important;
  max-height: 50vh !important;
}
</style>