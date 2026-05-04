<template>
  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-content complete-config" @click.stop>
      <div class="modal-header">
        <div class="header-icon complete">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="3"/>
            <path d="M12 1v6m0 6v6m4.22-10.22l4.24-4.24M6.34 17.66l-4.24 4.24M23 12h-6m-6 0H1m20.24 4.24l-4.24-4.24M6.34 6.34L2.1 2.1"/>
          </svg>
        </div>
        <h3>完整配置管理器</h3>
        <button class="close-btn" @click="$emit('close')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <div class="modal-body">
        <!-- 左侧导航 -->
        <div class="config-nav">
          <div class="nav-section">
            <h4>核心配置</h4>
            <button 
              v-for="item in coreConfigs" 
              :key="item.id"
              class="nav-item"
              :class="{ active: activeTab === item.id }"
              @click="activeTab = item.id"
            >
              <span class="nav-icon" v-html="item.icon"></span>
              <span class="nav-label">{{ item.name }}</span>
              <span v-if="configStatus[item.id]" class="nav-status configured">✓</span>
            </button>
          </div>

          <div class="nav-section">
            <h4>游戏功能</h4>
            <button 
              v-for="item in gameFeatures" 
              :key="item.id"
              class="nav-item"
              :class="{ active: activeTab === item.id }"
              @click="activeTab = item.id"
            >
              <span class="nav-icon" v-html="item.icon"></span>
              <span class="nav-label">{{ item.name }}</span>
              <span v-if="configStatus[item.id]" class="nav-status configured">✓</span>
            </button>
          </div>

          <div class="nav-section">
            <h4>工具集成</h4>
            <button 
              v-for="item in toolConfigs" 
              :key="item.id"
              class="nav-item"
              :class="{ active: activeTab === item.id }"
              @click="activeTab = item.id"
            >
              <span class="nav-icon" v-html="item.icon"></span>
              <span class="nav-label">{{ item.name }}</span>
              <span v-if="configStatus[item.id]" class="nav-status configured">✓</span>
            </button>
          </div>
        </div>

        <!-- 右侧内容区 -->
        <div class="config-content">
          <!-- 主配置 -->
          <div v-if="activeTab === 'main'" class="config-panel">
            <h3>主配置 (configs.main.ini)</h3>

            <!-- [main::general] 通用设置 -->
            <div class="config-section">
              <h4>通用设置</h4>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.new_app_ticket" type="checkbox" />
                  <span>生成新版认证票据 (new_app_ticket)</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.gc_token" type="checkbox" />
                  <span>启用游戏协调器令牌 (gc_token)</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.block_unknown_clients" type="checkbox" />
                  <span>阻止未知客户端</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.steam_deck" type="checkbox" />
                  <span>模拟 Steam Deck</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.enable_account_avatar" type="checkbox" />
                  <span>启用头像功能</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.enable_voice_chat" type="checkbox" />
                  <span>启用语音聊天</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.immediate_gameserver_stats" type="checkbox" />
                  <span>即时游戏服务器统计</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.matchmaking_server_list_actual_type" type="checkbox" />
                  <span>匹配服务器列表实际类型</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.matchmaking_server_details_via_source_query" type="checkbox" />
                  <span>通过 Source 查询获取服务器详情</span>
                </label>
              </div>
              <div class="form-group">
                <label>崩溃日志位置</label>
                <input v-model="configs.main.crash_printer_location" placeholder="可选" />
              </div>
            </div>

            <!-- [main::stats] 统计设置 -->
            <div class="config-section">
              <h4>统计设置</h4>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.disable_leaderboards_create_unknown" type="checkbox" />
                  <span>禁用未知排行榜创建</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.allow_unknown_stats" type="checkbox" />
                  <span>允许未知统计</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.stat_achievement_progress_functionality" type="checkbox" />
                  <span>统计成就进度功能</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.save_only_higher_stat_achievement_progress" type="checkbox" />
                  <span>只保存更高的统计成就进度</span>
                </label>
              </div>
              <div class="form-group">
                <label>分页成就图标数量</label>
                <input v-model.number="configs.main.paginated_achievements_icons" type="number" />
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.record_playtime" type="checkbox" />
                  <span>记录游戏时间</span>
                </label>
              </div>
            </div>

            <!-- [main::connectivity] 连接设置 -->
            <div class="config-section">
              <h4>连接设置</h4>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.disable_lan_only" type="checkbox" />
                  <span>禁用仅局域网模式</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.disable_networking" type="checkbox" />
                  <span>禁用网络功能</span>
                </label>
              </div>
              <div class="form-row">
                <div class="form-group">
                  <label>监听端口</label>
                  <input v-model.number="configs.main.listen_port" type="number" />
                </div>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.offline" type="checkbox" />
                  <span>离线模式</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.disable_sharing_stats_with_gameserver" type="checkbox" />
                  <span>禁用与游戏服务器共享统计</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.disable_source_query" type="checkbox" />
                  <span>禁用 Source 查询</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.share_leaderboards_over_network" type="checkbox" />
                  <span>网络共享排行榜</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.disable_lobby_creation" type="checkbox" />
                  <span>禁用地堡创建</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.download_steamhttp_requests" type="checkbox" />
                  <span>下载 SteamHTTP 请求</span>
                </label>
              </div>
            </div>

            <!-- [main::misc] 其他设置 -->
            <div class="config-section">
              <h4>其他设置</h4>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.achievements_bypass" type="checkbox" />
                  <span>成就绕过</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.force_steamhttp_success" type="checkbox" />
                  <span>强制 SteamHTTP 成功</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.disable_steamoverlaygameid_env_var" type="checkbox" />
                  <span>禁用 Steam 覆盖层游戏 ID 环境变量</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.enable_steam_preowned_ids" type="checkbox" />
                  <span>启用 Steam 预拥有 ID</span>
                </label>
              </div>
              <div class="form-group">
                <label>Steam 游戏统计报告目录</label>
                <input v-model="configs.main.steam_game_stats_reports_dir" placeholder="可选" />
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.free_weekend" type="checkbox" />
                  <span>免费周末</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.use_32bit_inventory_item_ids" type="checkbox" />
                  <span>使用 32 位库存物品 ID</span>
                </label>
              </div>
            </div>
          </div>

          <!-- 用户配置 -->
          <div v-if="activeTab === 'user'" class="config-panel">
            <h3>用户配置 (configs.user.ini)</h3>
            <div class="form-group">
              <label>用户名</label>
              <input v-model="configs.user.username" placeholder="Player" />
            </div>
            <div class="form-group">
              <label>语言</label>
              <select v-model="configs.user.language">
                <option value="schinese">简体中文</option>
                <option value="tchinese">繁体中文</option>
                <option value="english">英语</option>
                <option value="japanese">日语</option>
                <option value="korean">韩语</option>
              </select>
            </div>
            <div class="form-group">
              <label>存档路径</label>
              <input v-model="configs.user.save_path" placeholder="%appdata%/GSE Saves" />
            </div>
            <div class="form-group">
              <label>存档文件夹名称（覆盖默认"GSE Saves"）</label>
              <input v-model="configs.user.saves_folder_name" placeholder="可选" />
            </div>
            <div class="form-group">
              <label>本地存档路径（便携模式）</label>
              <input v-model="configs.user.local_save_path" placeholder="可选，设置后完全便携" />
            </div>
            <div class="form-group">
              <label>EncryptedAppTicket (Base64)</label>
              <textarea v-model="configs.user.ticket" placeholder="可选，用于某些需要票据验证的游戏" rows="3"></textarea>
            </div>
          </div>

          <!-- 应用配置 -->
          <div v-if="activeTab === 'app'" class="config-panel">
            <h3>应用配置 (configs.app.ini)</h3>
            <div class="form-group">
              <label>分支名称</label>
              <input v-model="configs.app.branch_name" placeholder="public" />
            </div>
            <div class="form-group">
              <label class="checkbox-label">
                <input v-model="configs.app.dlcs.unlock_all" type="checkbox" />
                <span>解锁所有 DLC</span>
              </label>
            </div>
          </div>

          <!-- 覆盖层配置 -->
          <div v-if="activeTab === 'overlay'" class="config-panel">
            <h3>覆盖层配置 (configs.overlay.ini)</h3>
            <div class="form-group">
              <label class="checkbox-label">
                <input v-model="configs.overlay.enable_experimental_overlay" type="checkbox" />
                <span>启用实验性游戏内 Overlay (Shift+Tab)</span>
              </label>
            </div>
            <div class="form-group">
              <label>快捷键</label>
              <input v-model="configs.overlay.hotkey" placeholder="Shift+Tab" />
            </div>
            <h4>通知设置</h4>
            <div class="form-row">
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.achievement" type="checkbox" />
                <span>成就通知</span>
              </label>
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.friend" type="checkbox" />
                <span>好友通知</span>
              </label>
            </div>
          </div>

          <!-- 成就配置 -->
          <div v-if="activeTab === 'achievements'" class="config-panel">
            <h3>成就配置 (achievements.json)</h3>
            <div class="panel-actions">
              <button class="btn-add" @click="addAchievement">添加成就</button>
              <button class="btn-secondary" @click="importAchievements">导入</button>
              <button class="btn-secondary" @click="exportAchievements">导出</button>
            </div>
            <div class="list-container">
              <div v-for="(ach, index) in configs.achievements.achievements" :key="index" class="list-item expandable">
                <div class="item-header" @click="toggleExpand('achievement', index)">
                  <span>{{ ach.displayName || ach.name || '未命名' }}</span>
                  <button class="btn-icon" @click.stop="removeAchievement(index)">×</button>
                </div>
                <div v-if="expandedItems[`achievement-${index}`]" class="item-body">
                  <input v-model="ach.name" placeholder="成就ID" />
                  <input v-model="ach.displayName" placeholder="显示名称" />
                  <textarea v-model="ach.description" placeholder="描述" rows="2"></textarea>
                  <label class="checkbox-label">
                    <input v-model="ach.hidden" type="checkbox" />
                    <span>隐藏成就</span>
                  </label>
                </div>
              </div>
            </div>
          </div>

          <!-- 统计配置 -->
          <div v-if="activeTab === 'stats'" class="config-panel">
            <h3>统计配置 (stats.json)</h3>
            <div class="panel-actions">
              <button class="btn-add" @click="addStat">添加统计项</button>
            </div>
            <div class="list-container">
              <div v-for="(stat, index) in configs.stats.stats" :key="index" class="list-item expandable">
                <div class="item-header" @click="toggleExpand('stat', index)">
                  <span>{{ stat.name || '未命名' }} ({{ stat.type }})</span>
                  <button class="btn-icon" @click.stop="removeStat(index)">×</button>
                </div>
                <div v-if="expandedItems[`stat-${index}`]" class="item-body">
                  <input v-model="stat.name" placeholder="统计名称" />
                  <select v-model="stat.type">
                    <option value="int">整数</option>
                    <option value="float">浮点数</option>
                    <option value="avgrate">平均值</option>
                  </select>
                  <input v-model.number="stat.defaultValue" type="number" placeholder="默认值" />
                </div>
              </div>
            </div>
          </div>

          <!-- 物品配置 -->
          <div v-if="activeTab === 'items'" class="config-panel">
            <h3>物品配置 (items.json)</h3>
            <div class="panel-actions">
              <button class="btn-add" @click="addItem">添加物品</button>
            </div>
            <div class="list-container">
              <div v-for="(item, index) in configs.items.itemDefinitions" :key="index" class="list-item">
                <input v-model="item.id" placeholder="物品ID" />
                <input v-model="item.name" placeholder="物品名称" />
                <input v-model.number="item.maxStackSize" type="number" placeholder="最大堆叠" />
                <button class="btn-icon" @click="removeItem(index)">×</button>
              </div>
            </div>
          </div>

          <!-- 模组配置 -->
          <div v-if="activeTab === 'mods'" class="config-panel">
            <h3>模组配置 (mods.json)</h3>
            <div class="panel-actions">
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

          <!-- 排行榜配置 -->
          <div v-if="activeTab === 'leaderboards'" class="config-panel">
            <h3>排行榜配置 (leaderboards.txt)</h3>
            <div class="panel-actions">
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

          <!-- 控制器配置 -->
          <div v-if="activeTab === 'controller'" class="config-panel">
            <h3>控制器配置</h3>
            <div class="form-group">
              <label>控制器类型</label>
              <select v-model="configs.controller.controllerType">
                <option value="xbox">Xbox</option>
                <option value="playstation">PlayStation</option>
                <option value="nintendo">Nintendo</option>
                <option value="generic">通用</option>
              </select>
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
          </div>

          <!-- 其他配置 -->
          <div v-if="activeTab === 'other'" class="config-panel">
            <h3>其他配置</h3>
            <div class="form-group">
              <label>已安装应用ID (每行一个)</label>
              <textarea v-model="otherConfigs.installedAppIds" rows="3" placeholder="480&#10;730"></textarea>
            </div>
            <div class="form-group">
              <label>订阅群组ID (每行一个)</label>
              <textarea v-model="otherConfigs.subscribedGroups" rows="3"></textarea>
            </div>
            <div class="form-group">
              <label>CD密钥 (每行一个)</label>
              <textarea v-model="otherConfigs.purchasedKeys" rows="3"></textarea>
            </div>
          </div>

          <!-- ColdClientLoader -->
          <div v-if="activeTab === 'coldclient'" class="config-panel">
            <h3>ColdClientLoader 配置</h3>
            <div class="form-group">
              <label class="checkbox-label">
                <input v-model="configs.coldClientLoader.enabled" type="checkbox" />
                <span>启用 ColdClientLoader</span>
              </label>
            </div>
            <div class="form-group">
              <label>注入模式</label>
              <select v-model="configs.coldClientLoader.injectionMode">
                <option value="direct">直接注入</option>
                <option value="loader">使用加载器</option>
              </select>
            </div>
            <div class="form-group">
              <label>启动参数</label>
              <input v-model="configs.coldClientLoader.launchArgs" placeholder="-windowed -novid" />
            </div>
            <div class="form-group">
              <label>额外DLL (每行一个)</label>
              <textarea v-model="coldClientDlls" rows="4" placeholder="extra.dll&#10;plugin.dll"></textarea>
            </div>
          </div>

          <!-- Lobby Connect -->
          <div v-if="activeTab === 'lobby'" class="config-panel">
            <h3>Lobby Connect 配置</h3>
            <div class="form-group">
              <label class="checkbox-label">
                <input v-model="configs.lobbyConnect.enabled" type="checkbox" />
                <span>启用 Lobby Connect</span>
              </label>
            </div>
            <div class="form-group">
              <label class="checkbox-label">
                <input v-model="configs.lobbyConnect.autoJoin" type="checkbox" />
                <span>自动加入大厅</span>
              </label>
            </div>
            <div class="form-group">
              <label>目标大厅ID</label>
              <input v-model="configs.lobbyConnect.targetLobbyId" placeholder="109775240970137214" />
            </div>
            <div class="form-group">
              <label>连接密码</label>
              <input v-model="configs.lobbyConnect.password" type="password" placeholder="可选" />
            </div>
          </div>
        </div>
      </div>

      <div class="modal-footer">
        <div class="footer-actions">
          <button class="btn-secondary" @click="loadAllConfigs">重新加载</button>
          <button class="btn-secondary" @click="exportAllConfigs">导出全部</button>
        </div>
        <div class="footer-actions">
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
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open, save } from '@tauri-apps/plugin-dialog'
import type { 
  MainConfig, UserConfig, AppConfig, OverlayConfig,
  AchievementsConfig, StatsConfig, ItemsConfig, ModsConfig, LeaderboardsConfig,
  ControllerConfig, ColdClientLoaderConfig, LobbyConnectConfig,
  Achievement, StatItem, ItemDefinition, WorkshopMod, Leaderboard
} from '../../../src/types/steam-config.types'

