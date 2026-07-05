<template>
  <div class="overlay-config-panel">
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
          <span class="guide-value">configs.overlay.ini</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">启用字段</span>
          <span class="guide-value">enable_experimental_overlay = 1</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">快捷键格式</span>
          <span class="guide-value">shift + tab（gbe_fork 格式）</span>
        </div>
        <div class="guide-item">
          <span class="guide-label">通知字段</span>
          <span class="guide-value">disable_achievement_notification、disable_friend_notification 等</span>
        </div>
      </div>
    </div>

    <!-- 启用开关 -->
    <div class="config-section">
      <label class="toggle-label">
        <input v-model="config.enableExperimentalOverlay" type="checkbox" class="toggle-input" />
        <span class="toggle-slider"></span>
        <span class="toggle-text">启用功能版 Overlay（Shift+Tab）</span>
      </label>
      <p class="config-hint">功能版功能，如遇到游戏崩溃或卡顿请关闭</p>
    </div>

    <template v-if="config.enableExperimentalOverlay">
      <!-- 快捷键设置 -->
      <div class="config-group">
        <label class="config-label">快捷键</label>
        <input v-model="config.overlayHotkey" type="text" class="config-input" placeholder="shift + tab" />
        <p class="config-hint">按下组合键显示/隐藏 Overlay</p>
      </div>

      <!-- Hook 延迟 -->
      <div class="config-group">
        <label>Hook 延迟（秒）</label>
        <input v-model.number="config.hookDelaySec" type="number" class="config-input" min="0" max="30" placeholder="0" />
        <p class="config-hint">游戏启动后延迟 Hook 的时间，避免冲突</p>
      </div>

      <!-- 渲染器检测超时 -->
      <div class="config-group">
        <label>渲染器检测超时（秒）</label>
        <input v-model.number="config.rendererDetectorTimeoutSec" type="number" class="config-input" min="1" max="60" placeholder="10" />
        <p class="config-hint">检测游戏渲染器的超时时间</p>
      </div>

      <!-- 通知与功能开关 -->
      <h4 class="section-title">通知与功能开关</h4>
      <div class="form-row">
        <label class="checkbox-label">
          <input v-model="config.notifications.disableAchievementNotification" type="checkbox" />
          <span>禁用成就通知</span>
        </label>
        <label class="checkbox-label">
          <input v-model="config.notifications.disableFriendNotification" type="checkbox" />
          <span>禁用好友通知</span>
        </label>
      </div>
      <div class="form-row">
        <label class="checkbox-label">
          <input v-model="config.notifications.disableAchievementProgress" type="checkbox" />
          <span>禁用成就进度</span>
        </label>
        <label class="checkbox-label">
          <input v-model="config.notifications.disableWarningAny" type="checkbox" />
          <span>禁用所有警告</span>
        </label>
      </div>
      <div class="form-row">
        <label class="checkbox-label">
          <input v-model="config.notifications.disableWarningBadAppid" type="checkbox" />
          <span>禁用 Bad AppID 警告</span>
        </label>
        <label class="checkbox-label">
          <input v-model="config.notifications.disableWarningLocalSave" type="checkbox" />
          <span>禁用本地存档警告</span>
        </label>
      </div>
      <div class="form-row">
        <label class="checkbox-label">
          <input v-model="config.notifications.uploadAchievementsIconsToGpu" type="checkbox" />
          <span>上传成就图标到 GPU</span>
        </label>
        <label class="checkbox-label">
          <input v-model="config.notifications.overlayAlwaysShowUserInfo" type="checkbox" />
          <span>始终显示用户信息</span>
        </label>
      </div>
      <div class="form-row">
        <label class="checkbox-label">
          <input v-model="config.notifications.overlayAlwaysShowFps" type="checkbox" />
          <span>始终显示 FPS</span>
        </label>
        <label class="checkbox-label">
          <input v-model="config.notifications.overlayAlwaysShowFrametime" type="checkbox" />
          <span>始终显示帧时间</span>
        </label>
      </div>
      <div class="form-row">
        <label class="checkbox-label">
          <input v-model="config.notifications.overlayAlwaysShowPlaytime" type="checkbox" />
          <span>始终显示游玩时间</span>
        </label>
      </div>

      <!-- 通知时长 -->
      <h4 class="section-title">通知时长</h4>
      <div class="form-row">
        <div class="form-group">
          <label>成就通知（秒）</label>
          <input v-model.number="config.notifications.notificationDurationAchievement" type="number" min="0" placeholder="默认" />
        </div>
        <div class="form-group">
          <label>邀请通知（秒）</label>
          <input v-model.number="config.notifications.notificationDurationInvitation" type="number" min="0" placeholder="默认" />
        </div>
      </div>
      <div class="form-row">
        <div class="form-group">
          <label>聊天通知（秒）</label>
          <input v-model.number="config.notifications.notificationDurationChat" type="number" min="0" placeholder="默认" />
        </div>
        <div class="form-group">
          <label>进度通知（秒）</label>
          <input v-model.number="config.notifications.notificationDurationProgress" type="number" min="0" placeholder="默认" />
        </div>
      </div>

      <!-- FPS 平均窗口 -->
      <div class="config-group">
        <label>FPS 平均窗口</label>
        <input v-model.number="config.fpsAveragingWindow" type="number" class="config-input" min="0.1" max="10" step="0.1" placeholder="1.0" />
        <p class="config-hint">FPS 计算的平均窗口（秒）</p>
      </div>

      <!-- 外观基础 -->
      <h4 class="section-title">外观基础</h4>
      <div class="form-row">
        <div class="form-group">
          <label>主题</label>
          <select v-model="config.appearance.theme">
            <option value="dark">深色 (dark)</option>
            <option value="light">浅色 (light)</option>
          </select>
        </div>
        <div class="form-group">
          <label>透明度 (0-1)</label>
          <input v-model.number="config.appearance.opacity" type="number" min="0" max="1" step="0.05" />
        </div>
      </div>
      <div class="form-row">
        <div class="form-group">
          <label>缩放比例</label>
          <input v-model.number="config.appearance.scale" type="number" min="0.5" max="3" step="0.1" />
        </div>
        <div class="form-group">
          <label class="checkbox-label">
            <input v-model="config.appearance.blur" type="checkbox" />
            <span>启用模糊效果</span>
          </label>
        </div>
      </div>

      <!-- 字体与图标 -->
      <h4 class="section-title">字体与图标</h4>
      <div class="form-row">
        <div class="form-group">
          <label>字体覆盖路径</label>
          <input v-model="config.appearance.fontOverride" placeholder="留空使用默认字体" />
          <p class="field-hint">覆盖 Overlay 使用的字体文件路径</p>
        </div>
        <div class="form-group">
          <label>字体大小</label>
          <input v-model.number="config.appearance.fontSize" type="number" min="8" max="72" placeholder="默认" />
        </div>
      </div>
      <div class="form-row">
        <div class="form-group">
          <label>字体字间距 X</label>
          <input v-model.number="config.appearance.fontGlyphExtraSpacingX" type="number" step="0.1" placeholder="默认" />
        </div>
        <div class="form-group">
          <label>字体字间距 Y</label>
          <input v-model.number="config.appearance.fontGlyphExtraSpacingY" type="number" step="0.1" placeholder="默认" />
        </div>
        <div class="form-group">
          <label>图标大小</label>
          <input v-model.number="config.appearance.iconSize" type="number" min="8" max="128" placeholder="默认" />
        </div>
      </div>

      <!-- 通知样式 -->
      <h4 class="section-title">通知样式</h4>
      <div class="form-row">
        <div class="form-group">
          <label>通知圆角</label>
          <input v-model.number="config.appearance.notificationRounding" type="number" min="0" placeholder="默认" />
        </div>
        <div class="form-group">
          <label>通知边距 X</label>
          <input v-model.number="config.appearance.notificationMarginX" type="number" placeholder="默认" />
        </div>
        <div class="form-group">
          <label>通知边距 Y</label>
          <input v-model.number="config.appearance.notificationMarginY" type="number" placeholder="默认" />
        </div>
      </div>
      <div class="form-row">
        <div class="form-group color-input">
          <label>通知背景色 RGBA</label>
          <div class="color-row">
            <input v-model.number="config.appearance.notificationR" type="number" min="0" max="255" placeholder="R" />
            <input v-model.number="config.appearance.notificationG" type="number" min="0" max="255" placeholder="G" />
            <input v-model.number="config.appearance.notificationB" type="number" min="0" max="255" placeholder="B" />
            <input v-model.number="config.appearance.notificationA" type="number" min="0" max="255" placeholder="A" />
          </div>
        </div>
      </div>

      <!-- 界面配色 -->
      <h4 class="section-title">界面配色</h4>
      <div class="form-row">
        <div class="form-group color-input">
          <label>背景色 RGBA</label>
          <div class="color-row">
            <input v-model.number="config.appearance.backgroundR" type="number" min="0" max="255" placeholder="R" />
            <input v-model.number="config.appearance.backgroundG" type="number" min="0" max="255" placeholder="G" />
            <input v-model.number="config.appearance.backgroundB" type="number" min="0" max="255" placeholder="B" />
            <input v-model.number="config.appearance.backgroundA" type="number" min="0" max="255" placeholder="A" />
          </div>
        </div>
      </div>
      <div class="form-row">
        <div class="form-group color-input">
          <label>元素色 RGBA</label>
          <div class="color-row">
            <input v-model.number="config.appearance.elementR" type="number" min="0" max="255" placeholder="R" />
            <input v-model.number="config.appearance.elementG" type="number" min="0" max="255" placeholder="G" />
            <input v-model.number="config.appearance.elementB" type="number" min="0" max="255" placeholder="B" />
            <input v-model.number="config.appearance.elementA" type="number" min="0" max="255" placeholder="A" />
          </div>
        </div>
      </div>
      <div class="form-row">
        <div class="form-group color-input">
          <label>元素悬停色 RGBA</label>
          <div class="color-row">
            <input v-model.number="config.appearance.elementHoveredR" type="number" min="0" max="255" placeholder="R" />
            <input v-model.number="config.appearance.elementHoveredG" type="number" min="0" max="255" placeholder="G" />
            <input v-model.number="config.appearance.elementHoveredB" type="number" min="0" max="255" placeholder="B" />
            <input v-model.number="config.appearance.elementHoveredA" type="number" min="0" max="255" placeholder="A" />
          </div>
        </div>
      </div>
      <div class="form-row">
        <div class="form-group color-input">
          <label>元素激活色 RGBA</label>
          <div class="color-row">
            <input v-model.number="config.appearance.elementActiveR" type="number" min="0" max="255" placeholder="R" />
            <input v-model.number="config.appearance.elementActiveG" type="number" min="0" max="255" placeholder="G" />
            <input v-model.number="config.appearance.elementActiveB" type="number" min="0" max="255" placeholder="B" />
            <input v-model.number="config.appearance.elementActiveA" type="number" min="0" max="255" placeholder="A" />
          </div>
        </div>
      </div>

      <!-- 通知位置 -->
      <h4 class="section-title">通知位置</h4>
      <div class="form-row three-col">
        <div class="form-group">
          <label>成就通知位置</label>
          <input v-model="config.appearance.posAchievement" placeholder="如 top-left" />
        </div>
        <div class="form-group">
          <label>邀请通知位置</label>
          <input v-model="config.appearance.posInvitation" placeholder="如 top-right" />
        </div>
        <div class="form-group">
          <label>聊天消息位置</label>
          <input v-model="config.appearance.posChatMsg" placeholder="如 bottom-left" />
        </div>
      </div>

      <!-- 统计样式 -->
      <h4 class="section-title">统计样式</h4>
      <div class="form-row">
        <div class="form-group color-input">
          <label>统计背景色 RGBA</label>
          <div class="color-row">
            <input v-model.number="config.appearance.statsBackgroundR" type="number" min="0" max="255" placeholder="R" />
            <input v-model.number="config.appearance.statsBackgroundG" type="number" min="0" max="255" placeholder="G" />
            <input v-model.number="config.appearance.statsBackgroundB" type="number" min="0" max="255" placeholder="B" />
            <input v-model.number="config.appearance.statsBackgroundA" type="number" min="0" max="255" placeholder="A" />
          </div>
        </div>
      </div>
      <div class="form-row">
        <div class="form-group color-input">
          <label>统计文字色 RGBA</label>
          <div class="color-row">
            <input v-model.number="config.appearance.statsTextR" type="number" min="0" max="255" placeholder="R" />
            <input v-model.number="config.appearance.statsTextG" type="number" min="0" max="255" placeholder="G" />
            <input v-model.number="config.appearance.statsTextB" type="number" min="0" max="255" placeholder="B" />
            <input v-model.number="config.appearance.statsTextA" type="number" min="0" max="255" placeholder="A" />
          </div>
        </div>
      </div>
      <div class="form-row">
        <div class="form-group">
          <label>统计位置 X</label>
          <input v-model.number="config.appearance.statsPosX" type="number" placeholder="默认" />
        </div>
        <div class="form-group">
          <label>统计位置 Y</label>
          <input v-model.number="config.appearance.statsPosY" type="number" placeholder="默认" />
        </div>
      </div>
      <div class="form-row">
        <div class="form-group">
          <label>成就解锁日期格式</label>
          <input v-model="config.appearance.achievementUnlockDatetimeFormat" placeholder="如 %Y-%m-%d %H:%M:%S" />
        </div>
      </div>

      <!-- 性能设置 -->
      <h4 class="section-title">性能设置</h4>
      <div class="form-row">
        <div class="form-group">
          <label>FPS 限制</label>
          <input v-model.number="config.performance.fpsLimit" type="number" min="1" max="240" />
        </div>
        <div class="form-group">
          <label class="checkbox-label">
            <input v-model="config.performance.hardwareAcceleration" type="checkbox" />
            <span>硬件加速</span>
          </label>
        </div>
        <div class="form-group">
          <label class="checkbox-label">
            <input v-model="config.performance.lowPerformanceMode" type="checkbox" />
            <span>低性能模式</span>
          </label>
        </div>
      </div>

      <!-- 功能开关 -->
      <h4 class="section-title">覆盖层功能</h4>
      <div class="form-row">
        <label class="checkbox-label">
          <input v-model="config.features.achievements" type="checkbox" />
          <span>成就</span>
        </label>
        <label class="checkbox-label">
          <input v-model="config.features.friends" type="checkbox" />
          <span>好友</span>
        </label>
        <label class="checkbox-label">
          <input v-model="config.features.chat" type="checkbox" />
          <span>聊天</span>
        </label>
      </div>
      <div class="form-row">
        <label class="checkbox-label">
          <input v-model="config.features.browser" type="checkbox" />
          <span>浏览器</span>
        </label>
        <label class="checkbox-label">
          <input v-model="config.features.settings" type="checkbox" />
          <span>设置</span>
        </label>
      </div>
    </template>

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
        <span>覆盖层配置已保存成功！</span>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
