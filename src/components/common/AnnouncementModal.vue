<template>
  <!-- 启动时「公告板」大号弹窗
       尺寸为程序窗口的 80%（宽高各 80%），支持滚动浏览长公告；
       markdown 正文由 marked 渲染，图片走 asset 协议、链接走外部浏览器打开
       仅可通过弹窗按钮关闭，点击空白处不关闭 -->
  <div class="announcement-overlay">
    <div class="announcement-modal">
      <!-- 弹窗标题栏 -->
      <div class="announcement-header">
        <h2 class="announcement-title">公告</h2>
        <button class="announcement-close" aria-label="关闭" @click="$emit('close')">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true">
            <path fill="currentColor" d="M3.72 3.72a.75.75 0 0 1 1.06 0L8 6.94l3.22-3.22a.75.75 0 1 1 1.06 1.06L9.06 8l3.22 3.22a.75.75 0 1 1-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 0 1-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 0 1 0-1.06Z" />
          </svg>
        </button>
      </div>

      <!-- 公告正文：markdown 渲染区（github-markdown 样式） -->
      <div
        class="announcement-body markdown-body"
        v-html="renderedHtml"
        @click="onContentClick"
      ></div>

      <!-- 底部操作区：关闭按钮 -->
      <div class="announcement-footer">
        <button class="modal-btn primary" @click="$emit('close')">知道了</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * AnnouncementModal.vue - 启动公告板弹窗
 *
 * 展示 resources/announcement/announcement.md 的渲染结果：
 * - 正文用 marked 将 markdown 编译为 HTML（github-markdown-css 样式，随主题变量自适应深浅色）
 * - 图片相对路径（如 img/banner.png）解析为 announcement 目录下真实文件，
 *   经 convertFileSrc 转成 asset:// 协议 URL，保证打包后也能加载资源目录图片
 * - 链接（http/https）点击时统一走 open_external_link（内置默认浏览器检测），
 *   避免 WebView2 内部直接导航
 */

import { computed } from 'vue'
import { marked } from 'marked'
import { convertFileSrc } from '@tauri-apps/api/core'
import { openExternalUrl } from '../../utils/externalLink'
import 'github-markdown-css/github-markdown.css'

const props = defineProps<{
  /** markdown 公告正文原文 */
  content: string
  /** announcement 目录的绝对路径（后端返回，用于定位公告内的相对图片） */
  basePath: string
}>()

defineEmits<{
  (e: 'close'): void
}>()

/**
 * 规范化相对路径并防止越界
 * 将 markdown 中的图片相对路径（可能含 ./ ../ 分隔）解析为无越界的规范相对路径
 * 若出现 .. 且前方已无层级时忽略该段，保证结果始终位于 announcement 目录内
 */
function normalizeRel(rel: string): string {
  const parts: string[] = []
  for (const seg of rel.split('/')) {
    if (!seg || seg === '.') continue
    if (seg === '..') {
      // 越界防御：越出 announcement 目录的 .. 一律忽略，不允许读取目录外文件
      parts.pop()
    } else {
      parts.push(seg)
    }
  }
  return parts.join('\\')
}

/**
 * 解析图片标记中的地址为可加载的 URL
 * 无论 markdown 语法（![alt](img/x.png)）还是 HTML 语法（<img src="img/x.png" width="300">），
 * 渲染后统一经 convertImgSrcs 调用本函数处理：
 * - http(s)、asset、data 开头的直接原样返回（外链/绝对资源/内嵌图片不处理）
 * - 其余视为相对路径，拼上 announcement 目录绝对路径后转 asset URL
 */
function resolveImageSrc(href: string): string {
  if (/^(https?:|asset:|data:|blob:)/i.test(href)) return href
  const fullPath = `${props.basePath.replace(/\\+$/, '')}\\${normalizeRel(decodeURIComponent(href))}`
  return convertFileSrc(fullPath)
}

/**
 * 渲染后统一修正 HTML 中所有 <img> 的相对路径 src
 * 为什么不只在 markdown renderer 里处理：markdown 语法图片会被 marked 转成 <img>，
 * 但用户手写 HTML 语法（<img src="..." width="300"> 可设定大小）同样会产生 <img>，
 * 两者统一在此转成 asset URL，保证一行多张 / 自定义尺寸的写法都能正常加载。
 * 解析用 DOMParser，兼容属性顺序任意、带引号大小写等变体。
 */
function convertImgSrcs(html: string): string {
  const doc = new DOMParser().parseFromString(html, 'text/html')
  doc.querySelectorAll('img').forEach((img) => {
    const src = img.getAttribute('src')
    if (src) img.setAttribute('src', resolveImageSrc(src))
  })
  return doc.body ? doc.body.innerHTML : html
}

