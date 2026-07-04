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
          placeholder="搜索配置项或内容..." 
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
              <span v-if="isContentOnlyMatch(item, searchQuery)" class="content-match">内容</span>
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
              <span v-if="isContentOnlyMatch(item, searchQuery)" class="content-match">内容</span>
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
              <span v-if="isContentOnlyMatch(item, searchQuery)" class="content-match">内容</span>
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
                  <input v-model="configs.main.newAppTicket" type="checkbox" />
                  <span>生成新版认证票据 (new_app_ticket)</span>
                </label>
                <p class="field-hint">启用后生成新版 Steam 认证票据，大多数游戏需要开启</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.gcToken" type="checkbox" />
                  <span>启用游戏协调器令牌 (gc_token)</span>
                </label>
                <p class="field-hint">用于 Valve 游戏的 GC 认证，如 CS:GO、Dota2 等需要开启</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.blockUnknownClients" type="checkbox" />
                  <span>阻止未知客户端</span>
                </label>
                <p class="field-hint">阻止非 Steam 官方客户端连接，增强安全性</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.steamDeck" type="checkbox" />
                  <span>模拟 Steam Deck</span>
                </label>
                <p class="field-hint">让游戏认为运行在 Steam Deck 上，可能解锁 Deck 专属内容</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.enableAccountAvatar" type="checkbox" />
                  <span>启用头像功能</span>
                </label>
                <p class="field-hint">允许游戏获取并显示用户头像，需配合头像文件使用</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.enableVoiceChat" type="checkbox" />
                  <span>启用语音聊天</span>
                </label>
                <p class="field-hint">启用 Steam 语音聊天功能，需要游戏支持</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.immediateGameserverStats" type="checkbox" />
                  <span>即时游戏服务器统计</span>
                </label>
                <p class="field-hint">立即上报游戏服务器统计数据，不等待批量提交</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.matchmakingServerListActualType" type="checkbox" />
                  <span>匹配服务器列表实际类型</span>
                </label>
                <p class="field-hint">返回实际的服务器列表类型，而非强制局域网类型</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.matchmakingServerDetailsViaSourceQuery" type="checkbox" />
                  <span>通过 Source 查询获取服务器详情</span>
                </label>
                <p class="field-hint">使用 Source 协议查询服务器详细信息</p>
              </div>
              <div class="form-group">
                <label>崩溃日志位置</label>
                <input v-model="configs.main.crashPrinterLocation" placeholder="可选" />
                <p class="field-hint">设置崩溃日志输出目录，留空则不输出</p>
              </div>
            </div>

            <!-- [main::stats] 统计设置 -->
            <div class="config-section">
              <h4>统计设置</h4>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.disableLeaderboardsCreateUnknown" type="checkbox" />
                  <span>禁用未知排行榜创建</span>
                </label>
                <p class="field-hint">阻止游戏自动创建未预定义的排行榜，避免数据混乱</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.allowUnknownStats" type="checkbox" />
                  <span>允许未知统计</span>
                </label>
                <p class="field-hint">允许游戏上报未预定义的统计数据，大多数游戏建议开启</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.statAchievementProgressFunctionality" type="checkbox" />
                  <span>统计成就进度功能</span>
                </label>
                <p class="field-hint">启用基于统计数据的成就进度追踪</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.saveOnlyHigherStatAchievementProgress" type="checkbox" />
                  <span>只保存更高的统计成就进度</span>
                </label>
                <p class="field-hint">仅当新进度高于旧进度时才保存，防止进度倒退</p>
              </div>
              <div class="form-group">
                <label>分页成就图标数量</label>
                <input v-model.number="configs.main.paginatedAchievementsIcons" type="number" />
                <p class="field-hint">每页加载的成就图标数量，默认 10</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.recordPlaytime" type="checkbox" />
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
                  <input v-model="configs.main.disableLanOnly" type="checkbox" />
                  <span>禁用仅局域网模式</span>
                </label>
                <p class="field-hint">取消局域网限制，允许非局域网连接。联机游戏建议关闭此项</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.disableNetworking" type="checkbox" />
                  <span>禁用网络功能</span>
                </label>
                <p class="field-hint">完全禁用网络功能，纯单机游戏可开启</p>
              </div>
              <div class="form-row">
                <div class="form-group">
                  <label>监听端口</label>
                  <input v-model.number="configs.main.listenPort" type="number" />
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
                  <input v-model="configs.main.disableSharingStatsWithGameserver" type="checkbox" />
                  <span>禁用与游戏服务器共享统计</span>
                </label>
                <p class="field-hint">阻止统计数据发送到游戏服务器</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.disableSourceQuery" type="checkbox" />
                  <span>禁用 Source 查询</span>
                </label>
                <p class="field-hint">禁用 Source 协议服务器查询</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.shareLeaderboardsOverNetwork" type="checkbox" />
                  <span>网络共享排行榜</span>
                </label>
                <p class="field-hint">通过网络与其他玩家共享排行榜数据</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.disableLobbyCreation" type="checkbox" />
                  <span>禁用大厅创建</span>
                </label>
                <p class="field-hint">阻止游戏创建 Steam 大厅，纯单机游戏可开启</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.downloadSteamhttpRequests" type="checkbox" />
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
                  <input v-model="configs.main.achievementsBypass" type="checkbox" />
                  <span>成就绕过</span>
                </label>
                <p class="field-hint">绕过成就解锁限制，允许解锁所有成就</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.forceSteamhttpSuccess" type="checkbox" />
                  <span>强制 SteamHTTP 成功</span>
                </label>
                <p class="field-hint">强制所有 SteamHTTP 请求返回成功，避免网络请求失败导致的问题</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.disableSteamoverlaygameidEnvVar" type="checkbox" />
                  <span>禁用 Steam 覆盖层游戏 ID 环境变量</span>
                </label>
                <p class="field-hint">阻止设置 SteamOverlayGameId 环境变量</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.enableSteamPreownedIds" type="checkbox" />
                  <span>启用 Steam 预拥有 ID</span>
                </label>
                <p class="field-hint">模拟 Steam 预拥有游戏 ID 列表</p>
              </div>
              <div class="form-group">
                <label>Steam 游戏统计报告目录</label>
                <input v-model="configs.main.steamGameStatsReportsDir" placeholder="可选" />
                <p class="field-hint">设置游戏统计报告输出目录</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.freeWeekend" type="checkbox" />
                  <span>免费周末</span>
                </label>
                <p class="field-hint">模拟免费周末活动状态</p>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.main.use32bitInventoryItemIds" type="checkbox" />
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
              <input v-model="configs.user.accountSteamid" placeholder="76561197960287930" />
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
              <input v-model="configs.user.ipCountry" placeholder="CN" />
              <p class="field-hint">ISO 3166-1-alpha-2 格式，游戏查询 IP 时上报的国家代码</p>
            </div>
            <div class="form-group">
              <label>存档文件夹名称</label>
              <input v-model="configs.user.savesFolderName" placeholder="覆盖默认的 GSE Saves" />
            </div>
            <div class="form-group">
              <label>本地存档路径（便携模式）</label>
              <input v-model="configs.user.localSavePath" placeholder="设置后完全便携" />
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
                  v-model="configs.app.dlcs.unlock_all"
                  type="radio" 
                  name="dlcMode" 
                  :value="true"
                />
                <span class="dlc-mode-label">解锁所有 DLC</span>
              </label>
              <label class="dlc-mode-option">
                <input 
                  v-model="configs.app.dlcs.unlock_all"
                  type="radio" 
                  name="dlcMode" 
                  :value="false"
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

          <!-- 覆盖层overlay配置 -->
          <div v-if="activeTab === 'overlay'" class="config-panel">
            <h3>覆盖层overlay配置 (configs.overlay.ini)</h3>
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
                  <span class="format-value">成就通知 / 好友通知 / 聊天通知 / 进度通知</span>
                </div>
                <div class="format-item">
                  <span class="format-label">用途</span>
                  <span class="format-value">模拟 Steam 游戏内覆盖层体验</span>
                </div>
              </div>
            </div>

            <!-- 通用设置 -->
            <div class="form-group">
              <label class="checkbox-label">
                <input v-model="configs.overlay.enableExperimentalOverlay" type="checkbox" />
                <span>启用实验性游戏内覆盖层 (Shift+Tab)</span>
              </label>
              <p class="field-hint">实验性功能，如遇到游戏崩溃或卡顿请关闭</p>
            </div>
            <div class="form-row">
              <div class="form-group">
                <label>快捷键</label>
                <input v-model="configs.overlay.overlayHotkey" placeholder="shift + tab" />
                <p class="field-hint">按下组合键显示/隐藏 Overlay</p>
              </div>
              <div class="form-group">
                <label>FPS 平均窗口（秒）</label>
                <input v-model.number="configs.overlay.fpsAveragingWindow" type="number" min="0.1" max="10" step="0.1" placeholder="1.0" />
                <p class="field-hint">FPS 计算的平均窗口</p>
              </div>
            </div>
            <div class="form-row">
              <div class="form-group">
                <label>Hook 延迟（秒）</label>
                <input v-model.number="configs.overlay.hookDelaySec" type="number" class="config-input" min="0" max="30" placeholder="0" />
                <p class="field-hint">游戏启动后延迟 Hook 的时间，避免冲突</p>
              </div>
              <div class="form-group">
                <label>渲染器检测超时（秒）</label>
                <input v-model.number="configs.overlay.rendererDetectorTimeoutSec" type="number" class="config-input" min="1" max="60" placeholder="10" />
                <p class="field-hint">检测游戏渲染器的超时时间</p>
              </div>
            </div>

            <!-- 通知与功能开关 -->
            <h4>通知与功能开关</h4>
            <div class="form-row">
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.disableAchievementNotification" type="checkbox" />
                <span>禁用成就通知</span>
              </label>
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.disableFriendNotification" type="checkbox" />
                <span>禁用好友通知</span>
              </label>
            </div>
            <div class="form-row">
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.disableAchievementProgress" type="checkbox" />
                <span>禁用成就进度</span>
              </label>
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.disableWarningAny" type="checkbox" />
                <span>禁用所有警告</span>
              </label>
            </div>
            <div class="form-row">
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.disableWarningBadAppid" type="checkbox" />
                <span>禁用 Bad AppID 警告</span>
              </label>
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.disableWarningLocalSave" type="checkbox" />
                <span>禁用本地存档警告</span>
              </label>
            </div>
            <div class="form-row">
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.uploadAchievementsIconsToGpu" type="checkbox" />
                <span>上传成就图标到 GPU</span>
              </label>
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.overlayAlwaysShowUserInfo" type="checkbox" />
                <span>始终显示用户信息</span>
              </label>
            </div>
            <div class="form-row">
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.overlayAlwaysShowFps" type="checkbox" />
                <span>始终显示 FPS</span>
              </label>
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.overlayAlwaysShowFrametime" type="checkbox" />
                <span>始终显示帧时间</span>
              </label>
            </div>
            <div class="form-row">
              <label class="checkbox-label">
                <input v-model="configs.overlay.notifications.overlayAlwaysShowPlaytime" type="checkbox" />
                <span>始终显示游玩时间</span>
              </label>
            </div>

            <!-- 通知时长 -->
            <h4>通知时长</h4>
            <div class="form-row">
              <div class="form-group">
                <label>成就通知（秒）</label>
                <input v-model.number="configs.overlay.notifications.notificationDurationAchievement" type="number" min="0" placeholder="默认" />
              </div>
              <div class="form-group">
                <label>邀请通知（秒）</label>
                <input v-model.number="configs.overlay.notifications.notificationDurationInvitation" type="number" min="0" placeholder="默认" />
              </div>
            </div>
            <div class="form-row">
              <div class="form-group">
                <label>聊天通知（秒）</label>
                <input v-model.number="configs.overlay.notifications.notificationDurationChat" type="number" min="0" placeholder="默认" />
              </div>
              <div class="form-group">
                <label>进度通知（秒）</label>
                <input v-model.number="configs.overlay.notifications.notificationDurationProgress" type="number" min="0" placeholder="默认" />
              </div>
            </div>

            <!-- 外观基础 -->
            <h4>外观基础</h4>
            <div class="form-row">
              <div class="form-group">
                <label>主题</label>
                <select v-model="configs.overlay.appearance.theme">
                  <option value="dark">深色 (dark)</option>
                  <option value="light">浅色 (light)</option>
                </select>
              </div>
              <div class="form-group">
                <label>透明度 (0-1)</label>
                <input v-model.number="configs.overlay.appearance.opacity" type="number" min="0" max="1" step="0.05" />
              </div>
            </div>
            <div class="form-row">
              <div class="form-group">
                <label>缩放比例</label>
                <input v-model.number="configs.overlay.appearance.scale" type="number" min="0.5" max="3" step="0.1" />
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.overlay.appearance.blur" type="checkbox" />
                  <span>启用模糊效果</span>
                </label>
              </div>
            </div>

            <!-- 字体与图标 -->
            <h4>字体与图标</h4>
            <div class="form-row">
              <div class="form-group">
                <label>字体覆盖路径</label>
                <input v-model="configs.overlay.appearance.fontOverride" placeholder="留空使用默认字体" />
                <p class="field-hint">覆盖 Overlay 使用的字体文件路径</p>
              </div>
              <div class="form-group">
                <label>字体大小</label>
                <input v-model.number="configs.overlay.appearance.fontSize" type="number" min="8" max="72" placeholder="默认" />
              </div>
            </div>
            <div class="form-row">
              <div class="form-group">
                <label>字体字间距 X</label>
                <input v-model.number="configs.overlay.appearance.fontGlyphExtraSpacingX" type="number" step="0.1" placeholder="默认" />
              </div>
              <div class="form-group">
                <label>字体字间距 Y</label>
                <input v-model.number="configs.overlay.appearance.fontGlyphExtraSpacingY" type="number" step="0.1" placeholder="默认" />
              </div>
              <div class="form-group">
                <label>图标大小</label>
                <input v-model.number="configs.overlay.appearance.iconSize" type="number" min="8" max="128" placeholder="默认" />
              </div>
            </div>

            <!-- 通知样式 -->
            <h4>通知样式</h4>
            <div class="form-row">
              <div class="form-group">
                <label>通知圆角</label>
                <input v-model.number="configs.overlay.appearance.notificationRounding" type="number" min="0" placeholder="默认" />
              </div>
              <div class="form-group">
                <label>通知边距 X</label>
                <input v-model.number="configs.overlay.appearance.notificationMarginX" type="number" placeholder="默认" />
              </div>
              <div class="form-group">
                <label>通知边距 Y</label>
                <input v-model.number="configs.overlay.appearance.notificationMarginY" type="number" placeholder="默认" />
              </div>
            </div>
            <div class="form-row">
              <div class="form-group color-input">
                <label>通知背景色 RGBA</label>
                <div class="color-row">
                  <input v-model.number="configs.overlay.appearance.notificationR" type="number" min="0" max="255" placeholder="R" />
                  <input v-model.number="configs.overlay.appearance.notificationG" type="number" min="0" max="255" placeholder="G" />
                  <input v-model.number="configs.overlay.appearance.notificationB" type="number" min="0" max="255" placeholder="B" />
                  <input v-model.number="configs.overlay.appearance.notificationA" type="number" min="0" max="255" placeholder="A" />
                </div>
              </div>
            </div>

            <!-- 界面配色 -->
            <h4>界面配色</h4>
            <div class="form-row">
              <div class="form-group color-input">
                <label>背景色 RGBA</label>
                <div class="color-row">
                  <input v-model.number="configs.overlay.appearance.backgroundR" type="number" min="0" max="255" placeholder="R" />
                  <input v-model.number="configs.overlay.appearance.backgroundG" type="number" min="0" max="255" placeholder="G" />
                  <input v-model.number="configs.overlay.appearance.backgroundB" type="number" min="0" max="255" placeholder="B" />
                  <input v-model.number="configs.overlay.appearance.backgroundA" type="number" min="0" max="255" placeholder="A" />
                </div>
              </div>
            </div>
            <div class="form-row">
              <div class="form-group color-input">
                <label>元素色 RGBA</label>
                <div class="color-row">
                  <input v-model.number="configs.overlay.appearance.elementR" type="number" min="0" max="255" placeholder="R" />
                  <input v-model.number="configs.overlay.appearance.elementG" type="number" min="0" max="255" placeholder="G" />
                  <input v-model.number="configs.overlay.appearance.elementB" type="number" min="0" max="255" placeholder="B" />
                  <input v-model.number="configs.overlay.appearance.elementA" type="number" min="0" max="255" placeholder="A" />
                </div>
              </div>
            </div>
            <div class="form-row">
              <div class="form-group color-input">
                <label>元素悬停色 RGBA</label>
                <div class="color-row">
                  <input v-model.number="configs.overlay.appearance.elementHoveredR" type="number" min="0" max="255" placeholder="R" />
                  <input v-model.number="configs.overlay.appearance.elementHoveredG" type="number" min="0" max="255" placeholder="G" />
                  <input v-model.number="configs.overlay.appearance.elementHoveredB" type="number" min="0" max="255" placeholder="B" />
                  <input v-model.number="configs.overlay.appearance.elementHoveredA" type="number" min="0" max="255" placeholder="A" />
                </div>
              </div>
            </div>
            <div class="form-row">
              <div class="form-group color-input">
                <label>元素激活色 RGBA</label>
                <div class="color-row">
                  <input v-model.number="configs.overlay.appearance.elementActiveR" type="number" min="0" max="255" placeholder="R" />
                  <input v-model.number="configs.overlay.appearance.elementActiveG" type="number" min="0" max="255" placeholder="G" />
                  <input v-model.number="configs.overlay.appearance.elementActiveB" type="number" min="0" max="255" placeholder="B" />
                  <input v-model.number="configs.overlay.appearance.elementActiveA" type="number" min="0" max="255" placeholder="A" />
                </div>
              </div>
            </div>

            <!-- 通知位置 -->
            <h4>通知位置</h4>
            <div class="form-row three-col">
              <div class="form-group">
                <label>成就通知位置</label>
                <input v-model="configs.overlay.appearance.posAchievement" placeholder="如 top-left" />
              </div>
              <div class="form-group">
                <label>邀请通知位置</label>
                <input v-model="configs.overlay.appearance.posInvitation" placeholder="如 top-right" />
              </div>
              <div class="form-group">
                <label>聊天消息位置</label>
                <input v-model="configs.overlay.appearance.posChatMsg" placeholder="如 bottom-left" />
              </div>
            </div>

            <!-- 统计样式 -->
            <h4>统计样式</h4>
            <div class="form-row">
              <div class="form-group color-input">
                <label>统计背景色 RGBA</label>
                <div class="color-row">
                  <input v-model.number="configs.overlay.appearance.statsBackgroundR" type="number" min="0" max="255" placeholder="R" />
                  <input v-model.number="configs.overlay.appearance.statsBackgroundG" type="number" min="0" max="255" placeholder="G" />
                  <input v-model.number="configs.overlay.appearance.statsBackgroundB" type="number" min="0" max="255" placeholder="B" />
                  <input v-model.number="configs.overlay.appearance.statsBackgroundA" type="number" min="0" max="255" placeholder="A" />
                </div>
              </div>
            </div>
            <div class="form-row">
              <div class="form-group color-input">
                <label>统计文字色 RGBA</label>
                <div class="color-row">
                  <input v-model.number="configs.overlay.appearance.statsTextR" type="number" min="0" max="255" placeholder="R" />
                  <input v-model.number="configs.overlay.appearance.statsTextG" type="number" min="0" max="255" placeholder="G" />
                  <input v-model.number="configs.overlay.appearance.statsTextB" type="number" min="0" max="255" placeholder="B" />
                  <input v-model.number="configs.overlay.appearance.statsTextA" type="number" min="0" max="255" placeholder="A" />
                </div>
              </div>
            </div>
            <div class="form-row">
              <div class="form-group">
                <label>统计位置 X</label>
                <input v-model.number="configs.overlay.appearance.statsPosX" type="number" placeholder="默认" />
              </div>
              <div class="form-group">
                <label>统计位置 Y</label>
                <input v-model.number="configs.overlay.appearance.statsPosY" type="number" placeholder="默认" />
              </div>
            </div>
            <div class="form-row">
              <div class="form-group">
                <label>成就解锁日期格式</label>
                <input v-model="configs.overlay.appearance.achievementUnlockDatetimeFormat" placeholder="如 %Y-%m-%d %H:%M:%S" />
              </div>
            </div>

            <!-- 性能设置 -->
            <h4>性能设置</h4>
            <div class="form-row">
              <div class="form-group">
                <label>FPS 限制</label>
                <input v-model.number="configs.overlay.performance.fpsLimit" type="number" min="1" max="240" />
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.overlay.performance.hardwareAcceleration" type="checkbox" />
                  <span>硬件加速</span>
                </label>
              </div>
              <div class="form-group">
                <label class="checkbox-label">
                  <input v-model="configs.overlay.performance.lowPerformanceMode" type="checkbox" />
                  <span>低性能模式</span>
                </label>
              </div>
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
              <label class="checkbox-label">
                <input v-model="configs.overlay.features.chat" type="checkbox" />
                <span>聊天</span>
              </label>
            </div>
            <div class="form-row">
              <label class="checkbox-label">
                <input v-model="configs.overlay.features.browser" type="checkbox" />
                <span>浏览器</span>
              </label>
              <label class="checkbox-label">
                <input v-model="configs.overlay.features.settings" type="checkbox" />
                <span>设置</span>
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

          <!-- 局域网联机配置 -->
          <div v-if="activeTab === 'lan'" class="config-panel">
            <h3>局域网联机 (custom_broadcasts.txt / auto_accept_invite.txt)</h3>
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
                  <span class="format-label">广播 IP 列表</span>
                  <span class="format-value">custom_broadcasts.txt（每行一个 IP/域名）</span>
                </div>
                <div class="format-item">
                  <span class="format-label">自动接受邀请</span>
                  <span class="format-value">auto_accept_invite.txt</span>
                </div>
                <div class="format-item">
                  <span class="format-label">接受所有人</span>
                  <span class="format-value">文件内容为 *</span>
                </div>
                <div class="format-item">
                  <span class="format-label">监听端口</span>
                  <span class="format-value">默认 UDP 47584</span>
                </div>
              </div>
            </div>
            <div class="form-group">
              <label class="checkbox-label">
                <input v-model="configs.lan.enabled" type="checkbox" />
                <span>启用局域网联机</span>
              </label>
            </div>
            <div class="form-group">
              <label>自定义广播 IP / 域名（每行一个）</label>
              <textarea v-model="lanBroadcastsText" rows="4" placeholder="例如：&#10;192.168.1.100&#10;friend.example.com"></textarea>
            </div>
            <div class="form-group">
              <label>自动接受邀请</label>
              <select v-model="configs.lan.autoAcceptInvite">
                <option value="none">不自动接受</option>
                <option value="all">接受所有人的邀请</option>
                <option value="whitelist">仅接受白名单用户的邀请</option>
              </select>
            </div>
            <div v-if="configs.lan.autoAcceptInvite === 'whitelist'" class="form-group">
              <label>白名单 SteamID64（每行一个）</label>
              <textarea v-model="lanWhitelistText" rows="4" placeholder="例如：&#10;76561198000000001&#10;76561198000000002"></textarea>
            </div>
            <div class="form-group">
              <label>监听端口</label>
              <input v-model.number="configs.lan.listenPort" type="number" min="1024" max="65535" placeholder="47584" />
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
                  <span class="format-label">支持语言</span>
                  <span class="format-value">每行一个语言代码，如 schinese、english</span>
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
            <div class="form-group">
              <label>支持语言 (每行一个)</label>
              <textarea v-model="otherConfigs.supportedLanguages" rows="3" placeholder="schinese&#10;english"></textarea>
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
import { ref, reactive, computed, watch, onMounted, onUnmounted } from 'vue'
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

