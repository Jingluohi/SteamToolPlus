/**
 * i18n 国际化配置
 * 使用 vue-i18n 实现多语言支持
 */

import { createI18n } from 'vue-i18n'
import zhCN from './zh-CN.json'
import enUS from './en-US.json'

/** 支持的语言列表 */
export const SUPPORTED_LOCALES = [
  { code: 'zh-CN', name: '简体中文' },
  { code: 'en-US', name: 'English' }
] as const

/** 默认语言 */
export const DEFAULT_LOCALE = 'zh-CN'

/**
 * 创建 i18n 实例
 * locale: 当前语言，从 localStorage 读取或使用默认值
 * fallbackLocale: 回退语言，当翻译缺失时使用
 */
const i18n = createI18n({
  legacy: false, // 使用 Composition API 模式
  locale: (() => {
    const savedLocale = localStorage.getItem('app-locale')
    // 检查保存的语言是否支持
    if (savedLocale && SUPPORTED_LOCALES.some(lang => lang.code === savedLocale)) {
      return savedLocale
    }
    return DEFAULT_LOCALE
  })(),
  fallbackLocale: DEFAULT_LOCALE,
  messages: {
    'zh-CN': zhCN,
    'en-US': enUS
  },
  // 格式化插值，支持 {count} 等占位符
  missingWarn: false,
  fallbackWarn: false
})

/**
 * 切换语言
 * @param locale 目标语言代码
 */
export function setLocale(locale: string): void {
  i18n.global.locale.value = locale as any
  localStorage.setItem('app-locale', locale)
}

/**
 * 获取当前语言
 */
export function getLocale(): string {
  return i18n.global.locale.value
}

export default i18n