/**
 * OverlayConfigPanel.vue - 覆盖层配置 Panel
 * 供 OverlayConfig.vue 单独弹窗和 CompleteConfigManager.vue 完整管理器复用
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

const showToast = ref(false)

/**
 * Overlay 配置对象
 * 字段名使用 camelCase，与后端 save_overlay_config 命令期望的 JSON 键名保持一致。
 * 外层使用 shallowReactive 减少深层代理开销；
 * 嵌套对象（notifications / appearance / performance / features）使用 reactive，
 * 确保 v-model 在表单字段上仍能正常响应。
 */
const config = shallowReactive({
  enableExperimentalOverlay: false,
  hookDelaySec: undefined as number | undefined,
  rendererDetectorTimeoutSec: undefined as number | undefined,
  overlayHotkey: 'shift + tab',
  fpsAveragingWindow: undefined as number | undefined,
  notifications: reactive({
    disableAchievementNotification: false,
    disableFriendNotification: false,
    disableAchievementProgress: false,
    disableWarningAny: false,
    disableWarningBadAppid: false,
    disableWarningLocalSave: false,
    uploadAchievementsIconsToGpu: true,
    overlayAlwaysShowUserInfo: false,
    overlayAlwaysShowFps: false,
    overlayAlwaysShowFrametime: false,
    overlayAlwaysShowPlaytime: false,
    notificationDurationAchievement: undefined as number | undefined,
    notificationDurationInvitation: undefined as number | undefined,
    notificationDurationChat: undefined as number | undefined,
    notificationDurationProgress: undefined as number | undefined,
  }),
  appearance: reactive({
    fontOverride: '',
    fontSize: undefined as number | undefined,
    fontGlyphExtraSpacingX: undefined as number | undefined,
    fontGlyphExtraSpacingY: undefined as number | undefined,
    iconSize: undefined as number | undefined,
    theme: 'dark',
    opacity: 0.95,
    scale: 1.0,
    blur: true,
    notificationRounding: undefined as number | undefined,
    notificationMarginX: undefined as number | undefined,
    notificationMarginY: undefined as number | undefined,
    notificationR: undefined as number | undefined,
    notificationG: undefined as number | undefined,
    notificationB: undefined as number | undefined,
    notificationA: undefined as number | undefined,
    achievementUnlockDatetimeFormat: '',
    backgroundR: undefined as number | undefined,
    backgroundG: undefined as number | undefined,
    backgroundB: undefined as number | undefined,
    backgroundA: undefined as number | undefined,
    elementR: undefined as number | undefined,
    elementG: undefined as number | undefined,
    elementB: undefined as number | undefined,
    elementA: undefined as number | undefined,
    elementHoveredR: undefined as number | undefined,
    elementHoveredG: undefined as number | undefined,
    elementHoveredB: undefined as number | undefined,
    elementHoveredA: undefined as number | undefined,
    elementActiveR: undefined as number | undefined,
    elementActiveG: undefined as number | undefined,
    elementActiveB: undefined as number | undefined,
    elementActiveA: undefined as number | undefined,
    posAchievement: '',
    posInvitation: '',
    posChatMsg: '',
    statsBackgroundR: undefined as number | undefined,
    statsBackgroundG: undefined as number | undefined,
    statsBackgroundB: undefined as number | undefined,
    statsBackgroundA: undefined as number | undefined,
    statsTextR: undefined as number | undefined,
    statsTextG: undefined as number | undefined,
    statsTextB: undefined as number | undefined,
    statsTextA: undefined as number | undefined,
    statsPosX: undefined as number | undefined,
    statsPosY: undefined as number | undefined,
  }),
  performance: reactive({
    hardwareAcceleration: true,
    fpsLimit: 60,
    lowPerformanceMode: false,
  }),
  features: reactive({
    achievements: true,
    friends: true,
    chat: true,
    browser: true,
    settings: true,
  }),
})

