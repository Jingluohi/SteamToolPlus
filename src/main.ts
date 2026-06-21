/**
 * 应用程序入口文件
 * 负责依赖注入、store初始化、应用挂载
 */

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import { setupGlobalErrorHandler } from './utils/logger'

// 导入全局样式
import './styles/main.css'

/**
 * 设置全局错误捕获
 * 将运行时错误记录到日志文件
 */
setupGlobalErrorHandler()

/**
 * 创建Vue应用实例
 */
const app = createApp(App)

/**
 * 注册Pinia状态管理
 */
const pinia = createPinia()
app.use(pinia)

/**
 * 注册Vue Router
 */
app.use(router)

/**
 * 挂载应用到DOM
 */
app.mount('#app')
