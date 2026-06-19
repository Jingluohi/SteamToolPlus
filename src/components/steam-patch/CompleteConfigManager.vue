<template>
  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-content complete-config" @click.stop>
      <!-- 头部 -->
      <div class="modal-header">
        <h3>完整配置管理器</h3>
        <div class="header-status" v-if="configuredCount > 0">
          <span class="status-badge">已配置 {{ configuredCount }}/{{ totalCount }} 项</span>
        </div>
        <button class="close-btn" @click="$emit('close')" title="关闭">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <!-- 搜索栏 -->
      <div class="search-bar">
        <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8"/>
          <line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
        <input 
          v-model="searchQuery" 
          type="text" 
          placeholder="搜索配置项..." 
          class="search-input"
        />
        <button v-if="searchQuery" class="clear-search" @click="searchQuery = ''">
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
              v-for="item in filteredCoreConfigs" 
              :key="item.id"
              class="nav-item"
              :class="{ 
                active: activeTab === item.id,
                configured: configStatus[item.id] 
              }"
              @click="activeTab = item.id"
            >
              <span class="nav-label">{{ item.name }}</span>
              <span v-if="configStatus[item.id]" class="nav-status configured">
                <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="13 4 6 11 3 8"/>
                </svg>
              </span>
            </button>
          </div>

          <div class="nav-section">
            <h4>游戏功能</h4>
            <button 
              v-for="item in filteredGameFeatures" 
              :key="item.id"
              class="nav-item"
              :class="{ 
                active: activeTab === item.id,
                configured: configStatus[item.id] 
              }"
              @click="activeTab = item.id"
            >
              <span class="nav-label">{{ item.name }}</span>
              <span v-if="configStatus[item.id]" class="nav-status configured">
                <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="13 4 6 11 3 8"/>
                </svg>
              </span>
            </button>
          </div>

          <div class="nav-section">
            <h4>工具集成</h4>
            <button 
              v-for="item in filteredToolConfigs" 
              :key="item.id"
              class="nav-item"
              :class="{ 
                active: activeTab === item.id,
                configured: configStatus[item.id] 
              }"
              @click="activeTab = item.id"
            >
              <span class="nav-label">{{ item.name }}</span>
              <span v-if="configStatus[item.id]" class="nav-status configured">
                <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="13 4 6 11 3 8"/>
                </svg>
              </span>
            </button>
          </div>
        </div>

        <!-- 右侧内容区 -->
        <div class="config-content">
          <!-- 主配置 -->
          <div v-if="activeTab === 'main'" class="config-panel">
            <h3>主配置 (configs.main.ini)</h3>

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
                  <input v-model="configs.main.new_app_ticket" type="checkbox" />
                  <span>生成新版认证票据 (new_app_ticket)</span>
                </label>
                <p class="field-hint">启用后生成新版 Steam 认证票据，大多数游戏需要开启</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.gc_token" type="checkbox" />
                  <span>启用游戏协调器令牌 (gc_token)</span>
                </label>
                <p class="field-hint">用于 Valve 游戏的 GC 认证，如 CS:GO、Dota2 等需要开启</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.block_unknown_clients" type="checkbox" />
                  <span>阻止未知客户端</span>
                </label>
                <p class="field-hint">阻止非 Steam 官方客户端连接，增强安全性</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.steam_deck" type="checkbox" />
                  <span>模拟 Steam Deck</span>
                </label>
                <p class="field-hint">让游戏认为运行在 Steam Deck 上，可能解锁 Deck 专属内容</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.enable_account_avatar" type="checkbox" />
                  <span>启用头像功能</span>
                </label>
                <p class="field-hint">允许游戏获取并显示用户头像，需配合头像文件使用</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.enable_voice_chat" type="checkbox" />
                  <span>启用语音聊天</span>
                </label>
                <p class="field-hint">启用 Steam 语音聊天功能，需要游戏支持</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.immediate_gameserver_stats" type="checkbox" />
                  <span>即时游戏服务器统计</span>
                </label>
                <p class="field-hint">立即上报游戏服务器统计数据，不等待批量提交</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.matchmaking_server_list_actual_type" type="checkbox" />
                  <span>匹配服务器列表实际类型</span>
                </label>
                <p class="field-hint">返回实际的服务器列表类型，而非强制局域网类型</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.matchmaking_server_details_via_source_query" type="checkbox" />
                  <span>通过 Source 查询获取服务器详情</span>
                </label>
                <p class="field-hint">使用 Source 协议查询服务器详细信息</p>
              </div>
              <div class="form-group">
                <label>崩溃日志位置</label>
                <input v-model="configs.main.crash_printer_location" placeholder="可选" />
                <p class="field-hint">设置崩溃日志输出目录，留空则不输出</p>
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
                <p class="field-hint">阻止游戏自动创建未预定义的排行榜，避免数据混乱</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.allow_unknown_stats" type="checkbox" />
                  <span>允许未知统计</span>
                </label>
                <p class="field-hint">允许游戏上报未预定义的统计数据，大多数游戏建议开启</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.stat_achievement_progress_functionality" type="checkbox" />
                  <span>统计成就进度功能</span>
                </label>
                <p class="field-hint">启用基于统计数据的成就进度追踪</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.save_only_higher_stat_achievement_progress" type="checkbox" />
                  <span>只保存更高的统计成就进度</span>
                </label>
                <p class="field-hint">仅当新进度高于旧进度时才保存，防止进度倒退</p>
              </div>
              <div class="form-group">
                <label>分页成就图标数量</label>
                <input v-model.number="configs.main.paginated_achievements_icons" type="number" />
                <p class="field-hint">每页加载的成就图标数量，默认 10</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.record_playtime" type="checkbox" />
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
                  <input v-model="configs.main.disable_lan_only" type="checkbox" />
                  <span>禁用仅局域网模式</span>
                </label>
                <p class="field-hint">取消局域网限制，允许非局域网连接。联机游戏建议关闭此项</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.disable_networking" type="checkbox" />
                  <span>禁用网络功能</span>
                </label>
                <p class="field-hint">完全禁用网络功能，纯单机游戏可开启</p>
              </div>
              <div class="form-row">
                <div class="form-group">
                  <label>监听端口</label>
                  <input v-model.number="configs.main.listen_port" type="number" />
                  <p class="field-hint">模拟器监听的 UDP 端口，默认 47584，联机时需保持一致</p>
                </div>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.offline" type="checkbox" />
                  <span>离线模式</span>
                </label>
                <p class="field-hint">强制离线模式，不尝试任何网络连接</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.disable_sharing_stats_with_gameserver" type="checkbox" />
                  <span>禁用与游戏服务器共享统计</span>
                </label>
                <p class="field-hint">阻止统计数据发送到游戏服务器</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.disable_source_query" type="checkbox" />
                  <span>禁用 Source 查询</span>
                </label>
                <p class="field-hint">禁用 Source 协议服务器查询</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.share_leaderboards_over_network" type="checkbox" />
                  <span>网络共享排行榜</span>
                </label>
                <p class="field-hint">通过网络与其他玩家共享排行榜数据</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.disable_lobby_creation" type="checkbox" />
                  <span>禁用大厅创建</span>
                </label>
                <p class="field-hint">阻止游戏创建 Steam 大厅，纯单机游戏可开启</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.download_steamhttp_requests" type="checkbox" />
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
                  <input v-model="configs.main.achievements_bypass" type="checkbox" />
                  <span>成就绕过</span>
                </label>
                <p class="field-hint">绕过成就解锁限制，允许解锁所有成就</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.force_steamhttp_success" type="checkbox" />
                  <span>强制 SteamHTTP 成功</span>
                </label>
                <p class="field-hint">强制所有 SteamHTTP 请求返回成功，避免网络请求失败导致的问题</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.disable_steamoverlaygameid_env_var" type="checkbox" />
                  <span>禁用 Steam 覆盖层游戏 ID 环境变量</span>
                </label>
                <p class="field-hint">阻止设置 SteamOverlayGameId 环境变量</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.enable_steam_preowned_ids" type="checkbox" />
                  <span>启用 Steam 预拥有 ID</span>
                </label>
                <p class="field-hint">模拟 Steam 预拥有游戏 ID 列表</p>
              </div>
              <div class="form-group">
                <label>Steam 游戏统计报告目录</label>
                <input v-model="configs.main.steam_game_stats_reports_dir" placeholder="可选" />
                <p class="field-hint">设置游戏统计报告输出目录</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.free_weekend" type="checkbox" />
                  <span>免费周末</span>
                </label>
                <p class="field-hint">模拟免费周末活动状态</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.use_32bit_inventory_item_ids" type="checkbox" />
                  <span>使用 32 位库存物品 ID</span>
                </label>
                <p class="field-hint">使用 32 位而非 64 位物品 ID，兼容旧版游戏</p>
              </div>
            </div>
          </div>

          <!-- 用户配置 -->
          <div v-if="activeTab === 'user'" class="config-panel">
            <h3>用户配置 (configs.user.ini)</h3>
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
                  <span class="format-label">用户配置文件</span>
                  <span class="format-value">configs.user.ini</span>
                </div>
                <div class="format-item">
                  <span class="format-label">用户名</span>
                  <span class="format-value">自定义用户名，如 Player</span>
                </div>
                <div class="format-item">
                  <span class="format-label">语言代码</span>
                  <span class="format-value">schinese / tchinese / english / japanese / korean</span>
                </div>
              </div>
            </div>
            <div class="form-group">
              <label>用户名</label>
              <input v-model="configs.user.username" placeholder="Player" />
            </div>
            <div class="form-group">
              <label>SteamID（Steam64 格式，可选）</label>
              <input v-model="configs.user.account_steamid" placeholder="76561197960287930" />
              <p class="field-hint">无效 ID 会被模拟器忽略，留空则自动生成</p>
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
              <label>IP 国家代码</label>
              <input v-model="configs.user.ip_country" placeholder="CN" />
              <p class="field-hint">ISO 3166-1-alpha-2 格式，游戏查询 IP 时上报的国家代码</p>
            </div>
            <div class="form-group">
              <label>存档文件夹名称</label>
              <input v-model="configs.user.saves_folder_name" placeholder="覆盖默认的 GSE Saves" />
            </div>
            <div class="form-group">
              <label>本地存档路径（便携模式）</label>
              <input v-model="configs.user.local_save_path" placeholder="设置后完全便携" />
            </div>
            <div class="form-group">
              <label>EncryptedAppTicket (Base64)</label>
              <textarea v-model="configs.user.ticket" placeholder="可选，用于需要票据验证的游戏" rows="3"></textarea>
            </div>
          </div>

          <!-- 应用配置 -->
          <div v-if="activeTab === 'app'" class="config-panel">
            <h3>应用配置 (configs.app.ini)</h3>
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
                  <span class="format-label">应用配置文件</span>
                  <span class="format-value">configs.app.ini</span>
                </div>
                <div class="format-item">
                  <span class="format-label">分支名称</span>
                  <span class="format-value">public（默认）或其他分支名</span>
                </div>
                <div class="format-item">
                  <span class="format-label">DLC 解锁</span>
                  <span class="format-value">勾选"解锁所有"或手动指定 DLC ID</span>
                </div>
                <div class="format-item">
                  <span class="format-label">用途</span>
                  <span class="format-value">控制应用版本分支和 DLC 解锁策略</span>
                </div>
              </div>
              <div class="format-example">
                <span class="format-example-title">DLC 列表示例（支持带名称）：</span>
                <pre class="format-code">367680=RimWorld - Name in Game Access