// 导航配置
const coreConfigs = [
  { id: 'main', name: '主配置' },
  { id: 'user', name: '用户配置' },
  { id: 'app', name: '应用配置' },
  { id: 'overlay', name: '覆盖层overlay' },
]

const gameFeatures = [
  { id: 'achievements', name: '成就系统' },
  { id: 'stats', name: '统计数据' },
  { id: 'items', name: '物品库存' },
  { id: 'mods', name: '创意工坊' },
  { id: 'leaderboards', name: '排行榜' },
  { id: 'controller', name: '控制器' },
  { id: 'lan', name: '局域网联机' },
  { id: 'other', name: '其他配置' },
]

const toolConfigs = [
  { id: 'coldclient', name: 'ColdClient' },
  { id: 'lobby', name: 'Lobby' },
]

/**
 * 各标签页的可搜索文本索引
 * 包含标签名、节标题、label、hint、placeholder 等所有可见文本，
 * 使搜索不仅能匹配导航名称，还能匹配配置项内部内容。
 */
const tabSearchIndex: Record<string, string> = {
  main: '主配置 configs.main.ini 通用设置 新版认证票据 new_app_ticket GC令牌 gc_token 阻止未知客户端 block_unknown_clients Steam Deck 头像 语音聊天 即时统计 匹配服务器列表 源查询 端口 listen_port 离线 网络 lobby 云 额外 dll extra',
  user: '用户配置 configs.user.ini 用户名 语言 国家 IP区域 存档文件夹 本地存档 加密票据 ticket Base64 alt_steamid',
  app: '应用配置 configs.app.ini 分支 beta 公开 public DLC 解锁 app路径 控制器 steam_input 云存档 windows linux 目录',
  overlay: '覆盖层overlay configs.overlay.ini 快捷键 shift tab hook 延迟 渲染器检测超时 fps 平均窗口 通知 成就 好友 进度 警告 上传gpu 用户信息 帧时间 游玩时间 时长 外观 主题 透明度 缩放 模糊 字体 字间距 图标 颜色 rgba 位置 统计 性能 硬件加速 低性能 功能 浏览器 聊天 设置',
  achievements: '成就系统 achievements.json 成就 通知 隐藏 名称 描述 图标 导入 导出',
  stats: '统计数据 stats.json int float avgrate 默认值 最小值 最大值 窗口大小',
  items: '物品库存 items.json default_items 初始库存 属性 堆叠 数量 物品ID',
  mods: '创意工坊 mods.json 模组 订阅 公开 私有 好友 预览图 文件ID',
  leaderboards: '排行榜 leaderboards.txt 排序 升序 降序 显示 数字 时间 秒 毫秒',
  controller: '控制器 controller xbox playstation nintendo 通用 死区 摇杆',
  lan: '局域网联机 LAN custom_broadcasts.txt auto_accept_invite.txt 广播 IP 域名 自动接受邀请 白名单 SteamID64 监听端口 UDP',
  other: '其他配置 installed_app_ids subscribed_groups purchased_keys supported_languages 已安装应用 订阅群组 购买密钥 支持语言',
  coldclient: 'ColdClient 注入模式 direct loader 额外DLL 启动参数',
  lobby: 'Lobby 大厅 自动加入 大厅ID 密码',
}

