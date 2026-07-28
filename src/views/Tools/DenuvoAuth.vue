<template>
  <div class="denuvo-auth-page">
    <!-- 页面标题 -->
    <div class="page-header">
      <h1 class="page-title">D 加密授权管理</h1>
      <p class="page-subtitle">
        从已购买游戏的 A 账号提取 D 加密授权凭证，保存后切换到目标 B 账号应用，实现 B 账号运行 D 加密游戏。
      </p>
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

    <!-- 手动授权表单 -->
    <div class="info-card form-card">
      <div class="info-card-header">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
          <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
        </svg>
        <span>授权配置</span>
      </div>

      <div class="form-grid">
        <div class="form-group">
          <label class="form-label" for="denuvo-app-id">AppID <span class="required">*</span></label>
          <input
            id="denuvo-app-id"
            v-model="form.appId"
            type="text"
            class="form-input"
            placeholder="例如：376420"
            :disabled="isExtracting || isSaving || isApplying"
          />
          <span class="form-hint">游戏的 Steam AppID，纯数字</span>
        </div>

        <div class="form-group">
          <label class="form-label" for="denuvo-game-name">游戏名称 <span class="required">*</span></label>
          <input
            id="denuvo-game-name"
            v-model="form.gameName"
            type="text"
            class="form-input"
            placeholder="例如：生化危机8"
            :disabled="isExtracting || isSaving || isApplying"
          />
          <span class="form-hint">用于备份列表展示，可自定义</span>
        </div>

        <div class="form-group form-group-wide">
          <label class="form-label" for="denuvo-steam-id">目标 SteamID64 <span class="required">*</span></label>
          <div class="input-with-btn">
            <input
              id="denuvo-steam-id"
              v-model="form.steamId"
              type="text"
              class="form-input"
              placeholder="例如：76561198xxxxxxxx"
              :disabled="isExtracting || isSaving || isApplying || isExtractingSteamId"
            />
            <Button
              variant="secondary"
              size="sm"
              :loading="isExtractingSteamId"
              :disabled="isExtracting || isSaving || isApplying || isExtractingSteamId"
              @click="extractCurrentSteamId"
            >
              提取当前
            </Button>
          </div>
          <span class="form-hint">需要授权的 B 账号 64 位 SteamID；点击“提取当前”可自动填入当前登录账号的 SteamID64</span>
        </div>

        <div class="form-group form-group-wide">
          <label class="form-label" for="denuvo-app-ticket">AppTicket（十六进制）</label>
          <textarea
            id="denuvo-app-ticket"
            v-model="form.appTicketHex"
            class="form-textarea"
            rows="3"
            placeholder="从注册表提取后自动填充，或手动粘贴十六进制字符串"
            :disabled="isExtracting || isSaving || isApplying"
          />
          <span class="form-hint">对应注册表 AppTicket 项的二进制数据</span>
        </div>

        <div class="form-group form-group-wide">
          <label class="form-label" for="denuvo-e-ticket">ETicket（十六进制）</label>
          <textarea
            id="denuvo-e-ticket"
            v-model="form.eTicketHex"
            class="form-textarea"
            rows="3"
            placeholder="从注册表提取后自动填充，或手动粘贴十六进制字符串"
            :disabled="isExtracting || isSaving || isApplying"
          />
          <span class="form-hint">对应注册表 ETicket 项的二进制数据</span>
        </div>
      </div>

      <!-- 表单验证错误提示 -->
      <div v-if="formErrors.length > 0" class="form-errors">
        <div v-for="(error, index) in formErrors" :key="index" class="form-error-item">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
          </svg>
          <span>{{ error }}</span>
        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="form-actions">
        <Button
          variant="primary"
          :loading="isExtracting"
          :disabled="isExtracting || isSaving || isApplying || !canExtract"
          @click="extractFromRegistry"
        >
          提取当前授权
        </Button>
        <Button
          variant="secondary"
          :loading="isSaving"
          :disabled="isExtracting || isSaving || isApplying"
          @click="saveEntry"
        >
          {{ isEditing ? '更新配置' : '保存配置' }}
        </Button>
        <Button
          variant="secondary"
          :loading="isApplying"
          :disabled="isExtracting || isSaving || isApplying || !canApply"
          @click="applyToRegistry"
        >
          应用授权到注册表
        </Button>
        <Button
          variant="ghost"
          :disabled="isExtracting || isSaving || isApplying"
          @click="resetForm"
        >
          重置
        </Button>
        <Button
          v-if="isEditing"
          variant="danger"
          :disabled="isExtracting || isSaving || isApplying"
          @click="deleteCurrentBackup"
        >
          删除配置
        </Button>
      </div>

      <div class="usage-notice">
        <strong>使用流程：</strong>
        <ol class="usage-steps">
          <li>在 A 账号（已购买游戏）登录 Steam，填入 AppID 与游戏名，点击“提取当前授权”。</li>
          <li>如需把授权迁移到 B 账号，把上方“目标 SteamID64”改成 B 账号的 SteamID64，点击“保存配置”。</li>
          <li>退出 A 账号，登录 B 账号，点击“应用授权到注册表”。</li>
          <li>从 B 账号启动游戏，D 加密将读取注册表中的授权凭证。</li>
        </ol>
      </div>
    </div>

    <!-- 已保存授权列表 -->
    <div class="info-card list-card">
      <div class="info-card-header">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
          <polyline points="14 2 14 8 20 8"/>
          <line x1="16" y1="13" x2="8" y2="13"/>
          <line x1="16" y1="17" x2="8" y2="17"/>
          <polyline points="10 9 9 9 8 9"/>
        </svg>
        <span>已保存授权配置</span>
      </div>

      <div v-if="loadingBackups" class="loading-state">
        <div class="spinner"></div>
        <span>加载中...</span>
      </div>

      <div v-else-if="backups.length === 0" class="empty-state">
        <p>暂无已保存的 D 加密授权配置</p>
        <p class="empty-hint">填写上方表单并点击“保存配置”后，会在这里显示</p>
      </div>

      <div v-else class="backup-list">
        <div
          v-for="backup in backups"
          :key="backup.appId"
          class="backup-item"
          :class="{ 'backup-active': backup.appId === Number(form.appId) }"
          @click="loadBackup(backup.appId)"
        >
          <div class="backup-main">
            <span class="backup-name" :title="backup.gameName">{{ backup.gameName }}</span>
            <span class="backup-app-id">AppID: {{ backup.appId }}</span>
          </div>
          <div class="backup-tags">
            <span v-if="backup.hasSteamId" class="backup-tag tag-steam-id">SteamID</span>
            <span v-if="backup.hasAppTicket" class="backup-tag tag-ticket">AppTicket</span>
            <span v-if="backup.hasETicket" class="backup-tag tag-ticket">ETicket</span>
          </div>
          <div class="backup-actions">
            <button
              class="backup-btn apply"
              title="应用授权到注册表"
              @click.stop="applyBackup(backup.appId)"
            >
              应用
            </button>
            <button
              class="backup-btn delete"
              title="删除该配置"
              @click.stop="deleteBackup(backup.appId)"
            >
              删除
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * DenuvoAuth.vue - D 加密授权手动管理页面
 * 支持从注册表提取授权、保存为本地 JSON 备份、切换到目标账号后写回注册表。
 * 修复了原保存按钮 disabled 状态不明显、保存失败无提示的问题。
 */

