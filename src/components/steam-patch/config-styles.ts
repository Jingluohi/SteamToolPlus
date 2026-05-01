/**
 * config-styles.ts - Steam补丁配置组件共享样式
 * 集中管理所有配置组件通用的CSS类名和样式常量
 * 避免在每个组件中重复定义相同样式
 */

/**
 * 配置组件通用的CSS类名集合
 * 使用这些类名可以确保所有配置组件样式一致
 */
export const configClasses = {
  // 配置组容器
  configGroup: 'config-group',
  // 配置标签
  configLabel: 'config-label',
  // 配置描述文本
  configDesc: 'config-desc',
  // 配置输入框
  configInput: 'config-input',
  // 配置文本域
  configTextarea: 'config-textarea',
  // 切换开关标签
  toggleLabel: 'toggle-label',
  // 切换开关输入（隐藏）
  toggleInput: 'toggle-input',
  // 切换开关滑块
  toggleSlider: 'toggle-slider',
  // 切换开关文本
  toggleText: 'toggle-text',
  // 复选框标签
  checkboxLabel: 'checkbox-label',
  // IP列表容器
  ipList: 'ip-list',
  // IP列表项
  ipItem: 'ip-item',
  // 添加按钮
  addBtn: 'add-btn',
  // 移除按钮
  removeBtn: 'remove-btn',
  // 单选按钮组
  radioGroup: 'radio-group',
  // 单选标签
  radioLabel: 'radio-label',
  // 白名单区域
  whitelistSection: 'whitelist-section'
} as const

/**
 * 配置类型与对应的颜色映射
 * 用于动态设置配置组件的图标颜色
 */
export const configTypeColors: Record<string, string> = {
  main: '#64748b',        // 灰色 - 主配置
  achievements: '#f59e0b', // 琥珀色 - 成就
  items: '#10b981',       // 翠绿色 - 物品
  dlc: '#a855f7',         // 紫色 - DLC
  leaderboards: '#ec4899', // 粉色 - 排行榜
  overlay: '#8b5cf6',     // 紫罗兰 - 覆盖层
  controller: '#ef4444',  // 红色 - 控制器
  lan: '#3b82f6',         // 蓝色 - 局域网
  stats: '#0ea5e9',       // 天蓝色 - 统计
  user: '#6366f1'         // 靛蓝色 - 用户
} as const

/**
 * 配置类型与对应的标题映射
 */
export const configTypeTitles: Record<string, string> = {
  main: '主配置',
  achievements: '成就系统配置',
  items: '物品与库存配置',
  dlc: 'DLC 与 Depot 配置',
  leaderboards: '排行榜配置',
  overlay: '游戏内 Overlay 配置',
  controller: '控制器支持配置',
  lan: '局域网联机配置',
  stats: '游戏统计配置',
  user: '用户配置'
} as const

/**
 * 生成配置组件通用的CSS样式字符串
 * 可注入到组件的style标签中
 */