990430=RimWorld - Soundtrack
1149640=RimWorld - Royalty
1392840=RimWorld - Ideology</pre>
              </div>
            </div>

            <!-- 分支名称 -->
            <div class="form-group">
              <label>分支名称</label>
              <input v-model="configs.app.branch_name" placeholder="public" />
              <p class="field-hint">Steam 应用分支名称，默认为 public。部分游戏有 beta 分支可填写对应名称</p>
            </div>

            <!-- DLC 模式选择（单选互斥） -->
            <div class="dlc-mode-selector">
              <label class="dlc-mode-option">
                <input 
                  type="radio" 
                  name="dlcMode" 
                  :checked="configs.app.dlcs.unlock_all" 
                  @change="onDlcModeChange(true)" 
                />
                <span class="dlc-mode-label">解锁所有 DLC</span>
              </label>
              <label class="dlc-mode-option">
                <input 
                  type="radio" 
                  name="dlcMode" 
                  :checked="!configs.app.dlcs.unlock_all" 
                  @change="onDlcModeChange(false)" 
                />
                <span class="dlc-mode-label">手动指定 DLC</span>
              </label>
            </div>
            <p class="field-hint" style="margin-top: -8px; margin-bottom: 16px;">
              部分游戏不适用"解锁所有"，可手动指定需要解锁的 DLC ID。两种模式互斥，只能选一种
            </p>

            <!-- 手动指定 DLC 列表 -->
            <div v-if="!configs.app.dlcs.unlock_all" class="dlc-manual-section">
              <div class="form-group">
                <label>DLC 列表</label>
                <textarea
                  v-model="configs.app.dlcs.custom_list"
                  class="dlc-textarea"
                  rows="6"
                  placeholder="格式1（纯ID）:&#10;123456&#10;789012&#10;&#10;格式2（带名称）:&#10;367680=RimWorld - Royalty&#10;1392840=RimWorld - Ideology"
                ></textarea>
                <p class="field-hint">每行一个 DLC ID，支持 "appid=DLC名称" 格式（名称仅用于显示）。在 SteamDB 搜索游戏可查看所有 DLC ID</p>
              </div>

            </div>

            <!-- Steam Input 控制器配置 -->
            <div class="config-section">
              <h4>Steam Input 控制器</h4>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.app.controller.steam_input" type="checkbox" />
                  <span>启用 Steam Input</span>
                </label>
                <p class="field-hint">启用后，模拟器会模拟 Steam Input API，让游戏识别手柄输入</p>
              </div>
              <div class="form-group">
                <label>控制器类型</label>
                <select v-model="configs.app.controller.type">
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
                  <input v-model="configs.app.cloud_saves.enabled" type="checkbox" />
                  <span>启用云存档</span>
                </label>
                <p class="field-hint">启用后，模拟器会模拟 Steam 云存档功能，支持存档同步</p>
              </div>
              <div class="form-row">
                <label class="checkbox-label">
                  <input v-model="configs.app.cloud_saves.create_default_dir" type="checkbox" />
                  <span>自动创建默认目录</span>
                </label>
                <label class="checkbox-label">
                  <input v-model="configs.app.cloud_saves.create_specific_dirs" type="checkbox" />
                  <span>自动创建特定目录</span>
                </label>
              </div>
              <p class="field-hint">自动创建目录可避免游戏因找不到存档目录而报错</p>
            </div>
          </div>

          <!-- 覆盖层配置 -->
          <div v-if="activeTab === 'overlay'" class="config-panel">
            <h3>覆盖层配置 (configs.overlay.ini)</h3>
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
                  <span class="format-label">覆盖层文件</span>
                  <span class="format-value">configs.overlay.ini</span>
                </div>
                <div class="format-item">
                  <span class="format-label">快捷键</span>
                  <span class="format-value">如 Shift+Tab，用于呼出游戏内覆盖层</span>
                </div>
                <div class="format-item">
                  <span class="format-label">通知类型</span>
                  <span class="format-value">成就通知 / 好友通知</span>
                </div>
                <div class="format-item">
                  <span class="format-label">用途</span>
                  <span class="format-value">模拟 Steam 游戏内覆盖层体验</span>
                </div>
              </div>
            </div>
            <div class="form-group">
              <label class="checkbox-label">
                <input v-model="configs.overlay.enable_experimental_overlay" type="checkbox" />
                <span>启用实验性游戏内覆盖层 (Shift+Tab)</span>
              </label>
            </div>
            <div class="form-group">
              <label>快捷键</label>
              <input v-model="configs.overlay.overlay_hotkey" placeholder="shift + tab" />
            </div>
            <h4>通知与功能开关</h4>
            <div class="form-row">
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.disable_achievement_notification" type="checkbox" />
                <span>禁用成就通知</span>
              </label>
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.disable_friend_notification" type="checkbox" />
                <span>禁用好友通知</span>
              </label>
            </div>
            <div class="form-row">
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.disable_achievement_progress" type="checkbox" />
                <span>禁用成就进度</span>
              </label>
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.disable_warning_any" type="checkbox" />
                <span>禁用所有警告</span>
              </label>
            </div>
            <div class="form-row">
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.overlay_always_show_fps" type="checkbox" />
                <span>始终显示 FPS</span>
              </label>
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.overlay_always_show_playtime" type="checkbox" />
                <span>始终显示游玩时间</span>
              </label>
            </div>

            <!-- 性能设置 -->
            <h4>性能设置</h4>
            <div class="form-group">
              <label>FPS 限制</label>
              <input v-model.number="configs.overlay.performance.fps_limit" type="number" min="1" max="240" />
            </div>
            <div class="form-row">
              <label class="checkbox-label">
                <input v-model="configs.overlay.performance.hardware_acceleration" type="checkbox" />
                <span>硬件加速</span>
              </label>
              <label class="checkbox-label">
                <input v-model="configs.overlay.performance.low_performance_mode" type="checkbox" />
                <span>低性能模式</span>
              </label>
            </div>

            <!-- 功能开关 -->
            <h4>覆盖层功能</h4>
            <div class="form-row">
              <label class="checkbox-label">
                <input v-model="configs.overlay.features.achievements" type="checkbox" />
                <span>成就</span>
              </label>
              <label class="checkbox-label">
                <input v-model="configs.overlay.features.friends" type="checkbox" />
                <span>好友</span>
              </label>
            </div>
            <div class="form-row">
              <label class="checkbox-label">
                <input v-model="configs.overlay.features.chat" type="checkbox" />
                <span>聊天</span>
              </label>
              <label class="checkbox-label">
                <input v-model="configs.overlay.features.browser" type="checkbox" />
                <span>浏览器</span>
              </label>
            </div>
          </div>

          <!-- 成就配置 -->
          <div v-if="activeTab === 'achievements'" class="config-panel">
            <h3>成就配置 (achievements.json)</h3>
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
                  <span class="format-label">成就数据文件</span>
                  <span class="format-value">achievements.json</span>
                </div>
                <div class="format-item">
                  <span class="format-label">成就图标</span>
                  <span class="format-value">PNG/JPG/BMP，推荐 64x64 或 128x128</span>
                </div>
                <div class="format-item">
                  <span class="format-label">成就ID</span>
                  <span class="format-value">英文字母+下划线，如 achievement_first_blood</span>
                </div>
                <div class="format-item">
                  <span class="format-label">导入格式</span>
                  <span class="format-value">JSON 数组，包含 name、displayName、description</span>
                </div>
              </div>
              <div class="format-example">
                <span class="format-example-title">JSON 示例：</span>
                <pre class="format-code">[
  {
    "name": "achievement_first_blood",
    "displayName": "第一滴血",
    "description": "完成首次击杀",
    "hidden": false
  }
]</pre>
              </div>
            </div>
            <div class="panel-actions">
              <button class="btn-add" @click="addAchievement">添加成就</button>
              <button class="btn-secondary" @click="importAchievements">导入</button>
              <button class="btn-secondary" @click="exportAchievements">导出</button>
            </div>
            <div class="list-container">
              <div v-for="(ach, index) in configs.achievements.achievements" :key="index" class="list-item expandable">
                <div class="item-header" @click="toggleExpand('achievement', index)">
                  <span class="item-title">{{ ach.displayName || ach.name || '未命名' }}</span>
                  <span class="item-badge" v-if="ach.hidden">隐藏</span>
                  <button class="btn-icon" @click.stop="removeAchievement(index)">×</button>
                </div>
                <div v-if="expandedItems[`achievement-${index}`]" class="item-body">
                  <div class="form-group">
                    <label>成就ID</label>
                    <input v-model="ach.name" placeholder="achievement_name" />
                  </div>
                  <div class="form-group">
                    <label>显示名称</label>
                    <input v-model="ach.displayName" placeholder="成就显示名称" />
                  </div>
                  <div class="form-group">
                    <label>描述</label>
                    <textarea v-model="ach.description" placeholder="成就描述" rows="2"></textarea>
                  </div>
                  <label class="checkbox-label">
                    <input v-model="ach.hidden" type="checkbox" />
                    <span>隐藏成就</span>
                  </label>
                </div>
              </div>
              <div v-if="configs.achievements.achievements.length === 0" class="empty-state">
                <p>暂无成就配置，点击"添加成就"开始配置</p>
              </div>
            </div>
          </div>

          <!-- 统计配置 -->
          <div v-if="activeTab === 'stats'" class="config-panel">
            <h3>统计配置 (stats.json)</h3>
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
                  <span class="format-label">统计数据文件</span>
                  <span class="format-value">stats.json</span>
                </div>
                <div class="format-item">
                  <span class="format-label">统计类型</span>
                  <span class="format-value">int / float / avgrate</span>
                </div>
                <div class="format-item">
                  <span class="format-label">统计名称</span>
                  <span class="format-value">英文字母+下划线，如 kills、deaths</span>
                </div>
                <div class="format-item">
                  <span class="format-label">默认值</span>
                  <span class="format-value">整数填 0，浮点数填 0.0</span>
                </div>
              </div>
            </div>
            <div class="panel-actions">
              <button class="btn-add" @click="addStat">添加统计项</button>
            </div>
            <div class="list-container">
              <div v-for="(stat, index) in configs.stats.stats" :key="index" class="list-item expandable">
                <div class="item-header" @click="toggleExpand('stat', index)">
                  <span class="item-title">{{ stat.name || '未命名' }}</span>
                  <span class="item-badge">{{ stat.type }}</span>
                  <button class="btn-icon" @click.stop="removeStat(index)">×</button>
                </div>
                <div v-if="expandedItems[`stat-${index}`]" class="item-body">
                  <div class="form-group">
                    <label>统计名称</label>
                    <input v-model="stat.name" placeholder="stat_name" />
                  </div>
                  <div class="form-group">
                    <label>类型</label>
                    <select v-model="stat.type">
                      <option value="int">整数</option>
                      <option value="float">浮点数</option>
                      <option value="avgrate">平均值</option>
                    </select>
                  </div>
                  <div class="form-group">
                    <label>默认值</label>
                    <input v-model.number="stat.defaultValue" type="number" placeholder="0" />
                  </div>
                </div>
              </div>
              <div v-if="configs.stats.stats.length === 0" class="empty-state">
                <p>暂无统计配置，点击"添加统计项"开始配置</p>
              </div>
            </div>
          </div>

          <!-- 物品配置 -->
          <div v-if="activeTab === 'items'" class="config-panel">
            <h3>物品配置 (items.json)</h3>
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
                  <span class="format-label">物品定义文件</span>
                  <span class="format-value">items.json</span>
                </div>
                <div class="format-item">
                  <span class="format-label">物品图标</span>
                  <span class="format-value">PNG/JPG/BMP 格式</span>
                </div>
                <div class="format-item">
                  <span class="format-label">物品ID</span>
                  <span class="format-value">数字，如 1001、1002</span>
                </div>
                <div class="format-item">
                  <span class="format-label">最大堆叠</span>
                  <span class="format-value">正整数，表示该物品最大堆叠数量</span>
                </div>
              </div>
            </div>
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
              <div v-if="configs.items.itemDefinitions.length === 0" class="empty-state">
                <p>暂无物品配置，点击"添加物品"开始配置</p>
              </div>
            </div>
          </div>

          <!-- 模组配置 -->
          <div v-if="activeTab === 'mods'" class="config-panel">
            <h3>模组配置 (mods.json)</h3>
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
                  <span class="format-label">模组定义文件</span>
                  <span class="format-value">mods.json</span>
                </div>
                <div class="format-item">
                  <span class="format-label">模组ID</span>
                  <span class="format-value">Steam 创意工坊文件 ID（纯数字）</span>
                </div>
                <div class="format-item">
                  <span class="format-label">可见性</span>
                  <span class="format-value">public / friends / private</span>
                </div>
                <div class="format-item">
                  <span class="format-label">获取方式</span>
                  <span class="format-value">在 Steam 创意工坊页面查看模组 URL</span>
                </div>
              </div>
              <div class="format-example">
                <span class="format-example-title">JSON 示例：</span>
                <pre class="format-code">[
  {
    "publishedFileId": "123456789",
    "title": "My Awesome Mod",
    "visibility": "public"
  }
]</pre>
              </div>
            </div>
            <div class="panel-actions">
              <button class="btn-add" @click="addMod">添加模组</button>
            </div>
            <div class="list-container">
              <div v-for="(mod, index) in configs.mods.subscribedMods" :key="index" class="list-item expandable">
                <div class="item-header" @click="toggleExpand('mod', index)">
                  <span class="item-title">{{ mod.title || '未命名' }}</span>
                  <span class="item-badge">{{ mod.visibility }}</span>
                  <button class="btn-icon" @click.stop="removeMod(index)">×</button>
                </div>
                <div v-if="expandedItems[`mod-${index}`]" class="item-body">
                  <div class="form-group">
                    <label>模组ID</label>
                    <input v-model="mod.publishedFileId" placeholder="模组文件ID" />
                  </div>
                  <div class="form-group">
                    <label>模组标题</label>
                    <input v-model="mod.title" placeholder="模组标题" />
                  </div>
                  <div class="form-group">
                    <label>可见性</label>
                    <select v-model="mod.visibility">
                      <option value="public">公开</option>
                      <option value="friends">好友</option>
                      <option value="private">私有</option>
                    </select>
                  </div>
                </div>
              </div>
              <div v-if="configs.mods.subscribedMods.length === 0" class="empty-state">
                <p>暂无模组配置，点击"添加模组"开始配置</p>
              </div>
            </div>
          </div>

          <!-- 排行榜配置 -->
          <div v-if="activeTab === 'leaderboards'" class="config-panel">
            <h3>排行榜配置 (leaderboards.txt)</h3>
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
                  <span class="format-label">排行榜定义文件</span>
                  <span class="format-value">leaderboards.txt</span>
                </div>
                <div class="format-item">
                  <span class="format-label">排行榜名称</span>
                  <span class="format-value">英文字母+下划线，如 high_score</span>
                </div>
                <div class="format-item">
                  <span class="format-label">排序方式</span>
                  <span class="format-value">asc（升序）或 desc（降序）</span>
                </div>
                <div class="format-item">
                  <span class="format-label">显示类型</span>
                  <span class="format-value">numeric（数字）或 time-sec（时间）</span>
                </div>
              </div>
              <div class="format-example">
                <span class="format-example-title">格式示例（NAME=sort=display）：</span>
                <pre class="format-code">high_score=desc=numeric