/**
 * 将 snake_case 键名递归转换为 camelCase
 * 用于把后端返回的 OverlayConfig 数据转换为前端表单结构
 */
function snakeToCamel(obj: any): any {
  if (obj === null || typeof obj !== 'object') return obj
  if (Array.isArray(obj)) return obj.map(snakeToCamel)
  const result: any = {}
  for (const [key, value] of Object.entries(obj)) {
    const camelKey = key.replace(/_([a-z])/g, (_, letter: string) => letter.toUpperCase())
    result[camelKey] = snakeToCamel(value)
  }
  return result
}

/**
 * 深度合并对象到现有目标
 * 与 deepMerge 不同，本函数会直接修改 target 的属性而不是创建新对象，
 * 从而保持 shallowReactive + reactive 嵌套对象的引用不变，避免破坏 v-model。
 * undefined 值会被跳过，保留前端默认值。
 */
function deepMergeInto(target: any, source: any) {
  if (source === null || typeof source !== 'object') return
  for (const key of Object.keys(source)) {
    const sourceValue = source[key]
    if (sourceValue === undefined) continue
    if (
      typeof sourceValue === 'object' &&
      !Array.isArray(sourceValue) &&
      target[key] &&
      typeof target[key] === 'object'
    ) {
      deepMergeInto(target[key], sourceValue)
    } else {
      target[key] = sourceValue
    }
  }
}

