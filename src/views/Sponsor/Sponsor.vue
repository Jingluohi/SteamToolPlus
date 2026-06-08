<template>
  <!--
    Sponsor.vue - 赞助程序页面
    显示微信支付二维码，支持用户赞助开发者
    图片通过 Tauri asset 协议加载，打包进程序
  -->
  <div class="sponsor-page">
    <div class="sponsor-content">
      <h1 class="sponsor-title">赞助程序</h1>
      <p class="sponsor-desc">
        如果您觉得本程序对您有帮助，欢迎赞助支持开发者继续维护和更新！
      </p>
      <div class="qr-code-container">
        <img
          class="qr-code-img"
          :src="weixinImgUrl"
          alt="微信支付二维码"
        />
      </div>
      <p class="sponsor-tip">推荐使用微信支付</p>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * Sponsor.vue - 赞助程序页面
 * 展示微信支付二维码图片
 * 通过后端命令获取嵌入到 exe 中的图片 Base64 数据
 */
import { invoke } from '@tauri-apps/api/core'
import { ref, onMounted } from 'vue'

const weixinImgUrl = ref('')

onMounted(async () => {
  try {
    // 调用后端命令获取嵌入到 exe 中的图片 Base64 数据
    const base64Data = await invoke<string>('get_sponsor_image_base64')
    weixinImgUrl.value = base64Data
  } catch (e) {
    console.error('加载赞助图片失败:', e)
  }
})
</script>

<style scoped>
.sponsor-page {
  width: 100%;
  min-height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 40px 20px;
  box-sizing: border-box;
}

.sponsor-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 24px;
  max-width: 500px;
  width: 100%;
}

.sponsor-title {
  font-size: 28px;
  font-weight: 700;
  color: var(--steam-text-primary);
  margin: 0;
  text-align: center;
}

.sponsor-desc {
  font-size: 15px;
  color: var(--steam-text-secondary);
  text-align: center;
  line-height: 1.6;
  margin: 0;
}

.qr-code-container {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 20px;
  background-color: var(--steam-bg-secondary);
  border-radius: 16px;
  border: 1px solid var(--steam-border);
}

.qr-code-img {
  width: 300px;
  height: auto;
  border-radius: 8px;
  display: block;
}

.sponsor-tip {
  font-size: 14px;
  color: var(--steam-text-secondary);
  margin: 0;
  text-align: center;
}
</style>