best_time=asc=time-sec</pre>
              </div>
            </div>
            <div class="panel-actions">
              <button class="btn-add" @click="addLeaderboard">添加排行榜</button>
            </div>
            <div class="list-container">
              <div v-for="(lb, index) in configs.leaderboards.leaderboards" :key="index" class="list-item expandable">
                <div class="item-header" @click="toggleExpand('leaderboard', index)">
                  <span class="item-title">{{ lb.displayName || lb.name || '未命名' }}</span>
                  <span class="item-badge">{{ lb.sortMethod === 'asc' ? '升序' : '降序' }}</span>
                  <button class="btn-icon" @click.stop="removeLeaderboard(index)">×</button>
                </div>
                <div v-if="expandedItems[`leaderboard-${index}`]" class="item-body">
                  <div class="form-group">
                    <label>排行榜ID</label>
                    <input v-model="lb.name" placeholder="leaderboard_name" />
                  </div>
                  <div class="form-group">
                    <label>显示名称</label>
                    <input v-model="lb.displayName" placeholder="排行榜显示名称" />
                  </div>
                  <div class="form-group">
                    <label>排序方式</label>
                    <select v-model="lb.sortMethod">
                      <option value="asc">升序</option>
                      <option value="desc">降序</option>
                    </select>
                  </div>
                </div>
              </div>
              <div v-if="configs.leaderboards.leaderboards.length === 0" class="empty-state">
                <p>暂无排行榜配置，点击"添加排行榜"开始配置</p>
              </div>
            </div>
          </div>

          <!-- 控制器配置 -->
          <div v-if="activeTab === 'controller'" class="config-panel">
            <h3>控制器配置 (controller.vdf)</h3>
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
                  <span class="format-label">控制器定义文件</span>
                  <span class="format-value">controller.vdf</span>
                </div>
                <div class="format-item">
                  <span class="format-label">控制器类型</span>
                  <span class="format-value">xbox / playstation / nintendo / generic</span>
                </div>
                <div class="format-item">
                  <span class="format-label">摇杆死区</span>
                  <span class="format-value">0.0 ~ 1.0，推荐 0.1 ~ 0.2</span>
                </div>
                <div class="format-item">
                  <span class="format-label">按键映射</span>
                  <span class="format-value">Steam Input 标准按键名称</span>
                </div>
              </div>
            </div>
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
                  <span class="format-label">已安装应用ID</span>
                  <span class="format-value">每行一个纯数字，如 480、730</span>
                </div>
                <div class="format-item">
                  <span class="format-label">订阅群组ID</span>
                  <span class="format-value">每行一个纯数字，Steam 群组 ID</span>
                </div>
                <div class="format-item">
                  <span class="format-label">CD密钥</span>
                  <span class="format-value">每行一个密钥，如 XXXXX-XXXXX-XXXXX</span>
                </div>
                <div class="format-item">
                  <span class="format-label">获取方式</span>
                  <span class="format-value">在 SteamDB 上搜索游戏查看相关信息</span>
                </div>
              </div>
            </div>
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
                  <span class="format-label">注入模式</span>
                  <span class="format-value">direct（直接注入）或 loader（使用加载器）</span>
                </div>
                <div class="format-item">
                  <span class="format-label">启动参数</span>
                  <span class="format-value">游戏启动命令行参数，如 -windowed -novid</span>
                </div>
                <div class="format-item">
                  <span class="format-label">额外DLL</span>
                  <span class="format-value">每行一个 DLL 文件名，如 extra.dll</span>
                </div>
                <div class="format-item">
                  <span class="format-label">用途</span>
                  <span class="format-value">绕过 Steam DRM，实现免Steam启动游戏</span>
                </div>
              </div>
            </div>
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
                  <span class="format-label">大厅ID</span>
                  <span class="format-value">纯数字，如 109775240970137214</span>
                </div>
                <div class="format-item">
                  <span class="format-label">连接密码</span>
                  <span class="format-value">可选，由房主设置的连接密码</span>
                </div>
                <div class="format-item">
                  <span class="format-label">用途</span>
                  <span class="format-value">直接加入指定 Steam 大厅，实现联机</span>
                </div>
                <div class="format-item">
                  <span class="format-label">注意事项</span>
                  <span class="format-value">需要游戏支持 Steam 大厅系统</span>
                </div>
              </div>
            </div>
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

      <!-- 底部操作栏 -->
      <div class="modal-footer">
        <button class="btn-cancel" @click="$emit('close')">取消</button>
        <button 
          class="btn-save" 
          @click="saveAllConfigs"
          :class="{ saving: isSaving, saved: isSaved }"
          :disabled="isSaving"
        >
          <svg v-if="!isSaving && !isSaved" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
            <polyline points="17 21 17 13 7 13 7 21"/>
            <polyline points="7 3 7 8 15 8"/>
          </svg>
          <svg v-if="isSaving" class="spin-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
          </svg>
          <svg v-if="isSaved" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="20 6 9 17 4 12"/>
          </svg>
          <span>{{ isSaving ? '保存中...' : isSaved ? '已保存' : '保存所有配置' }}</span>
        </button>
      </div>

    </div>

    <!-- 保存成功提示 -->
    <transition name="toast">
      <div v-if="showToast" class="toast-success">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
          <polyline points="22 4 12 14.01 9 11.01"/>
        </svg>
        <span>所有配置已保存成功！</span>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open, save } from '@tauri-apps/plugin-dialog'
