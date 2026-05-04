<template>
  <!--
    Troubleshooting.vue - 疑难解答页面
    显示常见问题及解决方法
  -->
  <div class="troubleshooting-page">
    <!-- 页面头部 -->
    <div class="page-header">
      <h1 class="page-title">疑难解答</h1>

      <!-- 搜索框区域 -->
      <div class="search-box">
        <div class="search-input-wrapper">
          <svg class="search-icon" viewBox="0 0 24 24" fill="currentColor">
            <path d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>
          </svg>
          <input
            v-model="searchQuery"
            type="text"
            class="search-input"
            placeholder="搜索关键词..."
            @input="handleSearch"
          />
          <button
            v-if="searchQuery"
            class="search-clear-btn"
            @click="clearSearch"
            title="清除搜索"
          >
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
            </svg>
          </button>
        </div>

        <!-- 搜索结果导航 -->
        <div v-if="matchCount > 0" class="search-nav">
          <span class="match-count">{{ currentMatchIndex + 1 }} / {{ matchCount }}</span>
          <button class="nav-btn" @click="navigateToPrev" title="上一个">
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z"/>
            </svg>
          </button>
          <button class="nav-btn" @click="navigateToNext" title="下一个">
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z"/>
            </svg>
          </button>
        </div>
        <div v-else-if="searchQuery && matchCount === 0" class="search-nav">
          <span class="match-count no-match">无匹配结果</span>
        </div>
      </div>
    </div>

    <!-- 内容区域 -->
    <div class="content-wrapper">
      <div class="troubleshooting-list">
        <div
          v-for="item in troubleshootingItems"
          :key="item.id"
          class="troubleshooting-item"
          :class="{ expanded: item.isExpanded }"
        >
          <!-- 问题标题（可点击展开/折叠） -->
          <div class="question" @click="toggleItem(item)">
            <span class="icon">Q</span>
            <span class="text">{{ item.question }}</span>
            <svg
              class="expand-icon"
              :class="{ expanded: item.isExpanded }"
              viewBox="0 0 24 24"
              fill="currentColor"
            >
              <path d="M7.41 8.59L12 13.17l4.59-4.58L18 10l-6 6-6-6 1.41-1.41z"/>
            </svg>
          </div>
          <!-- 答案内容（可折叠） -->
          <div v-show="item.isExpanded" class="answer">
            <span class="icon">A</span>
            <div class="text">
              <p v-for="(line, index) in item.answer" :key="index">{{ line }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * Troubleshooting.vue - 疑难解答页面
 */

import { ref, nextTick } from 'vue'

// 问题数据
interface TroubleshootingItem {
  id: number
  question: string
  answer: string[]
  isExpanded: boolean
}

const troubleshootingItems = ref<TroubleshootingItem[]>([
  {
    id: 1,
    question: '程序打开后显示空白/透明窗口',
    answer: [
      '1. 确保程序根目录下存在 resources/pic/background/default.png 背景图片',
      '2. 检查是否有杀毒软件拦截了程序',
      '3. 尝试删除 %appdata%/SteamToolPlus/config/ 目录后重新启动程序'
    ],
    isExpanded: false
  },
  {
    id: 2,
    question: '游戏下载进度卡在 0% 不动或到某一进度不动',
    answer: [
      '1. 检查网络连接是否正常',
      '2. 确认清单文件夹路径正确，包含有效的 manifest 文件',
      '3. 检查磁盘空间是否充足',
      '4. 看看任务管理器中ddv20.exe的磁盘占用，5mb/s以上就没事，耐心等待一下'
    ],
    isExpanded: false
  },
  {
    id: 3,
    question: '黑神话：悟空打不开',
    answer: [
      '1. 确保游戏放到纯英文路径',
      '2. 实在不会操作去下 https://pan.baidu.com/s/1gSJt0jh8JTdUD_Eep0CMnQ?pwd=igqq 提取码: igqq 记得解压再双击 bat'
    ],
    isExpanded: false
  },
  {
    id: 4,
    question: '程序无法启动，提示缺少 DLL 文件',
    answer: [
      '1. 安装最新的 Visual C++ Redistributable',
      '2. 确保系统已安装 WebView2 运行时',
      '3. 尝试重新下载并解压程序'
    ],
    isExpanded: false
  },
  {
    id: 5,
    question: '游戏添加到库后无法启动',
    answer: [
      '1. 检查游戏路径是否正确，确保 exe 文件存在',
      '2. 确认游戏已正确安装或下载完成',
      '3. 尝试直接从游戏目录运行 exe 文件，查看是否有错误提示',
      '4. 检查游戏是否需要额外的运行库或依赖'
    ],
    isExpanded: false
  },    
  {id: 6,
    question: '有些游戏脱壳失败',
    answer: [
      '1. 有些游戏的运行程序exe未被套壳，别管',
      '2. 联网或D加密我的程序的免steam补丁没有用',
    ],
    isExpanded: false
  },
  {
    id: 7,
    question: '背景图片无法显示',
    answer: [
      '1. 确保图片格式为 JPG、PNG 或 WebP',
      '2. 检查图片文件是否损坏',
      '3. 尝试重新导入背景图片',
      '4. 删除 %appdata%/SteamToolPlus/config/background.json 后重启程序'
    ],
    isExpanded: false
  },
  {
    id: 8,
    question: '游戏打不开',
    answer: [
      '1. 仔细查看补丁界面的使用说明',
      '2. 确保游戏被放在纯英文路径',
      '3. 确保系统杀毒软件没有隔离删除补丁文件的 dll'
    ],
    isExpanded: false
  },
  {
    id: 9,
    question: '报 "Cannot start Denuvo service"',
    answer: [
      '1. 确保设置了虚拟化环境（应用补丁那个下面有说明文档）'
    ],
    isExpanded: false
  },
  {
    id: 10,
    question: 'D 加密虚拟机安全吗',
    answer: [
      '1. 补丁我大部分都试过了，可以正常游玩',
      '2. 玩就别怕，怕就别玩',
      '3. VBS 其实原理和我这个一样，只是他那个可能是傻瓜式操作',
      '4. 任何说虚拟化环境不用改 bios 的都是瞎扯，毕竟 cpu 必须开虚拟化功能',
      '5. 安全启动可以不关，但是还是要走高级启动运行非官方签字驱动运行'
    ],
    isExpanded: false
  },
  {
    id: 11,
    question: '蓝奏云的补丁文件的密码是多少？',
    answer: [
      '1234'
    ],
    isExpanded: false
  }
])

// 搜索相关状态
const searchQuery = ref('')
const matchCount = ref(0)
const currentMatchIndex = ref(0)
const highlightMarks: HTMLElement[] = []

/**
 * 切换问题展开/折叠状态
 */
function toggleItem(item: TroubleshootingItem) {
  item.isExpanded = !item.isExpanded
}

/**
 * 处理搜索输入
 */
function handleSearch() {
  nextTick(() => {
    performSearch()
  })
}

/**
 * 执行搜索并高亮匹配项
 */
function performSearch() {
  // 清除之前的高亮
  clearHighlights()

  if (!searchQuery.value.trim()) {
    matchCount.value = 0
    currentMatchIndex.value = 0
    return
  }

  const contentDiv = document.querySelector('.troubleshooting-list') as HTMLElement
  if (!contentDiv) return

  const query = searchQuery.value.trim().toLowerCase()

  // 先搜索所有内容（包括折叠的），确定哪些项目需要展开
  const matchedItemIds = new Set<number>()
  troubleshootingItems.value.forEach(item => {
    const questionMatch = item.question.toLowerCase().includes(query)
    const answerMatch = item.answer.some(line => line.toLowerCase().includes(query))
    if (questionMatch || answerMatch) {
      matchedItemIds.add(item.id)
    }
  })

  // 展开所有匹配的项目
  troubleshootingItems.value.forEach(item => {
    if (matchedItemIds.has(item.id)) {
      item.isExpanded = true
    }
  })

  // 等待 DOM 更新后再进行高亮
  nextTick(() => {
    const walker = document.createTreeWalker(
      contentDiv,
      NodeFilter.SHOW_TEXT,
      null,
      false
    )

    const textNodes: Text[] = []
    let node: Node | null
    while ((node = walker.nextNode())) {
      if (node.textContent && node.textContent.toLowerCase().includes(query)) {
        textNodes.push(node as Text)
      }
    }

    // 从后往前处理，避免节点位置变化问题
    for (let i = textNodes.length - 1; i >= 0; i--) {
      const textNode = textNodes[i]
      const text = textNode.textContent || ''
      const lowerText = text.toLowerCase()
      const indices: number[] = []

      let index = lowerText.indexOf(query)
      while (index !== -1) {
        indices.push(index)
        index = lowerText.indexOf(query, index + 1)
      }

      // 从后往前替换，避免位置偏移
      for (let j = indices.length - 1; j >= 0; j--) {
        const matchIndex = indices[j]
        const matchText = text.substring(matchIndex, matchIndex + query.length)

        const beforeText = text.substring(0, matchIndex)
        const afterText = text.substring(matchIndex + query.length)

        const mark = document.createElement('mark')
        mark.className = 'search-highlight'
        mark.textContent = matchText

        const parent = textNode.parentNode
        if (parent) {
          if (afterText) {
            parent.insertBefore(document.createTextNode(afterText), textNode.nextSibling)
          }
          parent.insertBefore(mark, textNode.nextSibling)
          if (beforeText) {
            parent.insertBefore(document.createTextNode(beforeText), textNode.nextSibling)
          }
          parent.removeChild(textNode)
          highlightMarks.unshift(mark)
        }
      }
    }

    matchCount.value = highlightMarks.length
    currentMatchIndex.value = 0

    if (matchCount.value > 0) {
      scrollToMatch(0)
    }
  })
}

/**
 * 清除所有高亮
 */
function clearHighlights() {
  // 移除当前高亮样式
  highlightMarks.forEach(mark => {
    mark.classList.remove('current-match')
  })

  // 恢复原始文本节点
  const contentDiv = document.querySelector('.troubleshooting-list') as HTMLElement
  if (!contentDiv) return

  // 找到所有高亮标记并替换为文本节点
  const marks = contentDiv.querySelectorAll('mark.search-highlight')
  marks.forEach(mark => {
    const parent = mark.parentNode
    if (parent) {
      parent.replaceChild(document.createTextNode(mark.textContent || ''), mark)
      parent.normalize() // 合并相邻的文本节点
    }
  })

  highlightMarks.length = 0
}

/**
 * 清除搜索
 */
function clearSearch() {
  searchQuery.value = ''
  clearHighlights()
  matchCount.value = 0
  currentMatchIndex.value = 0
}

/**
 * 跳转到上一个匹配项
 */
function navigateToPrev() {
  if (matchCount.value === 0) return
  currentMatchIndex.value = (currentMatchIndex.value - 1 + matchCount.value) % matchCount.value
  scrollToMatch(currentMatchIndex.value)
}

/**
 * 跳转到下一个匹配项
 */
function navigateToNext() {
  if (matchCount.value === 0) return
  currentMatchIndex.value = (currentMatchIndex.value + 1) % matchCount.value
  scrollToMatch(currentMatchIndex.value)
}

/**
 * 滚动到指定匹配项
 */
function scrollToMatch(index: number) {
  if (index < 0 || index >= highlightMarks.length) return

  // 移除之前的高亮
  highlightMarks.forEach(mark => {
    mark.classList.remove('current-match')
  })

  // 添加当前高亮
  const currentMark = highlightMarks[index]
  currentMark.classList.add('current-match')

  // 滚动到可视区域
  currentMark.scrollIntoView({
    behavior: 'smooth',
    block: 'center'
  })
}
</script>

<style scoped>
.troubleshooting-page {
  height: 100%;
  display: flex;
  flex-direction: column;
}

/* 页面头部 */
.page-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 24px;
  background: transparent;
  border-bottom: 1px solid var(--steam-border);
}

