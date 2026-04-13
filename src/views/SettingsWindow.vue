<template>
  <div class="settings-window" :class="{ 'dark': isDark }">
    <!-- 标题栏 -->
    <header class="settings-header" data-tauri-drag-region="true">
      <h1 class="settings-title">设置</h1>
      <button class="close-btn" @click="closeWindow" title="关闭" data-tauri-drag-region="false">
        <svg viewBox="0 0 24 24" fill="currentColor">
          <path d="M18.3 5.7a1 1 0 0 0-1.4 0L12 10.6 7.1 5.7a1 1 0 0 0-1.4 1.4L10.6 12l-4.9 4.9a1 1 0 0 0 1.4 1.4L12 13.4l4.9 4.9a1 1 0 0 0 1.4-1.4L13.4 12l4.9-4.9a1 1 0 0 0 0-1.4z"/>
        </svg>
      </button>
    </header>

    <!-- 设置内容 -->
    <main class="settings-content">
      <div class="settings-section">
        <h2 class="section-title">下载设置</h2>
        <div class="setting-item">
          <span class="setting-label">默认下载路径</span>
          <div class="path-selector">
            <input
              type="text"
              v-model="settings.downloadPath"
              class="path-input"
              readonly
              placeholder="自动检测中..."
            />
            <button class="browse-btn" @click="selectDownloadPath">浏览</button>
          </div>

        </div>
      </div>

      <div class="settings-section">
        <h2 class="section-title">关于</h2>
        <div class="about-info">
          <p class="app-name">Steam Tool Plus</p>
          <p class="app-version">版本 v1.05</p>
          <p class="app-author">
            作者：<a href="#" class="author-link" @click.prevent="openBilibili">B站：鲸落_hi</a>
          </p>
          <p class="app-github">
            <a href="#" class="github-link" @click.prevent="openGithub">
              <svg class="github-icon" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
              </svg>
              <span>GitHub 仓库</span>
            </a>
          </p>
          <p class="app-license">本软件为免费开源软件，严禁任何形式的售卖行为</p>
          <p class="app-copyright">© 2026 Steam Tool Plus</p>
        </div>
      </div>
    </main>

    <!-- 底部按钮 -->
    <footer class="settings-footer">
      <div class="footer-left">
        <button class="btn btn-secondary" @click="resetSettings">恢复默认</button>
      </div>
      <div class="footer-right">
        <button class="btn btn-secondary" @click="closeWindow">取消</button>
        <button class="btn btn-primary" @click="saveSettings">保存</button>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'

// 主题状态 - 从 localStorage 读取
const isDark = computed(() => {
  const theme = localStorage.getItem('theme') || 'auto'
  if (theme === 'auto') {
    return window.matchMedia('(prefers-color-scheme: dark)').matches
  }
  return theme === 'dark'
})

// 设置数据
const settings = ref({
  minimizeToTray: false,
  downloadPath: ''
})

// 关闭窗口
const closeWindow = async () => {
  const window = getCurrentWindow()
  await window.close()
}

// 打开B站作者主页
const openBilibili = async () => {
  try {
    await invoke('open_external_link', { url: 'https://space.bilibili.com/3546728913519292' })
  } catch (error) {
    console.error('打开链接失败:', error)
    // 如果调用失败，尝试使用浏览器打开
    window.open('https://space.bilibili.com/3546728913519292', '_blank')
  }
}

// 打开GitHub仓库
const openGithub = async () => {
  try {
    await invoke('open_external_link', { url: 'https://github.com/Jingluohi/SteamToolPlus' })
  } catch (error) {
    console.error('打开链接失败:', error)
    // 如果调用失败，尝试使用浏览器打开
    window.open('https://github.com/Jingluohi/SteamToolPlus', '_blank')
  }
}

// 选择下载路径
const selectDownloadPath = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择下载路径'
    })
    if (selected) {
      settings.value.downloadPath = selected as string
    }
  } catch (error) {
    console.error('选择路径失败:', error)
  }
}

// 保存设置
const saveSettings = async () => {
  // 保存下载路径到 localStorage
  if (settings.value.downloadPath) {
    localStorage.setItem('customDownloadPath', settings.value.downloadPath)
    console.log('保存自定义下载路径:', settings.value.downloadPath)
  } else {
    localStorage.removeItem('customDownloadPath')
    console.log('清除自定义下载路径，使用默认路径')
  }
  
  // 保存其他设置
  localStorage.setItem('minimizeToTray', JSON.stringify(settings.value.minimizeToTray))
  
  console.log('保存设置:', settings.value)
  await closeWindow()
}

// 恢复默认设置
const resetSettings = () => {
  settings.value = {
    minimizeToTray: false,
    downloadPath: ''
  }
}