import * as SteamTypes from '../../../src/types/steam-config.types'

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
const searchQuery = ref('')
const isSaving = ref(false)
const isSaved = ref(false)
const showToast = ref(false)
/**
 * DLC 模式切换（单选互斥）
 * @param unlockAll true=解锁所有 DLC，false=手动指定 DLC
 */
function onDlcModeChange(unlockAll: boolean) {
  configs.app.dlcs.unlock_all = unlockAll
}

// 导航配置
const coreConfigs = [
  { id: 'main', name: '主配置' },
  { id: 'user', name: '用户配置' },
  { id: 'app', name: '应用配置' },
  { id: 'overlay', name: '覆盖层' },
]

const gameFeatures = [
  { id: 'achievements', name: '成就系统' },
  { id: 'stats', name: '统计数据' },
  { id: 'items', name: '物品库存' },
  { id: 'mods', name: '创意工坊' },
  { id: 'leaderboards', name: '排行榜' },
  { id: 'controller', name: '控制器' },
  { id: 'other', name: '其他配置' },
]

const toolConfigs = [
  { id: 'coldclient', name: 'ColdClient' },
  { id: 'lobby', name: 'Lobby' },
]

// 搜索过滤
const filteredCoreConfigs = computed(() => {
  if (!searchQuery.value) return coreConfigs
  return coreConfigs.filter(c => c.name.toLowerCase().includes(searchQuery.value.toLowerCase()))
})

