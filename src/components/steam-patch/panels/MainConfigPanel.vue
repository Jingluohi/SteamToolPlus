<template>
  <div class="main-config-panel">
    <!-- 格式说明 -->
    <div class="format-info">
      <div class="format-header">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"/>
          <line x1="12" y1="16" x2="12" y2="12"/>
          <line x1="12" y1="8" x2="12.01" y2="8"/>
        </svg>
        <span>格式说明</span>
      </div>
      <div class="format-grid">
        <div class="format-item">
          <span class="format-label">主配置文件</span>
          <span class="format-value">configs.main.ini</span>
        </div>
        <div class="format-item">
          <span class="format-label">文件格式</span>
          <span class="format-value">INI 格式（键=值）</span>
        </div>
        <div class="format-item">
          <span class="format-label">布尔值</span>
          <span class="format-value">1=启用，0=禁用</span>
        </div>
        <div class="format-item">
          <span class="format-label">数值</span>
          <span class="format-value">直接填写数字，如 300、32</span>
        </div>
      </div>
    </div>

    <!-- [main::general] 通用设置 -->
    <div class="config-section">
      <h4>通用设置</h4>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.newAppTicket" type="checkbox" />
          <span>生成新版认证票据 (new_app_ticket)</span>
        </label>
        <p class="field-hint">启用后生成新版 Steam 认证票据，大多数游戏需要开启</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.gcToken" type="checkbox" />
          <span>启用游戏协调器令牌 (gc_token)</span>
        </label>
        <p class="field-hint">用于 Valve 游戏的 GC 认证，如 CS:GO、Dota2 等需要开启</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.blockUnknownClients" type="checkbox" />
          <span>阻止未知客户端</span>
        </label>
        <p class="field-hint">阻止非 Steam 官方客户端连接，增强安全性</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.steamDeck" type="checkbox" />
          <span>模拟 Steam Deck</span>
        </label>
        <p class="field-hint">让游戏认为运行在 Steam Deck 上，可能解锁 Deck 专属内容</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.enableAccountAvatar" type="checkbox" />
          <span>启用头像功能</span>
        </label>
        <p class="field-hint">允许游戏获取并显示用户头像，需配合头像文件使用</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.enableVoiceChat" type="checkbox" />
          <span>启用语音聊天</span>
        </label>
        <p class="field-hint">启用 Steam 语音聊天功能，需要游戏支持</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.immediateGameserverStats" type="checkbox" />
          <span>即时游戏服务器统计</span>
        </label>
        <p class="field-hint">立即上报游戏服务器统计数据，不等待批量提交</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.matchmakingServerListActualType" type="checkbox" />
          <span>匹配服务器列表实际类型</span>
        </label>
        <p class="field-hint">返回实际的服务器列表类型，而非强制局域网类型</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.matchmakingServerDetailsViaSourceQuery" type="checkbox" />
          <span>通过 Source 查询获取服务器详情</span>
        </label>
        <p class="field-hint">使用 Source 协议查询服务器详细信息</p>
      </div>
      <div class="form-group">
        <label>崩溃日志位置</label>
        <input v-model="config.crashPrinterLocation" placeholder="可选" />
        <p class="field-hint">设置崩溃日志输出目录，留空则不输出</p>
      </div>
    </div>

    <!-- [main::stats] 统计设置 -->
    <div class="config-section">
      <h4>统计设置</h4>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.disableLeaderboardsCreateUnknown" type="checkbox" />
          <span>禁用未知排行榜创建</span>
        </label>
        <p class="field-hint">阻止游戏自动创建未预定义的排行榜，避免数据混乱</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.allowUnknownStats" type="checkbox" />
          <span>允许未知统计</span>
        </label>
        <p class="field-hint">允许游戏上报未预定义的统计数据，大多数游戏建议开启</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.statAchievementProgressFunctionality" type="checkbox" />
          <span>统计成就进度功能</span>
        </label>
        <p class="field-hint">启用基于统计数据的成就进度追踪</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.saveOnlyHigherStatAchievementProgress" type="checkbox" />
          <span>只保存更高的统计成就进度</span>
        </label>
        <p class="field-hint">仅当新进度高于旧进度时才保存，防止进度倒退</p>
      </div>
      <div class="form-group">
        <label>分页成就图标数量</label>
        <input v-model.number="config.paginatedAchievementsIcons" type="number" />
        <p class="field-hint">每页加载的成就图标数量，默认 10</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.recordPlaytime" type="checkbox" />
          <span>记录游戏时间</span>
        </label>
        <p class="field-hint">启用游戏时长统计，可在 Steam 个人资料中显示</p>
      </div>
    </div>

    <!-- [main::connectivity] 连接设置 -->
    <div class="config-section">
      <h4>连接设置</h4>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.disableLanOnly" type="checkbox" />
          <span>禁用仅局域网模式</span>
        </label>
        <p class="field-hint">取消局域网限制，允许非局域网连接。联机游戏建议关闭此项</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.disableNetworking" type="checkbox" />
          <span>禁用网络功能</span>
        </label>
        <p class="field-hint">完全禁用网络功能，纯单机游戏可开启</p>
      </div>
      <div class="form-row">
        <div class="form-group">
          <label>监听端口</label>
          <input v-model.number="config.listenPort" type="number" />
          <p class="field-hint">模拟器监听的 UDP 端口，默认 47584，联机时需保持一致</p>
        </div>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.offline" type="checkbox" />
          <span>离线模式</span>
        </label>
        <p class="field-hint">强制离线模式，不尝试任何网络连接</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.disableSharingStatsWithGameserver" type="checkbox" />
          <span>禁用与游戏服务器共享统计</span>
        </label>
        <p class="field-hint">阻止统计数据发送到游戏服务器</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.disableSourceQuery" type="checkbox" />
          <span>禁用 Source 查询</span>
        </label>
        <p class="field-hint">禁用 Source 协议服务器查询</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.shareLeaderboardsOverNetwork" type="checkbox" />
          <span>网络共享排行榜</span>
        </label>
        <p class="field-hint">通过网络与其他玩家共享排行榜数据</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.disableLobbyCreation" type="checkbox" />
          <span>禁用大厅创建</span>
        </label>
        <p class="field-hint">阻止游戏创建 Steam 大厅，纯单机游戏可开启</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.downloadSteamhttpRequests" type="checkbox" />
          <span>下载 SteamHTTP 请求</span>
        </label>
        <p class="field-hint">拦截并缓存 SteamHTTP 请求结果</p>
      </div>
    </div>

    <!-- [main::misc] 其他设置 -->
    <div class="config-section">
      <h4>其他设置</h4>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.achievementsBypass" type="checkbox" />
          <span>成就绕过</span>
        </label>
        <p class="field-hint">绕过成就解锁限制，允许解锁所有成就</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.forceSteamhttpSuccess" type="checkbox" />
          <span>强制 SteamHTTP 成功</span>
        </label>
        <p class="field-hint">强制所有 SteamHTTP 请求返回成功，避免网络请求失败导致的问题</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.disableSteamoverlaygameidEnvVar" type="checkbox" />
          <span>禁用 Steam 覆盖层游戏 ID 环境变量</span>
        </label>
        <p class="field-hint">阻止设置 SteamOverlayGameId 环境变量</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.enableSteamPreownedIds" type="checkbox" />
          <span>启用 Steam 预拥有 ID</span>
        </label>
        <p class="field-hint">模拟 Steam 预拥有游戏 ID 列表</p>
      </div>
      <div class="form-group">
        <label>Steam 游戏统计报告目录</label>
        <input v-model="config.steamGameStatsReportsDir" placeholder="可选" />
        <p class="field-hint">设置游戏统计报告输出目录</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.freeWeekend" type="checkbox" />
          <span>免费周末</span>
        </label>
        <p class="field-hint">模拟免费周末活动状态</p>
      </div>
      <div class="form-group">
        <label class="checkbox-label">
          <input v-model="config.use32bitInventoryItemIds" type="checkbox" />
          <span>使用 32 位库存物品 ID</span>
        </label>
        <p class="field-hint">使用 32 位而非 64 位物品 ID，兼容旧版游戏</p>
      </div>
    </div>

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
        <span>主配置已保存成功！</span>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
