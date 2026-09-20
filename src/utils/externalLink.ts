/**
 * externalLink.ts - 外部链接打开工具
 *
 * 统一调用 Rust 命令 open_external_link（http/https 链接的后端已内置
 * 「默认浏览器」前置检测）：系统未设置默认浏览器时，Rust 侧不会触发
 * ShellExecute，而是返回带 NO_DEFAULT_BROWSER_PREFIX 前缀的错误，
 * 避免 Windows 原生「找不到应用程序」弹窗；此处识别该错误后弹出程序内友好提示。
 */

import { invoke } from '@tauri-apps/api/core'
import { useNoBrowserModal } from '../composables/noBrowserModal'

/** 后端「未设置默认浏览器」错误前缀（与 common.rs 的 NO_DEFAULT_BROWSER_PREFIX 保持一致） */
export const NO_DEFAULT_BROWSER_PREFIX = 'E_NO_DEFAULT_BROWSER'

/**
 * 在系统默认浏览器中打开外部链接
 *
 * - 系统已设置默认浏览器：正常打开链接
 * - 未设置默认浏览器：弹出程序内友好提示（可前往系统设置），不触发系统错误弹窗
 * - 其他异常：原样抛出，由各调用点按原有方式处理（保持各页面现有行为不变）
 *
 * @param url 要打开的链接
 */
export async function openExternalUrl(url: string): Promise<void> {
  try {
    await invoke('open_external_link', { url })
  } catch (error) {
    if (String(error).startsWith(NO_DEFAULT_BROWSER_PREFIX)) {
      // 未设置默认浏览器：弹程序内提示，用户可选择前往系统设置
      useNoBrowserModal().show()
      return
    }
    // 其他错误不属于本次处理范围，继续抛给调用点
    throw error
  }
}