/**
 * 判断导航项是否匹配搜索词
 * 同时匹配导航名称和对应标签页的内容索引
 */
function configMatchesSearch(item: { id: string; name: string }, query: string): boolean {
  const q = query.toLowerCase()
  if (item.name.toLowerCase().includes(q)) return true
  const indexText = tabSearchIndex[item.id]
  return !!indexText && indexText.toLowerCase().includes(q)
}

/**
 * 判断导航项是否仅通过内容匹配（名称未命中但标签页内容命中）
 */
function isContentOnlyMatch(item: { id: string; name: string }, query: string): boolean {
  if (!query) return false
  const q = query.toLowerCase()
  if (item.name.toLowerCase().includes(q)) return false
  const indexText = tabSearchIndex[item.id]
  return !!indexText && indexText.toLowerCase().includes(q)
}

// 搜索过滤（同时匹配导航名称和配置项内容）
const filteredCoreConfigs = computed(() => {
  if (!searchQuery.value) return coreConfigs
  return coreConfigs.filter(c => configMatchesSearch(c, searchQuery.value))
})

const filteredGameFeatures = computed(() => {
  if (!searchQuery.value) return gameFeatures
  return gameFeatures.filter(c => configMatchesSearch(c, searchQuery.value))
})

const filteredToolConfigs = computed(() => {
  if (!searchQuery.value) return toolConfigs
  return toolConfigs.filter(c => configMatchesSearch(c, searchQuery.value))
})

