<template>
  <!--
    Help.vue - 使用说明页面（内嵌式）
    在主窗口内显示README.md内容，用户可以边看边操作程序
  -->
  <div class="help-page">
    <!-- 页面头部 -->
    <div class="help-header">
      <h1 class="help-title">使用说明</h1>
      <div class="help-actions">
        <button class="action-btn" @click="refreshContent" title="刷新">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
          </svg>
        </button>
        <button class="action-btn" @click="openInNewWindow" title="在新窗口中打开">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"/>
          </svg>
        </button>
      </div>
    </div>

    <!-- 内容区域 -->
    <div class="help-content-wrapper">
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
      <div v-else class="markdown-body" @click="handleContentClick" v-html="renderedContent"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * Help.vue - 使用说明页面（内嵌式）
 * 在主窗口内显示README.md内容，支持Markdown渲染
 */

import { ref, computed, onMounted } from 'vue'
import { marked } from 'marked'
import { open } from '@tauri-apps/plugin-shell'
import { invoke } from '@tauri-apps/api/core'
import { readReadmeFile } from '../../api/help.api'
import { openHelpWindow } from '../../api/window.api'
import hljs from 'highlight.js'

// 加载状态
const loading = ref(false)

// 错误信息
const error = ref('')

// README 原始内容
const readmeContent = ref('')

// 配置 marked 选项
marked.setOptions({
  breaks: true,
  gfm: true,
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
  headerIds: true,
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
 * 刷新内容
 */
async function refreshContent() {
  await loadReadme()
}

/**
 * 在新窗口中打开
 */
async function openInNewWindow() {
  try {
    // 在打开新窗口前，将当前主题保存到 localStorage，供独立窗口读取
    const html = document.documentElement
    const currentTheme = html.getAttribute('data-theme') || 'dark'
    localStorage.setItem('help-window-theme', currentTheme)

    await openHelpWindow()
  } catch (error) {
    console.error('打开帮助窗口失败:', error)
  }
}

/**
 * 处理内容区域点击事件 - 使用事件委托拦截链接点击
 */
async function handleContentClick(e: MouseEvent) {
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
      await open(href)
    } catch (error) {
      console.error('打开链接失败:', error)
      try {
        await invoke('open_external_link', { url: href })
      } catch (invokeError) {
        console.error('使用 invoke 打开链接也失败:', invokeError)
      }
    }
  }
}

// 组件挂载时加载内容
onMounted(() => {
  loadReadme()
})
</script>

<style scoped>
.help-page {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--steam-bg-secondary);
}

/* 页面头部 */
.help-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 24px;
  background: var(--steam-bg-primary);
  border-bottom: 1px solid var(--steam-border);
}

.help-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.help-actions {
  display: flex;
  gap: 8px;
}

.action-btn {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--steam-text-secondary);
  background: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.action-btn:hover {
  color: var(--steam-text-primary);
  background: var(--steam-bg-tertiary);
}

.action-btn svg {
  width: 18px;
  height: 18px;
}

/* 内容区域 */
.help-content-wrapper {
  flex: 1;
  overflow: hidden;
  position: relative;
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
}

.help-error svg {
  width: 48px;
  height: 48px;
}

/* Markdown 内容区域 */
.markdown-body {
  height: 100%;
  overflow-y: auto;
  padding: 24px 32px;
  font-size: 14px;
  line-height: 1.6;
  color: var(--steam-text-primary);
}

/* 滚动条样式 */
.markdown-body::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

.markdown-body::-webkit-scrollbar-track {
  background: transparent;
}

.markdown-body::-webkit-scrollbar-thumb {
  background: var(--steam-border);
  border-radius: 4px;
}

.markdown-body::-webkit-scrollbar-thumb:hover {
  background: var(--steam-border-light);
}
</style>

<style>
/* GitHub Markdown CSS 样式 */
.markdown-body {
  -ms-text-size-adjust: 100%;
  -webkit-text-size-adjust: 100%;
  word-wrap: break-word;
}

.markdown-body h1,
.markdown-body h2,
.markdown-body h3,
.markdown-body h4,
.markdown-body h5,
.markdown-body h6 {
  margin-top: 20px;
  margin-bottom: 12px;
  font-weight: 600;
  line-height: 1.25;
  color: var(--steam-text-primary);
}

.markdown-body h1 {
  font-size: 1.75em;
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

.markdown-body p {
  margin-top: 0;
  margin-bottom: 12px;
}

.markdown-body a {
  color: var(--steam-accent-blue);
  text-decoration: none;
}

.markdown-body a:hover {
  text-decoration: underline;
}

.markdown-body strong {
  font-weight: 600;
  color: var(--steam-text-primary);
}

.markdown-body ul,
.markdown-body ol {
  margin-top: 0;
  margin-bottom: 12px;
  padding-left: 2em;
  color: var(--steam-text-primary);
}

.markdown-body li {
  margin: 0.25em 0;
}

.markdown-body blockquote {
  margin: 0 0 12px 0;
  padding: 0 1em;
  color: var(--steam-text-muted);
  border-left: 0.25em solid var(--steam-border);
}

.markdown-body code {
  padding: 0.2em 0.4em;
  margin: 0;
  font-size: 85%;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
  background-color: var(--steam-bg-tertiary);
  border-radius: 4px;
  color: var(--steam-text-primary);
}

.markdown-body pre {
  margin-top: 0;
  margin-bottom: 12px;
  padding: 12px;
  overflow: auto;
  font-size: 85%;
  line-height: 1.45;
  background-color: var(--steam-bg-tertiary);
  border-radius: 6px;
  border: 1px solid var(--steam-border);
}

.markdown-body pre code {
  display: inline;
  padding: 0;
  margin: 0;
  overflow: visible;
  line-height: inherit;
  word-wrap: normal;
  background-color: transparent;
  border: 0;
  font-size: 100%;
}

.markdown-body .hljs {
  display: block;
  overflow-x: auto;
  padding: 0;
  background: transparent;
  color: var(--steam-text-primary);
}

.markdown-body hr {
  height: 0.25em;
  padding: 0;
  margin: 16px 0;
  background-color: var(--steam-border);
  border: 0;
}

.markdown-body table {
  display: block;
  width: 100%;
  width: max-content;
  max-width: 100%;
  overflow: auto;
  margin-top: 0;
  margin-bottom: 12px;
  border-spacing: 0;
  border-collapse: collapse;
}

.markdown-body table th,
.markdown-body table td {
  padding: 6px 12px;
  border: 1px solid var(--steam-border);
}

.markdown-body table th {
  font-weight: 600;
  background-color: var(--steam-bg-tertiary);
}

.markdown-body table tr {
  background-color: var(--steam-bg-primary);
  border-top: 1px solid var(--steam-border);
}

.markdown-body table tr:nth-child(2n) {
  background-color: var(--steam-bg-secondary);
}

.markdown-body img {
  max-width: 100%;
  box-sizing: content-box;
  background-color: var(--steam-bg-primary);
  border-radius: 4px;
}

.markdown-body ::selection {
  background-color: rgba(var(--steam-accent-blue-rgb), 0.3);
  color: var(--steam-text-primary);
}
</style>
