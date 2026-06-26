/**
 * Vite 环境类型声明
 * 声明 CSS 模块、静态资源等类型，供 TypeScript 识别
 */

/// <reference types="vite/client" />

declare module '*.css' {
  const content: string
  export default content
}
