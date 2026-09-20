<template>
  <!-- 底部下载条（仿 Steam） -->
  <!-- ① 有下载任务：全宽进度条，显示「正在下载中：中文名 英文名」+ 进度条 + 总进度，点击进入下载列表页 -->
  <div v-if="primary" class="download-bar" @click="goToDownloads">
    <span class="bar-info">
      <span class="bar-prefix">正在下载中：</span>
      <span class="bar-title">{{ displayName }}</span>
    </span>
    <div class="bar-progress-bg">
      <div class="bar-progress-fill" :style="{ width: percent + '%' }" />
    </div>
    <span class="bar-percent">{{ percent }}%</span>
  </div>

  <!-- ② 无下载任务：右下角「下载记录」入口（可自由拖动，位置持久化到 config.json），点击进入下载列表页 -->
  <div
    v-else
    class="record-bar"
    :style="{ right: recordPos.right + 'px', bottom: recordPos.bottom + 'px' }"
    title="点击查看下载记录，按住可拖动调整位置"
    @mousedown="onRecordMouseDown"
    @click="onRecordClick"
  >
    <svg class="record-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
      <polyline points="7 10 12 15 17 10" />
      <line x1="12" y1="15" x2="12" y2="3" />
    </svg>
    <span>下载记录</span>
  </div>
</template>

<script setup lang="ts">
/**
 * DownloadBar.vue - 底部下载条（仿 Steam）
 *
 * - 有下载任务：固定在窗口底部（全宽，高 50px），仅显示「正在下载中：游戏名」+ 总进度条；
 * - 无下载任务：右下角「下载记录」入口（宽 150px），可按住自由拖动，
 *   拖动后的位置（距窗口右/下边缘的 px）持久化到 config.json（recordButton 字段）；
 * - 两种情况点击都会进入 /downloads 下载列表页（详细操作在列表页进行）。
 *
 * 主题：全部使用 main.css 的 CSS 变量，深/浅色自适应。
 */

import { computed, watch, ref, onMounted, onBeforeUnmount } from 'vue'
import { useRouter } from 'vue-router'
import { useDownloadManagerStore } from '../../store/downloadManager.store'
import { useConfigStore } from '../../store/config.store'

const router = useRouter()
const downloadStore = useDownloadManagerStore()
const configStore = useConfigStore()

/** 记录按钮尺寸（px），拖动限位与窗口缩放钳制时使用 */
const RECORD_WIDTH = 150
const RECORD_HEIGHT = 44
/** 标题栏高度（px），与 TitleBar.vue 中 .title-bar 的 height 保持一致，按钮不允许拖入标题栏区域 */
const TITLE_BAR_HEIGHT = 40

/** 「下载记录」按钮位置：距窗口右/下边缘的距离（px），来自 config.json，未配置时用默认 16/16 */
const recordPos = ref({ right: 16, bottom: 16 })

// 配置加载完成后（或后续变更时）同步按钮位置，并钳制到合法范围（不得进入标题栏）
watch(
  () => configStore.config?.recordButton,
  (rb) => {
    if (rb) {
      recordPos.value = { right: rb.right, bottom: rb.bottom }
      clampToWindow()
    }
  },
  { immediate: true }
)

/** 主下载任务（进度最慢的活跃下载，仅用于取名字） */
const primary = computed(() => downloadStore.primaryDownload)

/** 显示名（「中文名 + 英文名」，异步解析回填 store，解析前兜底 gameId） */
const displayName = computed(() => primary.value?.gameName || primary.value?.gameId || '')

/** 展示进度：单任务用主任务进度，多任务用全局加权进度（仅统计正在下载，不含已暂停） */
const percent = computed(() => {
  const count = downloadStore.downloadingCount
  if (count <= 1) {
    return Math.max(0, Math.min(100, Math.round(primary.value?.downloadProgress.overallPercentage ?? 0)))
  }
  return downloadStore.globalPercent
})

// ==================== 名称解析 ====================