// 加载设置
onMounted(() => {
  // 从 localStorage 加载设置
  const savedMinimizeToTray = localStorage.getItem('minimizeToTray')
  if (savedMinimizeToTray) {
    settings.value.minimizeToTray = JSON.parse(savedMinimizeToTray)
  }
  
  const savedDownloadPath = localStorage.getItem('customDownloadPath')
  if (savedDownloadPath) {
    settings.value.downloadPath = savedDownloadPath
  }
  
  console.log('加载设置完成:', settings.value)
})
</script>

<style>
/* 全局样式 - 确保独立窗口也有正确的 CSS 变量 */
:root {
  /* 浅色主题默认值 */
  --bg-primary: #f8fafc;
  --bg-secondary: #f1f5f9;
  --bg-surface: #e2e8f0;
  --text-primary: #1e293b;
  --text-secondary: #64748b;
  --accent-color: #3b82f6;
  --accent-hover: #2563eb;
  --border-color: #cbd5e1;
  --card-border: #e2e8f0;
}

.dark {
  --bg-primary: #16213e;
  --bg-secondary: #1a1a2e;
  --bg-surface: #0f3460;
  --text-primary: #ffffff;
  --text-secondary: #a0a0a0;
  --accent-color: #3b82f6;
  --accent-hover: #60a5fa;
  --border-color: #1e3a5f;
  --card-border: #1e3a5f;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  height: 100%;
  width: 100%;
  overflow: hidden;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>

<style scoped>
/* 设置窗口容器 */
.settings-window {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100vh;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  overflow: hidden;
}

/* 标题栏 */
.settings-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 40px;
  padding: 0 16px;
  background-color: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
}

.settings-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.close-btn {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: background-color 0.15s ease, color 0.15s ease;
}

.close-btn:hover {
  background-color: #e11d48;
  color: #ffffff;
}

.close-btn svg {
  width: 14px;
  height: 14px;
}

/* 设置内容区域 */
.settings-content {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
}

.settings-section {
  margin-bottom: 24px;
}

.section-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin: 0 0 12px 0;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--border-color);
}

/* 设置项 */
.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 0;
  border-bottom: 1px solid var(--border-color);
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-label {
  font-size: 14px;
  color: var(--text-primary);
}

/* 切换开关 */
.toggle-switch {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 24px;
  cursor: pointer;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: var(--bg-surface);
  border-radius: 24px;
  transition: background-color 0.2s ease;
}

.toggle-slider::before {
  content: '';
  position: absolute;
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  border-radius: 50%;
  transition: transform 0.2s ease;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}

.toggle-switch input:checked + .toggle-slider {
  background-color: var(--accent-color);
}

.toggle-switch input:checked + .toggle-slider::before {
  transform: translateX(20px);
}

/* 路径选择器 */
.path-selector {
  display: flex;
  align-items: center;
  gap: 8px;
}

.path-input {
  width: 200px;
  height: 32px;
  padding: 0 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
}

.path-input::placeholder {
  color: var(--text-secondary);
}

.browse-btn {
  height: 32px;
  padding: 0 16px;
  border: none;
  border-radius: 6px;
  background-color: var(--bg-surface);
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.browse-btn:hover {
  background-color: var(--border-color);
}

/* 下拉选择框 */
.setting-select {
  height: 32px;
  padding: 0 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
  cursor: pointer;
}

/* 关于信息 */
.about-info {
  text-align: center;
  padding: 20px 0;
}

.app-name {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 4px 0;
}

.app-version {
  font-size: 13px;
  color: var(--text-secondary);
  margin: 0 0 4px 0;
}

.app-copyright {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 0;
}

.app-author {
  font-size: 13px;
  color: var(--text-secondary);
  margin: 0 0 4px 0;
}

.author-link {
  color: var(--accent-color);
  text-decoration: none;
  cursor: pointer;
  transition: opacity 0.15s ease;
}

.author-link:hover {
  opacity: 0.8;
  text-decoration: underline;
}

.app-license {
  font-size: 12px;
  color: #e11d48;
  margin: 8px 0 4px 0;
  font-weight: 500;
}

.app-github {
  margin: 8px 0;
}

.github-link {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background-color: var(--bg-surface);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  text-decoration: none;
  font-size: 13px;
  cursor: pointer;
  transition: background-color 0.15s ease, border-color 0.15s ease;
}

.github-link:hover {
  background-color: var(--border-color);
  border-color: var(--text-secondary);
}

.github-icon {
  width: 16px;
  height: 16px;
}

/* 底部按钮 */
.settings-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  background-color: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
}

.footer-left {
  display: flex;
  align-items: center;
}

.footer-right {
  display: flex;
  gap: 8px;
}

.btn {
  height: 36px;
  padding: 0 20px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.15s ease, opacity 0.15s ease;
}

.btn-primary {
  background-color: var(--accent-color);
  color: white;
}

.btn-primary:hover {
  opacity: 0.9;
}

.btn-secondary {
  background-color: var(--bg-surface);
  color: var(--text-primary);
}

.btn-secondary:hover {
  background-color: var(--border-color);
}
</style>