import { ref, computed, onMounted, watch } from 'vue'
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
 * 表单数据
 */
interface DenuvoForm {
  appId: string
  gameName: string
  steamId: string
  appTicketHex: string
  eTicketHex: string
}

/**
 * 已保存的授权备份列表项
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
 * 后端返回的完整授权条目
 */
interface DenuvoAuthEntry {
  appId: number
  gameName: string
  steamId?: string
  appTicketHex?: string
  eTicketHex?: string
  backupTime?: string
}

// ==================== 状态 ====================

/** 当前 Steam 活动用户 */
const activeUser = ref<ActiveSteamUserInfo | null>(null)
/** 当前 Steam 活动用户读取错误 */
const activeUserError = ref('')
/** 表单数据 */
const form = ref<DenuvoForm>({
  appId: '',
  gameName: '',
  steamId: '',
  appTicketHex: '',
  eTicketHex: ''
})
/** 表单验证错误列表 */
const formErrors = ref<string[]>([])
/** 是否正在从注册表提取完整授权 */
const isExtracting = ref(false)
/** 是否正在提取当前 SteamID64 */
const isExtractingSteamId = ref(false)
/** 是否正在保存配置 */
const isSaving = ref(false)
/** 是否正在应用到注册表 */
const isApplying = ref(false)
/** 是否正在加载备份列表 */
const loadingBackups = ref(false)
/** 已保存授权备份列表 */
const backups = ref<DenuvoAuthListItem[]>([])