// 所有匹配搜索的导航项（用于自动切换）
const allFilteredConfigs = computed(() => [
  ...filteredCoreConfigs.value,
  ...filteredGameFeatures.value,
  ...filteredToolConfigs.value,
])

/**
 * 当搜索词变化时，如果当前标签不在搜索结果中，自动切换到首个匹配标签
 */
watch(searchQuery, (query) => {
  if (!query) return
  const matched = allFilteredConfigs.value
  if (matched.length > 0 && !matched.some(c => c.id === activeTab.value)) {
    activeTab.value = matched[0].id
  }
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
// MainConfig / OverlayConfig / UserConfig 使用 camelCase 属性名以匹配 TS 类型
// AppConfig 保持 snake_case 以匹配接口，但移除类型断言让 controller/cloud_saves 被推断为非 undefined
const configs = reactive({
  main: {
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
    crashPrinterLocation: undefined,
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
    steamGameStatsReportsDir: undefined,
    freeWeekend: false,
    use32bitInventoryItemIds: false,
    // extra_dlls
    extraDlls: []
  } as SteamTypes.MainConfig,
  user: {
    username: 'Player',
    accountSteamid: undefined,
    language: 'schinese',
    ipCountry: 'CN',
    savesFolderName: undefined,
    localSavePath: undefined,
    ticket: undefined,
    altSteamid: undefined,
    altSteamidCount: undefined,
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
  },
  overlay: {
    enableExperimentalOverlay: false,
    hookDelaySec: undefined,
    rendererDetectorTimeoutSec: undefined,
    overlayHotkey: 'shift + tab',
    fpsAveragingWindow: undefined,
    notifications: {
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
      notificationDurationAchievement: undefined,
      notificationDurationInvitation: undefined,
      notificationDurationChat: undefined,
      notificationDurationProgress: undefined,
    },
    appearance: {
      fontOverride: undefined,
      fontSize: undefined,
      fontGlyphExtraSpacingX: undefined,
      fontGlyphExtraSpacingY: undefined,
      iconSize: undefined,
      theme: 'dark',
      opacity: 0.95,
      scale: 1.0,
      blur: true,
      notificationRounding: undefined,
      notificationMarginX: undefined,
      notificationMarginY: undefined,
      notificationR: undefined,
      notificationG: undefined,
      notificationB: undefined,
      notificationA: undefined,
      achievementUnlockDatetimeFormat: undefined,
      backgroundR: undefined,
      backgroundG: undefined,
      backgroundB: undefined,
      backgroundA: undefined,
      elementR: undefined,
      elementG: undefined,
      elementB: undefined,
      elementA: undefined,
      elementHoveredR: undefined,
      elementHoveredG: undefined,
      elementHoveredB: undefined,
      elementHoveredA: undefined,
      elementActiveR: undefined,
      elementActiveG: undefined,
      elementActiveB: undefined,
      elementActiveA: undefined,
      posAchievement: undefined,
      posInvitation: undefined,
      posChatMsg: undefined,
      statsBackgroundR: undefined,
      statsBackgroundG: undefined,
      statsBackgroundB: undefined,
      statsBackgroundA: undefined,
      statsTextR: undefined,
      statsTextG: undefined,
      statsTextB: undefined,
      statsTextA: undefined,
      statsPosX: undefined,
      statsPosY: undefined,
    },
    performance: {
      hardwareAcceleration: true,
      fpsLimit: 60,
      lowPerformanceMode: false,
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
  controller: {
    enabled: true,
    controllerType: 'xbox',
    bindings: [],
    deadzone: { leftStick: 0.1, rightStick: 0.1, leftTrigger: 0.1, rightTrigger: 0.1 },
    rumble: { enabled: true, intensity: 0.8 },
    customGlyphs: { enabled: false, path: '' }
  } as SteamTypes.ControllerConfig,
  lan: {
    enabled: false,
    customBroadcasts: [] as string[],
    autoAcceptInvite: 'none' as 'none' | 'all' | 'whitelist',
    whitelist: [] as string[],
    listenPort: 47584,
  },
  coldClientLoader: { enabled: false, injectionMode: 'direct', extraDlls: [], launchArgs: '' } as SteamTypes.ColdClientLoaderConfig,
  lobbyConnect: { enabled: false, autoJoin: false, targetLobbyId: '', password: '' } as SteamTypes.LobbyConnectConfig,
})

const otherConfigs = reactive({
  installedAppIds: '',
  subscribedGroups: '',
  purchasedKeys: '',
  supportedLanguages: '',
})

const coldClientDlls = computed({
  get: () => configs.coldClientLoader.extraDlls.join('\n'),
  set: (val: string) => { configs.coldClientLoader.extraDlls = val.split('\n').filter(s => s.trim()) }
})

// 局域网联机：广播 IP 列表 / 白名单文本编辑
const lanBroadcastsText = computed({
  get: () => configs.lan.customBroadcasts.join('\n'),
  set: (val: string) => { configs.lan.customBroadcasts = val.split('\n').map(s => s.trim()).filter(s => s !== '') }
})

const lanWhitelistText = computed({
  get: () => configs.lan.whitelist.join('\n'),
  set: (val: string) => { configs.lan.whitelist = val.split('\n').map(s => s.trim()).filter(s => s !== '') }
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

/**
 * 将 snake_case 字符串转换为 camelCase
 * 用于主配置 INI 键名与 TS 类型属性名之间的转换
 */
function snakeToCamel(s: string): string {
  return s.replace(/_(.)/g, (_, char: string) => char.toUpperCase())
}

// 将前端 main 配置对象转换为 INI 字符串
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
      if (!result.extraDlls) result.extraDlls = []
      result.extraDlls.push(t)
      continue
    }
    const k = t.slice(0, i).trim()
    const v = t.slice(i + 1).trim()
    const camelKey = snakeToCamel(k)

    if (k.startsWith('dll')) {
      // extra_dlls 段：dllN = 路径
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
 * 将前端 app 配置中的 controller 子对象转换为后端期望的 camelCase 格式
 * 前端内部使用 snake_case（steam_input / type），后端 save_app_config 期望 steamInput / type
 */
function convertAppControllerForRust(controller: any) {
  if (!controller) return {}
  return {
    steamInput: controller.steam_input ?? false,
    type: controller.type ?? ''
  }
}

/**
 * 将前端 app 配置中的 cloud_saves 子对象转换为后端期望的 camelCase 格式
 * 前端内部使用 snake_case，后端 save_app_config 期望 camelCase
 */
function convertCloudSavesForRust(cloudSaves: any) {
  if (!cloudSaves) return {}
  return {
    enabled: cloudSaves.enabled ?? false,
    createDefaultDir: cloudSaves.create_default_dir ?? false,
    createSpecificDirs: cloudSaves.create_specific_dirs ?? false,
    windowsDirs: cloudSaves.windows_dirs || [],
    linuxDirs: cloudSaves.linux_dirs || []
  }
}

// 将前端 app 配置转换为 Rust 后端期望的格式
function buildAppConfigForRust() {
  return {
    branchName: configs.app.branch_name,
    isBetaBranch: configs.app.is_beta_branch || false,
    appPaths: {},
    controller: convertAppControllerForRust(configs.app.controller),
    cloudSaves: convertCloudSavesForRust(configs.app.cloud_saves),
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
      invoke('save_lan_multiplayer_config', { gamePath: props.gamePath, config: configs.lan }),
      invoke('save_coldclient_config', { gamePath: props.gamePath, config: configs.coldClientLoader }),
      invoke('save_lobby_connect_config', { gamePath: props.gamePath, config: configs.lobbyConnect }),
      saveOtherConfigs(),
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
    // 广播各配置已保存事件，使对应的单独配置窗口同步刷新
    window.dispatchEvent(new CustomEvent('main-config-saved', { detail: { gamePath: props.gamePath } }))
    window.dispatchEvent(new CustomEvent('user-config-saved', { detail: { gamePath: props.gamePath } }))
    window.dispatchEvent(new CustomEvent('achievements-config-saved', { detail: { gamePath: props.gamePath } }))
    window.dispatchEvent(new CustomEvent('stats-config-saved', { detail: { gamePath: props.gamePath } }))
    window.dispatchEvent(new CustomEvent('items-config-saved', { detail: { gamePath: props.gamePath } }))
    window.dispatchEvent(new CustomEvent('leaderboards-config-saved', { detail: { gamePath: props.gamePath } }))
    window.dispatchEvent(new CustomEvent('controller-config-saved', { detail: { gamePath: props.gamePath } }))
    window.dispatchEvent(new CustomEvent('app-config-saved', { detail: { gamePath: props.gamePath } }))
    window.dispatchEvent(new CustomEvent('overlay-config-saved', { detail: { gamePath: props.gamePath } }))
    window.dispatchEvent(new CustomEvent('lan-config-saved', { detail: { gamePath: props.gamePath } }))
  } catch (error) {
    isSaving.value = false
    alert(`保存失败: ${error}`)
  }
}

/**
 * 仅重新加载应用配置（包含 DLC、分支、控制器、云存档等）
 * 用于响应 app-config-saved 同步事件，避免整页刷新并保留其它未保存的修改
 */
async function reloadAppConfig() {
  try {
    const result = await invoke<{ exists: boolean; config?: SteamTypes.SteamAppConfig }>('load_app_config', { gamePath: props.gamePath })
    if (result.exists && result.config) {
      const app = result.config
      configs.app.branch_name = app.branch_name || 'public'
      configs.app.is_beta_branch = app.is_beta_branch || false
      configs.app.dlcs.unlock_all = app.dlcs?.unlock_all ?? true

      if (app.controller) {
        configs.app.controller = {
          steam_input: app.controller.steam_input ?? false,
          type: app.controller.type ?? '',
        }
      }

      if (app.cloud_saves) {
        configs.app.cloud_saves = {
          enabled: app.cloud_saves.enabled ?? false,
          create_default_dir: app.cloud_saves.create_default_dir ?? false,
          create_specific_dirs: app.cloud_saves.create_specific_dirs ?? false,
          windows_dirs: app.cloud_saves.windows_dirs || [],
          linux_dirs: app.cloud_saves.linux_dirs || [],
        }
      }

      if (app.dlcs?.individual_dlcs && app.dlcs.individual_dlcs.length > 0) {
        configs.app.dlcs.custom_list = app.dlcs.individual_dlcs
          .filter((d: any) => d.enabled)
          .map((d: any) => {
            if (d.name && d.name !== `DLC ${d.app_id}` && d.name !== d.app_id) {
              return `${d.app_id}=${d.name}`
            }
            return d.app_id
          })
          .join('\n')
      } else {
        configs.app.dlcs.custom_list = ''
      }
    }
    configStatus.value.app = result.exists
  } catch (error) {
    // 加载失败时保持当前状态
  }
}

/**
 * 仅重新加载覆盖层配置
 * 用于响应 overlay-config-saved 同步事件，避免整页刷新
 */
async function reloadOverlayConfig() {
  try {
    const result = await invoke<{ exists: boolean; config?: SteamTypes.OverlayConfig }>('load_overlay_config', { gamePath: props.gamePath })
    if (result.exists && result.config) {
      Object.assign(configs.overlay, result.config)
    }
    configStatus.value.overlay = result.exists
  } catch (error) {
    // 加载失败时保持当前状态
  }
}

/**
 * 仅重新加载主配置（configs.main.ini）
 * 用于响应 main-config-saved 同步事件，避免整页刷新并保留其它未保存的修改
 */
async function reloadMainConfig() {
  try {
    const result = await invoke<{ exists: boolean; content?: string }>('load_main_config', { gamePath: props.gamePath })
    if (result.exists && result.content) {
      const parsed = parseMainConfigIni(result.content)
      Object.assign(configs.main, parsed)
    }
    configStatus.value.main = result.exists
  } catch (error) {
    // 加载失败时保持当前状态
  }
}

/**
 * 处理主配置同步事件
 * 当 MainConfig.vue 等其它组件保存 main 配置时，若 gamePath 匹配则自动重载主配置
 */
function handleMainConfigSaved(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    reloadMainConfig()
  }
}

/**
 * 仅重新加载用户配置（configs.user.ini）
 * 用于响应 user-config-saved 同步事件
 */
async function reloadUserConfig() {
  try {
    const result = await invoke<{ exists: boolean; config?: SteamTypes.UserConfig }>('load_user_config', { gamePath: props.gamePath })
    if (result.exists && result.config) {
      const c = result.config as any
      Object.keys(c).forEach((key) => {
        const camelKey = key.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase())
        if (key in configs.user || camelKey in configs.user) {
          (configs.user as any)[camelKey || key] = c[key]
        }
      })
    }
    configStatus.value.user = result.exists
  } catch (error) {
    // 加载失败时保持当前状态
  }
}

/**
 * 处理用户配置同步事件
 * 当 UserConfig.vue 等其它组件保存 user 配置时，若 gamePath 匹配则自动重载用户配置
 */
function handleUserConfigSaved(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    reloadUserConfig()
  }
}

/**
 * 仅重新加载成就配置（achievements.json）
 * 用于响应 achievements-config-saved 同步事件
 */
async function reloadAchievementsConfig() {
  try {
    const result = await invoke<{ exists: boolean; config?: SteamTypes.AchievementsConfig }>('load_achievements_config', { gamePath: props.gamePath })
    if (result.exists && result.config) {
      Object.assign(configs.achievements, result.config)
    }
    configStatus.value.achievements = result.exists
  } catch (error) {
    // 加载失败时保持当前状态
  }
}

/**
 * 处理成就配置同步事件
 * 当 AchievementsConfig.vue 等其它组件保存 achievements 配置时，若 gamePath 匹配则自动重载成就配置
 */
function handleAchievementsConfigSaved(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    reloadAchievementsConfig()
  }
}

/**
 * 仅重新加载统计配置（stats.json）
 * 用于响应 stats-config-saved 同步事件
 */
async function reloadStatsConfig() {
  try {
    const result = await invoke<{ exists: boolean; config?: SteamTypes.StatsConfig }>('load_stats_config', { gamePath: props.gamePath })
    if (result.exists && result.config) {
      Object.assign(configs.stats, result.config)
    }
    configStatus.value.stats = result.exists
  } catch (error) {
    // 加载失败时保持当前状态
  }
}

/**
 * 处理统计配置同步事件
 * 当 StatsConfig.vue 等其它组件保存 stats 配置时，若 gamePath 匹配则自动重载统计配置
 */
function handleStatsConfigSaved(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    reloadStatsConfig()
  }
}

/**
 * 仅重新加载物品配置（items.json / default_items.json）
 * 用于响应 items-config-saved 同步事件
 */
async function reloadItemsConfig() {
  try {
    const result = await invoke<{ exists: boolean; config?: SteamTypes.ItemsConfig }>('load_items_config', { gamePath: props.gamePath })
    if (result.exists && result.config) {
      Object.assign(configs.items, result.config)
    }
    configStatus.value.items = result.exists
  } catch (error) {
    // 加载失败时保持当前状态
  }
}

/**
 * 处理物品配置同步事件
 * 当 ItemsConfig.vue 等其它组件保存 items 配置时，若 gamePath 匹配则自动重载物品配置
 */
function handleItemsConfigSaved(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    reloadItemsConfig()
  }
}

