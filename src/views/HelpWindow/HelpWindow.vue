<template>
  <!--
    HelpWindow.vue - 使用说明窗口页面
    用于显示 README.md 内容，支持 Markdown 渲染
  -->
  <div class="help-window-container">
    <!-- 关闭按钮 - 悬浮在右上角 -->
    <button class="help-close-button" title="关闭" @click="handleClose">
      <svg viewBox="0 0 24 24" fill="currentColor">
        <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
      </svg>
    </button>

    <!-- 内容区域 -->
    <main class="help-content">
      <div v-if="loading" class="help-loading">
        <div class="help-loading-spinner"></div>
        <span>正在加载使用说明...</span>
      </div>
      <div v-else-if="error" class="help-error">
        <svg viewBox="0 0 24 24" fill="currentColor">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
        </svg>
        <span>{{ error }}</span>
      </div>
      <!-- GitHub 风格 Markdown 内容 -->
      <div v-else class="markdown-body" @click="handleContentClick" v-html="renderedContent"></div>
    </main>
  </div>
</template>

<script setup lang="ts">
/**
 * HelpWindow.vue - 使用说明窗口页面
 * 使用 marked 库进行专业的 Markdown 渲染
 * 达到 GitHub 网页端显示效果
 */

import { ref, computed, onMounted } from 'vue'
import { marked } from 'marked'
import { open } from '@tauri-apps/plugin-shell'
import { invoke } from '@tauri-apps/api/core'
import { readReadmeFile } from '../../api/help.api'
import { getConfig } from '../../api/config.api'
import { closeHelpWindow } from '../../api/window.api'
import hljs from 'highlight.js'

// 加载状态
const loading = ref(false)

// 错误信息
const error = ref('')

// README 原始内容
const readmeContent = ref('')

/**
 * 初始化主题
 * 优先从 localStorage 读取父窗口传递的主题，然后从配置读取
 */
async function initTheme() {
  try {
    // 首先尝试从 localStorage 读取父窗口传递的主题
    const parentTheme = localStorage.getItem('help-window-theme')

    if (parentTheme) {
      // 如果父窗口传递了主题，直接使用
      applyTheme(parentTheme === 'dark')
      return
    }

    // 如果没有 localStorage 主题，从配置读取
    const config = await getConfig()
    const themeMode = config.theme?.mode || 'auto'
    const followSystem = config.theme?.followSystem ?? true

    // 检测系统主题
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    const systemIsDark = mediaQuery.matches

    // 确定当前主题
    let isDark = true
    if (themeMode === 'auto' || followSystem) {
      isDark = systemIsDark
    } else {
      isDark = themeMode === 'dark'
    }

    applyTheme(isDark)
  } catch (e) {
    // 如果读取失败，默认使用深色主题
    console.warn('读取主题配置失败，使用默认主题:', e)
    applyTheme(true)
  }
}

/**
 * 应用主题到 DOM
 */
function applyTheme(isDark: boolean) {
  const html = document.documentElement
  const body = document.body

  if (isDark) {
    html.setAttribute('data-theme', 'dark')
    body.classList.remove('light-theme')
    body.classList.add('dark-theme')
  } else {
    html.setAttribute('data-theme', 'light')
    body.classList.remove('dark-theme')
    body.classList.add('light-theme')
  }
}

// 配置 marked 选项
marked.setOptions({
  // 启用 GitHub 风格的换行
  breaks: true,
  // 启用 GitHub 风格的 Markdown
  gfm: true,
  // 代码高亮
  highlight: function(code: string, lang: string | undefined) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return hljs.highlight(code, { language: lang }).value
      } catch (e) {
        console.warn('代码高亮失败:', e)
      }
    }
    return hljs.highlightAuto(code).value
  },
  // 标题渲染，添加锚点
  headerIds: true,
  // 允许 HTML 标签
  sanitize: false,
})

