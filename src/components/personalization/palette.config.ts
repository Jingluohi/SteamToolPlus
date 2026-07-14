/**
 * 超级调色盘配置
 * 定义可调 CSS 变量与预设主题
 */

/** 单个可调颜色变量 */
export interface ColorVariable {
  /** CSS 变量名 */
  key: string
  /** 显示名称 */
  label: string
  /** 深色主题默认值 */
  defaultDark: string
  /** 浅色主题默认值 */
  defaultLight: string
}

/** 颜色变量分组 */
export interface ColorVariableGroup {
  /** 分组唯一标识 */
  id: string
  /** 分组显示名称 */
  title: string
  /** 分组内的变量列表 */
  variables: ColorVariable[]
}

/** 单个预设主题 */
export interface ColorPreset {
  /** 预设唯一标识 */
  id: string
  /** 预设显示名称 */
  name: string
  /** 该预设覆盖的 CSS 变量；空对象表示「自选颜色」 */
  colors: Record<string, string>
}

/** 预设主题分类 */
export interface PresetCategory {
  /** 分类唯一标识 */
  id: string
  /** 分类标题 */
  title: string
  /** 分类副标题 */
  subtitle: string
  /** 分类下的预设列表 */
  presets: ColorPreset[]
}

/**
 * 可调颜色变量分组
 * 与 src/styles/main.css 中的 CSS 变量一一对应
 */
export const COLOR_VARIABLE_GROUPS: ColorVariableGroup[] = [
  {
    id: 'background',
    title: '背景颜色',
    variables: [
      { key: '--steam-bg-primary', label: '主背景', defaultDark: '#363636', defaultLight: '#f8f9fa' },
      { key: '--steam-bg-secondary', label: '次背景', defaultDark: '#404040', defaultLight: '#ffffff' },
      { key: '--steam-bg-tertiary', label: '三级背景', defaultDark: '#4a4a4a', defaultLight: '#f1f3f5' },
      { key: '--steam-bg-quaternary', label: '四级背景', defaultDark: '#545454', defaultLight: '#e9ecef' },
      { key: '--steam-bg-card', label: '卡片背景', defaultDark: '#434343', defaultLight: '#ffffff' },
      { key: '--steam-bg-hover', label: '悬停背景', defaultDark: '#686868', defaultLight: '#f1f3f5' }
    ]
  },
  {
    id: 'text',
    title: '文字颜色',
    variables: [
      { key: '--steam-text-primary', label: '主文字', defaultDark: '#ffffff', defaultLight: '#000000' },
      { key: '--steam-text-secondary', label: '次要文字', defaultDark: '#d0d4da', defaultLight: '#333333' },
      { key: '--steam-text-muted', label: '弱化文字', defaultDark: '#9ca3af', defaultLight: '#666666' },
      { key: '--steam-text-subtle', label: '极弱文字', defaultDark: '#6b7280', defaultLight: '#999999' }
    ]
  },
  {
    id: 'accent',
    title: '强调颜色',
    variables: [
      { key: '--steam-accent-blue', label: '主题蓝', defaultDark: '#60a5fa', defaultLight: '#4a9eff' },
      { key: '--steam-accent-blue-hover', label: '主题蓝悬停', defaultDark: '#7cb8fc', defaultLight: '#6ab2ff' },
      { key: '--steam-accent-green', label: '主题绿', defaultDark: '#7cb342', defaultLight: '#5c7e10' },
      { key: '--steam-accent-green-hover', label: '主题绿悬停', defaultDark: '#9ccc65', defaultLight: '#7a9f2d' },
      { key: '--steam-accent-gold', label: '主题金', defaultDark: '#fbbf24', defaultLight: '#f59e0b' },
      { key: '--steam-accent-purple', label: '主题紫', defaultDark: '#a78bfa', defaultLight: '#8b5cf6' },
      { key: '--steam-accent-orange', label: '主题橙', defaultDark: '#fb923c', defaultLight: '#f97316' }
    ]
  },
  {
    id: 'state',
    title: '状态颜色',
    variables: [
      { key: '--steam-error', label: '错误', defaultDark: '#dc2626', defaultLight: '#dc2626' },
      { key: '--steam-error-hover', label: '错误悬停', defaultDark: '#ef4444', defaultLight: '#ef4444' },
      { key: '--steam-success', label: '成功', defaultDark: '#16a34a', defaultLight: '#16a34a' },
      { key: '--steam-success-hover', label: '成功悬停', defaultDark: '#22c55e', defaultLight: '#22c55e' },
      { key: '--steam-warning', label: '警告', defaultDark: '#d97706', defaultLight: '#d97706' },
      { key: '--steam-warning-hover', label: '警告悬停', defaultDark: '#f59e0b', defaultLight: '#f59e0b' }
    ]
  },
  {
    id: 'border',
    title: '边框颜色',
    variables: [
      { key: '--steam-border', label: '主边框', defaultDark: 'rgba(255, 255, 255, 0.1)', defaultLight: 'rgba(0, 0, 0, 0.08)' },
      { key: '--steam-border-light', label: '亮边框', defaultDark: 'rgba(255, 255, 255, 0.15)', defaultLight: 'rgba(0, 0, 0, 0.12)' },
      { key: '--steam-border-subtle', label: '细边框', defaultDark: 'rgba(255, 255, 255, 0.08)', defaultLight: 'rgba(0, 0, 0, 0.05)' }
    ]
  }
]