/**
 * 仅重新加载排行榜配置（leaderboards.txt）
 * 用于响应 leaderboards-config-saved 同步事件
 */
async function reloadLeaderboardsConfig() {
  try {
    const result = await invoke<{ exists: boolean; config?: SteamTypes.LeaderboardsConfig }>('load_leaderboards_config', { gamePath: props.gamePath })
    if (result.exists && result.config) {
      Object.assign(configs.leaderboards, result.config)
    }
    configStatus.value.leaderboards = result.exists
  } catch (error) {
    // 加载失败时保持当前状态
  }
}

/**
 * 处理排行榜配置同步事件
 * 当 LeaderboardsConfig.vue 等其它组件保存 leaderboards 配置时，若 gamePath 匹配则自动重载排行榜配置
 */
function handleLeaderboardsConfigSaved(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    reloadLeaderboardsConfig()
  }
}

/**
 * 仅重新加载控制器配置（controller/*.txt）
 * 用于响应 controller-config-saved 同步事件
 */
async function reloadControllerConfig() {
  try {
    const result = await invoke<{ exists: boolean; config?: SteamTypes.ControllerConfig }>('load_controller_config', { gamePath: props.gamePath })
    if (result.exists && result.config) {
      Object.assign(configs.controller, result.config)
    }
    configStatus.value.controller = result.exists
  } catch (error) {
    // 加载失败时保持当前状态
  }
}