const filteredGameFeatures = computed(() => {
  if (!searchQuery.value) return gameFeatures
  return gameFeatures.filter(c => c.name.toLowerCase().includes(searchQuery.value.toLowerCase()))
})

const filteredToolConfigs = computed(() => {
  if (!searchQuery.value) return toolConfigs
  return toolConfigs.filter(c => c.name.toLowerCase().includes(searchQuery.value.toLowerCase()))
})

// 配置状态
const configStatus = ref<Record<string, boolean>>({})

// 计算已配置数量
const totalCount = computed(() => {
  return coreConfigs.length + gameFeatures.length + toolConfigs.length
})

const configuredCount = computed(() => {
  return Object.values(configStatus.value).filter(Boolean).length
})

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
  } as SteamTypes.MainConfig,
  user: {
    username: 'Player',
    account_steamid: undefined,
    language: 'schinese',
    ip_country: 'CN',
    saves_folder_name: undefined,
    local_save_path: undefined,
    ticket: undefined,
    alt_steamid: undefined,
    alt_steamid_count: undefined,
  } as SteamTypes.UserConfig,
  app: {
    branch_name: 'public',
    is_beta_branch: false,
    app_paths: {},
    dlcs: {
      unlock_all: true,
      custom_list: '',
    },
    // Steam Input 控制器配置
    controller: {
      steam_input: false,
      type: '',
    },
    // 云存档配置
    cloud_saves: {
      enabled: false,
      create_default_dir: false,
      create_specific_dirs: false,
      windows_dirs: [] as string[],
      linux_dirs: [] as string[],
    },
  } as SteamTypes.AppConfig,
  overlay: {
    enable_experimental_overlay: false,
    overlay_hotkey: 'shift + tab',
    notifications: {
      disable_achievement_notification: false,
      disable_friend_notification: false,
      disable_achievement_progress: false,
      disable_warning_any: false,
      disable_warning_bad_appid: false,
      disable_warning_local_save: false,
      upload_achievements_icons_to_gpu: true,
      overlay_always_show_user_info: false,
      overlay_always_show_fps: false,
      overlay_always_show_frametime: false,
      overlay_always_show_playtime: false,
    },
    appearance: {
      theme: 'dark',
      opacity: 0.95,
      scale: 1.0,
      blur: true,
    },
    performance: {
      hardware_acceleration: true,
      fps_limit: 60,
      low_performance_mode: false,
    },
    features: {
      achievements: true,
      friends: true,
      chat: true,
      browser: true,
      settings: true,
    },
  } as SteamTypes.OverlayConfig,
  achievements: { enabled: true, showNotifications: true, achievements: [] } as SteamTypes.AchievementsConfig,
  stats: { enabled: true, stats: [] } as SteamTypes.StatsConfig,
  items: { enabled: true, itemDefinitions: [], initialItems: [] } as SteamTypes.ItemsConfig,
  mods: { enabled: true, subscribedMods: [], autoUpdate: false } as SteamTypes.ModsConfig,
  leaderboards: { enabled: true, leaderboards: [] } as SteamTypes.LeaderboardsConfig,
  controller: { enabled: true, controllerType: 'xbox', deadzone: { leftStick: 0.1, rightStick: 0.1 } } as SteamTypes.ControllerConfig,
  coldClientLoader: { enabled: false, injectionMode: 'direct', extraDlls: [], launchArgs: '' } as SteamTypes.ColdClientLoaderConfig,
  lobbyConnect: { enabled: false, autoJoin: false, targetLobbyId: '', password: '' } as SteamTypes.LobbyConnectConfig,
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
  configs.achievements.achievements.push({ name: '', displayName: '', description: '', hidden: false } as SteamTypes.Achievement)
}
function removeAchievement(index: number) {
  configs.achievements.achievements.splice(index, 1)
}

