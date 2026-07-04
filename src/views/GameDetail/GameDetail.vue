<template>
  <div class="game-detail-page">
    <!-- 首次使用配置弹窗 -->
    <FirstTimeSetupModal
      :show="showFirstTimeSetup"
      @close="handleFirstTimeSetupClose"
      @confirm="handleFirstTimeSetupConfirm"
    />

    <!-- 二维码弹窗 -->
    <QRCodeModal
      v-model="showQRCodeModal"
      :title="qrCodeTitle"
      :qr-image-url="qrCodeImageUrl"
      :hint="qrCodeHint"
      @close="handleQRCodeClose"
    />

    <!-- 返回按钮和标题栏 -->
    <div class="detail-header">
      <button class="back-btn" @click="goBack">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M19 12H5M12 19l-7-7 7-7"/>
        </svg>
        <span>返回</span>
      </button>
      
      <div class="header-info">
        <h1 class="game-title">{{ game?.game_name }} {{ game?.chinese_name }}</h1>
        <span class="game-id">Game ID: {{ game?.game_id }}</span>
      </div>
      
      <!-- 分类标签 -->
      <div class="category-tags">
        <span 
          v-for="(tag, index) in game?.tags" 
          :key="index"
          class="category-tag"
          :style="{ backgroundColor: getCategoryColor(tag.patch_type) }"
        >
          {{ getCategoryName(tag.patch_type) }}
        </span>
      </div>
    </div>

    <!-- 主要内容区 -->
    <div class="detail-content">
      <!-- 左侧：游戏封面 -->
      <div class="cover-section">
        <img 
          :src="coverImage || '/default-cover.jpg'" 
          :alt="game?.chinese_name"
          class="game-cover"
          @error="handleImageError"
        />
      </div>

      <!-- 右侧：路径选择 -->
      <div class="paths-section">
        <!-- 游戏下载路径 - 仅当 downloadable 为 true 时显示 -->
        <div v-if="game?.downloadable" class="path-item">
          <label class="path-label">游戏下载路径</label>
          <div class="path-input-group">
            <input 
              type="text" 
              v-model="downloadPath"
              class="path-input"
              placeholder="请选择下载路径..."
              readonly
            />
            <button class="browse-btn" @click="selectDownloadPath">浏览</button>
          </div>
        </div>

        <div class="path-item">
          <label class="path-label">游戏路径选择</label>
          <div class="path-input-group">
            <input 
              type="text" 
              v-model="gamePath"
              class="path-input"
              placeholder="请选择游戏路径..."
              readonly
            />
            <button class="browse-btn" @click="selectGamePath">浏览</button>
          </div>
        </div>
      </div>
    </div>

    <!-- 底部：标签页 -->
    <div class="tabs-section">
      <div class="tabs-header">
        <button 
          v-for="tab in availableTabs" 
          :key="tab.id"
          class="tab-btn"
          :class="{ active: currentTab === tab.id, 'patch-tab': tab.id.startsWith('patch-') }"
          @click="currentTab = tab.id"
        >
          {{ tab.name }}
        </button>
      </div>

      <div class="tabs-content">
        <!-- 游戏下载标签页 -->
        <div v-if="currentTab === 'download'" class="tab-panel">
          <!-- 下载状态显示 -->
          <div v-if="existingGameData?.download_status === 'completed'" class="download-completed-notice">
            <div class="success-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                <polyline points="22 4 12 14.01 9 11.01"/>
              </svg>
            </div>
            <div class="success-text">
              <h4>游戏已下载</h4>
              <p>下载路径: {{ existingGameData.download_path }}</p>
              <p>安装路径: {{ existingGameData.install_path }}</p>
              <p class="patch-hint">请安装对应补丁（免steam、steam联机、局域网联机等）</p>
            </div>
            <!-- 验证完整性按钮 -->
            <button
              class="verify-integrity-btn"
              :class="{ loading: isVerifying }"
              :disabled="isVerifying"
              @click="verifyIntegrity"
            >
              <svg v-if="isVerifying" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
              </svg>
              <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                <polyline points="22 4 12 14.01 9 11.01"/>
              </svg>
              {{ isVerifying ? '验证中...' : '验证游戏完整性' }}
            </button>
          </div>

          <div v-else class="download-info">
            <!-- 清单文件夹检测状态，未找到时显示下载引导和文件选择 -->
            <div
              v-if="manifestCheckStatus === 'not_found'"
              class="import-no-files"
            >
              <!-- 下载引导 -->
              <div class="download-guide">
                <p class="download-guide-title">点击下载对应清单文件7z，id：{{ gameId }}</p>
                <div class="download-buttons">
                  <div class="download-btn-wrapper">
                    <button
                      class="download-patch-btn"
                      @click="openQingdanQRCode"
                    >
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                        <polyline points="7 10 12 15 17 10"/>
                        <line x1="12" y1="15" x2="12" y2="3"/>
                      </svg>
                      <span>夸克网盘</span>
                    </button>
                  </div>
                  <div class="backup-label">备用（容易和谐）：</div>
                  <div class="download-btn-wrapper">
                    <button
                      class="download-patch-btn"
                      @click="openDownloadUrl('https://pan.xunlei.com/s/VOw3jTAGHqYFsm49n2t_AeVGA1?pwd=3r6n')"
                    >
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                        <polyline points="7 10 12 15 17 10"/>
                        <line x1="12" y1="15" x2="12" y2="3"/>
                      </svg>
                      <span>迅雷网盘</span>
                    </button>
                    <p class="pwd-hint">提取码: 3r6n</p>
                  </div>
                  <div class="download-btn-wrapper">
                    <button
                      class="download-patch-btn"
                      @click="openDownloadUrl('https://pan.baidu.com/s/1FTZyknIObyzMuLAJC-Uj9g?pwd=8uwx')"
                    >
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                        <polyline points="7 10 12 15 17 10"/>
                        <line x1="12" y1="15" x2="12" y2="3"/>
                      </svg>
                      <span>百度网盘</span>
                    </button>
                    <p class="pwd-hint">提取码: 8uwx</p>
                  </div>
                </div>
              </div>

              <!-- 清单文件选择 -->
              <div class="import-source-select">
                <h4 class="source-select-title">清单文件选择</h4>

                <div class="radio-group">
                  <label class="radio-label">
                    <input
                      type="radio"
                      v-model="downloadManifestSourceMode"
                      value="7z"
                      name="download-manifest-source-mode"
                    />
                    <span>7z文件</span>
                  </label>
                  <label class="radio-label">
                    <input
                      type="radio"
                      v-model="downloadManifestSourceMode"
                      value="folder"
                      name="download-manifest-source-mode"
                    />
                    <span>.vdf / .lua 和 .manifest所在文件夹</span>
                  </label>
                </div>

                <div class="source-select-actions">
                  <button
                    v-if="downloadManifestSourceMode === '7z'"
                    class="select-source-btn"
                    :disabled="isPreparingDownloadManifest"
                    @click="selectDownloadManifestArchive"
                  >
                    <svg v-if="isPreparingDownloadManifest" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
                    </svg>
                    {{ isPreparingDownloadManifest ? '处理中...' : '选择7z文件' }}
                  </button>
                  <button
                    v-else
                    class="select-source-btn"
                    :disabled="isPreparingDownloadManifest"
                    @click="selectDownloadManifestFolder"
                  >
                    <svg v-if="isPreparingDownloadManifest" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
                    </svg>
                    {{ isPreparingDownloadManifest ? '处理中...' : '选择文件夹' }}
                  </button>
                </div>

                <p v-if="selectedDownloadManifestPath" class="source-select-info">
                  已选择: {{ selectedDownloadManifestPath }}
                </p>
              </div>
            </div>

            <!-- 下载路径显示 -->
            <div class="download-path-section">
              <label class="path-label">下载路径</label>
              <div class="path-display">{{ downloadPath || '未选择' }}</div>
              <p v-if="downloadPath" class="path-hint">自动设置为: {{ downloadPath }}</p>
            </div>
          </div>

          <!-- 下载说明 -->
          <div class="download-description">
            <div class="download-option">
              <h4>方法一【开始下载】</h4>
              <p>下载Steam正版分流文件，下载完成后需要<span class="highlight-red-bold">注入补丁</span>才能游玩</p>
            </div>
          </div>

          <!-- 下载按钮组 -->
          <div v-if="existingGameData?.download_status !== 'completed'" class="download-btn-group">
            <!-- 圆形下载进度条，仅下载中显示 -->
            <div
              v-if="isDownloading || existingGameData?.download_status === 'downloading'"
              class="circular-progress"
              :title="`总进度 ${downloadProgress.overallPercentage}%`"
            >
              <svg viewBox="0 0 36 36">
                <path
                  class="circle-bg"
                  d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
                />
                <path
                  class="circle-progress"
                  :stroke-dasharray="`${downloadProgress.overallPercentage}, 100`"
                  d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
                />
              </svg>
              <span class="progress-text">{{ downloadProgress.overallPercentage }}%</span>
            </div>

            <!-- 开始下载按钮 -->
            <button
              class="start-download-btn"
              :class="{ disabled: !canDownload, loading: isDownloading || existingGameData?.download_status === 'downloading' }"
              :disabled="!canDownload || isDownloading || existingGameData?.download_status === 'downloading'"
              @click="startDownload"
              :title="!canDownload ? '未找到清单文件，无法下载' : ''"
            >
              <svg v-if="isDownloading || existingGameData?.download_status === 'downloading'" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
              </svg>
              <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                <polyline points="7 10 12 15 17 10"/>
                <line x1="12" y1="15" x2="12" y2="3"/>
              </svg>
              {{ getDownloadButtonText() }}
            </button>

            <!-- 暂停/停止下载按钮 -->
            <button
              v-if="isDownloading || existingGameData?.download_status === 'downloading'"
              class="stop-download-btn"
              @click="stopDownload"
              title="停止下载"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="6" y="4" width="4" height="16"/>
                <rect x="14" y="4" width="4" height="16"/>
              </svg>
              暂停
            </button>
          </div>

          <!-- 下载日志区域 -->
          <div v-if="downloadLogs.length > 0" class="download-logs">
            <h4 class="logs-title">下载日志</h4>
            <div class="logs-content">
              <div
                v-for="(log, index) in downloadLogs"
                :key="index"
                class="log-line"
                :class="log.type"
              >
                <span class="log-time">{{ log.time }}</span>
                <span class="log-message">{{ log.message }}</span>
              </div>
            </div>
          </div>

          <!-- 下载进度组件 -->
          <DownloadProgress
            v-if="isMonitoring || downloadProgress.depots.length > 0 || existingGameData?.download_status === 'downloading'"
            :progress="downloadProgress"
            :is-monitoring="isMonitoring || existingGameData?.download_status === 'downloading'"
            class="download-progress-section"
          />
        </div>

        <!-- 其他分类标签页 -->
        <div
          v-for="tab in patchTabs"
          :key="tab.id"
          v-show="currentTab === tab.id"
          class="tab-panel"
        >
          <h3 class="panel-title">{{ tab.name }}</h3>
          <div class="patch-info">
            <!-- 本地补丁状态 -->
            <div class="patch-status" :class="{ 'local-exists': isPatchLocalExists(tab.patchType) }">
              <span class="status-icon">
                <svg v-if="isPatchLocalExists(tab.patchType)" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                  <polyline points="22 4 12 14.01 9 11.01"/>
                </svg>
                <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="10"/>
                  <line x1="12" y1="8" x2="12" y2="16"/>
                  <line x1="8" y1="12" x2="16" y2="12"/>
                </svg>
              </span>
              <span class="status-text">
                {{ isPatchLocalExists(tab.patchType) ? '本地补丁已存在' : '本地补丁未下载' }}
              </span>
            </div>

            <!-- 下载补丁区域（当本地不存在且有下载链接时显示） -->
            <div v-if="!isPatchLocalExists(tab.patchType) && tab.downloadUrls && tab.downloadUrls.length > 0" class="download-section">
              <p class="download-section-title">下载补丁：</p>
              <div class="download-buttons">
                <div
                  v-for="(item, index) in tab.downloadUrls"
                  :key="index"
                  class="download-btn-wrapper"
                >
                  <button
                    class="download-patch-btn"
                    @click="openDownloadUrl(item.url)"
                  >
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                      <polyline points="7 10 12 15 17 10"/>
                      <line x1="12" y1="15" x2="12" y2="3"/>
                    </svg>
                    <span>{{ getDownloadSourceName(item.source) }}</span>
                  </button>
                  <p v-if="item.pwd || item.source === 'lanzou'" class="pwd-hint">
                    提取码: {{ item.pwd || '1234' }}
                  </p>
                </div>
              </div>
              <p class="download-hint">
                （请先转存至您的网盘，避免和谐，也将给作者带来无限的更新动力）
              </p>
            </div>

            <!-- 未选择游戏路径时的提示 -->
            <p class="game-path-display warning" v-if="!gamePath">
              请先选择游戏路径
            </p>

            <!-- 选择补丁文件并应用按钮（当本地不存在时显示） -->
            <button
              v-if="!isPatchLocalExists(tab.patchType)"
              class="select-and-apply-btn"
              @click="selectAndApplyPatch(tab)"
              :disabled="!gamePath || applyingPatch"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                <polyline points="14 2 14 8 20 8"/>
                <line x1="12" y1="18" x2="12" y2="12"/>
                <line x1="9" y1="15" x2="15" y2="15"/>
              </svg>
              <span>{{ applyingPatch ? '应用中...' : '选择补丁文件（7z）并应用' }}</span>
            </button>

            <p class="patch-path">补丁路径: {{ tab.patchPath }}</p>
            <p class="game-path-display" v-if="gamePath">
              游戏路径: {{ gamePath }}
            </p>
            <p class="game-path-display warning" v-else>
              请先选择游戏路径
            </p>

            <!-- 应用补丁按钮 -->
            <button
              class="apply-patch-btn"
              @click="applyPatch(tab)"
              :disabled="!gamePath || applyingPatch || !isPatchLocalExists(tab.patchType)"
            >
              <span v-if="applyingPatch">应用中...</span>
              <span v-else>应用补丁</span>
            </button>

            <!-- D加密虚拟机类型显示虚拟化环境配置教程按钮 -->
            <button
              v-if="tab.patchType === 3"
              class="tutorial-btn"
              @click="openVirtualizationTutorial"
            >
              虚拟化环境配置教程
            </button>

            <!-- 补丁说明（显示在应用补丁按钮下方） -->
            <div v-if="getPatchReadme(tab.patchType)" class="patch-readme">
              <h4 class="readme-title">补丁说明</h4>
              <pre class="readme-content">{{ getPatchReadme(tab.patchType) }}</pre>
            </div>

            <!-- 应用结果提示 -->
            <div v-if="patchResult" class="patch-result" :class="{ success: patchResult.success, error: !patchResult.success }">
              <p v-if="patchResult.success" class="result-title">补丁应用成功！</p>
              <p v-else class="result-title">补丁应用完成，但有一些错误</p>

              <div v-if="patchResult.backed_up_files.length > 0" class="result-section">
                <p class="section-title">已备份文件 ({{ patchResult.backed_up_files.length }}个):</p>
                <ul class="file-list">
                  <li v-for="(file, index) in patchResult.backed_up_files.slice(0, 5)" :key="index">
                    {{ getFileName(file) }}
                  </li>
                  <li v-if="patchResult.backed_up_files.length > 5">
                    ...还有 {{ patchResult.backed_up_files.length - 5 }} 个文件
                  </li>
                </ul>
              </div>

              <div v-if="patchResult.copied_files.length > 0" class="result-section">
                <p class="section-title">已复制文件 ({{ patchResult.copied_files.length }}个):</p>
                <ul class="file-list">
                  <li v-for="(file, index) in patchResult.copied_files.slice(0, 5)" :key="index">
                    {{ getFileName(file) }}
                  </li>
                  <li v-if="patchResult.copied_files.length > 5">
                    ...还有 {{ patchResult.copied_files.length - 5 }} 个文件
                  </li>
                </ul>
              </div>

              <div v-if="patchResult.errors.length > 0" class="result-section error-section">
                <p class="section-title">错误 ({{ patchResult.errors.length }}个):</p>
                <ul class="file-list error-list">
                  <li v-for="(error, index) in patchResult.errors.slice(0, 3)" :key="index">
                    {{ error }}
                  </li>
                  <li v-if="patchResult.errors.length > 3">
                    ...还有 {{ patchResult.errors.length - 3 }} 个错误
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>

        <!-- 入库Steam标签页 -->
        <div v-if="currentTab === 'import'" class="tab-panel">
          <h3 class="panel-title">入库Steam</h3>
          <div class="import-steam-content">
            <div class="import-description">
              <p>将游戏清单导入Steam客户端，导入后可在Steam库中下载和启动游戏，如果库不显示游戏，请重启steam。</p>
              <p class="import-note">注意：部分游戏入库下载后需要配合补丁才能正常游玩。</p>
            </div>

            <!-- 入库按钮组 -->
            <div class="import-btn-group">
              <!-- OpenSteamTool内核入库按钮（推荐） -->
              <button
                class="import-opensteamtool-btn-large"
                :class="{ disabled: !canImportToSteam, loading: isImportingWithOpenSteamTool }"
                :disabled="!canImportToSteam || isImportingWithOpenSteamTool"
                @click="importWithOpenSteamTool"
              >
                <svg v-if="isImportingWithOpenSteamTool" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
                </svg>
                <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
                </svg>
                {{ isImportingWithOpenSteamTool ? '入库中...' : 'opensteamtool入库（推荐）' }}
              </button>

              <!-- SteamTools传统入库按钮 -->
              <button
                class="import-steamtools-btn-large"
                :class="{ disabled: !canImportToSteam, loading: isImportingToSteam }"
                :disabled="!canImportToSteam || isImportingToSteam"
                @click="importToSteam"
              >
                <svg v-if="isImportingToSteam" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
                </svg>
                <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
                </svg>
                {{ isImportingToSteam ? '入库中...' : 'steamtools入库' }}
              </button>

              <!-- 重启Steam按钮 -->
              <button
                class="restart-steam-btn"
                @click="restartSteam"
              >
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M23 4v6h-6"/>
                  <path d="M1 20v-6h6"/>
                  <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/>
                </svg>
                重启steam
              </button>
            </div>

            <!-- 自定义源已选择提示 -->
            <div v-if="importSourceReady" class="import-source-info">
              <span class="source-label">当前使用自定义清单源:</span>
              <span class="source-path" :title="selectedImportPath">{{ selectedImportPath }}</span>
              <button class="clear-source-btn" @click="clearImportSource">清除</button>
            </div>

            <!-- 没有vdf/lua时显示下载引导和文件选择 -->
            <div v-if="!hasLua && !importSourceReady" class="import-no-files">
              <!-- 下载引导 -->
              <div class="download-guide">
                <p class="download-guide-title">点击下载对应清单文件7z，id：{{ gameId }}</p>
                <div class="download-buttons">
                  <div class="download-btn-wrapper">
                    <button
                      class="download-patch-btn"
                      @click="openQingdanQRCode"
                    >
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                        <polyline points="7 10 12 15 17 10"/>
                        <line x1="12" y1="15" x2="12" y2="3"/>
                      </svg>
                      <span>夸克网盘</span>
                    </button>
                  </div>
                  <div class="backup-label">备用（容易和谐）：</div>
                  <div class="download-btn-wrapper">
                    <button
                      class="download-patch-btn"
                      @click="openDownloadUrl('https://pan.xunlei.com/s/VOw3jTAGHqYFsm49n2t_AeVGA1?pwd=3r6n')"
                    >
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                        <polyline points="7 10 12 15 17 10"/>
                        <line x1="12" y1="15" x2="12" y2="3"/>
                      </svg>
                      <span>迅雷网盘</span>
                    </button>
                    <p class="pwd-hint">提取码: 3r6n</p>
                  </div>
                  <div class="download-btn-wrapper">
                    <button
                      class="download-patch-btn"
                      @click="openDownloadUrl('https://pan.baidu.com/s/1FTZyknIObyzMuLAJC-Uj9g?pwd=8uwx')"
                    >
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                        <polyline points="7 10 12 15 17 10"/>
                        <line x1="12" y1="15" x2="12" y2="3"/>
                      </svg>
                      <span>百度网盘</span>
                    </button>
                    <p class="pwd-hint">提取码: 8uwx</p>
                  </div>
                </div>
              </div>

              <!-- 文件选择区域 -->
              <div class="import-source-select">
                <h4 class="source-select-title">清单文件选择</h4>

                <div class="radio-group">
                  <label class="radio-label">
                    <input
                      type="radio"
                      v-model="importSourceMode"
                      value="7z"
                      name="import-source-mode"
                    />
                    <span>7z文件</span>
                  </label>
                  <label class="radio-label">
                    <input
                      type="radio"
                      v-model="importSourceMode"
                      value="folder"
                      name="import-source-mode"
                    />
                    <span>.vdf / .lua 和 .manifest所在文件夹</span>
                  </label>
                </div>

                <div class="source-select-actions">
                  <button
                    v-if="importSourceMode === '7z'"
                    class="select-source-btn"
                    :disabled="isPreparingImport"
                    @click="selectImportArchive"
                  >
                    <svg v-if="isPreparingImport" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
                    </svg>
                    {{ isPreparingImport ? '处理中...' : '选择7z文件' }}
                  </button>
                  <button
                    v-else
                    class="select-source-btn"
                    :disabled="isPreparingImport"
                    @click="selectImportFolder"
                  >
                    <svg v-if="isPreparingImport" class="loading-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
                    </svg>
                    {{ isPreparingImport ? '处理中...' : '选择文件夹' }}
                  </button>
                </div>

                <p v-if="selectedImportPath && !importSourceReady" class="source-select-error">
                  所选位置未找到vdf或lua文件，无法入库
                </p>
              </div>
            </div>
          </div>
        </div>

        <!-- 解压即玩标签页 -->
        <div v-if="currentTab === 'extract-play'" class="tab-panel">
          <h3 class="panel-title">解压即玩</h3>
          <div class="extract-play-content">
            <div class="extract-play-description">
              <p>直接从网盘下载完整游戏文件（豪华版），下载完成后解压即可游玩，无需额外操作。</p>
              <p class="extract-play-note">（都是无法联机版，除非网盘文件特别标注或者手动注入联机补丁）</p>
            </div>

            <!-- 夸克网盘下载区域 -->
            <div class="download-section">
              <p class="download-section-title">下载游戏：</p>
              <div class="download-buttons">
                <div class="download-btn-wrapper">
                  <button
                    class="download-patch-btn"
                    :class="{ disabled: !quarkQRCodeExists }"
                    :disabled="!quarkQRCodeExists"
                    @click="openQuarkQRCode"
                    :title="quarkQRCodeExists ? '点击扫码下载' : '暂未上传完游戏'"
                  >
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                      <polyline points="7 10 12 15 17 10"/>
                      <line x1="12" y1="15" x2="12" y2="3"/>
                    </svg>
                    夸克网盘
                  </button>
                  <span v-if="!quarkQRCodeExists" class="pwd-hint">暂未上传完游戏</span>
                </div>
              </div>
              <p class="download-hint">（请先转存至您的网盘，避免和谐，也将给作者带来无限的更新动力）</p>
            </div>
          </div>
        </div>

        <!-- 修改器标签页 -->
        <div v-if="currentTab === 'trainer'" class="tab-panel">
          <h3 class="panel-title">游戏修改器</h3>
          <div class="trainer-content">
            <!-- 修改器下载区域 -->
            <div v-if="game?.trainer?.download_urls && game.trainer.download_urls.length > 0" class="download-section">
              <p class="download-section-title">修改器下载：</p>
              <div class="download-buttons">
                <div
                  v-for="(item, index) in game.trainer.download_urls"
                  :key="index"
                  class="download-btn-wrapper"
                >
                  <button
                    class="download-patch-btn"
                    @click="openDownloadUrl(item.url)"
                  >
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                      <polyline points="7 10 12 15 17 10"/>
                      <line x1="12" y1="15" x2="12" y2="3"/>
                    </svg>
                    <span>{{ getDownloadSourceName(item.source) }}</span>
                  </button>
                  <p v-if="item.pwd" class="pwd-hint">
                    提取码: {{ item.pwd }}
                  </p>
                </div>
              </div>
              <p class="download-hint">
                （请先转存至您的网盘，避免和谐，也将给作者带来无限的更新动力）
              </p>
            </div>

            <!-- 本地修改器内容 -->
            <div v-if="trainerContent" class="trainer-local-content">
              <h4 class="trainer-content-title">修改器说明</h4>
              <pre class="trainer-content-text">{{ trainerContent }}</pre>
            </div>
            <div v-else-if="game?.trainer?.local_path" class="trainer-no-content">
              <p>本地修改器文件不存在或无法读取</p>
              <p class="trainer-path">路径: {{ game.trainer.local_path }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { convertFileSrc } from '@tauri-apps/api/core'
import DownloadProgress from '../../components/download/DownloadProgress.vue'
import QRCodeModal from '../../components/common/QRCodeModal.vue'
import type { DownloadProgress as DownloadProgressType } from '../../types/download.types'
import type { GameConfigData } from '../../types'
import { getPatchSourcePath } from '../../types'
import { loadGamesConfigFromFile } from '../../api/game.api'
import { getCachedCoverImage } from '../../services/imageCache.service'
import { 
  getGameData, 
  upsertGameData, 
  updateGameDownloadStatus,
  finalizeGameDownload,
  type GameData 
} from '../../api/gameData.api'
import { getCategoryName, getCategoryColor } from '../../constants/game'
import { safeAsync } from '../../utils/async-helper'
import { getFileName, sanitizeFolderName } from '../../utils/file-helper'
import { useConfigStore } from '../../store/config.store'
import FirstTimeSetupModal from '../../components/manifest/FirstTimeSetupModal.vue'

const route = useRoute()
const router = useRouter()
const configStore = useConfigStore()

// 游戏ID
const gameId = computed(() => route.params.id as string)

// 游戏数据
const gamesConfig = ref<GameConfigData[]>([])
const game = computed(() => gamesConfig.value.find(g => g.game_id === gameId.value))

// 已存在的游戏数据（从game.json加载）
const existingGameData = ref<GameData | null>(null)

// 封面图片
const coverImage = ref('')

// 路径数据
const downloadPath = ref('')
const gamePath = ref('')

// 清单文件夹路径（自动检测）
const manifestFolderPath = ref('')

// 清单文件夹检测状态
const manifestCheckStatus = ref<'checking' | 'found' | 'not_found'>('checking')

// 当前选中的标签页
const currentTab = ref('')

// 修改器本地文件内容
const trainerContent = ref<string>('')

// 根据游戏数据设置默认标签页
const setDefaultTab = () => {
  if (game.value?.downloadable) {
    currentTab.value = 'download'
  } else if (game.value?.has_extract_play === true) {
    // 如果没有下载功能但有解压即玩，默认选中解压即玩标签
    currentTab.value = 'extract-play'
  } else if (game.value?.tags && game.value.tags.length > 0) {
    // 如果没有下载功能和解压即玩，默认选中第一个补丁标签
    currentTab.value = `patch-${game.value.tags[0].patch_type}`
  } else {
    currentTab.value = ''
  }
}

// 补丁应用状态
const applyingPatch = ref(false)
const patchResult = ref<{
  success: boolean
  backed_up_files: string[]
  copied_files: string[]
  errors: string[]
} | null>(null)

// 补丁说明缓存
const patchReadmes = ref<Map<number, string>>(new Map())

// 本地补丁文件存在状态缓存
const patchLocalStatus = ref<Map<number, boolean>>(new Map())

// 下载状态
const isDownloading = ref(false)
const isVerifying = ref(false)
const downloadLogs = ref<{ time: string; message: string; type: 'info' | 'success' | 'error' | 'warning' }[]>([])

// 下载进度监控
const isMonitoring = ref(false)
const downloadProgress = ref<DownloadProgressType>({
  totalDepots: 0,
  completedDepots: 0,
  overallPercentage: 0,
  depots: [],
  isComplete: false
})
let monitorInterval: number | null = null

// 入库Steam状态
const isImportingToSteam = ref(false)
const isImportingWithOpenSteamTool = ref(false)
const manifestExists = ref(false)
const hasLua = ref(false)
const hasVdf = ref(false)
const hasManifest = ref(false)
const showFirstTimeSetup = ref(false)

// 自定义清单源状态
const importSourceMode = ref<'7z' | 'folder'>('7z')
const selectedImportPath = ref('')
const selectedImportTempPath = ref('')
const isPreparingImport = ref(false)
const importSourceReady = computed(() => {
  if (!selectedImportPath.value) return false
  // 7z模式需要已经解压到临时目录并检测到lua
  if (importSourceMode.value === '7z' && !selectedImportTempPath.value) return false
  return hasLua.value
})

// 游戏下载标签页的自定义清单源状态
const downloadManifestSourceMode = ref<'7z' | 'folder'>('7z')
const selectedDownloadManifestPath = ref('')
const isPreparingDownloadManifest = ref(false)

// 二维码弹窗状态
const showQRCodeModal = ref(false)
const qrCodeTitle = ref('夸克网盘下载')
const qrCodeImageUrl = ref('')
const qrCodeHint = ref('请使用夸克APP扫码下载')

// 夸克网盘二维码是否存在
const quarkQRCodeExists = ref(false)

/**
 * 获取夸克网盘二维码图片路径
 * 图片存放在 resources/pic/Quark_QR/{game_id}.png
 */
const getQuarkQRCodePath = (gameId: string): string => {
  // 使用相对路径，程序根目录下的 resources/pic/Quark_QR/
  return `resources/pic/Quark_QR/${gameId}.png`
}

/**
 * 检查夸克网盘二维码是否存在
 */
const checkQuarkQRCodeExists = async (gameId: string): Promise<boolean> => {
  try {
    const qrPath = getQuarkQRCodePath(gameId)
    const exists = await invoke<boolean>('check_file_exists', { path: qrPath })
    return exists
  } catch {
    return false
  }
}

/**
 * 打开夸克网盘二维码弹窗
 */
const openQuarkQRCode = async () => {
  // 检查二维码是否存在
  const exists = await checkQuarkQRCodeExists(gameId.value)
  if (!exists) {
    return
  }

  // 设置弹窗内容
  qrCodeTitle.value = '夸克网盘下载'
  qrCodeHint.value = '请使用夸克APP扫码下载'

  // 获取二维码图片URL
  const qrPath = getQuarkQRCodePath(gameId.value)
  qrCodeImageUrl.value = convertFileSrc(qrPath)

  // 显示弹窗
  showQRCodeModal.value = true
}

/**
 * 关闭二维码弹窗
 */
const handleQRCodeClose = () => {
  showQRCodeModal.value = false
  qrCodeImageUrl.value = ''
}

/**
 * 打开清单下载夸克网盘二维码弹窗
 * 使用程序内置的 qingdan.png 二维码图片
 */
const openQingdanQRCode = async () => {
  qrCodeTitle.value = '夸克网盘下载'
  qrCodeHint.value = '请使用夸克APP扫码下载'
  qrCodeImageUrl.value = await invoke<string>('get_qingdan_image_base64')
  showQRCodeModal.value = true
}

const canImportToSteam = computed(() => {
  // 只要有lua就可以入库（内置清单或自定义源）
  return (hasLua.value || importSourceReady.value) && !isImportingToSteam.value
})
// 首次使用配置 - 关闭
function handleFirstTimeSetupClose() {
  showFirstTimeSetup.value = false
}

// 首次使用配置 - 确认
async function handleFirstTimeSetupConfirm() {
  try {
    // 保存标志到 config.json
    await configStore.updateConfig({
      launch: {
        startMinimizedToTray: configStore.config?.launch?.startMinimizedToTray ?? false,
        hideToTrayOnClose: configStore.config?.launch?.hideToTrayOnClose ?? false,
        verifyBeforeLaunch: configStore.config?.launch?.verifyBeforeLaunch ?? false,
        manifestImportInitialized: true
      }
    })
    alert('配置已保存，请完成初始化操作后重新点击入库按钮。')
  } catch (error) {
    alert(`保存配置失败: ${error}`)
  } finally {
    showFirstTimeSetup.value = false
  }
}

// 可用的标签页
// 优先级：游戏下载 > 解压即玩 > steam入库 > 各类补丁
const availableTabs = computed(() => {
  const tabs: { id: string; name: string }[] = []

  // 1. 如果有downloadable，添加游戏下载标签
  if (game.value?.downloadable) {
    tabs.push({ id: 'download', name: '游戏下载' })
  }

  // 2. 如果有解压即玩版本，添加解压即玩标签
  if (game.value?.has_extract_play === true) {
    tabs.push({ id: 'extract-play', name: '解压即玩' })
  }

  // 3. 入库Steam标签始终显示
  tabs.push({ id: 'import', name: '入库Steam' })

  // 4. 添加游戏分类标签（各类补丁）
  game.value?.tags.forEach(tag => {
    tabs.push({
      id: `patch-${tag.patch_type}`,
      name: getCategoryName(tag.patch_type)
    })
  })

  // 5. 如果有修改器配置且包含下载链接，添加修改器标签
  if (game.value?.trainer?.download_urls && game.value.trainer.download_urls.length > 0) {
    tabs.push({ id: 'trainer', name: '修改器' })
  }

  return tabs
})

// 补丁标签页
const patchTabs = computed(() => {
  return game.value?.tags.map(tag => ({
    id: `patch-${tag.patch_type}`,
    name: getCategoryName(tag.patch_type),
    patchType: tag.patch_type,
    patchPath: getPatchSourcePath(tag, game.value?.game_id || ''),
    downloadUrls: tag.download_urls || []
  })) || []
})

/**
 * 获取下载来源显示名称
 * @param source 网盘来源标识
 */
const getDownloadSourceName = (source: string): string => {
  const sourceMap: Record<string, string> = {
    'baidu': '百度网盘',
    'thunder': '迅雷网盘',
    'lanzou': '蓝奏云',
    'quark': '夸克网盘',
    'other': '其他网盘'
  }
  return sourceMap[source] || source || '未知网盘'
}

// 是否可以开始下载
const canDownload = computed(() => {
  return manifestCheckStatus.value === 'found' && downloadPath.value !== '' && !isDownloading.value
})

// 获取下载按钮文本
const getDownloadButtonText = () => {
  if (existingGameData.value?.download_status === 'downloading') {
    return '下载中...'
  }
  if (isDownloading.value) {
    return '下载中...'
  }
  if (existingGameData.value?.download_status === 'completed') {
    return '已下载'
  }
  return '开始下载'
}

// 返回上一页
const goBack = () => {
  router.back()
}

// 处理图片加载错误 - 尝试重新加载
const handleImageError = async (e: Event) => {
  const img = e.target as HTMLImageElement
  // 延迟后尝试重新加载图片
  if (gameId.value) {
    setTimeout(async () => {
      try {
        const { getGameCoverImage } = await import('../../api/game.api')
        const { convertFileSrc } = await import('@tauri-apps/api/core')
        const filePath = await getGameCoverImage(gameId.value)
        if (filePath) {
          img.src = convertFileSrc(filePath)
        }
      } catch {
        // 重试失败则隐藏图片
        img.style.display = 'none'
      }
    }, 500)
  } else {
    img.style.display = 'none'
  }
}

// 分类名称和颜色从 constants/game 导入

// 选择下载路径
const selectDownloadPath = async () => {
  try {
    const selected = await open({
      directory: true,
      title: '选择游戏下载路径'
    })
    if (selected) {
      downloadPath.value = selected
    }
  } catch (error) {
    alert('选择文件夹失败: ' + error)
  }
}

// 选择游戏路径
const selectGamePath = async () => {
  try {
    const selected = await open({
      directory: true,
      title: '选择游戏安装路径'
    })
    if (selected) {
      gamePath.value = selected
    }
  } catch (error) {
    alert('选择文件夹失败: ' + error)
  }
}

// 开始下载
const startDownload = async () => {
  // 检查是否有清单文件夹
  if (!manifestFolderPath.value) {
    alert('未找到清单文件夹，请先下载清单文件')
    return
  }

  // 检查下载路径
  if (!downloadPath.value) {
    alert('请先选择下载路径')
    return
  }

  isDownloading.value = true
  downloadLogs.value = [] // 清空之前的日志

  addDownloadLog('开始下载游戏...', 'info')
  addDownloadLog(`游戏: ${game.value?.game_name || gameId.value}`, 'info')
  addDownloadLog(`清单路径: ${manifestFolderPath.value}`, 'info')
  addDownloadLog(`下载路径: ${downloadPath.value}`, 'info')

  try {
    // 先保存游戏数据到game.json
    const gameData: GameData = {
      game_id: gameId.value,
      game_name: game.value?.game_name || '',
      chinese_name: game.value?.chinese_name || '',
      game_type: 'downloaded',
      install_path: downloadPath.value,
      exe_path: '', // 下载完成后需要用户设置
      is_favorite: false,
      is_installed: false,
      play_time: 0,
      added_date: new Date().toISOString(),
      download_status: 'downloading',
      download_progress: 0,
      download_path: downloadPath.value
    }
    await upsertGameData(gameData)
    existingGameData.value = gameData

    // 调用Rust后端执行下载命令
    const result = await invoke<{
      success: boolean
      message: string
    }>('start_game_download', {
      manifestPath: manifestFolderPath.value,
      downloadPath: downloadPath.value,
      gameId: gameId.value
    })

    if (result.success) {
      addDownloadLog('下载命令已启动', 'success')
      addDownloadLog(result.message, 'info')
      // 启动进度监控
      startProgressMonitoring()
    } else {
      addDownloadLog(`下载启动失败: ${result.message}`, 'error')
      await updateGameDownloadStatus(gameId.value, 'error', 0)
    }
  } catch (error) {
    addDownloadLog(`下载出错: ${error}`, 'error')
    await updateGameDownloadStatus(gameId.value, 'error', 0)
  } finally {
    isDownloading.value = false
  }
}

/**
 * 停止下载
 * 终止 ddv20.exe 进程，并将游戏状态设置为未下载
 */
const stopDownload = async () => {
  try {
    addDownloadLog('正在停止下载...', 'info')
    await invoke('stop_download', {
      gameId: gameId.value
    })
    addDownloadLog('下载已停止', 'success')

    // 停止进度监控
    stopProgressMonitoring()

    // 重新加载游戏数据以更新状态显示
    existingGameData.value = await getGameData(gameId.value)

    isDownloading.value = false
  } catch (error) {
    addDownloadLog(`停止下载失败: ${error}`, 'error')
  }
}

/**
 * 解析进度文件名获取depot ID和百分比
 * 文件名格式: "{百分比}% - {depotId}.json"
 */
const parseProgressFileName = (fileName: string): { depotId: string; percentage: number } | null => {
  const match = fileName.match(/^(\d+)%\s*-\s*(\d+)\.json$/)
  if (match) {
    return {
      percentage: parseInt(match[1], 10),
      depotId: match[2]
    }
  }
  return null
}

/**
 * 验证游戏完整性
 * 使用与下载相同的参数重新运行 ddv20.exe，验证并补全缺失文件
 */
const verifyIntegrity = async () => {
  if (!manifestFolderPath.value) {
    alert('未找到清单文件夹')
    return
  }

  if (!existingGameData.value?.download_path) {
    alert('未找到下载路径')
    return
  }

  isVerifying.value = true
  downloadLogs.value = []
  addDownloadLog('开始验证游戏完整性...', 'info')
  addDownloadLog(`游戏: ${game.value?.game_name || gameId.value}`, 'info')
  addDownloadLog(`清单路径: ${manifestFolderPath.value}`, 'info')
  addDownloadLog(`下载路径: ${existingGameData.value.download_path}`, 'info')

  try {
    // 调用与下载相同的命令，ddv20.exe 会自动验证并补全
    const result = await invoke<{
      success: boolean
      message: string
    }>('start_game_download', {
      manifestPath: manifestFolderPath.value,
      downloadPath: existingGameData.value.download_path,
      gameId: gameId.value
    })

    if (result.success) {
      addDownloadLog('验证命令已启动', 'success')
      addDownloadLog(result.message, 'info')
      // 启动进度监控
      startProgressMonitoring()
    } else {
      addDownloadLog(`验证启动失败: ${result.message}`, 'error')
    }
  } catch (error) {
    addDownloadLog(`验证出错: ${error}`, 'error')
  } finally {
    isVerifying.value = false
  }
}

/**
 * 扫描进度文件
 */
const scanProgressFiles = async () => {
  try {
    // 获取指定游戏的进度文件
    const progressFiles = await invoke<Array<{ name: string; path: string }>>('get_download_progress_files', {
      gameId: gameId.value
    })

    // 更新每个depot的进度
    const updatedDepots = [...downloadProgress.value.depots]

    for (const file of progressFiles) {
      const parsed = parseProgressFileName(file.name)
      if (parsed) {
        // 读取文件内容获取已下载文件数量
        const fileContent = await invoke<Record<string, string[]>>('read_json_file', {
          filePath: file.path
        }).catch(() => ({}))

        const downloadedFiles = Object.keys(fileContent).length

        // 查找对应的depot并更新
        const depotIndex = updatedDepots.findIndex(d => d.depotId === parsed.depotId)
        if (depotIndex !== -1) {
          updatedDepots[depotIndex] = {
            depotId: parsed.depotId,
            percentage: parsed.percentage,
            downloadedFiles,
            totalFiles: downloadedFiles,
            status: parsed.percentage >= 100 ? 'completed' : 'downloading'
          }
        } else {
          // 如果depot不在列表中，添加它
          updatedDepots.push({
            depotId: parsed.depotId,
            percentage: parsed.percentage,
            downloadedFiles,
            totalFiles: downloadedFiles,
            status: parsed.percentage >= 100 ? 'completed' : 'downloading'
          })
        }
      }
    }

    // 计算总体进度
    const completedDepots = updatedDepots.filter(d => d.status === 'completed').length
    const overallPercentage = updatedDepots.length > 0
      ? Math.round(updatedDepots.reduce((sum, d) => sum + d.percentage, 0) / updatedDepots.length)
      : 0

    downloadProgress.value = {
      totalDepots: downloadProgress.value.totalDepots || updatedDepots.length,
      completedDepots,
      overallPercentage,
      depots: updatedDepots,
      isComplete: updatedDepots.length > 0 && updatedDepots.every(d => d.status === 'completed')
    }

    // 更新下载状态到game.json
    if (existingGameData.value) {
      await updateGameDownloadStatus(gameId.value, downloadProgress.value.isComplete ? 'completed' : 'downloading', overallPercentage)
    }

    // 如果下载完成，停止监控
    if (downloadProgress.value.isComplete) {
      stopProgressMonitoring()
      addDownloadLog('所有depot下载完成！', 'success')
      // 更新游戏数据为已完成
      await updateGameDownloadStatus(gameId.value, 'completed', 100)
      // 执行下载完成收尾：扫描目录、定位 exe、标记已安装
      try {
        const finalizedGame = await finalizeGameDownload(gameId.value)
        existingGameData.value = finalizedGame
        addDownloadLog('游戏已入库，可前往游戏库查看', 'success')
      } catch (finalizeError) {
        addDownloadLog(`入库处理失败: ${finalizeError}`, 'error')
        existingGameData.value = await getGameData(gameId.value)
      }
    }
  } catch (error) {
    // 扫描进度文件失败时静默处理
  }
}

/**
 * 开始监控下载进度
 */
const startProgressMonitoring = async () => {
  if (isMonitoring.value) return

  isMonitoring.value = true

  // 首先读取manifest文件夹，获取所有depot ID
  try {
    const result = await invoke<{
      jsonFiles: string[]
      vdfFiles: string[]
      manifestFiles: string[]
    }>('read_manifest_folder', { folderPath: manifestFolderPath.value })

    // 从manifest文件名中提取depot ID
    // 文件名格式: "{depotId}_{version}.manifest"
    // 注意: result.manifestFiles 是完整路径，如 D:\...\261550\261551_8211251051201469236.manifest
    const depotIds = result.manifestFiles.map(filePath => {
      // 使用更健壮的正则，匹配路径分隔符后的文件名
      const match = filePath.match(/[\\/](\d+)_\d+\.manifest$/)
      return match ? match[1] : null
    }).filter(id => id !== null) as string[]

    // 初始化所有depot的进度状态
    downloadProgress.value.totalDepots = depotIds.length
    downloadProgress.value.depots = depotIds.map(depotId => ({
      depotId,
      percentage: 0,
      downloadedFiles: 0,
      totalFiles: 0,
      status: 'pending' as const
    }))

    addDownloadLog(`检测到 ${depotIds.length} 个depot`, 'info')
  } catch (error) {
    addDownloadLog(`读取manifest文件夹失败: ${error}`, 'error')
  }

  // 立即扫描一次
  await scanProgressFiles()

  // 设置定时扫描
  monitorInterval = window.setInterval(() => {
    scanProgressFiles()
  }, 1000)
}

/**
 * 停止监控下载进度
 */
const stopProgressMonitoring = () => {
  isMonitoring.value = false
  if (monitorInterval) {
    clearInterval(monitorInterval)
    monitorInterval = null
  }
}

// 应用补丁
const applyPatch = async (tab: any) => {
  if (!gamePath.value) {
    alert('请先选择游戏路径')
    return
  }

  applyingPatch.value = true
  patchResult.value = null

  try {
    const result = await invoke<{
      success: boolean
      backed_up_files: string[]
      copied_files: string[]
      errors: string[]
    }>('apply_patch', {
      patchSourcePath: tab.patchPath,
      gamePath: gamePath.value
    })

    patchResult.value = result
    
    if (result.success) {
      alert('补丁应用成功！')
    } else {
      alert('补丁应用完成，但有一些错误，请查看详情')
    }
  } catch (error) {
    alert('应用补丁失败: ' + error)
    patchResult.value = {
      success: false,
      backed_up_files: [],
      copied_files: [],
      errors: [String(error)]
    }
  } finally {
    applyingPatch.value = false
  }
}

// 检查本地补丁文件是否存在
const checkPatchLocalStatus = async () => {
  if (!game.value) return

  for (const tag of game.value.tags) {
    try {
      const patchPath = getPatchSourcePath(tag, game.value.game_id)
      const result = await invoke<string>('check_patch_file_exists', {
        patchSourcePath: patchPath
      })
      patchLocalStatus.value.set(tag.patch_type, result !== '')
    } catch (error) {
      patchLocalStatus.value.set(tag.patch_type, false)
    }
  }
}

// 获取本地补丁存在状态
const isPatchLocalExists = (patchType: number): boolean => {
  return patchLocalStatus.value.get(patchType) || false
}

// 打开下载链接
const openDownloadUrl = async (url: string) => {
  try {
    await invoke('open_external_link', { url })
  } catch (error) {
    alert('打开下载链接失败: ' + error)
  }
}

// 打开虚拟化环境配置教程视频
const openVirtualizationTutorial = async () => {
  try {
    await invoke('open_virtualization_tutorial')
  } catch (error) {
    alert('打开视频失败: ' + error)
  }
}

// 选择本地补丁文件并直接应用
const selectAndApplyPatch = async (_tab: any) => {
  if (!gamePath.value) {
    alert('请先选择游戏路径')
    return
  }

  try {
    // 打开文件选择对话框
    const selected = await open({
      title: '选择补丁压缩包文件',
      filters: [{
        name: '7z压缩包',
        extensions: ['7z']
      }],
      multiple: false
    })

    if (!selected) {
      return // 用户取消了选择
    }

    const archivePath = Array.isArray(selected) ? selected[0] : selected

    // 确认应用
    const confirmApply = confirm(`确定要将补丁应用到游戏目录吗？\n\n补丁文件: ${archivePath}\n游戏路径: ${gamePath.value}\n\n应用前会自动备份原有文件。`)
    if (!confirmApply) {
      return
    }

    applyingPatch.value = true
    patchResult.value = null

    // 调用后端命令直接应用补丁
    const result = await invoke<{
      success: boolean
      backed_up_files: string[]
      copied_files: string[]
      errors: string[]
    }>('apply_patch_from_file', {
      archivePath: archivePath,
      gamePath: gamePath.value
    })

    patchResult.value = result

    if (result.success) {
      alert('补丁应用成功！')
    } else {
      alert('补丁应用完成，但有一些错误，请查看详情')
    }
  } catch (error) {
    alert('应用补丁失败: ' + error)
    patchResult.value = {
      success: false,
      backed_up_files: [],
      copied_files: [],
      errors: [String(error)]
    }
  } finally {
    applyingPatch.value = false
  }
}

// 加载封面图片 - 带重试机制确保100%加载成功
const loadCoverImage = async (retryCount = 0): Promise<void> => {
  if (!gameId.value) return

  try {
    const cachedUrl = await getCachedCoverImage(gameId.value)
    if (cachedUrl) {
      coverImage.value = cachedUrl
      return
    }
  } catch {
    // 缓存获取失败，继续尝试直接加载
  }

  // 如果缓存未返回URL，尝试直接调用后端获取路径
  try {
    const { getGameCoverImage } = await import('../../api/game.api')
    const { convertFileSrc } = await import('@tauri-apps/api/core')
    const filePath = await getGameCoverImage(gameId.value)
    if (filePath) {
      coverImage.value = convertFileSrc(filePath)
      return
    }
  } catch {
    // 直接加载也失败
  }

  // 如果都失败了且重试次数小于3次，延迟后重试
  if (retryCount < 3) {
    setTimeout(() => loadCoverImage(retryCount + 1), 300 * (retryCount + 1))
  }
}

/**
 * 加载修改器本地文件内容
 */
const loadTrainerContent = async () => {
  if (!game.value?.trainer?.local_path) {
    return
  }

  try {
    const content = await invoke<string>('read_text_file', {
      filePath: game.value.trainer.local_path
    })
    trainerContent.value = content
  } catch {
    // 读取失败时保持为空字符串
    trainerContent.value = ''
  }
}

// 页面加载时确保游戏数据已加载，并自动检测清单文件夹
onMounted(async () => {
  // 加载游戏配置
  if (gamesConfig.value.length === 0) {
    const config = await safeAsync(
      () => loadGamesConfigFromFile(),
      '加载游戏配置失败'
    )
    if (config) gamesConfig.value = config
  }

  // 加载已存在的游戏数据
  const gameData = await safeAsync(
    () => getGameData(gameId.value),
    '加载游戏数据失败'
  )
  if (gameData) {
    // 检查游戏目录是否实际存在（防止用户手动删除游戏文件）
    let isGameDirExists = false
    if (gameData.install_path) {
      try {
        isGameDirExists = await invoke<boolean>('check_directory_exists', {
          path: gameData.install_path
        })
      } catch {
        isGameDirExists = false
      }
    }

    // 如果记录为已下载但实际目录不存在，重置状态
    if (gameData.download_status === 'completed' && !isGameDirExists) {
      console.log('游戏目录不存在，重置下载状态:', gameData.install_path)
      gameData.download_status = ''
      gameData.is_installed = false
      // 更新到 game.json
      await safeAsync(
        () => upsertGameData(gameData),
        '重置游戏状态失败'
      )
    }

    existingGameData.value = gameData
    // 恢复下载路径：如果游戏已下载且目录仍存在，使用保存的路径；否则优先使用全局默认下载路径
    const defaultPath = configStore.config?.gameDirs?.defaultDownloadPath
    if (gameData.download_status === 'completed' && isGameDirExists && gameData.download_path) {
      downloadPath.value = gameData.download_path
    } else if (defaultPath) {
      const folderName = sanitizeFolderName(gameData.game_name || gameId.value)
      downloadPath.value = `${defaultPath}\\${folderName}`
    } else if (gameData.download_path) {
      downloadPath.value = gameData.download_path
    }
    if (gameData.install_path) {
      gamePath.value = gameData.install_path
    }
    // 如果正在下载中，恢复监控
    if (gameData.download_status === 'downloading') {
      startProgressMonitoring()
    }

    // 补偿处理：之前版本下载完成后可能只设置了 completed 但没有标记 is_installed
    // 如果检测到已完成但未安装，自动执行收尾
    if (gameData.download_status === 'completed' && !gameData.is_installed) {
      try {
        const finalizedGame = await finalizeGameDownload(gameId.value)
        existingGameData.value = finalizedGame
      } catch {
        // 补偿失败时保持原数据，不影响页面显示
      }
    }
  }

  // 加载封面图片 - 使用重试机制确保100%加载
  await loadCoverImage()

  // 设置默认标签页
  setDefaultTab()
  // 自动填充下载路径（不依赖清单文件夹是否存在）
  await autoSetDownloadPath()
  // 自动检测清单文件夹
  await checkManifestFolder()
  // 加载所有补丁说明
  await loadPatchReadmes()
  // 检查夸克网盘二维码是否存在
  quarkQRCodeExists.value = await checkQuarkQRCodeExists(gameId.value)
  // 检查本地补丁文件状态
  await checkPatchLocalStatus()
  // 检查游戏清单文件是否存在（用于入库Steam按钮）
  await checkGameManifest()
  // 加载修改器本地文件内容
  await loadTrainerContent()
})

// 组件卸载时清理定时器
onUnmounted(() => {
  stopProgressMonitoring()
})

// 加载所有补丁说明
const loadPatchReadmes = async () => {
  if (!game.value) return

  for (const tag of game.value.tags) {
    try {
      const readme = await invoke<string>('get_patch_readme', {
        gameId: game.value.game_id,
        patchType: tag.patch_type
      })
      if (readme) {
        patchReadmes.value.set(tag.patch_type, readme)
      }
    } catch (error) {
    }
  }
}

// 获取补丁说明
const getPatchReadme = (patchType: number): string => {
  return patchReadmes.value.get(patchType) || ''
}

/**
 * 检查 manifest 文件夹是否存在
 * 自动查找 resources/manifest/游戏ID 目录
 * 并自动转换格式（Lua -> VDF）
 */
const checkManifestFolder = async () => {
  manifestCheckStatus.value = 'checking'
  manifestFolderPath.value = ''

  try {
    // 构建清单文件夹路径：resources/manifest/游戏ID
    const manifestPath = await invoke<string>('get_manifest_path', {
      gameId: gameId.value
    })

    if (manifestPath) {
      manifestFolderPath.value = manifestPath

      // 扫描并转换清单文件格式（如果没有VDF但有Lua，自动转换）
      const scanResult = await invoke<{
        success: boolean
        hasVdf: boolean
        hasLua: boolean
        hasManifest: boolean
        converted: boolean
        message: string
      }>('scan_and_convert_manifest_for_download', {
        folderPath: manifestPath
      })

      if (scanResult.success) {
        manifestCheckStatus.value = 'found'
        if (scanResult.converted) {
          addDownloadLog(`已自动将Lua转换为VDF格式`, 'success')
        }
        // 自动设置下载路径
        await autoSetDownloadPath()
      } else {
        manifestCheckStatus.value = 'not_found'
        addDownloadLog(`清单文件检查失败: ${scanResult.message}`, 'error')
      }
    } else {
      manifestCheckStatus.value = 'not_found'
    }
  } catch (error) {
    manifestCheckStatus.value = 'not_found'
  }
}

/**
 * 自动设置下载路径
 * 如果游戏已下载且未卸载，使用保存的路径；否则优先使用全局配置的默认下载路径
 * 路径格式：默认路径\游戏英文名 或 D:\SteamGame\游戏英文名
 * 仅当游戏可下载时设置
 */
const autoSetDownloadPath = async () => {
  // 如果游戏不可下载，不设置下载路径
  if (!game.value?.downloadable) {
    downloadPath.value = ''
    return
  }

  try {
    // 构建下载路径，使用游戏英文名作为文件夹名（清理非法字符）
    const rawFolderName = game.value?.game_name || gameId.value
    const folderName = sanitizeFolderName(rawFolderName)

    // 如果游戏已下载且未卸载，使用保存的路径
    if (existingGameData.value?.download_status === 'completed' && existingGameData.value?.download_path) {
      downloadPath.value = existingGameData.value.download_path
      return
    }

    // 优先使用全局配置中的默认下载路径
    const defaultPath = configStore.config?.gameDirs?.defaultDownloadPath
    if (defaultPath) {
      const path = `${defaultPath}\\${folderName}`
      downloadPath.value = path
    } else {
      // 获取可用的游戏盘符（优先D盘，其次C盘）
      const drive = await invoke<string>('get_available_drive')
      const path = `${drive}\\SteamGame\\${folderName}`
      downloadPath.value = path
    }
  } catch (error) {
    // 自动设置下载路径失败时静默处理
  }
}

/**
 * 添加下载日志
 * @param message 日志消息
 * @param type 日志类型
 */
const addDownloadLog = (message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') => {
  const now = new Date()
  const time = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:${now.getSeconds().toString().padStart(2, '0')}`
  downloadLogs.value.push({ time, message, type })

  // 限制日志数量
  if (downloadLogs.value.length > 100) {
    downloadLogs.value = downloadLogs.value.slice(-80)
  }
}

/**
 * 检查游戏清单文件状态
 * 有vdf时自动转换为lua，有lua时直接使用
 */
const checkGameManifest = async () => {
  try {
    const result = await invoke<{
      exists: boolean
      hasLua: boolean
      hasVdf: boolean
      hasManifest: boolean
      canImport: boolean
    }>('check_game_manifest_exists', {
      gameId: gameId.value
    })

    manifestExists.value = result.exists
    hasLua.value = result.hasLua
    hasVdf.value = result.hasVdf
    hasManifest.value = result.hasManifest

    // 有vdf但没有lua时，自动转换vdf为lua
    if (result.hasVdf && !result.hasLua) {
      await convertVdfToLuaInManifestFolder()
    }
  } catch (error) {
    manifestExists.value = false
    hasLua.value = false
    hasVdf.value = false
    hasManifest.value = false
  }
}

/**
 * 将内置清单文件夹中的vdf转换为lua
 */
const convertVdfToLuaInManifestFolder = async () => {
  try {
    const manifestPath = await invoke<string>('get_manifest_path', {
      gameId: gameId.value
    })

    if (!manifestPath) return

    const vdfFiles = await invoke<string[]>('get_vdf_files_in_folder', {
      folder: manifestPath
    })

    for (const vdfFile of vdfFiles) {
      try {
        const convertResult = await invoke<{
          success: boolean
          outputPath: string
          message: string
        }>('convert_vdf_to_lua', {
          filePath: vdfFile
        })

        if (convertResult.success) {
          hasLua.value = true
        }
      } catch {
        // 单个文件转换失败继续处理下一个
      }
    }
  } catch {
    // 转换失败时保持当前状态
  }
}

/**
 * 扫描指定文件夹并转换vdf为lua
 * 返回扫描后的文件状态
 */
const scanAndConvertFolder = async (folderPath: string) => {
  hasLua.value = false
  hasVdf.value = false
  hasManifest.value = false

  const scanResult = await invoke<{
    luaFiles: string[]
    manifestFiles: string[]
    vdfFiles: string[]
  }>('scan_manifest_folder', {
    folderPath
  })

  hasLua.value = scanResult.luaFiles.length > 0
  hasVdf.value = scanResult.vdfFiles.length > 0
  hasManifest.value = scanResult.manifestFiles.length > 0

  // 有vdf但没有lua时，自动转换
  if (hasVdf.value && !hasLua.value) {
    for (const vdfFile of scanResult.vdfFiles) {
      try {
        const convertResult = await invoke<{
          success: boolean
          outputPath: string
          message: string
        }>('convert_vdf_to_lua', {
          filePath: vdfFile
        })

        if (convertResult.success) {
          hasLua.value = true
        }
      } catch {
        // 单个文件转换失败继续处理下一个
      }
    }
  }

  return scanResult
}

/**
 * 选择7z压缩包作为清单源
 */
const selectImportArchive = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [
        { name: '7z压缩包', extensions: ['7z'] }
      ],
      title: '选择清单7z压缩包'
    })

    if (!selected || typeof selected !== 'string') {
      return
    }

    selectedImportPath.value = selected
    selectedImportTempPath.value = ''
    isPreparingImport.value = true

    try {
      // 解压到临时目录
      const tempFolder = await invoke<string>('extract_archive', {
        archivePath: selected
      })

      selectedImportTempPath.value = tempFolder

      // 扫描并转换
      await scanAndConvertFolder(tempFolder)

      if (!hasLua.value) {
        alert('未在压缩包中找到vdf或lua文件，无法入库')
      }
    } catch (error) {
      alert(`解压失败: ${error}`)
      selectedImportPath.value = ''
      selectedImportTempPath.value = ''
    } finally {
      isPreparingImport.value = false
    }
  } catch (error) {
    alert(`选择文件失败: ${error}`)
  }
}

/**
 * 选择lua所在文件夹作为清单源
 */
const selectImportFolder = async () => {
  try {
    const selected = await open({
      directory: true,
      title: '选择包含lua/vdf文件的文件夹'
    })

    if (!selected || typeof selected !== 'string') {
      return
    }

    selectedImportPath.value = selected
    selectedImportTempPath.value = ''
    isPreparingImport.value = true

    try {
      // 扫描并转换
      await scanAndConvertFolder(selected)

      if (!hasLua.value) {
        alert('未在文件夹中找到vdf或lua文件，无法入库')
      }
    } catch (error) {
      alert(`扫描失败: ${error}`)
      selectedImportPath.value = ''
    } finally {
      isPreparingImport.value = false
    }
  } catch (error) {
    alert(`选择文件夹失败: ${error}`)
  }
}

/**
 * 清除自定义清单源选择
 */
const clearImportSource = () => {
  selectedImportPath.value = ''
  selectedImportTempPath.value = ''
  // 重新检查内置清单
  checkGameManifest()
}

/**
 * 选择 7z 压缩包作为游戏下载的清单源
 * 解压到 resources/manifest/{game_id}/ 后重新检测
 */
const selectDownloadManifestArchive = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [
        { name: '7z压缩包', extensions: ['7z'] }
      ],
      title: '选择清单7z压缩包'
    })

    if (!selected || typeof selected !== 'string') {
      return
    }

    selectedDownloadManifestPath.value = selected
    isPreparingDownloadManifest.value = true

    try {
      addDownloadLog(`正在解压清单压缩包...`, 'info')
      const targetFolder = await invoke<string>('extract_manifest_archive', {
        archivePath: selected,
        gameId: gameId.value
      })

      addDownloadLog(`清单已解压到: ${targetFolder}`, 'success')

      // 记录手动导入的清单游戏ID到缓存
      try {
        await invoke('add_imported_manifest_game_id', {
          gameId: gameId.value
        })
      } catch (cacheError) {
        console.warn('记录清单导入缓存失败:', cacheError)
      }

      // 重新检测清单文件夹状态
      await checkManifestFolder()
    } catch (error) {
      addDownloadLog(`解压清单压缩包失败: ${error}`, 'error')
      alert(`解压失败: ${error}`)
      selectedDownloadManifestPath.value = ''
    } finally {
      isPreparingDownloadManifest.value = false
    }
  } catch (error) {
    alert(`选择文件失败: ${error}`)
  }
}

/**
 * 选择 lua 所在文件夹作为游戏下载的清单源
 * 直接复制/使用 resources/manifest/{game_id}/ 目录
 */
const selectDownloadManifestFolder = async () => {
  try {
    const selected = await open({
      directory: true,
      title: '选择包含lua/vdf文件的文件夹'
    })

    if (!selected || typeof selected !== 'string') {
      return
    }

    selectedDownloadManifestPath.value = selected
    isPreparingDownloadManifest.value = true

    try {
      addDownloadLog(`正在复制清单文件夹...`, 'info')
      const targetFolder = await invoke<string>('copy_folder_to_manifest', {
        sourcePath: selected,
        gameId: gameId.value
      })

      addDownloadLog(`清单已复制到: ${targetFolder}`, 'success')
      // 重新检测清单文件夹状态
      await checkManifestFolder()
    } catch (error) {
      addDownloadLog(`复制清单文件夹失败: ${error}`, 'error')
      alert(`复制失败: ${error}`)
      selectedDownloadManifestPath.value = ''
    } finally {
      isPreparingDownloadManifest.value = false
    }
  } catch (error) {
    alert(`选择文件夹失败: ${error}`)
  }
}

/**
 * 导入游戏到Steam
 */
const importToSteam = async () => {
  if (isImportingToSteam.value) return

  // 检查是否是第一次使用清单入库（从config.json读取）
  const hasCompletedSetup = configStore.config?.launch?.manifestImportInitialized
  if (!hasCompletedSetup) {
    // 显示首次使用配置弹窗
    showFirstTimeSetup.value = true
    return
  }

  // 检查是否设置了Steam路径
  let steamPath = configStore.config?.gameDirs?.steamPath
  
  if (!steamPath) {
    // 未设置Steam路径，弹出选择对话框
    const selected = await open({
      directory: true,
      title: '请选择Steam安装目录'
    })
    
    if (!selected) {
      // 用户取消了选择
      return
    }
    
    // 保存选择的Steam路径到配置
    steamPath = selected
    await configStore.updateConfig({
      gameDirs: {
        steamPath: selected,
        coversPath: configStore.config?.gameDirs?.coversPath || 'data/covers'
      }
    })
  }

  isImportingToSteam.value = true

  try {
    let result: {
      success: boolean
      message: string
      importedLua: number
      importedManifest: number
      convertedVdf: number
    }

    if (importSourceReady.value) {
      // 使用自定义清单源
      const folderPath = importSourceMode.value === '7z'
        ? selectedImportTempPath.value
        : selectedImportPath.value

      const scanResult = await invoke<{
        luaFiles: string[]
        manifestFiles: string[]
        vdfFiles: string[]
      }>('scan_manifest_folder', {
        folderPath
      })

      result = await invoke<{
        success: boolean
        message: string
        importedLua: number
        importedManifest: number
        convertedVdf: number
      }>('import_manifest_to_steam', {
        steamPath,
        luaFiles: scanResult.luaFiles,
        manifestFiles: scanResult.manifestFiles,
        vdfFiles: scanResult.vdfFiles
      })
    } else {
      // 使用内置清单
      result = await invoke<{
        success: boolean
        message: string
        importedLua: number
        importedManifest: number
        convertedVdf: number
      }>('import_game_manifest_to_steam', {
        gameId: gameId.value
      })
    }

    if (result.success) {
      // 显示成功弹窗，包含重启Steam按钮
      const shouldRestart = confirm(
        `入库成功！\n\n` +
        `- Lua文件: ${result.importedLua}个\n` +
        `- Manifest文件: ${result.importedManifest}个\n` +
        `${result.convertedVdf > 0 ? `- VDF转换: ${result.convertedVdf}个\n` : ''}` +
        `\n请重启Steam以查看导入的游戏。\n\n是否立即重启Steam？`
      )

      if (shouldRestart) {
        await restartSteam()
      }
    } else {
      alert(`入库失败: ${result.message}`)
    }
  } catch (error) {
    alert(`入库失败: ${error}`)
  } finally {
    isImportingToSteam.value = false
  }
}

/**
 * 使用OpenSteamTool内核导入游戏到Steam
 */
const importWithOpenSteamTool = async () => {
  if (isImportingWithOpenSteamTool.value) return

  if (!game.value) {
    alert('游戏数据未加载')
    return
  }

  // 解析AppID
  const appId = parseInt(game.value.game_id, 10)
  if (isNaN(appId) || appId <= 0) {
    alert('游戏ID无效，无法作为Steam AppID使用')
    return
  }

  // 检查Steam路径
  let steamPath = configStore.config?.gameDirs?.steamPath
  if (!steamPath) {
    const selected = await open({
      directory: true,
      title: '请选择Steam安装目录'
    })

    if (!selected) {
      return
    }

    steamPath = selected
    await configStore.updateConfig({
      gameDirs: {
        steamPath: selected,
        coversPath: configStore.config?.gameDirs?.coversPath || 'data/covers'
      }
    })
  }

  // 高级模式确认
  const advancedMode = configStore.config?.opensteamtool?.advancedMode ?? false
  if (advancedMode) {
    const confirmAdvanced = confirm(
      '高级模式已启用，将写入Windows注册表。\n\n' +
      '这通常用于Denuvo/在线游戏，但也可能带来风险。\n\n' +
      '是否继续？'
    )
    if (!confirmAdvanced) {
      return
    }
  }

  isImportingWithOpenSteamTool.value = true

  try {
    let result: {
      success: boolean
      message: string
      kernelInstalled: boolean
      luaWritten: boolean
      manifestCopied: number
      steamRestarted: boolean
      advancedEnabled: boolean
    }

    if (importSourceReady.value) {
      // 使用自定义清单源
      const folderPath = importSourceMode.value === '7z'
        ? selectedImportTempPath.value
        : selectedImportPath.value

      result = await invoke<{
        success: boolean
        message: string
        kernelInstalled: boolean
        luaWritten: boolean
        manifestCopied: number
        steamRestarted: boolean
        advancedEnabled: boolean
      }>('import_manifest_with_opensteamtool', {
        steamPath: steamPath,
        folderPath: folderPath,
        gameName: game.value.chinese_name || game.value.game_name || gameId.value,
        appId: appId,
        advancedMode: advancedMode
      })
    } else {
      // 使用内置清单
      result = await invoke<{
        success: boolean
        message: string
        kernelInstalled: boolean
        luaWritten: boolean
        manifestCopied: number
        steamRestarted: boolean
        advancedEnabled: boolean
      }>('import_game_with_opensteamtool', {
        steamPath: steamPath,
        gameId: gameId.value,
        gameName: game.value.chinese_name || game.value.game_name || gameId.value,
        appId: appId,
        advancedMode: advancedMode
      })
    }

    if (result.success) {
      const message =
        `OpenSteamTool入库成功！\n\n` +
        `游戏: ${game.value.chinese_name || game.value.game_name}\n` +
        `AppID: ${appId}\n` +
        `内核DLL: ${result.kernelInstalled ? '已安装' : '未安装'}\n` +
        `Lua文件: ${result.luaWritten ? '已写入' : '未写入'}\n` +
        `Manifest文件: ${result.manifestCopied}个\n` +
        `Steam: ${result.steamRestarted ? '已重启' : '未重启'}\n` +
        `${result.advancedEnabled ? '高级模式: 已启用（写入注册表）' : ''}`
      alert(message)
    } else {
      alert(`OpenSteamTool入库失败: ${result.message}`)
    }
  } catch (error) {
    alert(`OpenSteamTool入库失败: ${error}`)
  } finally {
    isImportingWithOpenSteamTool.value = false
  }
}

/**
 * 重启Steam
 */
const restartSteam = async () => {
  try {
    const result = await invoke<{
      success: boolean
      message: string
    }>('restart_steam')

    if (result.success) {
      alert('Steam重启成功！')
    } else {
      alert(`重启Steam失败: ${result.message}`)
    }
  } catch (error) {
    alert(`重启Steam失败: ${error}`)
  }
}
</script>

<style scoped>
.game-detail-page {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  padding: 14px;
  background-color: var(--steam-bg-secondary);
}

/* 头部区域 */
.detail-header {
  display: flex;
  align-items: center;
  gap: 24px;
  margin-bottom: 14px;
  padding-bottom: 10px;
  border-bottom: 1px solid var(--steam-border);
}

.back-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  border: none;
  border-radius: 8px;
  background-color: var(--steam-bg-primary);
  color: var(--steam-text-primary);
  font-size: 14px;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.back-btn:hover {
  background-color: var(--steam-bg-hover);
}

.back-btn svg {
  width: 16px;
  height: 16px;
}

.header-info {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 16px;
}

.game-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0;
}

.game-id {
  font-size: 13px;
  color: var(--steam-text-secondary);
  font-family: 'Courier New', monospace;
}

.category-tags {
  display: flex;
  gap: 8px;
}

.category-tag {
  padding: 4px 12px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
  color: white;
}

/* 主要内容区 */
.detail-content {
  display: flex;
  gap: 24px;
  margin-bottom: 14px;
  align-items: center;
}

.cover-section {
  flex-shrink: 0;
}

.game-cover {
  width: 400px;
  height: auto;
  max-height: 300px;
  object-fit: contain;
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.paths-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.path-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.path-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

.path-input-group {
  display: flex;
  gap: 8px;
}

.path-input {
  flex: 1;
  padding: 10px 12px;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  background-color: var(--steam-bg-primary);
  color: var(--steam-text-primary);
  font-size: 13px;
  outline: none;
}

.browse-btn {
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  background-color: var(--steam-accent-blue);
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.browse-btn:hover {
  background-color: var(--steam-accent-hover);
}

/* 标签页区域 */
.tabs-section {
  background-color: var(--steam-bg-primary);
  border-radius: 12px;
  overflow: hidden;
}

.tabs-header {
  display: flex;
  border-bottom: 1px solid var(--steam-border);
}

.tab-btn {
  padding: 12px 24px;
  border: none;
  background-color: transparent;
  color: var(--steam-text-secondary);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  border-bottom: 2px solid transparent;
}

.tab-btn:hover {
  color: var(--steam-text-primary);
  background-color: var(--steam-bg-hover);
}

.tab-btn.active {
  color: var(--steam-accent-blue);
  border-bottom-color: var(--steam-accent-blue);
  background-color: var(--steam-bg-secondary);
}

/* 补丁类标签页使用红色粗体显示 */
.tab-btn.patch-tab {
  color: #ef4444;
  font-weight: 700;
}

.tab-btn.patch-tab:hover {
  color: #f87171;
  background-color: rgba(239, 68, 68, 0.1);
}

.tab-btn.patch-tab.active {
  color: #ef4444;
  border-bottom-color: #ef4444;
  background-color: rgba(239, 68, 68, 0.08);
}

.tabs-content {
  padding: 14px;
}

.tab-panel {
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.panel-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 10px 0;
}

/* 下载完成提示 */
.download-completed-notice {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px;
  background-color: rgba(16, 185, 129, 0.1);
  border-radius: 8px;
  border-left: 4px solid #10b981;
  margin-bottom: 12px;
}

.success-icon {
  width: 48px;
  height: 48px;
  color: #10b981;
  flex-shrink: 0;
}

.success-icon svg {
  width: 100%;
  height: 100%;
}

.success-text h4 {
  margin: 0 0 5px 0;
  font-size: 16px;
  color: #10b981;
}

.success-text p {
  margin: 0 0 3px 0;
  font-size: 13px;
  color: var(--steam-text-secondary);
}

.success-text .patch-hint {
  margin-top: 8px;
  font-size: 15px;
  font-weight: 700;
  color: #ef4444;
}

/* 通用红色加粗高亮文本 */
.highlight-red-bold {
  color: #ef4444;
  font-weight: 700;
}

/* 验证完整性按钮 - 样式与开始下载按钮一致 */
.verify-integrity-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px 32px;
  border: none;
  border-radius: 8px;
  background-color: var(--steam-accent-blue);
  color: white;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: background-color 0.15s ease;
  margin-left: auto;
}

.verify-integrity-btn:hover:not(:disabled) {
  background-color: var(--steam-accent-hover);
}

.verify-integrity-btn:disabled {
  background-color: var(--steam-text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

.verify-integrity-btn svg {
  width: 18px;
  height: 18px;
}

.download-info {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-bottom: 14px;
}

.info-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  border-radius: 8px;
  font-size: 13px;
}

.info-item.warning {
  background-color: rgba(245, 158, 11, 0.1);
  color: #f59e0b;
}

.info-item.success {
  background-color: rgba(16, 185, 129, 0.1);
  color: #10b981;
}

.info-item.checking {
  background-color: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
}

.status-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}

.status-icon svg {
  width: 20px;
  height: 20px;
}

.spin-icon {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.warning-icon {
  font-size: 12px;
}

.download-path-section {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.path-display {
  padding: 12px 16px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  font-size: 13px;
  color: var(--steam-text-secondary);
  font-family: 'Courier New', monospace;
}

.start-download-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px 32px;
  border: none;
  border-radius: 8px;
  background-color: var(--steam-accent-blue);
  color: white;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.start-download-btn svg {
  width: 18px;
  height: 18px;
}

.start-download-btn:hover:not(.disabled) {
  background-color: var(--steam-accent-hover);
}

.start-download-btn.disabled {
  background-color: var(--steam-text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

.start-download-btn.loading {
  cursor: wait;
}

/* 下载按钮组 */
.download-btn-group {
  display: flex;
  gap: 7px;
  align-items: center;
  position: relative;
  flex-wrap: wrap;
}

/* 圆形下载进度条 */
.circular-progress {
  position: relative;
  width: 40px;
  height: 40px;
  flex-shrink: 0;
}

.circular-progress svg {
  width: 100%;
  height: 100%;
  transform: rotate(-90deg);
}

.circle-bg {
  fill: none;
  stroke: var(--steam-border);
  stroke-width: 3;
}

.circle-progress {
  fill: none;
  stroke: var(--steam-accent-blue);
  stroke-width: 3;
  stroke-linecap: round;
  transition: stroke-dasharray 0.3s ease;
}

.progress-text {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 11px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

/* 暂停下载按钮 */
.stop-download-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px 24px;
  border: none;
  border-radius: 8px;
  background-color: #dc3545;
  color: white;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.stop-download-btn svg {
  width: 18px;
  height: 18px;
}

.stop-download-btn:hover {
  background-color: #c82333;
}

/* 入库Steam按钮 */
.import-steam-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px 24px;
  border: none;
  border-radius: 8px;
  background-color: #10b981;
  color: white;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.import-steam-btn svg {
  width: 18px;
  height: 18px;
}

.import-steam-btn:hover:not(.disabled) {
  background-color: #059669;
}

.import-steam-btn.disabled {
  background-color: var(--steam-text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

.import-steam-btn.loading {
  cursor: wait;
}

/* 下载说明样式 */
.download-description {
  margin: 12px 0;
  padding: 10px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  border-left: 4px solid #3b82f6;
}

.download-option {
  margin-bottom: 10px;
}

.download-option:last-child {
  margin-bottom: 0;
}

.download-option h4 {
  margin: 0 0 5px 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.download-option p {
  margin: 0;
  font-size: 13px;
  color: var(--steam-text-secondary);
  line-height: 1.5;
}

/* 解压即玩标签页样式 */
.extract-play-content {
  padding: 12px;
}

.extract-play-description {
  margin-bottom: 14px;
  padding: 10px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  border-left: 4px solid #8b5cf6;
}

.extract-play-description p {
  margin: 0 0 5px 0;
  font-size: 14px;
  color: var(--steam-text-primary);
  line-height: 1.6;
}

.extract-play-description p:last-child {
  margin-bottom: 0;
}

.extract-play-note {
  font-size: 13px;
  color: var(--steam-text-secondary);
  font-style: italic;
}

.loading-icon {
  animation: spin 1s linear infinite;
}

.path-hint {
  margin: 3px 0 0 0;
  font-size: 12px;
  color: var(--steam-text-secondary);
  font-style: italic;
}

/* 下载日志样式 */
.download-logs {
  margin-top: 12px;
  padding: 10px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--steam-border);
}

.logs-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0 0 7px 0;
}

.logs-content {
  max-height: 200px;
  overflow-y: auto;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  line-height: 1.6;
}

.log-line {
  display: flex;
  gap: 12px;
  padding: 2px 0;
}

.log-time {
  color: var(--steam-text-muted);
  flex-shrink: 0;
}

.log-message {
  color: var(--steam-text-primary);
  word-break: break-all;
}

.log-line.success .log-message {
  color: #10b981;
}

.log-line.error .log-message {
  color: #ef4444;
}

.log-line.warning .log-message {
  color: #f59e0b;
}

/* 下载进度区域样式 */
.download-progress-section {
  margin-top: 12px;
}

.patch-info {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

/* 补丁说明样式 - 显示在应用补丁按钮下方 */
.patch-readme {
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  padding: 10px;
  border-left: 4px solid var(--steam-accent-blue);
  margin-top: 10px;
}

.readme-title {
  margin: 0 0 7px 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.readme-content {
  margin: 0;
  padding: 0;
  font-size: 13px;
  line-height: 1.6;
  color: var(--steam-text-secondary);
  white-space: pre-wrap;
  word-wrap: break-word;
  font-family: inherit;
  background: none;
  border: none;
}

/* 补丁状态样式 */
.patch-status {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 10px 14px;
  border-radius: 8px;
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
  font-size: 13px;
  font-weight: 500;
  margin-bottom: 7px;
}

.patch-status.local-exists {
  background-color: rgba(16, 185, 129, 0.1);
  color: #10b981;
}

.status-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
}

.status-icon svg {
  width: 18px;
  height: 18px;
}

/* 下载区域样式 */
.download-section {
  margin-bottom: 10px;
}

.download-section-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
  margin: 0 0 7px 0;
}

.download-buttons {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 10px;
  margin-bottom: 7px;
}

.backup-label {
  font-size: 13px;
  color: var(--steam-text-secondary);
  white-space: nowrap;
  margin-left: 6px;
}

.download-btn-wrapper {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 7px;
}

.download-patch-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 10px 24px;
  border: none;
  border-radius: 8px;
  background-color: #3b82f6;
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.download-patch-btn:hover {
  background-color: #2563eb;
}

.download-patch-btn svg {
  width: 18px;
  height: 18px;
}

.pwd-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 0;
}

/* 下载提示文字 */
.download-hint {
  font-size: 12px;
  color: var(--steam-text-secondary);
  margin: 4px 0 7px 0;
  font-style: italic;
}

/* 选择补丁文件并应用按钮 */
.select-and-apply-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 10px 24px;
  border: none;
  border-radius: 8px;
  background-color: #8b5cf6;
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.15s ease;
  margin-bottom: 7px;
}

.select-and-apply-btn:hover:not(:disabled) {
  background-color: #7c3aed;
}

.select-and-apply-btn:disabled {
  background-color: var(--steam-text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

.select-and-apply-btn svg {
  width: 18px;
  height: 18px;
}

.patch-path {
  margin: 0;
  font-size: 13px;
  color: var(--steam-text-secondary);
  font-family: 'Courier New', monospace;
}

.game-path-display {
  margin: 0;
  font-size: 13px;
  color: var(--steam-text-secondary);
  font-family: 'Courier New', monospace;
}

.game-path-display.warning {
  color: #f59e0b;
}

.apply-patch-btn {
  padding: 10px 24px;
  border: none;
  border-radius: 8px;
  background-color: var(--steam-accent-blue);
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.15s ease;
  align-self: flex-start;
}

.apply-patch-btn:hover:not(:disabled) {
  background-color: var(--steam-accent-hover);
}

.apply-patch-btn:disabled {
  background-color: var(--steam-text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

/* 虚拟化环境配置教程按钮 */
.tutorial-btn {
  margin-top: 7px;
  padding: 12px 24px;
  background: rgba(156, 39, 176, 0.2);
  color: #ce93d8;
  border: 1px solid rgba(156, 39, 176, 0.5);
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  align-self: flex-start;
}

.tutorial-btn:hover {
  background: rgba(156, 39, 176, 0.3);
  border-color: rgba(156, 39, 176, 0.7);
}

/* 补丁结果提示 */
.patch-result {
  margin-top: 10px;
  padding: 10px;
  border-radius: 8px;
  background-color: var(--steam-bg-secondary);
}

.patch-result.success {
  border-left: 4px solid #10b981;
}

.patch-result.error {
  border-left: 4px solid #ef4444;
}

.result-title {
  margin: 0 0 7px 0;
  font-size: 14px;
  font-weight: 600;
}

.patch-result.success .result-title {
  color: #10b981;
}

.patch-result.error .result-title {
  color: #ef4444;
}

.result-section {
  margin-bottom: 7px;
}

.result-section:last-child {
  margin-bottom: 0;
}

.section-title {
  margin: 0 0 5px 0;
  font-size: 12px;
  font-weight: 500;
  color: var(--steam-text-secondary);
}

.file-list {
  margin: 0;
  padding-left: 16px;
  font-size: 12px;
  color: var(--steam-text-secondary);
}

.file-list li {
  margin-bottom: 2px;
}

.error-section .section-title {
  color: #ef4444;
}

.error-list {
  color: #ef4444;
}

/* 修改器标签页样式 */
.trainer-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.trainer-local-content {
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  padding: 10px;
  border-left: 4px solid #f59e0b;
}

.trainer-content-title {
  margin: 0 0 7px 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.trainer-content-text {
  margin: 0;
  padding: 0;
  font-size: 13px;
  line-height: 1.6;
  color: var(--steam-text-secondary);
  white-space: pre-wrap;
  word-wrap: break-word;
  font-family: inherit;
  background: none;
  border: none;
  max-height: 400px;
  overflow-y: auto;
}

.trainer-no-content {
  padding: 12px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  text-align: center;
  color: var(--steam-text-secondary);
}

.trainer-no-content p {
  margin: 0 0 5px 0;
}

.trainer-path {
  font-size: 12px;
  font-family: 'Courier New', monospace;
  color: var(--steam-text-muted);
}

/* 入库Steam标签页样式 */
.import-steam-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
  align-items: flex-start;
}

.import-btn-group {
  display: flex;
  flex-direction: row;
  gap: 7px;
  align-items: center;
  flex-wrap: wrap;
}

.import-description {
  padding: 10px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  border-left: 4px solid #10b981;
}

.import-description p {
  margin: 0 0 5px 0;
  font-size: 14px;
  color: var(--steam-text-primary);
  line-height: 1.5;
}

.import-description p:last-child {
  margin-bottom: 0;
}

.import-note {
  font-size: 13px;
  color: var(--steam-text-secondary);
  font-style: italic;
}

.import-opensteamtool-btn-large {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 12px 26px;
  border: none;
  border-radius: 8px;
  background-color: #10b981;
  color: white;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.import-opensteamtool-btn-large:hover:not(.disabled) {
  background-color: #059669;
}

.import-opensteamtool-btn-large.disabled {
  background-color: var(--steam-text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

.import-opensteamtool-btn-large.loading {
  cursor: wait;
}

.import-opensteamtool-btn-large svg {
  width: 16px;
  height: 16px;
}

.import-steamtools-btn-large {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 12px 26px;
  border: none;
  border-radius: 8px;
  background-color: #3b82f6;
  color: white;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.import-steamtools-btn-large:hover:not(.disabled) {
  background-color: #2563eb;
}

.import-steamtools-btn-large.disabled {
  background-color: var(--steam-text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

.import-steamtools-btn-large.loading {
  cursor: wait;
}

.import-steamtools-btn-large svg {
  width: 16px;
  height: 16px;
}

.restart-steam-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 13px 26px;
  border: none;
  border-radius: 8px;
  background-color: #64748b;
  color: white;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.restart-steam-btn:hover {
  background-color: #475569;
}

.restart-steam-btn svg {
  width: 16px;
  height: 16px;
}

.import-error {
  color: #ef4444;
  font-size: 14px;
  margin: 0;
}

/* 自定义清单源信息 */
.import-source-info {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 10px 14px;
  background-color: rgba(59, 130, 246, 0.1);
  border-radius: 8px;
  border-left: 3px solid #3b82f6;
  font-size: 13px;
}

.import-source-info .source-label {
  color: var(--steam-text-secondary);
  flex-shrink: 0;
}

.import-source-info .source-path {
  color: var(--steam-text-primary);
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-family: 'Courier New', monospace;
}

.import-source-info .clear-source-btn {
  padding: 4px 10px;
  border: none;
  border-radius: 4px;
  background-color: rgba(239, 68, 68, 0.2);
  color: #ef4444;
  font-size: 12px;
  cursor: pointer;
  transition: background-color 0.15s ease;
  flex-shrink: 0;
}

.import-source-info .clear-source-btn:hover {
  background-color: rgba(239, 68, 68, 0.3);
}

/* 无清单文件时的引导区域 */
.import-no-files {
  display: flex;
  flex-direction: column;
  gap: 10px;
  width: 100%;
}

.download-guide {
  display: flex;
  flex-direction: column;
  gap: 7px;
}

.download-guide-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
  margin: 0;
}

/* 清单源选择区域 */
.import-source-select {
  padding: 10px;
  background-color: var(--steam-bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--steam-border);
  display: flex;
  flex-direction: column;
  gap: 7px;
}

.source-select-title {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.radio-group {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.radio-label {
  display: inline-flex;
  align-items: center;
  gap: 7px;
  cursor: pointer;
  font-size: 13px;
  color: var(--steam-text-primary);
}

.radio-label input[type="radio"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
  accent-color: var(--steam-accent-blue);
}

.source-select-actions {
  display: flex;
  gap: 7px;
  align-items: center;
}

.select-source-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  background-color: var(--steam-accent-blue);
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.select-source-btn:hover:not(:disabled) {
  background-color: var(--steam-accent-hover);
}

.select-source-btn:disabled {
  background-color: var(--steam-text-secondary);
  cursor: not-allowed;
  opacity: 0.5;
}

.select-source-btn svg {
  width: 16px;
  height: 16px;
}

.source-select-error {
  margin: 0;
  font-size: 13px;
  color: #ef4444;
}

.source-select-info {
  margin: 0;
  font-size: 12px;
  color: var(--steam-text-secondary);
  word-break: break-all;
}
</style>