/**
 * 处理控制器配置同步事件
 * 当 ControllerConfig.vue 等其它组件保存 controller 配置时，若 gamePath 匹配则自动重载控制器配置
 */
function handleControllerConfigSaved(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    reloadControllerConfig()
  }
}

/**
 * 仅重新加载局域网联机配置（custom_broadcasts.txt / auto_accept_invite.txt）
 * 用于响应 lan-config-saved 同步事件
 */
async function reloadLanConfig() {
  try {
    const result = await invoke<{ exists: boolean; config?: { enabled: boolean; customBroadcasts: string[]; autoAcceptInvite: 'none' | 'all' | 'whitelist'; whitelist: string[]; listenPort: number } }>('load_lan_multiplayer_config', { gamePath: props.gamePath })
    if (result.exists && result.config) {
      configs.lan.enabled = result.config.enabled ?? true
      configs.lan.customBroadcasts = result.config.customBroadcasts || []
      configs.lan.autoAcceptInvite = result.config.autoAcceptInvite || 'none'
      configs.lan.whitelist = result.config.whitelist || []
      configs.lan.listenPort = result.config.listenPort || 47584
    }
    configStatus.value.lan = result.exists
  } catch (error) {
    // 加载失败时保持当前状态
  }
}

/**
 * 处理局域网联机配置同步事件
 * 当 LanMultiplayerConfig.vue 保存 lan 配置时，若 gamePath 匹配则自动重载局域网联机配置
 */
