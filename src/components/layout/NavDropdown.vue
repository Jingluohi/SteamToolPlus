<template>
  <!--
    NavDropdown.vue - 导航下拉菜单组件
    用于标题栏的导航菜单，支持 hover 触发和路由跳转
  -->
  <div
    class="nav-menu"
    @mouseenter="showMenu = true"
    @mouseleave="showMenu = false"
  >
    <button class="nav-menu-btn">
      <span>{{ title }}</span>
      <svg class="dropdown-icon" viewBox="0 0 24 24" fill="currentColor">
        <path d="M7 10l5 5 5-5z"/>
      </svg>
    </button>
    <!-- 下拉菜单 -->
    <Transition name="dropdown">
      <div v-show="showMenu" class="dropdown-menu">
        <RouterLink
          v-for="item in items"
          :key="item.path"
          :to="item.path"
          class="dropdown-item"
          @click="showMenu = false"
        >
          {{ item.name }}
        </RouterLink>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
/**
 * NavDropdown.vue - 导航下拉菜单组件
 * 提取自 TitleBar.vue，复用重复的菜单结构
 */

import { ref } from 'vue'

/**
 * 菜单项接口
 */
export interface NavMenuItem {
  /** 显示名称 */
  name: string
  /** 路由路径 */
  path: string
}

/**
 * 组件属性定义
 */
interface Props {
  /** 菜单标题 */
  title: string
  /** 菜单项列表 */
  items: NavMenuItem[]
}

defineProps<Props>()

// 菜单显示状态
const showMenu = ref(false)
</script>

<style scoped>
.nav-menu {
  position: relative;
}

.nav-menu-btn {
  display: flex;
  align-items: center;
  gap: 2px;
  padding: 0 8px;
  height: 32px;
  font-size: 16px;
  font-weight: 500;
  color: var(--steam-text-primary);
  background: rgba(0, 0, 0, 0.25);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 4px;
  transition: all var(--transition-fast);
}

.nav-menu-btn:hover {
  background: rgba(0, 0, 0, 0.4);
  border-color: rgba(255, 255, 255, 0.15);
  transform: translateY(-1px);
}

/* 浅色模式下黄一点 */
[data-theme="light"] .nav-menu-btn {
  background: rgba(251, 191, 36, 0.15);
  border-color: rgba(251, 191, 36, 0.25);
  color: #8b6914;
}

[data-theme="light"] .nav-menu-btn:hover {
  background: rgba(251, 191, 36, 0.25);
  border-color: rgba(251, 191, 36, 0.4);
}

.dropdown-icon {
  width: 16px;
  height: 16px;
  opacity: 0.7;
}

/* 下拉菜单 */
.dropdown-menu {
  position: absolute;
  top: 100%;
  left: 0;
  width: 150px;
  min-width: 150px;
  background: var(--steam-bg-secondary);
  border: 1px solid var(--steam-border);
  border-radius: 8px;
  box-shadow: var(--shadow-steam);
  padding: 8px 0;
  z-index: 1000;
  margin-top: 4px;
}

.dropdown-item {
  display: block;
  width: 100%;
  padding: 10px 16px;
  font-size: 14px;
  color: var(--steam-text-primary);
  background: transparent;
  border: none;
  text-align: left;
  cursor: pointer;
  transition: background var(--transition-fast);
  text-decoration: none;
}

.dropdown-item:hover {
  background: var(--steam-accent-hover);
}

/* 下拉动画 */
.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.2s ease-out;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}
</style>