// ==================== 计算属性 ====================

/**
 * 当前是否处于编辑模式：AppID 与已有备份匹配
 */
const isEditing = computed(() => {
  const appId = Number(form.value.appId)
  return !Number.isNaN(appId) && appId > 0 && backups.value.some(b => b.appId === appId)
})

/**
 * 是否满足提取条件：AppID 合法且为正整数
 */
const canExtract = computed(() => {
  const appId = Number(form.value.appId)
  return !Number.isNaN(appId) && appId > 0
})

/**
 * 是否满足应用条件：存在对应 AppID 的本地备份
 */
const canApply = computed(() => {
  const appId = Number(form.value.appId)
  return !Number.isNaN(appId) && appId > 0 && backups.value.some(b => b.appId === appId)
})

// ==================== 生命周期 ====================

onMounted(() => {
  loadActiveUser()
  loadBackupList()
})

// ==================== 监听 ====================

/**
 * AppID 变化时清空验证错误，并尝试自动加载已有备份
 */
watch(() => form.value.appId, async (newVal) => {
  formErrors.value = []
  const appId = Number(newVal)
  if (Number.isNaN(appId) || appId <= 0) return

  // 如果本地已有该 AppID 的备份，自动填充其他字段，避免用户重复输入
  const existing = backups.value.find(b => b.appId === appId)
  if (existing) {
    await loadBackup(appId)
  }
})

// ==================== 当前 Steam 用户 ====================

/**
 * 获取当前 Steam 活动用户
 */
async function loadActiveUser() {
  try {
    activeUser.value = await invoke<ActiveSteamUserInfo>('get_active_steam_user')
    activeUserError.value = ''
  } catch (error) {
    activeUser.value = null
    activeUserError.value = String(error)
  }
}

/**
 * 提取当前登录账号的 SteamID64 并填入目标 SteamID64 字段
 */
async function extractCurrentSteamId() {
  isExtractingSteamId.value = true
  try {
    const user = await invoke<ActiveSteamUserInfo>('get_active_steam_user')
    form.value.steamId = user.steamId64
    activeUser.value = user
    activeUserError.value = ''
  } catch (error) {
    alert('提取当前 SteamID64 失败：' + String(error) + '\n\n请确保 Steam 客户端已登录。')
  } finally {
    isExtractingSteamId.value = false
  }
}

// ==================== 表单验证 ====================

/**
 * 验证保存表单，返回错误信息数组
 */
