/**
 * Vue Router配置
 * 定义所有页面路由
 */

import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

/**
 * 路由配置列表
 */
const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Browse',
    component: () => import('./views/Browse/Browse.vue'),
    meta: { title: '浏览' }
  },
  {
    path: '/library',
    name: 'Library',
    component: () => import('./views/Library/Library.vue'),
    meta: { title: '库' }
  },
  {
    path: '/game/:id',
    name: 'GameDetail',
    component: () => import('./views/GameDetail/GameDetail.vue'),
    meta: { title: '游戏详情' }
  },
  {
    path: '/download',
    name: 'Download',
    component: () => import('./views/Download/Download.vue'),
    meta: { title: '本体下载' }
  },
  {
    path: '/patch',
    name: 'Patch',
    component: () => import('./views/Patch/Patch.vue'),
    meta: { title: '免Steam补丁' }
  },
  {
    path: '/settings',
    name: 'GlobalSettings',
    component: () => import('./views/GlobalSettings/GlobalSettings.vue'),
    meta: { title: '全局设置' }
  },
  {
    path: '/extensions',
    name: 'ExtensionManager',
    component: () => import('./views/ExtensionManager/ExtensionManager.vue'),
    meta: { title: '管理扩展' }
  },
  {
    path: '/about',
    name: 'About',
    component: () => import('./views/About/About.vue'),
    meta: { title: '关于软件' }
  },
  {
    path: '/help',
    name: 'Help',
    component: () => import('./views/Help/Help.vue'),
    meta: { title: '使用说明' }
  },
  {
    path: '/help-window',
    name: 'HelpWindow',
    component: () => import('./views/HelpWindow/HelpWindow.vue'),
    meta: { title: '使用说明' }
  },
  {
    path: '/update-check',
    name: 'UpdateCheck',
    component: () => import('./views/UpdateCheck/UpdateCheck.vue'),
    meta: { title: '检测更新' }
  }
]

/**
 * 创建路由器实例
 * 配置滚动行为，记住页面滚动位置
 */
const router = createRouter({
  history: createWebHistory(),
  routes,
  scrollBehavior(to, from, savedPosition) {
    // 如果有保存的滚动位置（如点击浏览器后退按钮），则恢复到该位置
    if (savedPosition) {
      return savedPosition
    }
    // 否则滚动到顶部
    return { top: 0 }
  }
})

export default router
