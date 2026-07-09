/**
 * 帮助相关IPC调用封装
 * 统一处理帮助文档相关的后端通信
 */

import { invoke } from '@tauri-apps/api/core'

/**
 * 读取使用说明文档内容
 * @returns README.md 文件内容
 */
export async function readReadmeFile(): Promise<string> {
  return invoke<string>('read_readme_file')
}