/**
 * MainConfigPanel.vue - 主配置 Panel
 * 供 MainConfig.vue 单独弹窗和 CompleteConfigManager.vue 完整管理器复用
 * 统一加载、保存、默认值和同步逻辑
 */

import { ref, shallowReactive, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { CONFIG_EVENTS } from '../../../constants/config-events'

const props = defineProps<{
  gamePath: string
}>()

const emit = defineEmits<{
  saved: []
}>()

/**
 * 主配置数据模型
 * 默认值与 Rust 后端 MainConfig::default_config() 保持一致。
 * 所有字段均为顶层标量或数组，使用 shallowReactive 即可满足响应式需求，
 * 同时避免深层代理带来的内存开销。
 */
const config = shallowReactive({
  // [main::general]
  newAppTicket: true,
  gcToken: true,
  blockUnknownClients: false,
  steamDeck: false,
  enableAccountAvatar: false,
  enableVoiceChat: false,
  immediateGameserverStats: false,
  matchmakingServerListActualType: false,
  matchmakingServerDetailsViaSourceQuery: false,
  crashPrinterLocation: '',
  // [main::stats]
  disableLeaderboardsCreateUnknown: false,
  allowUnknownStats: true,
  statAchievementProgressFunctionality: true,
  saveOnlyHigherStatAchievementProgress: true,
  paginatedAchievementsIcons: 10,
  recordPlaytime: false,
  // [main::connectivity]
  disableLanOnly: false,
  disableNetworking: false,
  listenPort: 47584,
  offline: false,
  disableSharingStatsWithGameserver: false,
  disableSourceQuery: false,
  shareLeaderboardsOverNetwork: false,
  disableLobbyCreation: false,
  downloadSteamhttpRequests: false,
  // [main::misc]
  achievementsBypass: false,
  forceSteamhttpSuccess: false,
  disableSteamoverlaygameidEnvVar: false,
  enableSteamPreownedIds: false,
  steamGameStatsReportsDir: '',
  freeWeekend: false,
  use32bitInventoryItemIds: false,
  // [extra_dlls]
  extraDlls: [] as string[]
})

const showToast = ref(false)

/**
 * 将 snake_case 字符串转换为 camelCase
 * 用于主配置 INI 键名与 TS 类型属性名之间的转换
 */
function snakeToCamel(s: string): string {
  return s.replace(/_(.)/g, (_, char: string) => char.toUpperCase())
}

/**
 * 将前端主配置对象转换为 INI 字符串
 */
function buildMainConfigIni(main: any): string {
  const lines: string[] = []
  const boolFields = (fields: string[]) => {
    for (const f of fields) {
      const camelKey = snakeToCamel(f)
      if (main[camelKey]) lines.push(`${f} = 1`)
    }
  }

  // [main::general]
  lines.push('[main::general]')
  boolFields([
    'new_app_ticket', 'gc_token', 'block_unknown_clients', 'steam_deck',
    'enable_account_avatar', 'enable_voice_chat', 'immediate_gameserver_stats',
    'matchmaking_server_list_actual_type', 'matchmaking_server_details_via_source_query'
  ])
  if (main.crashPrinterLocation) {
    lines.push(`crash_printer_location = ${main.crashPrinterLocation}`)
  }

  // [main::stats]
  lines.push('')
  lines.push('[main::stats]')
  boolFields([
    'disable_leaderboards_create_unknown', 'allow_unknown_stats',
    'stat_achievement_progress_functionality', 'save_only_higher_stat_achievement_progress',
    'record_playtime'
  ])
  if (main.paginatedAchievementsIcons !== undefined && main.paginatedAchievementsIcons !== 10) {
    lines.push(`paginated_achievements_icons = ${main.paginatedAchievementsIcons}`)
  }

  // [main::connectivity]
  lines.push('')
  lines.push('[main::connectivity]')
  boolFields([
    'disable_lan_only', 'disable_networking', 'offline',
    'disable_sharing_stats_with_gameserver', 'disable_source_query',
    'share_leaderboards_over_network', 'disable_lobby_creation', 'download_steamhttp_requests'
  ])
  if (main.listenPort !== undefined && main.listenPort !== 47584) {
    lines.push(`listen_port = ${main.listenPort}`)
  }

  // [main::misc]
  lines.push('')
  lines.push('[main::misc]')
  boolFields([
    'achievements_bypass', 'force_steamhttp_success',
    'disable_steamoverlaygameid_env_var', 'enable_steam_preowned_ids',
    'free_weekend', 'use_32bit_inventory_item_ids'
  ])
  if (main.steamGameStatsReportsDir) {
    lines.push(`steam_game_stats_reports_dir = ${main.steamGameStatsReportsDir}`)
  }

  // [extra_dlls]
  if (main.extraDlls && main.extraDlls.length > 0) {
    lines.push('')
    lines.push('[extra_dlls]')
    main.extraDlls.forEach((dll: string, index: number) => {
      lines.push(`dll${index + 1} = ${dll}`)
    })
  }

  return lines.join('\n')
}

/**
 * 解析主配置 INI 字符串为对象
 */
function parseMainConfigIni(content: string): Partial<any> {
  const result: any = {}
  const boolKeys = new Set([
    'new_app_ticket', 'gc_token', 'block_unknown_clients', 'steam_deck',
    'enable_account_avatar', 'enable_voice_chat', 'immediate_gameserver_stats',
    'matchmaking_server_list_actual_type', 'matchmaking_server_details_via_source_query',
    'disable_leaderboards_create_unknown', 'allow_unknown_stats',
    'stat_achievement_progress_functionality', 'save_only_higher_stat_achievement_progress',
    'record_playtime', 'disable_lan_only', 'disable_networking', 'offline',
    'disable_sharing_stats_with_gameserver', 'disable_source_query',
    'share_leaderboards_over_network', 'disable_lobby_creation', 'download_steamhttp_requests',
    'achievements_bypass', 'force_steamhttp_success', 'disable_steamoverlaygameid_env_var',
    'enable_steam_preowned_ids', 'free_weekend', 'use_32bit_inventory_item_ids'
  ])
  const intKeys = new Set(['paginated_achievements_icons', 'listen_port'])

  for (const line of content.split('\n')) {
    const t = line.trim()
    if (!t || t.startsWith('[') || t.startsWith('#')) continue
    const i = t.indexOf('=')
    if (i < 0) {
      if (!result.extraDlls) result.extraDlls = []
      result.extraDlls.push(t)
      continue
    }
    const k = t.slice(0, i).trim()
    const v = t.slice(i + 1).trim()
    const camelKey = snakeToCamel(k)

    if (k.startsWith('dll')) {
      if (!result.extraDlls) result.extraDlls = []
      result.extraDlls.push(v)
    } else if (boolKeys.has(k)) {
      result[camelKey] = v === '1' || v === 'true'
    } else if (intKeys.has(k)) {
      result[camelKey] = parseInt(v, 10) || 0
    } else {
      result[camelKey] = v
    }
  }
  return result
}

/**
 * 加载现有的主配置
 */
async function loadMainConfig() {
  try {
    const result = await invoke<{ exists: boolean; content?: string | null }>('load_main_config', {
      gamePath: props.gamePath
    })

    if (result.content) {
      const parsed = parseMainConfigIni(result.content)
      Object.assign(config, parsed)
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
    loadMainConfig()
  }
}

onMounted(() => {
  loadMainConfig()
  // 监听主配置保存事件，与完整配置管理器/其它单独窗口实时同步
  window.addEventListener(CONFIG_EVENTS.MAIN_SAVED, onConfigSavedEvent)
})

onUnmounted(() => {
  window.removeEventListener(CONFIG_EVENTS.MAIN_SAVED, onConfigSavedEvent)
})

/**
 * 保存主配置
 */
async function saveConfig() {
  try {
    const result = await invoke<{ success: boolean; message: string }>('save_main_config', {
      gamePath: props.gamePath,
      config: {
        mainIni: buildMainConfigIni(config)
      }
    })

    if (result.success) {
      showToast.value = true
      setTimeout(() => {
        showToast.value = false
      }, 3000)
      emit('saved')
      // 广播主配置已保存事件，通知完整配置管理器等其它窗口刷新
      window.dispatchEvent(new CustomEvent(CONFIG_EVENTS.MAIN_SAVED, {
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
  load: loadMainConfig,
  save: saveConfig
})
</script>

<style scoped>
.main-config-panel {
  position: relative;
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

.form-row .form-group {
  flex: 1;
  min-width: 200px;
  margin-bottom: 0;
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

.form-group input[type="text"],
.form-group input[type="number"],
.form-group input:not([type="checkbox"]) {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 13px;
  outline: none;
}

.form-group input:focus {
  border-color: var(--steam-accent-blue);
}

.field-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 6px 0 0 0;
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

/* 格式说明 */
.format-info {
  background-color: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 10px;
  padding: 16px 20px;
  margin-bottom: 20px;
}

.format-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 10px;
  font-size: 13px;
  font-weight: 600;
  color: var(--steam-accent-blue);
}

.format-header svg {
  width: 16px;
  height: 16px;
}

.format-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
}

.format-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
}

.format-label {
  color: var(--steam-text-secondary);
  white-space: nowrap;
}

.format-value {
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