function validateSaveForm(): string[] {
  const errors: string[] = []
  const appId = Number(form.value.appId)

  if (Number.isNaN(appId) || appId <= 0) {
    errors.push('AppID 必须是大于 0 的数字')
  }

  if (!form.value.gameName.trim()) {
    errors.push('请填写游戏名称')
  }

  if (!form.value.steamId.trim()) {
    errors.push('请填写目标 SteamID64')
  } else if (!/^\d{17}$/.test(form.value.steamId.trim())) {
    errors.push('SteamID64 应为 17 位纯数字')
  }

  const hexPattern = /^[0-9a-fA-F\s]*$/
  if (form.value.appTicketHex.trim() && !hexPattern.test(form.value.appTicketHex)) {
    errors.push('AppTicket 必须是有效的十六进制字符串')
  }
  if (form.value.eTicketHex.trim() && !hexPattern.test(form.value.eTicketHex)) {
    errors.push('ETicket 必须是有效的十六进制字符串')
  }

  return errors
}

// ==================== 备份列表 ====================

/**
 * 加载已保存授权备份列表
 */
async function loadBackupList() {
  loadingBackups.value = true
  try {
    backups.value = await invoke<DenuvoAuthListItem[]>('list_denuvo_auth_backups')
  } catch (error) {
    backups.value = []
    alert('加载备份列表失败：' + String(error))
  } finally {
    loadingBackups.value = false
  }
}

/**
 * 加载指定 AppID 的备份到表单
 */
async function loadBackup(appId: number) {
  try {
    const entry = await invoke<DenuvoAuthEntry>('load_denuvo_auth_backup', { appId })
    form.value = {
      appId: String(entry.appId),
      gameName: entry.gameName || '',
      steamId: entry.steamId || '',
      appTicketHex: entry.appTicketHex || '',
      eTicketHex: entry.eTicketHex || ''
    }
    formErrors.value = []
  } catch (error) {
    alert('加载备份失败：' + String(error))
  }
}

/**
 * 从注册表提取当前账号的 D 加密授权
 */
async function extractFromRegistry() {
  formErrors.value = []
  const appId = Number(form.value.appId)

  if (Number.isNaN(appId) || appId <= 0) {
    formErrors.value.push('提取前请填写正确的 AppID')
    return
  }

  isExtracting.value = true
  try {
    const entry = await invoke<DenuvoAuthEntry>('read_denuvo_auth_from_registry', { appId })

    // 合并已填写的游戏名，避免覆盖用户自定义名称
    const gameName = form.value.gameName.trim() || entry.gameName || ''

    form.value = {
      appId: String(entry.appId),
      gameName,
      steamId: entry.steamId || activeUser.value?.steamId64 || '',
      appTicketHex: entry.appTicketHex || '',
      eTicketHex: entry.eTicketHex || ''
    }

    alert('已从注册表提取当前授权，请核对 SteamID64 是否正确')
  } catch (error) {
    alert('提取授权失败：' + String(error) + '\n\n请确保该 AppID 已在当前账号启动过一次游戏，注册表中存在授权信息。')
  } finally {
    isExtracting.value = false
  }
}

/**
 * 保存当前表单为本地备份
 */
async function saveEntry() {
  formErrors.value = validateSaveForm()
  if (formErrors.value.length > 0) {
    // 错误已经显示在表单下方，不再弹窗打扰
    return
  }

  isSaving.value = true
  try {
    const entry: DenuvoAuthEntry = {
      appId: Number(form.value.appId),
      gameName: form.value.gameName.trim(),
      steamId: form.value.steamId.trim() || undefined,
      appTicketHex: form.value.appTicketHex.trim() || undefined,
      eTicketHex: form.value.eTicketHex.trim() || undefined
    }

    await invoke('save_denuvo_auth_entry', { entry })
    await loadBackupList()

    // 让 isEditing 计算属性立即生效
    form.value.appId = String(entry.appId)

    alert(isEditing.value ? '配置已更新' : '配置已保存')
  } catch (error) {
    alert('保存配置失败：' + String(error))
  } finally {
    isSaving.value = false
  }
}

/**
 * 将当前 AppID 的本地备份应用到注册表
 */
