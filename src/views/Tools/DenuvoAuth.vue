<template>
  <div class="denuvo-auth-page">
    <!-- 页面标题 -->
    <div class="page-header">
      <h1 class="page-title">D 加密授权管理</h1>
      <p class="page-subtitle">
        将已通过正版账号完成 D 加密授权游戏的凭证迁移到另一个 Steam 账号，使目标账号无需再次购买即可运行该游戏。
      </p>
    </div>

    <!-- 使用流程说明 -->
    <div class="info-card usage-guide-card">
      <div class="info-card-header">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"/>
          <line x1="12" y1="16" x2="12" y2="12"/>
          <line x1="12" y1="8" x2="12.01" y2="8"/>
        </svg>
        <span>使用流程</span>
      </div>
      <ol class="usage-steps">
        <li>
          <strong>获取授权</strong>：在拥有该游戏的 Steam 账号 A 上启动游戏一次，确保游戏已正常进入（D 加密完成在线授权）。
        </li>
        <li>
          <strong>提取凭证</strong>：保持账号 A 登录 Steam，在本页面输入 AppID 与游戏名称，点击“提取当前授权”。系统会自动读取注册表中的 SteamID、AppTicket、ETicket。
        </li>
        <li>
          <strong>切换目标账号</strong>：退出账号 A，登录你想要授权的 Steam 账号 B。页面顶部会显示当前活动用户的 SteamID64。
        </li>
        <li>
          <strong>填入目标账号</strong>：在下方“目标授权 SteamID64”中填入账号 B 的 SteamID64，可直接点击“填入当前活动用户”自动读取。
        </li>
        <li>
          <strong>应用授权</strong>：点击“保存配置”，然后在右侧列表点击“应用授权到注册表”，程序会自动将授权写入注册表，最后用账号 B 启动游戏。
        </li>
      </ol>
      <div class="usage-notice">
        <strong>注意：</strong>AppTicket / ETicket 是 D 加密授权凭证，通常无需手动填写，点击“提取当前授权”即可自动获取。目标 SteamID64 必须与启动游戏时的当前登录账号一致。
      </div>
    </div>

    <!-- 当前 Steam 活动用户 -->
    <div class="info-card active-user-card">
      <div class="info-card-header">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
          <circle cx="12" cy="7" r="4"/>
        </svg>
        <span>当前 Steam 活动用户</span>
      </div>
      <div v-if="activeUser" class="active-user-info">
        <div class="info-item">
          <span class="info-label">SteamID64</span>
          <span class="info-value">{{ activeUser.steamId64 }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">AccountID</span>
          <span class="info-value">{{ activeUser.accountId }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">Universe</span>
          <span class="info-value">{{ activeUser.universe }}</span>
        </div>
      </div>
      <div v-else class="active-user-empty">
        {{ activeUserError || '未检测到 Steam 活动用户，请确保 Steam 客户端已登录' }}
      </div>
    </div>

    <div class="main-content">
      <!-- 左侧：编辑表单 -->
      <div class="left-panel">
        <div class="form-card">
          <h3 class="form-title">{{ isEditing ? '编辑授权配置' : '添加授权配置' }}</h3>

          <div class="form-group">
            <label class="form-label">
              AppID
              <span class="required">*</span>
            </label>
            <input
              v-model="form.appId"
              type="text"
              class="form-input"
              placeholder="请输入 Steam AppID，例如 1234560"
              :disabled="isEditing"
              @input="onAppIdInput"
            />
            <p class="form-hint">Steam 游戏的 AppID，可在 Steam 商店页面 URL 或 SteamDB 中查看</p>
          </div>

          <div class="form-group">
            <label class="form-label">
              游戏名称
              <span class="required">*</span>
            </label>
            <input
              v-model="form.gameName"
              type="text"
              class="form-input"
              placeholder="请输入游戏名称，用于识别"
            />
          </div>

          <div class="form-group">
            <label class="form-label">目标授权 SteamID64（切换后的账号）</label>
            <div class="input-with-action">
              <input
                v-model="form.steamId"
                type="text"
                class="form-input"
                placeholder="76561198xxxxxxxx"
              />
              <button
                type="button"
                class="inline-action-btn"
                :disabled="!activeUser"
                @click="fillTargetSteamId"
              >
                填入当前活动用户
              </button>
            </div>
            <p class="form-hint">
              这是你想要授权给的 Steam 账号 ID（账号 B）。请先登录目标账号，然后点击右侧按钮自动填入当前活动用户的 SteamID64。
            </p>
          </div>

          <div class="form-group">
            <label class="form-label">AppTicket（十六进制，可选）</label>
            <textarea
              v-model="form.appTicketHex"
              class="form-textarea"
              rows="3"
              placeholder="通常无需手动填写，点击“提取当前授权”自动获取"
            />
            <p class="form-hint">来自原账号 A 的 D 加密授权凭证，首次使用建议通过“提取当前授权”自动读取。</p>
          </div>

          <div class="form-group">
            <label class="form-label">ETicket（十六进制，可选）</label>
            <textarea
              v-model="form.eTicketHex"
              class="form-textarea"
              rows="3"
              placeholder="通常无需手动填写，点击“提取当前授权”自动获取"
            />
            <p class="form-hint">来自原账号 A 的 D 加密授权凭证，首次使用建议通过“提取当前授权”自动读取。</p>
          </div>

          <div class="form-actions">
            <Button
              variant="primary"
              :loading="isExtracting"
              :disabled="!canExtract"
              @click="extractCurrentAuth"
            >
              提取当前授权
            </Button>
            <Button
              variant="secondary"
              :loading="isSaving"
              :disabled="!canSave"
              @click="saveEntry"
            >
              {{ isEditing ? '更新配置' : '保存配置' }}
            </Button>
            <Button
              variant="ghost"
              @click="resetForm"
            >
              清空
            </Button>
          </div>
        </div>
      </div>

      <!-- 右侧：已保存列表 -->
      <div class="right-panel">
        <div class="list-card">
          <div class="list-header">
            <h3 class="list-title">已保存的授权配置</h3>
            <span class="list-count">共 {{ backupList.length }} 条</span>
          </div>

          <div v-if="backupList.length === 0" class="empty-state">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
              <polyline points="14 2 14 8 20 8"/>
              <line x1="12" y1="18" x2="12" y2="12"/>
              <line x1="9" y1="15" x2="15" y2="15"/>
            </svg>
            <p>暂无配置，请在左侧添加</p>
          </div>

          <div v-else class="backup-list">
            <div
              v-for="item in backupList"
              :key="item.appId"
              class="backup-item"
              :class="{ active: form.appId === String(item.appId) }"
              @click="loadBackupItem(item.appId)"
            >
              <div class="backup-main">
                <div class="backup-title">{{ item.gameName }}</div>
                <div class="backup-meta">
                  <span class="backup-appid">AppID: {{ item.appId }}</span>
                  <span class="backup-badges">
                    <span v-if="item.hasSteamId" class="badge badge-steamid">SteamID</span>
                    <span v-if="item.hasAppTicket" class="badge badge-ticket">AppTicket</span>
                    <span v-if="item.hasETicket" class="badge badge-ticket">ETicket</span>
                  </span>
                </div>
              </div>
              <div class="backup-actions" @click.stop>
                <button
                  class="icon-btn apply-btn"
                  title="应用授权到注册表"
                  :disabled="isApplyingId === item.appId"
                  @click="applyBackup(item.appId)"
                >
                  <svg v-if="isApplyingId !== item.appId" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
                    <polyline points="17 21 17 13 7 13 7 21"/>
                    <polyline points="7 3 7 8 15 8"/>
                  </svg>
                  <span v-else class="mini-spinner"></span>
                </button>
                <button
                  class="icon-btn delete-btn"
                  title="删除配置"
                  @click="deleteBackup(item.appId)"
                >
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="3 6 5 6 21 6"/>
                    <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
                  </svg>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * DenuvoAuth.vue - D 加密授权管理页面
 * 允许用户手动配置 AppID 和游戏名称，从注册表提取授权信息并保存，
 * 切换 Steam 账号后可将保存的授权写回注册表。
 */

import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import Button from '../../components/common/Button.vue'

/**
 * 当前 Steam 活动用户信息
 */
interface ActiveSteamUserInfo {
  accountId: number
  universe: string
  steamId64: string
}

/**
 * 单个 D 加密授权条目（完整数据）
 */
interface DenuvoAuthEntry {
  appId: number
  gameName: string
  steamId?: string
  appTicketHex?: string
  eTicketHex?: string
  backupTime?: string
}

/**
 * D 加密授权列表项（精简数据）
 */
interface DenuvoAuthListItem {
  appId: number
  gameName: string
  hasSteamId: boolean
  hasAppTicket: boolean
  hasETicket: boolean
  backupTime?: string
}

/**
 * 表单数据
 */
const form = ref({
  appId: '',
  gameName: '',
  steamId: '',
  appTicketHex: '',
  eTicketHex: '',
})

/**
 * 当前 Steam 活动用户
 */
const activeUser = ref<ActiveSteamUserInfo | null>(null)
const activeUserError = ref('')

/**
 * 已保存的授权配置列表
 */
const backupList = ref<DenuvoAuthListItem[]>([])

/**
 * 编辑状态：当前正在编辑的 AppID
 */
const isEditing = computed(() => {
  const appIdNum = Number(form.value.appId)
  if (Number.isNaN(appIdNum) || appIdNum === 0) return false
  return backupList.value.some(item => item.appId === appIdNum)
})

/**
 * 加载状态
 */
const isExtracting = ref(false)
const isSaving = ref(false)
const isApplyingId = ref<number | null>(null)

/**
 * 是否可以提取授权
 */
const canExtract = computed(() => {
  const appId = Number(form.value.appId)
  return !Number.isNaN(appId) && appId > 0 && !isExtracting.value
})

/**
 * 是否可以保存配置
 */
const canSave = computed(() => {
  const appId = Number(form.value.appId)
  return (
    !Number.isNaN(appId) &&
    appId > 0 &&
    form.value.gameName.trim().length > 0 &&
    !isSaving.value
  )
})

/**
 * 页面加载时获取当前 Steam 用户和已保存列表
 */
onMounted(() => {
  loadActiveUser()
  loadBackupList()
})

/**
 * 加载当前 Steam 活动用户信息
 */
async function loadActiveUser() {
  try {
    activeUser.value = await invoke<ActiveSteamUserInfo>('get_active_steam_user')
    activeUserError.value = ''
  } catch (error) {
    activeUser.value = null
    activeUserError.value = error as string
  }
}

/**
 * 将目标授权 SteamID64 自动填入当前活动用户的 SteamID64
 * 用于切换到目标账号 B 后，快速把账号 B 的 ID 填入表单
 */
function fillTargetSteamId() {
  if (activeUser.value) {
    form.value.steamId = activeUser.value.steamId64
  }
}

/**
 * 加载已保存的授权配置列表
 */
async function loadBackupList() {
  try {
    backupList.value = await invoke<DenuvoAuthListItem[]>('list_denuvo_auth_backups')
  } catch (error) {
    backupList.value = []
    alert('加载配置列表失败：' + (error as string))
  }
}

/**
 * AppID 输入处理：仅允许数字
 */
function onAppIdInput(event: Event) {
  const input = event.target as HTMLInputElement
  form.value.appId = input.value.replace(/[^0-9]/g, '')
}

/**
 * 从注册表提取当前授权并填充到表单
 */
async function extractCurrentAuth() {
  const appId = Number(form.value.appId)
  if (Number.isNaN(appId) || appId <= 0) return

  isExtracting.value = true
  try {
    const entry = await invoke<DenuvoAuthEntry>('read_denuvo_auth_from_registry', {
      appId,
    })

    form.value.steamId = entry.steamId || ''
    form.value.appTicketHex = entry.appTicketHex || ''
    form.value.eTicketHex = entry.eTicketHex || ''

    // 如果当前没有填写游戏名称，使用默认占位提示
    if (!form.value.gameName) {
      form.value.gameName = entry.gameName || `AppID ${appId}`
    }

    alert('提取当前授权成功')
  } catch (error) {
    alert('提取当前授权失败：' + (error as string))
  } finally {
    isExtracting.value = false
  }
}

/**
 * 保存当前表单配置到本地备份文件
 */
async function saveEntry() {
  const appId = Number(form.value.appId)
  if (Number.isNaN(appId) || appId <= 0 || !form.value.gameName.trim()) return

  isSaving.value = true
  try {
    const entry: DenuvoAuthEntry = {
      appId,
      gameName: form.value.gameName.trim(),
      steamId: form.value.steamId.trim() || undefined,
      appTicketHex: form.value.appTicketHex.trim() || undefined,
      eTicketHex: form.value.eTicketHex.trim() || undefined,
    }

    // 如果已存在备份，保留原来的备份时间
    if (isEditing.value) {
      try {
        const existing = await invoke<DenuvoAuthEntry>('load_denuvo_auth_backup', { appId })
        entry.backupTime = existing.backupTime
      } catch {
        entry.backupTime = new Date().toISOString()
      }
    } else {
      entry.backupTime = new Date().toISOString()
    }

    await invoke('save_denuvo_auth_entry', { entry })
    await loadBackupList()
    alert(isEditing.value ? '配置已更新' : '配置已保存')
  } catch (error) {
    alert('保存配置失败：' + (error as string))
  } finally {
    isSaving.value = false
  }
}

/**
 * 从列表加载某个配置到表单
 */
async function loadBackupItem(appId: number) {
  try {
    const entry = await invoke<DenuvoAuthEntry>('load_denuvo_auth_backup', { appId })
    form.value.appId = String(entry.appId)
    form.value.gameName = entry.gameName
    form.value.steamId = entry.steamId || ''
    form.value.appTicketHex = entry.appTicketHex || ''
    form.value.eTicketHex = entry.eTicketHex || ''
  } catch (error) {
    alert('加载配置失败：' + (error as string))
  }
}

/**
 * 将本地备份的授权写回注册表
 */
async function applyBackup(appId: number) {
  isApplyingId.value = appId
  try {
    await invoke('apply_denuvo_auth_backup', { appId })
    alert(`已将 AppID ${appId} 的授权写入注册表`)
  } catch (error) {
    alert('应用授权失败：' + (error as string))
  } finally {
    isApplyingId.value = null
  }
}

/**
 * 删除本地授权配置
 */
async function deleteBackup(appId: number) {
  if (!confirm(`确定要删除 AppID ${appId} 的授权配置吗？`)) return

  try {
    await invoke('delete_denuvo_auth_backup', { appId })
    // 如果当前正在编辑该条目，清空表单
    if (form.value.appId === String(appId)) {
      resetForm()
    }
    await loadBackupList()
  } catch (error) {
    alert('删除配置失败：' + (error as string))
  }
}

/**
 * 重置表单为初始状态
 */
function resetForm() {
  form.value = {
    appId: '',
    gameName: '',
    steamId: '',
    appTicketHex: '',
    eTicketHex: '',
  }
}
</script>

<style scoped>
/* 页面根容器 */
.denuvo-auth-page {
  width: 100%;
  height: 100%;
  padding: 20px 24px 24px;
  box-sizing: border-box;
  overflow-y: auto;
  color: var(--steam-text-primary);
  font-size: 14px;
}

/* 页面标题区域 */
.page-header {
  margin-bottom: 20px;
}

.page-title {
  margin: 0 0 8px;
  font-size: 20px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.page-subtitle {
  margin: 0;
  font-size: 13px;
  line-height: 1.5;
  color: var(--steam-text-secondary);
}

/* 当前用户信息卡片 */
.info-card {
  background: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border-color);
  border-radius: 8px;
  padding: 14px 16px;
  margin-bottom: 20px;
}

.active-user-card {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.info-card-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

.info-card-header svg {
  width: 18px;
  height: 18px;
  color: var(--steam-accent-blue);
}

.active-user-info {
  display: flex;
  flex-wrap: wrap;
  gap: 16px 32px;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.info-label {
  font-size: 12px;
  color: var(--steam-text-tertiary);
}

.info-value {
  font-size: 14px;
  color: var(--steam-text-primary);
  font-family: 'Consolas', 'Monaco', monospace;
  word-break: break-all;
}

.active-user-empty {
  font-size: 13px;
  color: var(--steam-text-tertiary);
  font-style: italic;
}

/* 使用流程说明卡片 */
.usage-guide-card {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.usage-steps {
  margin: 0;
  padding-left: 20px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.usage-steps li {
  font-size: 13px;
  line-height: 1.6;
  color: var(--steam-text-secondary);
}

.usage-steps li strong {
  color: var(--steam-text-primary);
  font-weight: 600;
}

.usage-notice {
  background: rgba(26, 159, 255, 0.08);
  border: 1px solid rgba(26, 159, 255, 0.2);
  border-radius: 6px;
  padding: 10px 12px;
  font-size: 13px;
  line-height: 1.5;
  color: var(--steam-text-secondary);
}

.usage-notice strong {
  color: var(--steam-accent-blue);
}

/* 主内容：左右分栏 */
.main-content {
  display: flex;
  gap: 20px;
  align-items: flex-start;
}

.left-panel {
  flex: 1;
  min-width: 0;
}

.right-panel {
  flex: 1;
  min-width: 0;
}

/* 表单卡片 */
.form-card,
.list-card {
  background: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border-color);
  border-radius: 8px;
  padding: 16px;
}

.form-title,
.list-title {
  margin: 0 0 16px;
  font-size: 16px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

/* 表单元素 */
.form-group {
  margin-bottom: 14px;
}

.form-label {
  display: block;
  margin-bottom: 6px;
  font-size: 13px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

.required {
  color: #e81123;
  margin-left: 2px;
}

.form-input,
.form-textarea {
  width: 100%;
  padding: 8px 12px;
  box-sizing: border-box;
  background: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border-color);
  border-radius: 4px;
  color: var(--steam-text-primary);
  font-size: 13px;
  font-family: inherit;
  outline: none;
  transition: border-color 0.15s ease;
  resize: vertical;
}

.form-input::placeholder,
.form-textarea::placeholder {
  color: var(--steam-text-tertiary);
}

.form-input:focus,
.form-textarea:focus {
  border-color: var(--steam-accent-blue);
}

.form-input:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}

/* 带操作按钮的输入框 */
.input-with-action {
  display: flex;
  gap: 8px;
}

.input-with-action .form-input {
  flex: 1;
  min-width: 0;
}

.inline-action-btn {
  flex-shrink: 0;
  padding: 8px 12px;
  background: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border-color);
  border-radius: 4px;
  color: var(--steam-text-primary);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  white-space: nowrap;
}

.inline-action-btn:hover:not(:disabled) {
  background: var(--steam-bg-hover);
  border-color: var(--steam-accent-blue);
  color: var(--steam-accent-blue);
}

.inline-action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.form-hint {
  margin: 4px 0 0;
  font-size: 12px;
  color: var(--steam-text-tertiary);
  line-height: 1.4;
}

.form-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin-top: 20px;
}

/* 列表卡片 */
.list-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 14px;
}

.list-count {
  font-size: 12px;
  color: var(--steam-text-tertiary);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px 20px;
  color: var(--steam-text-tertiary);
  text-align: center;
}

.empty-state svg {
  width: 48px;
  height: 48px;
  margin-bottom: 12px;
  opacity: 0.5;
}

.empty-state p {
  margin: 0;
  font-size: 13px;
}

/* 备份列表 */
.backup-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.backup-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 12px;
  background: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border-color);
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s ease, border-color 0.15s ease;
}