const props = defineProps<{
  gamePath: string
  gameId: string
}>()

const emit = defineEmits<{
  close: []
  saved: []
}>()

const activeTab = ref('main')
const expandedItems = ref<Record<string, boolean>>({})

// 导航配置
const coreConfigs = [
  { id: 'main', name: '主配置', icon: '⚙️' },
  { id: 'user', name: '用户配置', icon: '👤' },
  { id: 'app', name: '应用配置', icon: '📱' },
  { id: 'overlay', name: '覆盖层', icon: '🖥️' },
]

const gameFeatures = [
  { id: 'achievements', name: '成就系统', icon: '🏆' },
  { id: 'stats', name: '统计数据', icon: '📊' },
  { id: 'items', name: '物品库存', icon: '📦' },
  { id: 'mods', name: '创意工坊', icon: '🔧' },
  { id: 'leaderboards', name: '排行榜', icon: '🏅' },
  { id: 'controller', name: '控制器', icon: '🎮' },
  { id: 'other', name: '其他配置', icon: '📋' },
]

const toolConfigs = [
  { id: 'coldclient', name: 'ColdClient', icon: '❄️' },
  { id: 'lobby', name: 'Lobby', icon: '👥' },
]

// 配置状态
const configStatus = ref<Record<string, boolean>>({})