/**
 * 加载现有配置
 * 从后端读取 configs.overlay.ini 并填充到表单
 */
async function loadOverlayConfig() {
  try {
    const result = await invoke<{
      exists: boolean
      config?: any
    }>('load_overlay_config', {
      gamePath: props.gamePath,
    })

    if (result.exists && result.config) {
      // 后端返回 snake_case 键名，需转换为 camelCase 以匹配前端表单结构
      const camelConfig = snakeToCamel(result.config)
      // 深度合并到现有配置对象，保持 reactive 嵌套对象引用不变
      deepMergeInto(config, camelConfig)
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
    loadOverlayConfig()
  }
}

onMounted(() => {
  loadOverlayConfig()
  // 监听覆盖层配置保存事件，与完整配置管理器/其它单独窗口实时同步
  window.addEventListener(CONFIG_EVENTS.OVERLAY_SAVED, onConfigSavedEvent)
})

onUnmounted(() => {
  window.removeEventListener(CONFIG_EVENTS.OVERLAY_SAVED, onConfigSavedEvent)
})

/**
 * 保存配置
 * 将配置发送到后端 save_overlay_config 命令
 */
async function saveConfig() {
  try {
    const result = await invoke<{ success: boolean; message: string }>('save_overlay_config', {
      gamePath: props.gamePath,
      config,
    })

    if (result.success) {
      showToast.value = true
      setTimeout(() => {
        showToast.value = false
      }, 3000)
      emit('saved')
      // 广播覆盖层配置已保存事件，使其他弹窗/页面同步刷新
      window.dispatchEvent(new CustomEvent(CONFIG_EVENTS.OVERLAY_SAVED, {
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
  load: loadOverlayConfig,
  save: saveConfig
})
</script>

<style scoped>
.overlay-config-panel {
  position: relative;
}

.config-section {
  margin-bottom: 24px;
  padding-bottom: 20px;
  border-bottom: 1px solid var(--steam-border);
}

.config-section:last-of-type {
  border-bottom: none;
}

.config-group {
  margin-bottom: 20px;
}

.config-group label,
.form-group label {
  display: block;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin-bottom: 8px;
}

.config-label {
  display: block;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin-bottom: 8px;
}

.config-input,
.form-group input,
.form-group select,
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
.form-group input:focus,
.form-group select:focus,
select:focus {
  border-color: var(--steam-accent-blue);
}

.config-hint,
.field-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 6px 0 0 0;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 24px 0 14px 0;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--steam-border);
}

.form-row {
  display: flex;
  gap: 16px;
  flex-wrap: wrap;
  margin-bottom: 16px;
}

.form-row .form-group {
  flex: 1;
  min-width: 200px;
  margin-bottom: 0;
}

.form-row.three-col .form-group {
  flex: 1;
  min-width: 150px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  color: var(--steam-text-primary);
  font-size: 14px;
  font-weight: normal;
}

.checkbox-label input[type="checkbox"] {
  width: 18px;
  height: 18px;
  accent-color: var(--steam-accent-blue);
  cursor: pointer;
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
  position: relative;
  width: 44px;
  height: 24px;
  background-color: var(--steam-border);
  border-radius: 12px;
  transition: background-color 0.2s ease;
  flex-shrink: 0;
}

.toggle-slider::after {
  content: '';
  position: absolute;
  top: 2px;
  left: 2px;
  width: 20px;
  height: 20px;
  background-color: white;
  border-radius: 50%;
  transition: transform 0.2s ease;
}

.toggle-input:checked + .toggle-slider {
  background-color: var(--steam-accent-blue);
}

.toggle-input:checked + .toggle-slider::after {
  transform: translateX(20px);
}

.toggle-text {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.color-input .color-row {
  display: flex;
  gap: 8px;
}

.color-input .color-row input {
  flex: 1;
  min-width: 60px;
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
  gap: 8px;
  margin-bottom: 10px;
  font-size: 13px;
  font-weight: 600;
  color: var(--steam-accent-blue);
}

.guide-header svg {
  width: 16px;
  height: 16px;
}

.guide-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
}

.guide-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
}

.guide-label {
  color: var(--steam-text-secondary);
  white-space: nowrap;
}

.guide-value {
  color: var(--steam-text-primary);
  font-family: 'Courier New', monospace;
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