async function applyToRegistry() {
  formErrors.value = []
  const appId = Number(form.value.appId)

  if (Number.isNaN(appId) || appId <= 0) {
    formErrors.value.push('应用前请填写正确的 AppID')
    return
  }

  if (!backups.value.some(b => b.appId === appId)) {
    formErrors.value.push('未找到该 AppID 的本地备份，请先保存配置')
    return
  }

  isApplying.value = true
  try {
    await invoke('apply_denuvo_auth_backup', { appId })
    alert('授权已成功应用到注册表，现在可以启动游戏')
  } catch (error) {
    alert('应用授权失败：' + String(error))
  } finally {
    isApplying.value = false
  }
}

/**
 * 应用指定 AppID 的备份到注册表（从列表操作）
 */
async function applyBackup(appId: number) {
  try {
    await invoke('apply_denuvo_auth_backup', { appId })
    alert(`AppID ${appId} 的授权已应用到注册表`)
  } catch (error) {
    alert('应用授权失败：' + String(error))
  }
}

/**
 * 删除当前表单对应的备份
 */
async function deleteCurrentBackup() {
  const appId = Number(form.value.appId)
  if (Number.isNaN(appId) || appId <= 0) return
  await deleteBackup(appId)
}

/**
 * 删除指定 AppID 的备份
 */
async function deleteBackup(appId: number) {
  if (!confirm(`确定要删除 AppID ${appId} 的授权配置吗？`)) return

  try {
    await invoke('delete_denuvo_auth_backup', { appId })
    await loadBackupList()

    // 如果删除的是当前编辑项，清空表单
    if (Number(form.value.appId) === appId) {
      resetForm()
    }
  } catch (error) {
    alert('删除配置失败：' + String(error))
  }
}

/**
 * 重置表单到初始状态
 */
function resetForm() {
  form.value = {
    appId: '',
    gameName: '',
    steamId: '',
    appTicketHex: '',
    eTicketHex: ''
  }
  formErrors.value = []
}
</script>

<style scoped>
.denuvo-auth-page {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  padding: 24px;
  box-sizing: border-box;
  color: var(--steam-text-primary);
}

.page-header {
  margin-bottom: 20px;
}

.page-title {
  font-size: 20px;
  font-weight: 600;
  margin: 0 0 8px;
  color: var(--steam-text-primary);
}

.page-subtitle {
  font-size: 14px;
  margin: 0;
  color: var(--steam-text-muted);
  line-height: 1.5;
}

.info-card {
  background: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 20px;
}

.info-card-header {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 16px;
  font-weight: 500;
  margin-bottom: 14px;
  color: var(--steam-text-primary);
}

.info-card-header svg {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}

.info-item {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 8px;
  font-size: 14px;
}

.info-label {
  min-width: 90px;
  color: var(--steam-text-muted);
}

.info-value {
  color: var(--steam-text-primary);
  font-family: monospace;
  word-break: break-all;
}

.active-user-empty {
  font-size: 14px;
  color: var(--steam-text-muted);
  padding: 12px 0;
}

.form-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-group-wide {
  grid-column: 1 / -1;
}

.input-with-btn {
  display: flex;
  gap: 10px;
  align-items: stretch;
}

.input-with-btn .form-input {
  flex: 1;
  min-width: 0;
}

.form-label {
  font-size: 14px;
  color: var(--steam-text-primary);
  font-weight: 500;
}

.required {
  color: var(--steam-error);
  margin-left: 2px;
}

.form-input,
.form-textarea {
  background: var(--steam-input-bg);
  border: 1px solid var(--steam-input-border);
  border-radius: 4px;
  padding: 10px 12px;
  color: var(--steam-text-primary);
  font-size: 14px;
  font-family: inherit;
  outline: none;
  transition: border-color 0.15s ease-out, box-shadow 0.15s ease-out;
}

