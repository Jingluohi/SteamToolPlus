/**
 * noBrowserModal.ts - 「未设置默认浏览器」提示弹窗单例状态
 *
 * 使用模块级 ref 保存状态，保证全局只有一份：
 * 弹窗组件在 App.vue 中只挂载一次，任意页面触发时共用一个弹窗实例，
 * 避免在每个页面里重复实现弹窗逻辑。
 */

import { ref } from 'vue'

/** 弹窗是否可见（模块级单例，跨组件共享） */
const visible = ref(false)

/**
 * 使用全局「未设置默认浏览器」提示弹窗
 * 返回弹窗的显示状态与控制方法
 */
export function useNoBrowserModal() {
  /** 显示提示弹窗 */
  function show() {
    visible.value = true
  }

  /** 关闭提示弹窗 */
  function hide() {
    visible.value = false
  }

  return { visible, show, hide }
}