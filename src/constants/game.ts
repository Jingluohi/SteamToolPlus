/**
 * game.ts - 游戏相关的常量配置
 * 集中管理游戏分类、颜色等硬编码配置
 */

/**
 * 补丁分类配置
 * patch_type 对应的名称和颜色
 */
export const PATCH_CATEGORIES = {
  0: { name: '免Steam补丁', color: '#1b9fff' },
  1: { name: '局域网联机', color: '#4caf50' },
  2: { name: 'Steam联机', color: '#ff9800' },
  3: { name: 'D加密虚拟机', color: '#9c27b0' },
  4: { name: 'Epic联机', color: '#e91e63' }
} as const

/**
 * 获取分类名称
 * @param patchType - 补丁类型编号
 * @returns 分类名称
 */
export function getCategoryName(patchType: number): string {
  return PATCH_CATEGORIES[patchType as keyof typeof PATCH_CATEGORIES]?.name || '未知分类'
}

/**
 * 获取分类颜色
 * @param patchType - 补丁类型编号
 * @returns 颜色值（十六进制）
 */
export function getCategoryColor(patchType: number): string {
  return PATCH_CATEGORIES[patchType as keyof typeof PATCH_CATEGORIES]?.color || '#666'
}