.page-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

/* 搜索框区域 */
.search-box {
  display: flex;
  align-items: center;
  gap: 12px;
}

.search-input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  width: 280px;
}

.search-icon {
  position: absolute;
  left: 12px;
  width: 18px;
  height: 18px;
  color: var(--steam-text-muted);
  pointer-events: none;
}

.search-input {
  width: 100%;
  height: 36px;
  padding: 0 36px 0 38px;
  font-size: 14px;
  color: var(--steam-text-primary);
  background: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 6px;
  outline: none;
  transition: all 0.15s ease;
}

.search-input::placeholder {
  color: var(--steam-text-muted);
}

.search-input:focus {
  border-color: var(--steam-accent-blue);
  background: var(--steam-bg-primary);
}

.search-clear-btn {
  position: absolute;
  right: 8px;
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--steam-text-muted);
  background: transparent;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.search-clear-btn:hover {
  color: var(--steam-text-primary);
  background: var(--steam-bg-tertiary);
}

.search-clear-btn svg {
  width: 16px;
  height: 16px;
}

/* 搜索导航 */
.search-nav {
  display: flex;
  align-items: center;
  gap: 4px;
}

.match-count {
  font-size: 13px;
  color: var(--steam-text-secondary);
  min-width: 60px;
  text-align: center;
  user-select: none;
}