// 配置数据 - 使用完整的默认配置
const configs = reactive({
  main: {
    // [main::general]
    new_app_ticket: true,
    gc_token: true,
    block_unknown_clients: false,
    steam_deck: false,
    enable_account_avatar: false,
    enable_voice_chat: false,
    immediate_gameserver_stats: false,
    matchmaking_server_list_actual_type: false,
    matchmaking_server_details_via_source_query: false,
    crash_printer_location: undefined,
    // [main::stats]
    disable_leaderboards_create_unknown: false,
    allow_unknown_stats: true,
    stat_achievement_progress_functionality: true,
    save_only_higher_stat_achievement_progress: true,
    paginated_achievements_icons: 10,
    record_playtime: false,
    // [main::connectivity]
    disable_lan_only: false,
    disable_networking: false,
    listen_port: 47584,
    offline: false,
    disable_sharing_stats_with_gameserver: false,
    disable_source_query: false,
    share_leaderboards_over_network: false,
    disable_lobby_creation: false,
    download_steamhttp_requests: false,
    // [main::misc]
    achievements_bypass: false,
    force_steamhttp_success: false,
    disable_steamoverlaygameid_env_var: false,
    enable_steam_preowned_ids: false,
    steam_game_stats_reports_dir: undefined,
    free_weekend: false,
    use_32bit_inventory_item_ids: false,
    // extra_dlls
    extra_dlls: []
  } as MainConfig,
  user: {
    username: 'Player',
    language: 'schinese',
    save_path: '%appdata%/GSE Saves',
    avatar_path: undefined,
    use_default_avatar: true,
    saves_folder_name: undefined,
    local_save_path: undefined,
    ticket: undefined
  } as UserConfig,
  app: { branch_name: 'public', dlcs: { unlock_all: true, individualDlcs: [] } } as AppConfig,
  overlay: { enable_experimental_overlay: false, hotkey: 'Shift+Tab', notifications: { achievement: true, friend: true } } as OverlayConfig,
  achievements: { enabled: true, showNotifications: true, achievements: [] } as AchievementsConfig,
  stats: { enabled: true, stats: [] } as StatsConfig,
  items: { enabled: true, itemDefinitions: [], initialItems: [] } as ItemsConfig,
  mods: { enabled: true, subscribedMods: [], autoUpdate: false } as ModsConfig,
  leaderboards: { enabled: true, leaderboards: [] } as LeaderboardsConfig,
  controller: { enabled: true, controllerType: 'xbox', deadzone: { leftStick: 0.1, rightStick: 0.1 } } as ControllerConfig,
  coldClientLoader: { enabled: false, injectionMode: 'direct', extraDlls: [], launchArgs: '' } as ColdClientLoaderConfig,
  lobbyConnect: { enabled: false, autoJoin: false, targetLobbyId: '', password: '' } as LobbyConnectConfig,
})