// 渲染后的内容
const renderedContent = computed(() => {
  if (!readmeContent.value) return ''
  return marked.parse(readmeContent.value) as string
})

/**
 * 加载 README 文件内容
 */
async function loadReadme() {
  loading.value = true
  error.value = ''

  try {
    const content = await readReadmeFile()
    readmeContent.value = content
  } catch (err) {
    error.value = err instanceof Error ? err.message : '读取使用说明失败，请确保 resources/README.md 文件存在'
    readmeContent.value = ''
  } finally {
    loading.value = false
  }
}

/**
 * 处理内容区域点击事件 - 使用事件委托拦截链接点击
 */
async function handleContentClick(e: MouseEvent) {
  // 查找点击的目标是否是链接
  const target = e.target as HTMLElement
  const link = target.closest('a[href]') as HTMLAnchorElement | null

  if (!link) return

  const href = link.getAttribute('href')
  if (!href) return

  // 如果是外部链接（http/https）
  if (href.startsWith('http://') || href.startsWith('https://')) {
    e.preventDefault()
    e.stopPropagation()
    try {
      // 使用 Tauri shell 插件在默认浏览器中打开链接
      await open(href)
    } catch (error) {
      console.error('打开链接失败:', error)
      // 如果 shell 插件失败，尝试使用 invoke
      try {
        await invoke('open_external_link', { url: href })
      } catch (invokeError) {
        console.error('使用 invoke 打开链接也失败:', invokeError)
      }
    }
  }
  // 如果是锚点链接（#开头），让默认行为处理页面内跳转
}

/**
 * 处理关闭窗口
 */
async function handleClose() {
  await closeHelpWindow()
}

// 组件挂载时加载内容
onMounted(async () => {
  await initTheme()
  await loadReadme()
})
</script>

<style>
/* 全局样式重置 */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  width: 100%;
  height: 100%;
  overflow: hidden;
}
</style>

<style scoped>
/* 帮助窗口容器 */
.help-window-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--steam-bg-primary);
  color: var(--steam-text-primary);
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Noto Sans', Helvetica, Arial, sans-serif;
  position: relative;
}

/* 关闭按钮 - 悬浮在右上角 */
.help-close-button {
  position: absolute;
  top: 12px;
  right: 12px;
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--steam-text-muted);
  background: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s ease;
  z-index: 100;
}

.help-close-button:hover {
  background: #e81123;
  color: white;
  border-color: #e81123;
}

.help-close-button svg {
  width: 18px;
  height: 18px;
}

/* 内容区域 */
.help-content {
  flex: 1;
  overflow: hidden;
  position: relative;
}

/* GitHub Markdown 样式 */
.markdown-body {
  height: 100%;
  overflow-y: auto;
  padding: 32px 40px;
  font-size: 16px;
  line-height: 1.6;
  color: var(--steam-text-primary);
  background: var(--steam-bg-primary);
}

/* 滚动条样式 */
.markdown-body::-webkit-scrollbar {
  width: 10px;
  height: 10px;
}

.markdown-body::-webkit-scrollbar-track {
  background: var(--steam-bg-primary);
}

.markdown-body::-webkit-scrollbar-thumb {
  background: var(--steam-border);
  border-radius: 5px;
}

.markdown-body::-webkit-scrollbar-thumb:hover {
  background: var(--steam-border-light);
}

/* 加载状态 */
.help-loading {
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  color: var(--steam-text-muted);
  background: var(--steam-bg-primary);
}

.help-loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--steam-border);
  border-top-color: var(--steam-accent-blue);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* 错误状态 */
.help-error {
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  color: var(--steam-error);
  text-align: center;
  padding: 32px;
  background: var(--steam-bg-primary);
}

.help-error svg {
  width: 48px;
  height: 48px;
}
</style>

<style>
/* GitHub Markdown CSS 样式 - 支持深色/浅色主题 */
.markdown-body {
  /* 基础排版 */
  -ms-text-size-adjust: 100%;
  -webkit-text-size-adjust: 100%;
  word-wrap: break-word;
}

