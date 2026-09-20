<template>
  <!--
    NoBrowserModal.vue - 「未设置默认浏览器」提示弹窗
    点击打开链接但系统未设置默认浏览器时弹出，
    引导用户前往系统「默认应用」设置页；
    用于替代 Windows 原生「标题为 URL、内容为找不到应用程序」的弹窗。
  -->
  <BaseModal
    v-model="visible"
    title="无法打开链接"
    :width="'440px'"
    cancel-text="取消"
    confirm-text="去系统设置"
    confirm-variant="primary"
    @confirm="goToSystemSettings"
  >
    <template #body>
      <p class="no-browser-message">
        未检测到系统默认浏览器，请先在系统设置中设置默认浏览器，再点击按钮打开链接。
      </p>
    </template>
  </BaseModal>
</template>

<script setup lang="ts">
/**
 * NoBrowserModal.vue
 * 全局单例弹窗：显示状态由 composables/noBrowserModal.ts 管理，
 * 在 App.vue 中挂载一次，所有页面共用，无需在业务页面重复引入。
 */

import { invoke } from '@tauri-apps/api/core'
import BaseModal from './BaseModal.vue'
import { useNoBrowserModal } from '../../composables/noBrowserModal'

/** 弹窗显示状态与控制方法（模块级单例） */
const { visible, hide } = useNoBrowserModal()

/**
 * 「去系统设置」按钮：关闭弹窗并打开系统「默认应用」设置页
 * ms-settings: 协议由系统设置应用处理，不依赖默认浏览器，可正常打开
 */
async function goToSystemSettings() {
  hide()
  try {
    await invoke('open_external_link', { url: 'ms-settings:defaultapps' })
  } catch {
    // 设置页打不开时静默处理，不影响用户后续操作
  }
}
</script>

<style scoped>
/* 提示正文 */
.no-browser-message {
  margin: 0;
  font-size: 14px;
  line-height: 1.6;
  color: var(--steam-text-secondary);
}
</style>