function ensureName() {
  const p = primary.value
  if (p) void downloadStore.ensureGameName(p.gameId)
}

watch(primary, ensureName)
onMounted(() => {
  ensureName()
  // 首次解析可能未命中（如游戏配置刚加载/数据尚未落库），短轮询重试直到拿到真实名称
  retryNameUntilResolved()
  // 监听窗口缩放，钳制按钮位置防止跑出屏幕
  window.addEventListener('resize', onWindowResize)
})

/** 名称重试定时器（unmount 清理） */
let nameRetryTimer: ReturnType<typeof setInterval> | null = null

function retryNameUntilResolved() {
  const pid = primary.value?.gameId
  if (!pid) return
  let tries = 0
  nameRetryTimer = window.setInterval(() => {
    tries++
    void downloadStore.ensureGameName(pid).then(() => {
      const st = downloadStore.downloads.get(pid)
      // 已拿到真实名称（不再是纯数字兜底）或达到重试上限则停止
      const resolved = st && st.gameName && st.gameName !== pid && !/^\d+$/.test(st.gameName)
      if (resolved || tries >= 8) {
        if (nameRetryTimer) {
          window.clearInterval(nameRetryTimer)
          nameRetryTimer = null
        }
      }
    })
  }, 800)
}

onBeforeUnmount(() => {
  if (nameRetryTimer) {
    window.clearInterval(nameRetryTimer)
    nameRetryTimer = null
  }
  // 卸载时清理拖拽监听，避免泄漏
  window.removeEventListener('mousemove', onRecordMouseMove)
  window.removeEventListener('mouseup', onRecordMouseUp)
  window.removeEventListener('resize', onWindowResize)
})

// ==================== 交互 ====================

/** 点击 → 跳转下载列表页（拖拽后不触发，避免误跳转） */
function goToDownloads() {
  router.push('/downloads')
}

// ==================== 「下载记录」按钮拖拽 ====================
// 拖动时记录起点（鼠标坐标 + 初始右/下偏移），拖动过程中实时换算新的右/下偏移；
// 松开后把位置保存到 config.json（recordButton），实现「移动并记住位置」。

/** 拖拽起点状态 */
let isDragging = false
let dragged = false
let dragStartX = 0
let dragStartY = 0
let dragStartRight = 0
let dragStartBottom = 0

/** 数值钳制到 [min, max] */
function clampNum(v: number, min: number, max: number) {
  return Math.min(max, Math.max(min, v))
}

/**
 * 计算 bottom 偏移的可移动范围（px）
 * 最小 0（贴窗口底部）；最大受标题栏约束：按钮上边缘不得高于标题栏下边缘
 */
function bottomRange() {
  const maxBottom = Math.max(0, window.innerHeight - TITLE_BAR_HEIGHT - RECORD_HEIGHT)
  return { maxBottom }
}

/** 把当前位置钳制在窗口可见范围内（窗口缩放后避免按钮跑出屏幕或进入标题栏） */
function clampToWindow() {
  const maxRight = Math.max(0, window.innerWidth - RECORD_WIDTH)
  const { maxBottom } = bottomRange()
  recordPos.value = {
    right: clampNum(recordPos.value.right, 0, maxRight),
    bottom: clampNum(recordPos.value.bottom, 0, maxBottom),
  }
}

/** 鼠标按下：记录起点并开始监听移动/松开 */
function onRecordMouseDown(e: MouseEvent) {
  // 仅响应鼠标左键
  if (e.button !== 0) return
  isDragging = true
  dragged = false
  dragStartX = e.clientX
  dragStartY = e.clientY
  dragStartRight = recordPos.value.right
  dragStartBottom = recordPos.value.bottom
  window.addEventListener('mousemove', onRecordMouseMove)
  window.addEventListener('mouseup', onRecordMouseUp)
}

