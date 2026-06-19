<template>
  <div class="video-tutorial-page">
    <div class="tutorial-content">
      <h1 class="page-title">视频教程</h1>
      <p class="page-subtitle">观看视频教程，快速上手 Steam Tool Plus</p>

      <!-- 按钮区域 -->
      <div class="button-section">
        <Button variant="primary" size="lg" @click="openXunleiLink">
          查看迅雷网盘
        </Button>
        <p class="extract-code">提取码：4a4p</p>
        <Button variant="primary" size="lg" @click="openQuarkLink">
          查看夸克网盘
        </Button>
        <p class="extract-code">提取码：无</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/plugin-shell'
import Button from '../../components/common/Button.vue'

/**
 * 打开迅雷网盘下载链接
 */
async function openXunleiLink() {
  try {
    await open('https://pan.xunlei.com/s/VOvTQBOnWZWdlDxvsE2z5ODFA1?pwd=4a4p')
  } catch (error) {
    // 打开链接失败时静默处理
  }
}

/**
 * 打开夸克网盘下载链接
 */
async function openQuarkLink() {
  try {
    await open('https://pan.quark.cn/s/6096f8c37ef5')
  } catch (error) {
    // 打开链接失败时静默处理
  }
}
</script>

<style scoped>
.video-tutorial-page {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
}

.tutorial-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 32px;
}

.page-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--steam-text-primary);
}

.page-subtitle {
  font-size: 14px;
  color: var(--steam-text-secondary);
  margin-top: -20px;
}

.button-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.extract-code {
  font-size: 14px;
  color: var(--steam-text-secondary);
  font-family: monospace;
}
</style>