/* 标题样式 */
.markdown-body h1,
.markdown-body h2,
.markdown-body h3,
.markdown-body h4,
.markdown-body h5,
.markdown-body h6 {
  margin-top: 24px;
  margin-bottom: 16px;
  font-weight: 600;
  line-height: 1.25;
  color: var(--steam-text-primary);
}

.markdown-body h1 {
  font-size: 2em;
  padding-bottom: 0.3em;
  border-bottom: 1px solid var(--steam-border);
}

.markdown-body h2 {
  font-size: 1.5em;
  padding-bottom: 0.3em;
  border-bottom: 1px solid var(--steam-border);
}

.markdown-body h3 {
  font-size: 1.25em;
}

.markdown-body h4 {
  font-size: 1em;
}

.markdown-body h5 {
  font-size: 0.875em;
}

.markdown-body h6 {
  font-size: 0.85em;
  color: var(--steam-text-muted);
}

/* 段落和文本 */
.markdown-body p {
  margin-top: 0;
  margin-bottom: 16px;
}

.markdown-body a {
  color: var(--steam-accent-blue);
  text-decoration: none;
  cursor: pointer;
}

.markdown-body a:hover {
  text-decoration: underline;
}

.markdown-body a:active,
.markdown-body a:focus {
  outline: 2px solid var(--steam-accent-blue);
  outline-offset: -2px;
}

/* 粗体和斜体 */
.markdown-body strong {
  font-weight: 600;
  color: var(--steam-text-primary);
}

.markdown-body em {
  font-style: italic;
}

.markdown-body del {
  text-decoration: line-through;
}

/* 列表样式 */
.markdown-body ul,
.markdown-body ol {
  margin-top: 0;
  margin-bottom: 16px;
  padding-left: 2em;
  color: var(--steam-text-primary);
}
.markdown-body ul ul,
.markdown-body ul ol,
.markdown-body ol ol,
.markdown-body ol ul {
  margin-top: 0;
  margin-bottom: 0;
}

.markdown-body li {
  margin: 0.25em 0;
}

.markdown-body li > p {
  margin-top: 16px;
}

.markdown-body li + li {
  margin-top: 0.25em;
}

.markdown-body li::marker {
  color: var(--steam-text-muted);
}

/* 任务列表 */
.markdown-body input[type="checkbox"] {
  margin-right: 0.5em;
  accent-color: var(--steam-accent-blue);
}

/* 引用块 */
.markdown-body blockquote {
  margin: 0 0 16px 0;
  padding: 0 1em;
  color: var(--steam-text-muted);
  border-left: 0.25em solid var(--steam-border);
}

.markdown-body blockquote > :first-child {
  margin-top: 0;
}

.markdown-body blockquote > :last-child {
  margin-bottom: 0;
}

/* 代码样式 */
.markdown-body code {
  padding: 0.2em 0.4em;
  margin: 0;
  font-size: 85%;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, 'Liberation Mono', monospace;
  background-color: var(--steam-bg-tertiary);
  border-radius: 6px;
  color: var(--steam-text-primary);
}

.markdown-body pre {
  margin-top: 0;
  margin-bottom: 16px;
  padding: 16px;
  overflow: auto;
  font-size: 85%;
  line-height: 1.45;
  background-color: var(--steam-bg-secondary);
  border-radius: 6px;
  border: 1px solid var(--steam-border);
}

.markdown-body pre code {
  display: inline;
  max-width: auto;
  padding: 0;
  margin: 0;
  overflow: visible;
  line-height: inherit;
  word-wrap: normal;
  background-color: transparent;
  border: 0;
  font-size: 100%;
}

/* 代码高亮 - 适配深色/浅色主题 */
.markdown-body .hljs {
  display: block;
  overflow-x: auto;
  padding: 0;
  background: transparent;
  color: var(--steam-text-primary);
}