export function generateConfigStyles(): string {
  return `
    /* 配置组容器 */
    .config-group {
      margin-bottom: 20px;
    }

    /* 配置标签 */
    .config-label {
      display: block;
      font-size: 14px;
      font-weight: 600;
      color: var(--steam-text-primary);
      margin-bottom: 8px;
    }

    /* 配置描述文本 */
    .config-desc {
      font-size: 12px;
      color: var(--steam-text-secondary);
      margin: 0 0 12px 0;
    }

    /* 配置输入框 */
    .config-input {
      width: 100%;
      padding: 10px 12px;
      border: 1px solid var(--steam-border);
      border-radius: 8px;
      background-color: var(--steam-bg-secondary);
      color: var(--steam-text-primary);
      font-size: 14px;
      outline: none;
      transition: border-color 0.15s ease;
    }

    .config-input:focus {
      border-color: var(--steam-accent-blue);
    }

    /* 配置文本域 */
    .config-textarea {
      width: 100%;
      padding: 10px 12px;
      border: 1px solid var(--steam-border);
      border-radius: 8px;
      background-color: var(--steam-bg-secondary);
      color: var(--steam-text-primary);
      font-size: 13px;
      font-family: 'Courier New', monospace;
      resize: vertical;
      outline: none;
      transition: border-color 0.15s ease;
    }

    .config-textarea:focus {
      border-color: var(--steam-accent-blue);
    }

    /* 切换开关标签 */
    .toggle-label {
      display: flex;
      align-items: center;
      gap: 12px;
      cursor: pointer;
    }

    /* 切换开关输入（隐藏原生checkbox） */
    .toggle-input {
      display: none;
    }

    /* 切换开关滑块 */
    .toggle-slider {
      width: 48px;
      height: 26px;
      background-color: var(--steam-border);
      border-radius: 13px;
      position: relative;
      transition: background-color 0.2s ease;
      flex-shrink: 0;
    }

    .toggle-slider::after {
      content: '';
      position: absolute;
      width: 22px;
      height: 22px;
      background-color: white;
      border-radius: 50%;
      top: 2px;
      left: 2px;
      transition: transform 0.2s ease;
    }

    .toggle-input:checked + .toggle-slider {
      background-color: var(--steam-accent-blue);
    }

    .toggle-input:checked + .toggle-slider::after {
      transform: translateX(22px);
    }

    /* 切换开关文本 */
    .toggle-text {
      font-size: 14px;
      color: var(--steam-text-primary);
    }

    /* 复选框标签 */
    .checkbox-label {
      display: flex;
      align-items: center;
      gap: 8px;
      cursor: pointer;
      font-size: 14px;
      color: var(--steam-text-primary);
    }

    /* IP列表容器 */
    .ip-list {
      display: flex;
      flex-direction: column;
      gap: 8px;
    }

    /* IP列表项 */
    .ip-item {
      display: flex;
      gap: 8px;
      align-items: center;
    }

    .ip-item .config-input {
      flex: 1;
    }

    /* 添加按钮 */
    .add-btn {
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 6px;
      padding: 10px;
      border: 1px dashed var(--steam-border);
      border-radius: 8px;
      background-color: transparent;
      color: var(--steam-accent-blue);
      font-size: 13px;
      font-weight: 500;
      cursor: pointer;
      transition: all 0.15s ease;
      width: 100%;
    }

    .add-btn:hover {
      border-color: var(--steam-accent-blue);
      background-color: rgba(59, 130, 246, 0.05);
    }

    .add-btn svg {
      width: 16px;
      height: 16px;
    }

    /* 移除按钮 */
    .remove-btn {
      width: 36px;
      height: 36px;
      border: none;
      border-radius: 8px;
      background-color: rgba(239, 68, 68, 0.1);
      color: #ef4444;
      cursor: pointer;
      display: flex;
      align-items: center;
      justify-content: center;
      transition: all 0.15s ease;
      flex-shrink: 0;
    }

    .remove-btn:hover {
      background-color: rgba(239, 68, 68, 0.2);
    }

    .remove-btn svg {
      width: 16px;
      height: 16px;
    }

    /* 单选按钮组 */
    .radio-group {
      display: flex;
      flex-direction: column;
      gap: 10px;
    }

    /* 单选标签 */
    .radio-label {
      display: flex;
      align-items: center;
      gap: 8px;
      cursor: pointer;
      font-size: 14px;
      color: var(--steam-text-primary);
    }

    .radio-label input[type="radio"] {
      width: 18px;
      height: 18px;
      accent-color: var(--steam-accent-blue);
    }

    /* 白名单区域 */
    .whitelist-section {
      margin-top: 12px;
      padding: 12px;
      background-color: var(--steam-bg-secondary);
      border-radius: 8px;
    }

    .whitelist-section .config-textarea {
      background-color: var(--steam-bg-primary);
    }
  `
}

/**
 * 通用的保存配置函数类型定义
 */
export type SaveConfigFunction<T> = (
  command: string,
  gamePath: string,
  config: T
) => Promise<{ success: boolean; message: string }>

/**
 * 通用的加载配置函数类型定义
 */
export type LoadConfigFunction<T> = (
  command: string,
  gamePath: string
) => Promise<{ exists: boolean; config?: T }>