.match-count.no-match {
  color: var(--steam-error);
}

.nav-btn {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--steam-text-secondary);
  background: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.nav-btn:hover {
  color: var(--steam-text-primary);
  background: var(--steam-bg-tertiary);
  border-color: var(--steam-border-light);
}

.nav-btn svg {
  width: 18px;
  height: 18px;
}

/* 内容区域 */
.content-wrapper {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

/* 疑难解答列表 */
.troubleshooting-list {
  width: 80%;
  max-width: 1200px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

/* 单个问题项 */
.troubleshooting-item {
  background: transparent;
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  overflow: hidden;
}

/* 问题 */
.question {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px 20px;
  background: rgba(23, 26, 33, 0.3);
  border-bottom: 1px solid transparent;
  cursor: pointer;
  transition: all 0.2s ease;
  user-select: none;
}

.question:hover {
  background: rgba(23, 26, 33, 0.5);
}

.troubleshooting-item.expanded .question {
  border-bottom-color: var(--steam-border);
}

.question .icon {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--steam-accent-blue);
  color: white;
  font-size: 14px;
  font-weight: 600;
  border-radius: 50%;
  flex-shrink: 0;
}

.question .text {
  flex: 1;
  font-size: 15px;
  font-weight: 500;
  color: var(--steam-text-primary);
}

/* 展开/折叠箭头 */
.expand-icon {
  width: 24px;
  height: 24px;
  color: var(--steam-text-muted);
  transition: transform 0.2s ease;
  flex-shrink: 0;
}

.expand-icon.expanded {
  transform: rotate(180deg);
}

/* 答案 */
.answer {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 16px 20px;
  background: rgba(23, 26, 33, 0.2);
}

.answer .icon {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--steam-success);
  color: white;
  font-size: 14px;
  font-weight: 600;
  border-radius: 50%;
  flex-shrink: 0;
}