.markdown-body .hljs-comment,
.markdown-body .hljs-quote {
  color: var(--steam-text-muted);
  font-style: italic;
}

.markdown-body .hljs-keyword,
.markdown-body .hljs-selector-tag,
.markdown-body .hljs-subst {
  color: var(--steam-error);
}

.markdown-body .hljs-number,
.markdown-body .hljs-literal,
.markdown-body .hljs-variable,
.markdown-body .hljs-template-variable,
.markdown-body .hljs-tag .hljs-attr {
  color: var(--steam-accent-blue);
}

.markdown-body .hljs-string,
.markdown-body .hljs-doctag {
  color: var(--steam-accent-blue-hover);
}

.markdown-body .hljs-title,
.markdown-body .hljs-section,
.markdown-body .hljs-selector-id {
  color: var(--steam-accent-purple);
}

.markdown-body .hljs-subst {
  font-weight: normal;
}

.markdown-body .hljs-type,
.markdown-body .hljs-class .hljs-title {
  color: var(--steam-accent-orange);
}

.markdown-body .hljs-tag,
.markdown-body .hljs-name,
.markdown-body .hljs-attribute {
  color: var(--steam-accent-green);
}

.markdown-body .hljs-regexp,
.markdown-body .hljs-link {
  color: var(--steam-accent-blue-hover);
}

.markdown-body .hljs-symbol,
.markdown-body .hljs-bullet {
  color: var(--steam-accent-blue);
}

.markdown-body .hljs-built_in,
.markdown-body .hljs-builtin-name {
  color: var(--steam-accent-orange);
}

.markdown-body .hljs-meta {
  color: var(--steam-text-muted);
}

.markdown-body .hljs-deletion {
  background: rgba(var(--steam-error-rgb), 0.2);
  color: var(--steam-error);
}

.markdown-body .hljs-addition {
  background: rgba(var(--steam-accent-green-rgb), 0.2);
  color: var(--steam-accent-green);
}

.markdown-body .hljs-emphasis {
  font-style: italic;
}

.markdown-body .hljs-strong {
  font-weight: bold;
}

/* 水平线 */
.markdown-body hr {
  height: 0.25em;
  padding: 0;
  margin: 24px 0;
  background-color: var(--steam-border);
  border: 0;
}

.markdown-body hr::before {
  display: table;
  content: '';
}

.markdown-body hr::after {
  display: table;
  clear: both;
  content: '';
}

/* 表格样式 */
.markdown-body table {
  display: block;
  width: 100%;
  width: max-content;
  max-width: 100%;
  overflow: auto;
  margin-top: 0;
  margin-bottom: 16px;
  border-spacing: 0;
  border-collapse: collapse;
}

.markdown-body table th,
.markdown-body table td {
  padding: 6px 13px;
  border: 1px solid var(--steam-border);
}

.markdown-body table th {
  font-weight: 600;
  background-color: var(--steam-bg-secondary);
}

.markdown-body table tr {
  background-color: var(--steam-bg-primary);
  border-top: 1px solid var(--steam-border);
}

.markdown-body table tr:nth-child(2n) {
  background-color: var(--steam-bg-secondary);
}

.markdown-body table img {
  background-color: transparent;
}

/* 图片样式 */
.markdown-body img {
  max-width: 100%;
  box-sizing: content-box;
  background-color: var(--steam-bg-primary);
  border-radius: 6px;
}

.markdown-body img[align='right'] {
  padding-left: 20px;
}

.markdown-body img[align='left'] {
  padding-right: 20px;
}

/* Emoji */
.markdown-body .emoji {
  max-width: none;
  vertical-align: text-top;
  background-color: transparent;
}

/* 锚点标题 */
.markdown-body h1:hover .anchor,
.markdown-body h2:hover .anchor,
.markdown-body h3:hover .anchor,
.markdown-body h4:hover .anchor,
.markdown-body h5:hover .anchor,
.markdown-body h6:hover .anchor {
  text-decoration: none;
}