/**
 * 预设主题分类
 * 仿照 QQ 超级调色盘的分类与命名
 */
export const PRESET_CATEGORIES: PresetCategory[] = [
  {
    id: 'classic',
    title: '精品尝鲜',
    subtitle: '感受色彩活力，定制你的主题',
    presets: [
      {
        id: 'night-gray',
        name: '夜幕灰',
        colors: {
          '--steam-accent-blue': '#9ca3af',
          '--steam-bg-primary': '#2a2a2a',
          '--steam-bg-secondary': '#333333',
          '--steam-bg-card': '#3d3d3d'
        }
      },
      {
        id: 'classic-blue',
        name: '经典蓝',
        colors: {
          '--steam-accent-blue': '#3b82f6',
          '--steam-bg-primary': '#1e3a5f',
          '--steam-bg-secondary': '#243b55',
          '--steam-bg-card': '#274968'
        }
      },
      {
        id: 'ocean-blue',
        name: '地海蔚蓝',
        colors: {
          '--steam-accent-blue': '#0ea5e9',
          '--steam-bg-primary': '#0c4a6e',
          '--steam-bg-secondary': '#075985',
          '--steam-bg-card': '#0a5a82'
        }
      },
      {
        id: 'sakura-pink',
        name: '樱语红粉',
        colors: {
          '--steam-accent-blue': '#f472b6',
          '--steam-bg-primary': '#831843',
          '--steam-bg-secondary': '#9d174d',
          '--steam-bg-card': '#a61e55'
        }
      },
      {
        id: 'spring-green',
        name: '触手生春',
        colors: {
          '--steam-accent-blue': '#34d399',
          '--steam-bg-primary': '#064e3b',
          '--steam-bg-secondary': '#065f46',
          '--steam-bg-card': '#047857'
        }
      }
    ]
  },
  {
    id: 'national',
    title: '水墨国风',
    subtitle: '秋水共长天一色',
    presets: [
      {
        id: 'elegant-gold',
        name: '典雅金',
        colors: {
          '--steam-accent-blue': '#d4af37',
          '--steam-bg-primary': '#3d342b',
          '--steam-bg-secondary': '#4a3f35',
          '--steam-bg-card': '#56493d'
        }
      },
      {
        id: 'lilac-purple',
        name: '丁香紫',
        colors: {
          '--steam-accent-blue': '#c4b5fd',
          '--steam-bg-primary': '#4c1d95',
          '--steam-bg-secondary': '#5b21b6',
          '--steam-bg-card': '#6d28d9'
        }
      },
      {
        id: 'bean-green',
        name: '芸豆青',
        colors: {
          '--steam-accent-blue': '#86efac',
          '--steam-bg-primary': '#14532d',
          '--steam-bg-secondary': '#166534',
          '--steam-bg-card': '#15803d'
        }
      },
      {
        id: 'cinnabar',
        name: '辰砂色',
        colors: {
          '--steam-accent-blue': '#f87171',
          '--steam-bg-primary': '#7f1d1d',
          '--steam-bg-secondary': '#991b1b',
          '--steam-bg-card': '#b91c1c'
        }
      },
      {
        id: 'custom-national',
        name: '自选颜色',
        colors: {}
      }
    ]
  },
  {
    id: 'mystery',
    title: '神秘美学',
    subtitle: '每天一点甜，生活好运连连',
    presets: [
      {
        id: 'orange-wish',
        name: '心想事橙',
        colors: {
          '--steam-accent-blue': '#fb923c',
          '--steam-bg-primary': '#7c2d12',
          '--steam-bg-secondary': '#9a3412',
          '--steam-bg-card': '#c2410c'
        }
      },
      {
        id: 'green-relax',
        name: '不再焦绿',
        colors: {
          '--steam-accent-blue': '#a3e635',
          '--steam-bg-primary': '#365314',
          '--steam-bg-secondary': '#3f6212',
          '--steam-bg-card': '#4d7c0f'
        }
      },
      {
        id: 'blue-emotion',
        name: '蓝以为情',
        colors: {
          '--steam-accent-blue': '#60a5fa',
          '--steam-bg-primary': '#1e3a8a',
          '--steam-bg-secondary': '#1e40af',
          '--steam-bg-card': '#1d4ed8'
        }
      },
      {
        id: 'purple-yours',
        name: '紫属于你',
        colors: {
          '--steam-accent-blue': '#c084fc',
          '--steam-bg-primary': '#581c87',
          '--steam-bg-secondary': '#6b21a8',
          '--steam-bg-card': '#7e22ce'
        }
      },
      {
        id: 'custom-mystery',
        name: '自选颜色',
        colors: {}
      }
    ]
  }
]

/**
 * 判断一个预设是否为「自选颜色」占位预设
 * @param preset 预设对象
 */
export function isCustomPreset(preset: ColorPreset): boolean {
  return !preset.colors || Object.keys(preset.colors).length === 0
}