.backup-item:hover,
.backup-item.active {
  background: var(--steam-bg-hover);
  border-color: var(--steam-accent-blue);
}

.backup-main {
  min-width: 0;
  flex: 1;
}

.backup-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.backup-meta {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  margin-top: 6px;
}

.backup-appid {
  font-size: 12px;
  color: var(--steam-text-secondary);
  font-family: 'Consolas', 'Monaco', monospace;
}

.backup-badges {
  display: flex;
  gap: 4px;
}

.badge {
  padding: 1px 5px;
  border-radius: 3px;
  font-size: 12px;
  font-weight: 500;
}

.badge-steamid {
  background: rgba(26, 159, 255, 0.15);
  color: var(--steam-accent-blue);
}

.badge-ticket {
  background: rgba(76, 175, 80, 0.15);
  color: #4caf50;
}

.backup-actions {
  display: flex;
  gap: 6px;
  flex-shrink: 0;
}

.icon-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  padding: 0;
  background: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border-color);
  border-radius: 4px;
  color: var(--steam-text-primary);
  cursor: pointer;
  transition: all 0.15s ease;
}

.icon-btn svg {
  width: 16px;
  height: 16px;
}

.icon-btn:hover:not(:disabled) {
  background: var(--steam-bg-hover);
}

.icon-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.apply-btn:hover:not(:disabled) {
  border-color: var(--steam-accent-blue);
  color: var(--steam-accent-blue);
}

.delete-btn:hover:not(:disabled) {
  border-color: #e81123;
  color: #e81123;
}

.mini-spinner {
  width: 14px;
  height: 14px;
  border: 2px solid var(--steam-border-color);
  border-top-color: var(--steam-accent-blue);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* 响应式：较窄时上下堆叠 */
@media (max-width: 1200px) {
  .main-content {
    flex-direction: column;
  }

  .left-panel,
  .right-panel {
    width: 100%;
  }
}
</style>
