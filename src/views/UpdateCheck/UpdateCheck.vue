<template>
  <!--
    UpdateCheck.vue - 检测更新页面
    显示最新版本下载链接
  -->
  <div class="update-check-page">
    <div class="update-content">


      <!-- 按钮区域 -->
      <div class="button-section">
        <Button variant="primary" size="lg" @click="openDownloadLink">
          查看百度网盘
        </Button>
        <p class="extract-code">提取码：v1xm</p>
        <Button variant="primary" size="lg" @click="openXunleiLink">
          查看迅雷网盘
        </Button>
        <p class="extract-code">提取码：2tmn</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * UpdateCheck.vue - 检测更新页面
 * 显示百度网盘下载链接
 */

import { open } from '@tauri-apps/plugin-shell'
import Button from '../../components/common/Button.vue'

/**
 * 打开百度网盘下载链接
 */
async function openDownloadLink() {
  try {
    await open('https://pan.baidu.com/s/1XbcZOLQcn4500z-SL1RDug?pwd=v1xm')
  } catch (error) {
    // 打开链接失败时静默处理
  }
}

/**
 * 打开迅雷网盘下载链接
 */
async function openXunleiLink() {
  try {
    await open('https://pan.xunlei.com/s/VOrkq4Tq0c0Sootmhpp4433yA1?pwd=2tmn#')
  } catch (error) {
    // 打开链接失败时静默处理
  }
}
</script>

<style scoped>
.update-check-page {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
}

.update-content {
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
