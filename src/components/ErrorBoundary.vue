<template>
  <!-- 如果没有错误，显示子组件 -->
  <slot v-if="!hasError"></slot>
  
  <!-- 如果有错误，显示错误界面 -->
  <div v-else class="error-boundary">
    <div class="error-container">
      <svg class="error-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="10"/>
        <line x1="12" y1="8" x2="12" y2="12"/>
        <line x1="12" y1="16" x2="12.01" y2="16"/>
      </svg>
      <h2 class="error-title">程序出现错误</h2>
      <p class="error-message">{{ errorMessage }}</p>
      <div class="error-actions">
        <button class="error-btn primary" @click="reload">重新加载</button>
        <button class="error-btn secondary" @click="reset">重置状态</button>
      </div>
      <details class="error-details" v-if="errorDetails">
        <summary>查看详细信息</summary>
        <pre class="error-stack">{{ errorDetails }}</pre>
      </details>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onErrorCaptured } from 'vue'

// 错误状态
const hasError = ref(false)
const errorMessage = ref('')
const errorDetails = ref('')

// 捕获错误
onErrorCaptured((err, instance, info) => {
  hasError.value = true
  errorMessage.value = err instanceof Error ? err.message : String(err)
  errorDetails.value = err instanceof Error ? err.stack || '' : ''
  
  // 记录到控制台
  console.error('ErrorBoundary 捕获到错误:', err)
  console.error('组件:', instance)
  console.error('信息:', info)
  
  // 阻止错误继续传播
  return false
})

// 重新加载页面
const reload = () => {
  window.location.reload()
}

// 重置错误状态（尝试继续）
const reset = () => {
  hasError.value = false
  errorMessage.value = ''
  errorDetails.value = ''
}
</script>

<style scoped>
.error-boundary {
  width: 100%;
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--bg-primary);
  padding: 20px;
}

.error-container {
  max-width: 500px;
  text-align: center;
  padding: 40px;
  background-color: var(--bg-secondary);
  border-radius: 12px;
  border: 1px solid var(--border-color);
}

.error-icon {
  width: 64px;
  height: 64px;
  color: #ef4444;
  margin-bottom: 20px;
}

.error-title {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 12px 0;
}

.error-message {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0 0 24px 0;
  line-height: 1.5;
}

.error-actions {
  display: flex;
  gap: 12px;
  justify-content: center;
  margin-bottom: 20px;
}

.error-btn {
  padding: 10px 20px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  border: none;
}

.error-btn.primary {
  background-color: var(--accent-color);
  color: white;
}

.error-btn.primary:hover {
  opacity: 0.9;
}

.error-btn.secondary {
  background-color: var(--bg-surface);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.error-btn.secondary:hover {
  background-color: var(--border-color);
}

.error-details {
  text-align: left;
  margin-top: 20px;
}

.error-details summary {
  font-size: 12px;
  color: var(--text-secondary);
  cursor: pointer;
  user-select: none;
}

.error-stack {
  margin-top: 10px;
  padding: 12px;
  background-color: var(--bg-primary);
  border-radius: 6px;
  font-size: 11px;
  color: var(--text-secondary);
  overflow-x: auto;
  white-space: pre-wrap;
  word-break: break-all;
  max-height: 200px;
  overflow-y: auto;
}
</style>