const otherConfigs = reactive({
  installedAppIds: '',
  subscribedGroups: '',
  purchasedKeys: '',
})

const coldClientDlls = computed({
  get: () => configs.coldClientLoader.extraDlls.join('\n'),
  set: (val: string) => { configs.coldClientLoader.extraDlls = val.split('\n').filter(s => s.trim()) }
})

// 展开/收起
function toggleExpand(type: string, index: number) {
  const key = `${type}-${index}`
  expandedItems.value[key] = !expandedItems.value[key]
}

// 成就操作
function addAchievement() {
  configs.achievements.achievements.push({ name: '', displayName: '', description: '', hidden: false } as Achievement)
}
function removeAchievement(index: number) {
  configs.achievements.achievements.splice(index, 1)
}

// 统计操作
function addStat() {
  configs.stats.stats.push({ name: '', type: 'int', defaultValue: 0 } as StatItem)
}
function removeStat(index: number) {
  configs.stats.stats.splice(index, 1)
}

// 物品操作
function addItem() {
  configs.items.itemDefinitions.push({ id: '', name: '', stackable: true, maxStackSize: 99 } as ItemDefinition)
}
function removeItem(index: number) {
  configs.items.itemDefinitions.splice(index, 1)
}

// 模组操作
function addMod() {
  configs.mods.subscribedMods.push({ publishedFileId: '', title: '', visibility: 'public', files: [] } as WorkshopMod)
}
function removeMod(index: number) {
  configs.mods.subscribedMods.splice(index, 1)
}