function handleLanConfigSaved(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    reloadLanConfig()
  }
}

/**
 * 处理应用配置同步事件
 * 当 DlcConfig.vue 等其它组件保存 app 配置时，若 gamePath 匹配则自动重载应用配置
 */
function handleAppConfigSaved(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    reloadAppConfig()
  }
}

/**
 * 处理覆盖层配置同步事件
 * 当 OverlayConfig.vue 等其它组件保存 overlay 配置时，若 gamePath 匹配则自动重载覆盖层配置
 */
function handleOverlayConfigSaved(event: Event) {
  const customEvent = event as CustomEvent<{ gamePath: string }>
  if (customEvent.detail?.gamePath === props.gamePath) {
    reloadOverlayConfig()
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
    invoke<{ exists: boolean; config?: { enabled: boolean; customBroadcasts: string[]; autoAcceptInvite: 'none' | 'all' | 'whitelist'; whitelist: string[]; listenPort: number } }>('load_lan_multiplayer_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: SteamTypes.ColdClientLoaderConfig }>('load_coldclient_config', { gamePath: props.gamePath }),
    invoke<{ exists: boolean; config?: SteamTypes.LobbyConnectConfig }>('load_lobby_connect_config', { gamePath: props.gamePath }),
  ])

  const [main, user, app, overlay, achievements, stats, items, mods, leaderboards, controller, lan, coldclient, lobbyconnect] = results

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
        steam_input: app.config.controller.steam_input ?? false,
        type: app.config.controller.type ?? '',
      }
    }

    // 加载云存档配置
    if (app.config.cloud_saves) {
      configs.app.cloud_saves = {
        enabled: app.config.cloud_saves.enabled ?? false,
        create_default_dir: app.config.cloud_saves.create_default_dir ?? false,
        create_specific_dirs: app.config.cloud_saves.create_specific_dirs ?? false,
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
  if (lan.exists && lan.config) {
    configs.lan.enabled = lan.config.enabled ?? true
    configs.lan.customBroadcasts = lan.config.customBroadcasts || []
    configs.lan.autoAcceptInvite = lan.config.autoAcceptInvite || 'none'
    configs.lan.whitelist = lan.config.whitelist || []
    configs.lan.listenPort = lan.config.listenPort || 47584
  }
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
    lan: lan.exists,
    coldclient: coldclient.exists,
    lobbyconnect: lobbyconnect.exists,
  }

  // 加载其他配置文件（ installed_app_ids / subscribed_groups / purchased_keys ）
  await loadOtherConfigs()
}