// 统计操作
function addStat() {
  configs.stats.stats.push({ name: '', type: 'int', defaultValue: 0 } as SteamTypes.StatItem)
}
function removeStat(index: number) {
  configs.stats.stats.splice(index, 1)
}

// 物品操作
function addItem() {
  configs.items.itemDefinitions.push({ id: '', name: '', stackable: true, maxStackSize: 99 } as SteamTypes.ItemDefinition)
}
function removeItem(index: number) {
  configs.items.itemDefinitions.splice(index, 1)
}

// 模组操作
function addMod() {
  configs.mods.subscribedMods.push({ publishedFileId: '', title: '', visibility: 'public', files: [] } as SteamTypes.WorkshopMod)
}
function removeMod(index: number) {
  configs.mods.subscribedMods.splice(index, 1)
}

// 排行榜操作
function addLeaderboard() {
  configs.leaderboards.leaderboards.push({ name: '', displayName: '', sortMethod: 'desc', displayType: 'numeric', entries: [] } as SteamTypes.Leaderboard)
}
function removeLeaderboard(index: number) {
  configs.leaderboards.leaderboards.splice(index, 1)
}

// 导入导出
async function importAchievements() {
  const file = await open({ filters: [{ name: 'JSON', extensions: ['json'] }] })
  if (file) {
    const result = await invoke<{ success: boolean; achievements?: SteamTypes.Achievement[] }>('import_achievements_from_file', { filePath: file })
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

// 将前端 main 配置对象转换为 INI 字符串
function buildMainConfigIni(main: any): string {
  const lines: string[] = []
  const boolFields = (fields: string[]) => {
    for (const f of fields) {
      if (main[f]) lines.push(`${f} = 1`)
    }
  }

  // [main::general]
  lines.push('[main::general]')
  boolFields([
    'new_app_ticket', 'gc_token', 'block_unknown_clients', 'steam_deck',
    'enable_account_avatar', 'enable_voice_chat', 'immediate_gameserver_stats',
    'matchmaking_server_list_actual_type', 'matchmaking_server_details_via_source_query'
  ])
  if (main.crash_printer_location) {
    lines.push(`crash_printer_location = ${main.crash_printer_location}`)
  }

  // [main::stats]
  lines.push('')
  lines.push('[main::stats]')
  boolFields([
    'disable_leaderboards_create_unknown', 'allow_unknown_stats',
    'stat_achievement_progress_functionality', 'save_only_higher_stat_achievement_progress',
    'record_playtime'
  ])
  if (main.paginated_achievements_icons !== undefined && main.paginated_achievements_icons !== 10) {
    lines.push(`paginated_achievements_icons = ${main.paginated_achievements_icons}`)
  }

  // [main::connectivity]
  lines.push('')
  lines.push('[main::connectivity]')
  boolFields([
    'disable_lan_only', 'disable_networking', 'offline',
    'disable_sharing_stats_with_gameserver', 'disable_source_query',
    'share_leaderboards_over_network', 'disable_lobby_creation', 'download_steamhttp_requests'
  ])
  if (main.listen_port !== undefined && main.listen_port !== 47584) {
    lines.push(`listen_port = ${main.listen_port}`)
  }

  // [main::misc]
  lines.push('')
  lines.push('[main::misc]')
  boolFields([
    'achievements_bypass', 'force_steamhttp_success',
    'disable_steamoverlaygameid_env_var', 'enable_steam_preowned_ids',
    'free_weekend', 'use_32bit_inventory_item_ids'
  ])
  if (main.steam_game_stats_reports_dir) {
    lines.push(`steam_game_stats_reports_dir = ${main.steam_game_stats_reports_dir}`)
  }

  // [main::extra]
  if (main.extra_dlls && main.extra_dlls.length > 0) {
    lines.push('')
    lines.push('[main::extra]')
    for (const dll of main.extra_dlls) {
      lines.push(dll)
    }
  }

  return lines.join('\n')
}

// 解析主配置 INI 字符串为对象
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
      // extra dlls section
      if (!result.extra_dlls) result.extra_dlls = []
      result.extra_dlls.push(t)
      continue
    }
    const k = t.slice(0, i).trim()
    const v = t.slice(i + 1).trim()

    if (boolKeys.has(k)) {
      result[k] = v === '1' || v === 'true'
    } else if (intKeys.has(k)) {
      result[k] = parseInt(v, 10) || 0
    } else {
      result[k] = v
    }
  }
  return result
}