// 排行榜操作
function addLeaderboard() {
  configs.leaderboards.leaderboards.push({ name: '', displayName: '', sortMethod: 'desc', displayType: 'numeric', entries: [] } as Leaderboard)
}
function removeLeaderboard(index: number) {
  configs.leaderboards.leaderboards.splice(index, 1)
}

// 导入导出
async function importAchievements() {
  const file = await open({ filters: [{ name: 'JSON', extensions: ['json'] }] })
  if (file) {
    const result = await invoke<{ success: boolean; achievements?: Achievement[] }>('import_achievements_from_file', { filePath: file })
    if (result.success && result.achievements) {
      configs.achievements.achievements.push(...result.achievements)
    }
  }
}

async function exportAchievements() {
  const result = await invoke<{ success: boolean; data?: string }>('export_achievements_config', { gamePath: props.gamePath })
  if (result.success && result.data) {
    const file = await save({ filters: [{ name: 'JSON', extensions: ['json'] }] })
    if (file) {
      await invoke('write_text_file', { path: file, content: result.data })
    }
  }
}

async function exportAllConfigs() {
  const allConfigs = JSON.stringify(configs, null, 2)
  const file = await save({ filters: [{ name: 'JSON', extensions: ['json'] }], defaultPath: 'steam_settings_backup.json' })
  if (file) {
    await invoke('write_text_file', { path: file, content: allConfigs })
  }
}