/** 鼠标移动：实时换算右/下偏移并限制在窗口范围内（不得进入标题栏） */
function onRecordMouseMove(e: MouseEvent) {
  if (!isDragging) return
  const dx = e.clientX - dragStartX
  const dy = e.clientY - dragStartY
  // 移动超过阈值视为拖拽（用于抑制点击跳转）
  if (Math.abs(dx) > 3 || Math.abs(dy) > 3) dragged = true

  const maxRight = Math.max(0, window.innerWidth - RECORD_WIDTH)
  const { maxBottom } = bottomRange()
  recordPos.value = {
    right: clampNum(dragStartRight - dx, 0, maxRight),
    bottom: clampNum(dragStartBottom - dy, 0, maxBottom),
  }
}

/** 鼠标松开：结束拖拽，若确实拖动过则把位置保存到 config.json */
function onRecordMouseUp() {
  if (!isDragging) return
  isDragging = false
  window.removeEventListener('mousemove', onRecordMouseMove)
  window.removeEventListener('mouseup', onRecordMouseUp)
  if (dragged) {
    void configStore.updateConfig({
      recordButton: {
        right: Math.round(recordPos.value.right),
        bottom: Math.round(recordPos.value.bottom),
      },
    })
  }
}

/** 窗口缩放：钳制位置，防止按钮被挤出屏幕 */
function onWindowResize() {
  if (!isDragging) clampToWindow()
}

/** 点击处理：拖拽过的松开不触发跳转 */
function onRecordClick() {
  if (dragged) return
  goToDownloads()
}
</script>

<style scoped>
/* ① 有下载：全宽进度条，固定在窗口底部，位于主内容之上、弹窗之下 */
.download-bar {
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  height: 50px;
  display: flex;
  align-items: center;
  gap: 18px;
  padding: 0 22px;
  background: var(--steam-panel-bg, #232b3f);
  border-top: 1px solid var(--steam-panel-border, #37415d);
  box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.25);
  cursor: pointer;
  z-index: 950;
  user-select: none;
}

.download-bar:hover {
  background: var(--steam-bg-tertiary, #303a54);
}

/* 名称区：前缀 + 游戏名 */
.bar-info {
  display: flex;
  align-items: baseline;
  gap: 2px;
  flex-shrink: 0;
  max-width: 55%;
  overflow: hidden;
}

.bar-prefix {
  flex-shrink: 0;
  font-size: 14px;
  color: var(--steam-text-muted, #a6b0c6);
  white-space: nowrap;
}

.bar-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary, #ffffff);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 进度条 */
.bar-progress-bg {
  flex: 1;
  min-width: 60px;
  height: 8px;
  background: var(--steam-bg-primary, #1c2230);
  border-radius: 4px;
  overflow: hidden;
}

.bar-progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--steam-accent-blue, #60a5fa), var(--steam-accent-green, #7cb342));
  border-radius: 4px;
  transition: width 0.25s ease;
}

/* 下载总进度文字（等宽数字，避免跳动） */
.bar-percent {
  flex-shrink: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary, #ffffff);
  font-variant-numeric: tabular-nums;
  white-space: nowrap;
}

/* ② 无下载：右下角「下载记录」入口（宽 150px，半透明卡片风格）
   位置（right/bottom）由 JS 从 config.json 读取并绑定到内联样式，可自由拖动 */
.record-bar {
  position: fixed;
  width: 150px;
  height: 44px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  background: var(--steam-panel-bg, rgba(35, 43, 63, 0.92));
  border: 1px solid var(--steam-panel-border, #37415d);
  border-radius: 22px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.3);
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary, #ffffff);
  cursor: grab;
  z-index: 950;
  user-select: none;
  transition: background-color 0.15s ease, border-color 0.15s ease;
}

.record-bar:active {
  cursor: grabbing;
}

.record-bar:hover {
  background: var(--steam-bg-tertiary, #303a54);
  border-color: var(--steam-border-light, rgba(255, 255, 255, 0.15));
}

.record-icon {
  width: 16px;
  height: 16px;
  color: var(--steam-accent-blue, #60a5fa);
  flex-shrink: 0;
}
</style>