.markdown-body h1:hover .anchor .octicon-link,
.markdown-body h2:hover .anchor .octicon-link,
.markdown-body h3:hover .anchor .octicon-link,
.markdown-body h4:hover .anchor .octicon-link,
.markdown-body h5:hover .anchor .octicon-link,
.markdown-body h6:hover .anchor .octicon-link {
  visibility: visible;
}

.markdown-body h1 .octicon-link,
.markdown-body h2 .octicon-link,
.markdown-body h3 .octicon-link,
.markdown-body h4 .octicon-link,
.markdown-body h5 .octicon-link,
.markdown-body h6 .octicon-link {
  color: var(--steam-text-primary);
  vertical-align: middle;
  visibility: hidden;
}

.markdown-body h1 .anchor,
.markdown-body h2 .anchor,
.markdown-body h3 .anchor,
.markdown-body h4 .anchor,
.markdown-body h5 .anchor,
.markdown-body h6 .anchor {
  float: left;
  padding-right: 4px;
  margin-left: -20px;
  line-height: 1;
}

/* 定义列表 */
.markdown-body dl {
  margin-top: 0;
  margin-bottom: 16px;
  padding: 0;
}

.markdown-body dl dt {
  padding: 0;
  margin-top: 16px;
  font-size: 1em;
  font-style: italic;
  font-weight: 600;
}

.markdown-body dl dd {
  padding: 0 16px;
  margin-bottom: 16px;
}

/* 脚注 */
.markdown-body .footnotes {
  font-size: 12px;
  color: var(--steam-text-muted);
  border-top: 1px solid var(--steam-border);
}

.markdown-body .footnotes ol {
  padding-left: 16px;
}

.markdown-body .footnotes li {
  position: relative;
}

.markdown-body .footnotes li:target::before {
  position: absolute;
  top: -8px;
  right: -8px;
  bottom: -8px;
  left: -24px;
  pointer-events: none;
  content: '';
  border: 2px solid var(--steam-accent-blue);
  border-radius: 6px;
}

.markdown-body .footnotes li:target {
  color: var(--steam-text-primary);
}

.markdown-body .footnotes .data-footnote-backref g-emoji {
  font-family: monospace;
}

/* 提示框样式 */
.markdown-body .markdown-alert {
  padding: 0 1em;
  margin-bottom: 16px;
  color: inherit;
  border-left: 0.25em solid var(--steam-border);
}

.markdown-body .markdown-alert > :first-child {
  margin-top: 0;
}

.markdown-body .markdown-alert > :last-child {
  margin-bottom: 0;
}

.markdown-body .markdown-alert.markdown-alert-note {
  border-left-color: var(--steam-accent-blue);
}

.markdown-body .markdown-alert.markdown-alert-tip {
  border-left-color: var(--steam-accent-green);
}

.markdown-body .markdown-alert.markdown-alert-important {
  border-left-color: var(--steam-accent-purple);
}

.markdown-body .markdown-alert.markdown-alert-warning {
  border-left-color: var(--steam-warning);
}

.markdown-body .markdown-alert.markdown-alert-caution {
  border-left-color: var(--steam-error);
}

/* 数学公式 */
.markdown-body .math {
  display: inline-block;
}

.markdown-body .math.display {
  display: block;
}

/* 目录 */
.markdown-body .toc {
  margin-bottom: 16px;
  padding: 16px;
  background-color: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 6px;
}

.markdown-body .toc ul {
  margin: 0;
  padding-left: 1.5em;
}

.markdown-body .toc li {
  margin: 0.25em 0;
}

.markdown-body .toc a {
  color: var(--steam-accent-blue);
}

/* 选择文本 */
.markdown-body ::selection {
  background-color: rgba(var(--steam-accent-blue-rgb), 0.3);
  color: var(--steam-text-primary);
}
</style>