// 保存所有配置
async function saveAllConfigs() {
  try {
    const promises = [
      invoke('save_main_config', { gamePath: props.gamePath, config: configs.main }),
      invoke('save_user_config', { gamePath: props.gamePath, config: configs.user }),
      invoke('save_overlay_config', { gamePath: props.gamePath, config: configs.overlay }),
      invoke('save_achievements_config', { gamePath: props.gamePath, config: configs.achievements }),
      invoke('save_stats_config', { gamePath: props.gamePath, config: configs.stats }),
      invoke('save_items_config', { gamePath: props.gamePath, config: configs.items }),
      invoke('save_mods_config', { gamePath: props.gamePath, config: configs.mods }),
      invoke('save_leaderboards_config', { gamePath: props.gamePath, config: configs.leaderboards }),
      invoke('save_controller_config', { gamePath: props.gamePath, config: configs.controller }),
    ]
    await Promise.all(promises)
    emit('saved')
    alert('所有配置已保存！')
  } catch (error) {
    alert(`保存失败: ${error}`)
  }
}

// 加载所有配置
async function loadAllConfigs() {
  const results = await Promise.all([
    invoke<{ exists: boolean; config?: MainConfig }>('load_main_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: UserConfig }>('load_user_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: OverlayConfig }>('load_overlay_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: AchievementsConfig }>('load_achievements_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: StatsConfig }>('load_stats_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: ItemsConfig }>('load_items_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: ModsConfig }>('load_mods_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: LeaderboardsConfig }>('load_leaderboards_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: ControllerConfig }>('load_controller_config', { gamePath: props.gamePath }),
  ])

  const [main, user, overlay, achievements, stats, items, mods, leaderboards, controller] = results

  if (main.exists && main.config) Object.assign(configs.main, main.config)
  if (user.exists && user.config) Object.assign(configs.user, user.config)
  if (overlay.exists && overlay.config) Object.assign(configs.overlay, overlay.config)
  if (achievements.exists && achievements.config) Object.assign(configs.achievements, achievements.config)
  if (stats.exists && stats.config) Object.assign(configs.stats, stats.config)
  if (items.exists && items.config) Object.assign(configs.items, items.config)
  if (mods.exists && mods.config) Object.assign(configs.mods, mods.config)
  if (leaderboards.exists && leaderboards.config) Object.assign(configs.leaderboards, leaderboards.config)
  if (controller.exists && controller.config) Object.assign(configs.controller, controller.config)

  // 更新配置状态
  configStatus.value = {
    main: main.exists,
    user: user.exists,
    overlay: overlay.exists,
    achievements: achievements.exists,
    stats: stats.exists,
    items: items.exists,
    mods: mods.exists,
    leaderboards: leaderboards.exists,
    controller: controller.exists,
  }
}

