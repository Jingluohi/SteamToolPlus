<template>
  <!--
    About.vue - 关于软件页面
    显示软件信息、版本号、作者信息、GitHub链接等
  -->
  <div class="about-page">
    <div class="about-content">
      <!-- Logo区域 -->
      <div class="logo-section">
        <div class="app-logo">
          <img src="../../../src-tauri/icons/128x128.png" alt="Steam Tool Plus" />
        </div>
        <h1 class="app-name">Steam Tool Plus</h1>
        <p class="app-version">版本 v1.20</p>
      </div>

      <!-- 作者信息区域（来自旧版设置） -->
      <div class="author-section">
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
  </div>
</template>

<script setup lang="ts">
/**
 * About.vue - 关于软件页面
 * 显示软件信息、版本号、作者信息、GitHub链接等
 */

import { open } from '@tauri-apps/plugin-shell'

/**
 * 打开B站作者主页 - 使用系统默认浏览器
 */
async function openBilibili() {
  try {
    // 使用 Tauri shell 插件在默认浏览器中打开链接
    await open('https://space.bilibili.com/405707676')
  } catch (error) {
    // 打开链接失败时静默处理
  }
}

/**
 * 打开GitHub仓库 - 使用系统默认浏览器
 */
async function openGithub() {
  try {
    // 使用 Tauri shell 插件在默认浏览器中打开链接
    await open('https://github.com/Jingluohi/SteamToolPlus')
  } catch (error) {
    // 打开链接失败时静默处理
  }
}
</script>

<style scoped>
.about-page {
  height: 100%;
  overflow-y: auto;
  padding: 24px;
}

.about-content {
  max-width: 600px;
  margin: 0 auto;
  background: rgba(23, 26, 33, 0.3);
  backdrop-filter: blur(10px);
  border-radius: 12px;
  padding: 40px;
}

/* Logo区域 */
.logo-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 24px;
}

.app-logo {
  width: 80px;
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--steam-accent-blue);
  border-radius: 20px;
  color: white;
  margin-bottom: 16px;
  overflow: hidden;
}

.app-logo img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.app-name {
  font-size: 24px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin-bottom: 4px;
}

.app-version {
  font-size: 16px;
  color: var(--steam-text-secondary);
}

/* 作者信息区域（来自旧版设置） */
.author-section {
  text-align: center;
  padding-top: 16px;
  border-top: 1px solid var(--steam-border);
}

.app-author {
  font-size: 16px;
  color: var(--steam-text-secondary);
  margin: 0 0 12px 0;
}

.author-link {
  color: var(--steam-accent-blue);
  text-decoration: none;
  cursor: pointer;
  transition: opacity 0.15s ease;
}

.author-link:hover {
  opacity: 0.8;
  text-decoration: underline;
}

.app-license {
  font-size: 14px;
  color: #e11d48;
  margin: 12px 0 8px 0;
  font-weight: 500;
}

.app-copyright {
  font-size: 14px;
  color: var(--steam-text-secondary);
  margin: 8px 0 0 0;
}

.app-github {
  margin: 12px 0;
}

.github-link {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background-color: var(--steam-bg-tertiary);
  border: 1px solid var(--steam-border);
  border-radius: 6px;
  color: var(--steam-text-primary);
  text-decoration: none;
  font-size: 15px;
  cursor: pointer;
  transition: background-color 0.15s ease, border-color 0.15s ease;
}

.github-link:hover {
  background-color: var(--steam-border);
  border-color: var(--steam-text-secondary);
}

.github-icon {
  width: 16px;
  height: 16px;
}
</style>