.form-input::placeholder,
.form-textarea::placeholder {
  color: var(--steam-text-subtle);
}

.form-input:focus,
.form-textarea:focus {
  border-color: var(--steam-accent-blue);
  box-shadow: 0 0 0 2px rgba(var(--steam-accent-blue-rgb), 0.25);
}

.form-input:disabled,
.form-textarea:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  background: var(--steam-bg-tertiary);
}

.form-textarea {
  resize: vertical;
  min-height: 80px;
  font-family: monospace;
  line-height: 1.4;
}

.form-hint {
  font-size: 12px;
  color: var(--steam-text-muted);
  line-height: 1.4;
}

.form-errors {
  margin-top: 16px;
  padding: 12px 14px;
  background: rgba(var(--steam-error-rgb), 0.12);
  border: 1px solid rgba(var(--steam-error-rgb), 0.3);
  border-radius: 6px;
}

.form-error-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: var(--steam-error);
  margin-bottom: 6px;
}

.form-error-item:last-child {
  margin-bottom: 0;
}

.form-error-item svg {
  width: 18px;
  height: 18px;
  flex-shrink: 0;
}

.form-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  margin-top: 20px;
  padding-top: 16px;
  border-top: 1px solid var(--steam-border);
}

/* 修复 disabled 按钮视觉反馈不明显的问题 */
.form-actions :deep(.btn:disabled) {
  opacity: 0.45 !important;
  filter: grayscale(0.4);
  cursor: not-allowed;
}

.usage-notice {
  margin-top: 20px;
  padding: 14px;
  background: var(--steam-bg-tertiary);
  border-radius: 6px;
  font-size: 14px;
  color: var(--steam-text-secondary);
  line-height: 1.6;
}

.usage-notice strong {
  color: var(--steam-text-primary);
}

.usage-steps {
  margin: 10px 0 0;
  padding-left: 20px;
}

.usage-steps li {
  margin-bottom: 6px;
}

.loading-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 32px 0;
  color: var(--steam-text-muted);
  font-size: 14px;
  gap: 10px;
}

.empty-state p {
  margin: 0;
}

.empty-hint {
  font-size: 12px;
  color: var(--steam-text-subtle);
}

.spinner {
  width: 24px;
  height: 24px;
  border: 2px solid var(--steam-border-light);
  border-top-color: var(--steam-accent-blue);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.backup-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.backup-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 12px 14px;
  background: var(--steam-bg-tertiary);
  border: 1px solid transparent;
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s ease-out, border-color 0.15s ease-out;
}

.backup-item:hover {
  background: var(--steam-bg-hover);
}

.backup-active {
  border-color: var(--steam-accent-blue);
  background: rgba(var(--steam-accent-blue-rgb), 0.12);
}

.backup-main {
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
  flex: 1;
}

.backup-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--steam-text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.backup-app-id {
  font-size: 12px;
  color: var(--steam-text-muted);
  font-family: monospace;
}

.backup-tags {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.backup-tag {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 4px;
  color: white;
}

.tag-steam-id {
  background: var(--steam-accent-blue);
}

.tag-ticket {
  background: var(--steam-accent-green);
}

.backup-actions {
  display: flex;
  gap: 8px;
}

.backup-btn {
  padding: 6px 12px;
  font-size: 12px;
  border-radius: 4px;
  border: none;
  cursor: pointer;
  transition: background 0.15s ease-out;
  color: white;
}

.backup-btn.apply {
  background: var(--steam-accent-green);
}

.backup-btn.apply:hover {
  background: var(--steam-accent-green-hover);
}

.backup-btn.delete {
  background: var(--steam-error);
}

.backup-btn.delete:hover {
  background: var(--steam-error-hover);
}

@media (max-width: 900px) {
  .form-grid {
    grid-template-columns: 1fr;
  }

  .backup-item {
    flex-direction: column;
    align-items: flex-start;
  }
}
</style>