onMounted(() => {
  loadAllConfigs()
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
  max-width: 1100px;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
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
  background-color: rgba(139, 92, 246, 0.1);
  color: #8b5cf6;
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

.modal-body {
  flex: 1;
  overflow: hidden;
  display: flex;
}

.modal-footer {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid var(--steam-border);
  flex-shrink: 0;
}

.footer-actions {
  display: flex;
  gap: 12px;
}

/* 左侧导航 */
.config-nav {
  width: 200px;
  border-right: 1px solid var(--steam-border);
  overflow-y: auto;
  padding: 16px;
  flex-shrink: 0;
}

.nav-section {
  margin-bottom: 20px;
}

.nav-section h4 {
  font-size: 12px;
  font-weight: 600;
  color: var(--steam-text-secondary);
  text-transform: uppercase;
  margin: 0 0 8px 0;
  padding-left: 8px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 8px 12px;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s ease;
  background-color: transparent;
  color: var(--steam-text-primary);
  text-align: left;
}

.nav-item:hover {
  background-color: var(--steam-bg-tertiary);
}

.nav-item.active {
  background-color: var(--steam-accent-blue);
  color: white;
}

.nav-icon {
  font-size: 16px;
}

.nav-label {
  flex: 1;
}

.nav-status {
  font-size: 12px;
  color: #10b981;
}

/* 右侧内容 */
.config-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

.config-panel {
  animation: fadeIn 0.2s ease;
}

.config-panel h3 {
  font-size: 16px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 20px 0;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--steam-border);
}

.config-panel h4 {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 20px 0 12px 0;
}

/* 配置分组 */
.config-section {
  background-color: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 16px;
}

.config-section h4 {
  margin-top: 0;
  margin-bottom: 16px;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--steam-border);
  color: var(--steam-accent-blue);
}

/* 表单 */
.form-group {
  margin-bottom: 16px;
}

.form-group > label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-text-primary);
  margin-bottom: 6px;
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
  font-size: 14px;
  outline: none;
  transition: border-color 0.15s ease;
}

.form-group input:focus,
.form-group select:focus,
.form-group textarea:focus {
  border-color: var(--steam-accent-blue);
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 14px;
  color: var(--steam-text-primary);
  margin-bottom: 0;
}

.checkbox-label input[type="checkbox"] {
  width: 18px;
  height: 18px;
  accent-color: var(--steam-accent-blue);
  flex-shrink: 0;
  margin: 0;
}

.checkbox-label span {
  line-height: 1.4;
}

/* 面板操作 */
.panel-actions {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
}

/* 列表 */
.list-container {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.list-item {
  background-color: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  padding: 12px;
  display: flex;
  gap: 8px;
  align-items: center;
}

.list-item.expandable {
  flex-direction: column;
  align-items: stretch;
}

.item-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  cursor: pointer;
}

.item-body {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--steam-border);
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.list-item input,
.list-item select,
.list-item textarea {
  flex: 1;
  padding: 8px 10px;
  border: 1px solid var(--steam-border);
  border-radius: 6px;
  background-color: var(--steam-bg-primary);
  color: var(--steam-text-primary);
  font-size: 13px;
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

.btn-secondary {
  padding: 10px 16px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
}

.btn-secondary:hover {
  background-color: var(--steam-border);
}

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

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

@media (max-width: 768px) {
  .modal-body {
    flex-direction: column;
  }
  
  .config-nav {
    width: 100%;
    border-right: none;
    border-bottom: 1px solid var(--steam-border);
    display: flex;
    overflow-x: auto;
  }
  
  .nav-section {
    display: flex;
    gap: 4px;
  }
  
  .nav-section h4 {
    display: none;
  }
  
  .form-row {
    grid-template-columns: 1fr;
  }
}
</style>