// 将前端 app 配置转换为 Rust 后端期望的格式
function buildAppConfigForRust() {
  return {
    branchName: configs.app.branch_name,
    isBetaBranch: configs.app.is_beta_branch || false,
    appPaths: {},
    controller: configs.app.controller || {},
    cloudSaves: configs.app.cloud_saves || {},
    dlcs: {
      unlockAll: configs.app.dlcs.unlock_all,
      dlcList: configs.app.dlcs.custom_list || '',
    }
  }
}

// 保存所有配置
async function saveAllConfigs() {
  if (isSaving.value) return
  
  try {
    isSaving.value = true
    isSaved.value = false
    
    const promises = [
      invoke('save_main_config', { gamePath: props.gamePath, config: { mainIni: buildMainConfigIni(configs.main) } }),
      invoke('save_user_config', { gamePath: props.gamePath, config: configs.user }),
      invoke('save_app_config', { gamePath: props.gamePath, config: buildAppConfigForRust() }),
      invoke('save_overlay_config', { gamePath: props.gamePath, config: configs.overlay }),
      invoke('save_achievements_config', { gamePath: props.gamePath, config: configs.achievements }),
      invoke('save_stats_config', { gamePath: props.gamePath, config: configs.stats }),
      invoke('save_items_config', { gamePath: props.gamePath, config: configs.items }),
      invoke('save_mods_config', { gamePath: props.gamePath, config: configs.mods }),
      invoke('save_leaderboards_config', { gamePath: props.gamePath, config: configs.leaderboards }),
      invoke('save_controller_config', { gamePath: props.gamePath, config: configs.controller }),
      invoke('save_coldclient_config', { gamePath: props.gamePath, config: configs.coldClientLoader }),
      invoke('save_lobby_connect_config', { gamePath: props.gamePath, config: configs.lobbyConnect }),
    ]
    await Promise.all(promises)
    
    isSaving.value = false
    isSaved.value = true
    
    // 显示成功提示
    showToast.value = true
    setTimeout(() => {
      showToast.value = false
      isSaved.value = false
    }, 3000)
    
    emit('saved')
  } catch (error) {
    isSaving.value = false
    alert(`保存失败: ${error}`)
  }
}

// 加载所有配置
async function loadAllConfigs() {
  const results = await Promise.all([
    invoke<{ exists: boolean; content?: string }>('load_main_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: SteamTypes.UserConfig }>('load_user_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: SteamTypes.SteamAppConfig }>('load_app_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: SteamTypes.OverlayConfig }>('load_overlay_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: SteamTypes.AchievementsConfig }>('load_achievements_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: SteamTypes.StatsConfig }>('load_stats_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: SteamTypes.ItemsConfig }>('load_items_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: SteamTypes.ModsConfig }>('load_mods_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: SteamTypes.LeaderboardsConfig }>('load_leaderboards_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: SteamTypes.ControllerConfig }>('load_controller_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: SteamTypes.ColdClientLoaderConfig }>('load_coldclient_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: SteamTypes.LobbyConnectConfig }>('load_lobby_connect_config', { gamePath: props.gamePath }),
  ])

  const [main, user, app, overlay, achievements, stats, items, mods, leaderboards, controller, coldclient, lobbyconnect] = results

  // 加载主配置（Rust 返回原始 INI 字符串，前端解析为对象）
  if (main.exists && main.content) {
    const parsed = parseMainConfigIni(main.content)
    Object.assign(configs.main, parsed)
  }

  // 加载用户配置（Rust 返回 snake_case，需要映射）
  if (user.exists && user.config) {
    const c = user.config as any
    Object.keys(c).forEach((key) => {
      const camelKey = key.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase())
      if (key in configs.user || camelKey in configs.user) {
        (configs.user as any)[camelKey || key] = c[key]
      }
    })
  }

  // 加载应用配置
  if (app.exists && app.config) {
    // 将 Rust 格式转换为前端格式（snake_case 字符串格式，用于 textarea 编辑）
    configs.app.branch_name = app.config.branch_name || 'public'
    configs.app.is_beta_branch = app.config.is_beta_branch || false
    configs.app.dlcs.unlock_all = app.config.dlcs?.unlock_all ?? true

    // 加载控制器配置
    if (app.config.controller) {
      configs.app.controller = {
        steam_input: app.config.controller.steam_input,
        type: app.config.controller.type,
      }
    }

    // 加载云存档配置
    if (app.config.cloud_saves) {
      configs.app.cloud_saves = {
        enabled: app.config.cloud_saves.enabled,
        create_default_dir: app.config.cloud_saves.create_default_dir,
        create_specific_dirs: app.config.cloud_saves.create_specific_dirs,
        windows_dirs: app.config.cloud_saves.windows_dirs || [],
        linux_dirs: app.config.cloud_saves.linux_dirs || [],
      }
    }

    // 将 individual_dlcs 转换为 custom_list 字符串（用于编辑，保留名称信息）
    if (app.config.dlcs?.individual_dlcs && app.config.dlcs.individual_dlcs.length > 0) {
      configs.app.dlcs.custom_list = app.config.dlcs.individual_dlcs
        .filter((d: any) => d.enabled)
        .map((d: any) => {
          // 如果有名称且与 app_id 不同，使用 appid=Name 格式
          if (d.name && d.name !== `DLC ${d.app_id}` && d.name !== d.app_id) {
            return `${d.app_id}=${d.name}`
          }
          return d.app_id
        })
        .join('\n')
    }
  }

  // 加载覆盖层配置（直接赋值，字段名一致）
  if (overlay.exists && overlay.config) {
    Object.assign(configs.overlay, overlay.config)
  }
  if (achievements.exists && achievements.config) Object.assign(configs.achievements, achievements.config)
  if (stats.exists && stats.config) Object.assign(configs.stats, stats.config)
  if (items.exists && items.config) Object.assign(configs.items, items.config)
  if (mods.exists && mods.config) Object.assign(configs.mods, mods.config)
  if (leaderboards.exists && leaderboards.config) Object.assign(configs.leaderboards, leaderboards.config)
  if (controller.exists && controller.config) Object.assign(configs.controller, controller.config)
  if (coldclient.exists && coldclient.config) Object.assign(configs.coldClientLoader, coldclient.config)
  if (lobbyconnect.exists && lobbyconnect.config) Object.assign(configs.lobbyConnect, lobbyconnect.config)

  // 更新配置状态
  configStatus.value = {
    main: main.exists,
    user: user.exists,
    app: app.exists,
    overlay: overlay.exists,
    achievements: achievements.exists,
    stats: stats.exists,
    items: items.exists,
    mods: mods.exists,
    leaderboards: leaderboards.exists,
    controller: controller.exists,
    coldclient: coldclient.exists,
    lobbyconnect: lobbyconnect.exists,
  }
}

onMounted(() => {
  loadAllConfigs()
})
</script>

<style scoped>
/* 遮罩层 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

/* 模态框主体 */
.modal-content {
  background-color: var(--steam-bg-primary);
  border-radius: 12px;
  border: 1px solid var(--steam-border);
  width: 90%;
  max-width: 1200px;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 24px 48px rgba(0, 0, 0, 0.3);
}