/**
 * 加载其他配置文件
 * 与 AdvancedConfig.vue 保持一致的文件集合
 */
async function loadOtherConfigs() {
  try {
    const [appIds, groupIds, keys, languages] = await Promise.all([
      invoke<string[]>('load_installed_app_ids', { gamePath: props.gamePath }),
      invoke<string[]>('load_subscribed_groups', { gamePath: props.gamePath }),
      invoke<string[]>('load_purchased_keys', { gamePath: props.gamePath }),
      invoke<string[]>('load_supported_languages', { gamePath: props.gamePath }),
    ])

    otherConfigs.installedAppIds = appIds.join('\n')
    otherConfigs.subscribedGroups = groupIds.join('\n')
    otherConfigs.purchasedKeys = keys.join('\n')
    otherConfigs.supportedLanguages = languages.join('\n')
  } catch (error) {
    // 加载失败时使用默认值
  }
}

/**
 * 保存其他配置文件
 * 与 AdvancedConfig.vue 保持一致：空内容也会写入文件，避免残留旧数据
 */
async function saveOtherConfigs() {
  const appIds = otherConfigs.installedAppIds
    .split('\n')
    .map(s => s.trim())
    .filter(s => s !== '')

  const groupIds = otherConfigs.subscribedGroups
    .split('\n')
    .map(s => s.trim())
    .filter(s => s !== '')

  const keys = otherConfigs.purchasedKeys
    .split('\n')
    .map(s => s.trim())
    .filter(s => s !== '' && !s.startsWith('#'))

  const languages = otherConfigs.supportedLanguages
    .split('\n')
    .map(s => s.trim())
    .filter(s => s !== '')

  await Promise.all([
    invoke('save_installed_app_ids', { gamePath: props.gamePath, appIds }),
    invoke('save_subscribed_groups', { gamePath: props.gamePath, groupIds }),
    invoke('save_purchased_keys', { gamePath: props.gamePath, keys }),
    invoke('save_supported_languages', { gamePath: props.gamePath, languages }),
  ])
}

onMounted(() => {
  loadAllConfigs()
  window.addEventListener('main-config-saved', handleMainConfigSaved)
  window.addEventListener('user-config-saved', handleUserConfigSaved)
  window.addEventListener('achievements-config-saved', handleAchievementsConfigSaved)
  window.addEventListener('stats-config-saved', handleStatsConfigSaved)
  window.addEventListener('items-config-saved', handleItemsConfigSaved)
  window.addEventListener('leaderboards-config-saved', handleLeaderboardsConfigSaved)
  window.addEventListener('controller-config-saved', handleControllerConfigSaved)
  window.addEventListener('app-config-saved', handleAppConfigSaved)
  window.addEventListener('overlay-config-saved', handleOverlayConfigSaved)
  window.addEventListener('lan-config-saved', handleLanConfigSaved)
})

onUnmounted(() => {
  window.removeEventListener('main-config-saved', handleMainConfigSaved)
  window.removeEventListener('user-config-saved', handleUserConfigSaved)
  window.removeEventListener('achievements-config-saved', handleAchievementsConfigSaved)
  window.removeEventListener('stats-config-saved', handleStatsConfigSaved)
  window.removeEventListener('items-config-saved', handleItemsConfigSaved)
  window.removeEventListener('leaderboards-config-saved', handleLeaderboardsConfigSaved)
  window.removeEventListener('controller-config-saved', handleControllerConfigSaved)
  window.removeEventListener('app-config-saved', handleAppConfigSaved)
  window.removeEventListener('overlay-config-saved', handleOverlayConfigSaved)
  window.removeEventListener('lan-config-saved', handleLanConfigSaved)
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

.color-input .color-row {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
}

.color-input .color-row input {
  text-align: center;
  padding: 8px 4px;
}

.form-row.three-col {
  grid-template-columns: repeat(3, 1fr);
}

/* 搜索内容匹配提示 */
.nav-item .content-match {
  font-size: 10px;
  color: var(--steam-accent-blue);
  margin-left: auto;
  padding: 1px 4px;
  background-color: rgba(var(--steam-accent-blue-rgb, 107, 170, 255), 0.1);
  border-radius: 4px;
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
