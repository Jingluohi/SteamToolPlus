<template>
  <!--
    PalettePreview.vue - 超级调色盘实时预览窗口
    渲染一个简化的程序界面缩略图，自动使用当前 CSS 变量
  -->
  <div class="palette-preview">
    <div class="preview-window">
      <!-- 模拟标题栏 -->
      <div class="preview-titlebar">
        <div class="preview-titlebar-dots">
          <span class="dot red"></span>
          <span class="dot yellow"></span>
          <span class="dot green"></span>
        </div>
        <span class="preview-title">Steam Tool Plus</span>
      </div>

      <!-- 模拟主体内容 -->
      <div class="preview-body">
        <div class="preview-sidebar">
          <div class="preview-sidebar-item active"></div>
          <div class="preview-sidebar-item"></div>
          <div class="preview-sidebar-item"></div>
        </div>
        <div class="preview-content">
          <div class="preview-card">
            <div class="preview-card-image"></div>
            <div class="preview-card-info">
              <div class="preview-card-title"></div>
              <div class="preview-card-subtitle"></div>
            </div>
          </div>
          <div class="preview-card">
            <div class="preview-card-image"></div>
            <div class="preview-card-info">
              <div class="preview-card-title"></div>
              <div class="preview-card-subtitle"></div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="preview-caption">实时预览</div>
  </div>
</template>

<script setup lang="ts">
/**
 * PalettePreview.vue - 实时预览窗口
 * 组件本身不维护状态，颜色完全由外层 CSS 变量驱动
 */
</script>

<style scoped>
.palette-preview {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 16px;
  border-radius: var(--radius-lg);
  background: rgba(var(--steam-bg-primary-rgb), 0.4);
  border: 1px solid var(--steam-border);
}

.preview-window {
  width: 100%;
  max-width: 280px;
  aspect-ratio: 16 / 10;
  border-radius: var(--radius-md);
  overflow: hidden;
  box-shadow: var(--shadow-md);
  display: flex;
  flex-direction: column;
  background: var(--steam-bg-primary);
  border: 1px solid var(--steam-border);
}

.preview-titlebar {
  height: 24px;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0 10px;
  background: var(--steam-bg-secondary);
  border-bottom: 1px solid var(--steam-border);
}

.preview-titlebar-dots {
  display: flex;
  gap: 4px;
}

.dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
}

.dot.red { background: #ff5f56; }
.dot.yellow { background: #ffbd2e; }
.dot.green { background: #27c93f; }

.preview-title {
  font-size: 10px;
  color: var(--steam-text-muted);
  margin-left: auto;
}

.preview-body {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.preview-sidebar {
  width: 36px;
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 8px 6px;
  background: var(--steam-bg-secondary);
  border-right: 1px solid var(--steam-border);
}

.preview-sidebar-item {
  width: 100%;
  height: 6px;
  border-radius: 3px;
  background: var(--steam-bg-tertiary);
}

.preview-sidebar-item.active {
  background: var(--steam-accent-blue);
}

.preview-content {
  flex: 1;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
  padding: 8px;
  overflow-y: auto;
}

.preview-card {
  border-radius: var(--radius-sm);
  overflow: hidden;
  background: var(--steam-bg-card);
  border: 1px solid var(--steam-border);
  display: flex;
  flex-direction: column;
}

.preview-card-image {
  aspect-ratio: 16 / 9;
  background: linear-gradient(135deg, var(--steam-accent-blue), var(--steam-accent-purple));
}

.preview-card-info {
  padding: 6px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.preview-card-title {
  height: 6px;
  width: 70%;
  border-radius: 3px;
  background: var(--steam-text-primary);
}

.preview-card-subtitle {
  height: 4px;
  width: 50%;
  border-radius: 2px;
  background: var(--steam-text-muted);
}

.preview-caption {
  font-size: 12px;
  color: var(--steam-text-muted);
}
</style>