/* 头部 */
.modal-header {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px 24px;
  border-bottom: 1px solid var(--steam-border);
  flex-shrink: 0;
}

.modal-header h3 {
  flex: 1;
  font-size: 18px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0;
}

.header-status {
  display: flex;
  align-items: center;
}

.status-badge {
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-accent-blue);
  background-color: rgba(var(--steam-accent-blue-rgb, 107, 170, 255), 0.1);
  padding: 4px 12px;
  border-radius: 20px;
  border: 1px solid rgba(var(--steam-accent-blue-rgb, 107, 170, 255), 0.2);
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

/* 搜索栏 */
.search-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  border-bottom: 1px solid var(--steam-border);
  flex-shrink: 0;
}

.search-icon {
  width: 16px;
  height: 16px;
  color: var(--steam-text-secondary);
  flex-shrink: 0;
}

.search-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 13px;
  outline: none;
  transition: border-color 0.15s ease;
}

.search-input:focus {
  border-color: var(--steam-accent-blue);
}

.search-input::placeholder {
  color: var(--steam-text-secondary);
}

.clear-search {
  width: 24px;
  height: 24px;
  border: none;
  border-radius: 6px;
  background-color: transparent;
  color: var(--steam-text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.clear-search:hover {
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-primary);
}

/* 主体布局 */
.modal-body {
  flex: 1;
  overflow: hidden;
  display: flex;
}

/* 底部操作栏 */
.modal-footer {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  padding: 12px 24px;
  border-top: 1px solid var(--steam-border);
  flex-shrink: 0;
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
  font-size: 11px;
  font-weight: 600;
  color: var(--steam-text-secondary);
  text-transform: uppercase;
  margin: 0 0 8px 0;
  padding-left: 8px;
  letter-spacing: 0.5px;
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
  position: relative;
}

.nav-item:hover {
  background-color: var(--steam-bg-tertiary);
}

.nav-item.active {
  background-color: var(--steam-accent-blue);
  color: white;
}

.nav-item.configured:not(.active) {
  color: #10b981;
}

.nav-item.configured:not(.active)::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 60%;
  background-color: #10b981;
  border-radius: 2px;
}

.nav-label {
  flex: 1;
}

.nav-status {
  font-size: 12px;
  color: #10b981;
}

.nav-status svg {
  width: 14px;
  height: 14px;
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

/* 格式说明 */
.format-info {
  background-color: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  padding: 14px 16px;
  margin-bottom: 16px;
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
  margin-bottom: 10px;
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

.format-example {
  background-color: var(--steam-bg-primary);
  border-radius: 6px;
  padding: 10px 12px;
}

.format-example-title {
  font-size: 12px;
  font-weight: 500;
  color: var(--steam-text-secondary);
  margin-bottom: 6px;
  display: block;
}

.format-code {
  font-size: 12px;
  color: #e2e8f0;
  background-color: #1e293b;
  padding: 8px 12px;
  border-radius: 4px;
  overflow-x: auto;
  line-height: 1.5;
  margin: 0;
  white-space: pre;
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
  box-sizing: border-box;
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

.field-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 4px 0 0 0;
}

.dlc-textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
  color: var(--steam-text-primary);
  font-size: 13px;
  font-family: 'Consolas', 'Courier New', monospace;
  resize: vertical;
  outline: none;
  margin-top: 8px;
}

.dlc-textarea:focus {
  border-color: var(--steam-accent-blue);
}

/* DLC 模式选择器（单选互斥） */
.dlc-mode-selector {
  display: flex;
  gap: 24px;
  margin: 16px 0;
  padding: 12px 16px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--steam-border);
}

.dlc-mode-option {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 14px;
  color: var(--steam-text-primary);
}

.dlc-mode-option input[type="radio"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
}

.dlc-mode-label {
  font-weight: 500;
}

.dlc-manual-section {
  margin-top: 16px;
}

/* 高级选项 */
.advanced-toggle {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  color: var(--steam-text-secondary);
  font-size: 13px;
  padding: 10px 0;
  border-top: 1px solid var(--steam-border);
  margin-top: 12px;
}

.advanced-toggle:hover {
  color: var(--steam-accent-blue);
}

.chevron {
  width: 16px;
  height: 16px;
  transition: transform 0.2s ease;
  flex-shrink: 0;
}

.chevron.rotated {
  transform: rotate(180deg);
}

.advanced-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
}

.advanced-content {
  margin-top: 12px;
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
  padding: 4px 0;
}

.item-title {
  font-weight: 500;
  color: var(--steam-text-primary);
}

.item-badge {
  font-size: 11px;
  font-weight: 500;
  color: var(--steam-text-secondary);
  background-color: var(--steam-bg-tertiary);
  padding: 2px 8px;
  border-radius: 4px;
  text-transform: capitalize;
}

.item-body {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--steam-border);
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.item-body .form-group {
  margin-bottom: 0;
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

/* 空状态 */
.empty-state {
  text-align: center;
  padding: 32px 16px;
  color: var(--steam-text-secondary);
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
  transition: all 0.2s ease;
  background-color: var(--steam-accent-blue);
  color: white;
  position: relative;
  overflow: hidden;
}

.btn-primary:hover:not(:disabled) {
  background-color: var(--steam-accent-hover);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(107, 170, 255, 0.3);
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-primary.saved {
  background-color: #10b981;
}

.btn-secondary {
  display: flex;
  align-items: center;
  gap: 6px;
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
  flex-shrink: 0;
}

.btn-icon:hover {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

/* 旋转图标 */
.spin-icon {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* Toast 提示 */
.toast-success {
  position: absolute;
  bottom: 80px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 20px;
  background-color: #10b981;
  color: white;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  z-index: 9999;
}

.toast-success svg {
  width: 20px;
  height: 20px;
}

/* Toast 动画 */
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(-50%) translateY(20px);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(-20px);
}

/* 淡入动画 */
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

/* 取消按钮 */
.btn-cancel {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 6px 16px;
  border: 1px solid var(--steam-border);
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  background-color: var(--steam-bg-tertiary);
  color: var(--steam-text-secondary);
  white-space: nowrap;
}

.btn-cancel:hover {
  background-color: var(--steam-border);
  color: var(--steam-text-primary);
}

/* 保存按钮 */
.btn-save {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 6px 20px;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  background-color: var(--steam-accent-blue);
  color: white;
  white-space: nowrap;
  position: relative;
  overflow: hidden;
}

.btn-save:hover:not(:disabled) {
  background-color: var(--steam-accent-hover);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(107, 170, 255, 0.3);
}

.btn-save:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-save.saving {
  background-color: var(--steam-accent-blue);
  cursor: wait;
}

.btn-save.saved {
  background-color: #10b981;
}

.btn-save svg {
  width: 16px;
  height: 16px;
}

/* 响应式 */
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
  
  .modal-header h3 {
    font-size: 16px;
  }
  
  .status-badge {
    font-size: 12px;
  }
}
</style>