.answer .text {
  flex: 1;
  font-size: 14px;
  line-height: 1.8;
  color: var(--steam-text-primary);
}

.answer .text p {
  margin: 0 0 8px 0;
}

.answer .text p:last-child {
  margin-bottom: 0;
}

.answer .text code {
  padding: 2px 6px;
  background: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border);
  border-radius: 4px;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
  font-size: 12px;
  color: var(--steam-text-primary);
}

.answer .text a {
  color: var(--steam-accent-blue);
  text-decoration: none;
}

.answer .text a:hover {
  text-decoration: underline;
}

/* 滚动条样式 */
.content-wrapper::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

.content-wrapper::-webkit-scrollbar-track {
  background: transparent;
}

.content-wrapper::-webkit-scrollbar-thumb {
  background: var(--steam-border);
  border-radius: 4px;
}

.content-wrapper::-webkit-scrollbar-thumb:hover {
  background: var(--steam-border-light);
}

/* 搜索高亮样式 */
.search-highlight {
  background-color: rgba(255, 193, 7, 0.4);
  color: var(--steam-text-primary);
  border-radius: 2px;
  padding: 1px 2px;
}

.search-highlight.current-match {
  background-color: rgba(255, 152, 0, 0.7);
  box-shadow: 0 0 0 2px rgba(255, 152, 0, 0.5);
}
</style>