/** 编译后的公告 HTML（同步渲染）：markdown → HTML → 统一修正图片 src */
const renderedHtml = computed(() => convertImgSrcs(marked.parse(props.content, { gfm: true }) as string))

/**
 * 正文区点击委托：
 * 拦截所有 <a>，将 http/https 链接交给外部浏览器打开；
 * 锚点 (#) 与 asset 内链不拦截；其余非法协议一律阻止默认导航
 */
function onContentClick(e: MouseEvent) {
  const target = e.target as HTMLElement
  const link = target.closest('a')
  if (!link) return
  const href = link.getAttribute('href')
  if (!href) return

  // https/http 外链：交给系统默认浏览器
  if (/^https?:/i.test(href)) {
    e.preventDefault()
    openExternalUrl(href)
    return
  }
  // 页内锚点 / 程序内资源：允许浏览器/asset 协议自身行为
  if (href.startsWith('#') || /^asset:/i.test(href)) return
  // 其他未知协议阻止默认跳转
  e.preventDefault()
}
</script>

<style scoped>
/* 弹窗遮罩：覆盖全窗口，深色半透明背景 */
.announcement-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
  backdrop-filter: blur(4px);
}

/* 公告弹窗主体：宽高各占程序窗口的 80% */
.announcement-modal {
  width: 80vw;
  height: 80vh;
  display: flex;
  flex-direction: column;
  background: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border-color);
  border-radius: 4px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
  overflow: hidden;
}

/* 标题栏：与更新弹窗标题样式一致，右侧增加关闭图标按钮 */
.announcement-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 16px 14px 24px;
  background: var(--steam-bg-tertiary);
  border-bottom: 1px solid var(--steam-border-color);
  flex-shrink: 0;
}

.announcement-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.announcement-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  padding: 0;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: var(--steam-text-secondary);
  cursor: pointer;
  transition: all 0.15s ease-out;
}

.announcement-close:hover {
  background: var(--steam-bg-primary);
  color: var(--steam-text-primary);
}

/* 公告正文滚动区：占满中间剩余空间，溢出滚动 */
.announcement-body {
  flex: 1;
  overflow-y: auto;
  padding: 20px 24px;
  /* 用程序主题变量覆盖 github-markdown 默认颜色，深浅色主题自适应 */
  --fgColor-default: var(--steam-text-primary);
  --fgColor-muted: var(--steam-text-secondary);
  --fgColor-accent: var(--steam-accent-blue);
  --fgColor-success: var(--steam-accent-green);
  --fgColor-danger: #eb4b4b;
  --bgColor-default: transparent;
  --bgColor-muted: var(--steam-bg-tertiary);
  --bgColor-neutral-muted: var(--steam-bg-tertiary);
  --borderColor-default: var(--steam-border-color);
  --borderColor-muted: var(--steam-border-color);
  --borderColor-accent-emphasis: var(--steam-accent-blue);
  color: var(--steam-text-primary);
}

/* 正文区滚动条：跟随主题 */
.announcement-body::-webkit-scrollbar {
  width: 10px;
}

.announcement-body::-webkit-scrollbar-thumb {
  background: var(--steam-border-color);
  border-radius: 5px;
}

.announcement-body::-webkit-scrollbar-thumb:hover {
  background: var(--steam-text-secondary);
}

/* 正文区图片：默认 inline 布局（同一行多个图片自然并排），
   并排图片之间留出间距，垂直居中对齐更美观；
   width/height 属性或 style 可在公告 markdown 中自定义大小；
   display 需显式覆盖为 inline（全局 main.css 对 img 设置了 display:block，
   否则公告里的多张图会各自独占一行无法并排） */
.announcement-body :deep(img) {
  display: inline;
  max-width: 100%;
  height: auto;
  vertical-align: middle;
  margin-right: 8px;
}

.announcement-body :deep(img:last-child) {
  margin-right: 0;
}

/* 链接颜色跟随主题强调色，hover 变绿（与程序其他链接一致） */
.announcement-body :deep(a) {
  color: var(--steam-accent-blue);
}

.announcement-body :deep(a:hover) {
  color: var(--steam-accent-green);
}

/* 底部按钮区 */
.announcement-footer {
  display: flex;
  justify-content: flex-end;
  padding: 14px 24px;
  background: var(--steam-bg-tertiary);
  border-top: 1px solid var(--steam-border-color);
  flex-shrink: 0;
}

.modal-btn {
  padding: 8px 18px;
  font-size: 14px;
  border-radius: 4px;
  border: 1px solid var(--steam-accent-blue);
  cursor: pointer;
  transition: all 0.15s ease-out;
}

.modal-btn.primary {
  background: var(--steam-accent-blue);
  color: white;
}

.modal-btn.primary:hover {
  background: var(--steam-accent-green);
  border-color: var(--steam-accent-green);
}
</style>