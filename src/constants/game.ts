/**
 * game.ts - 游戏相关的常量配置
 * 集中管理游戏分类、颜色等硬编码配置
 */

/**
 * 补丁分类配置
 * patch_type 对应的名称和颜色
 */
export const PATCH_CATEGORIES = {
  0: { name: '免Steam补丁', description: '无需Steam即可运行游戏', color: '#1b9fff', group: 'single' },
  1: { name: '局域网联机补丁', description: '支持局域网联机游玩', color: '#4caf50', group: 'online' },
  2: { name: 'Steam联机补丁', description: '支持Steam平台联机', color: '#ff9800', group: 'online' },
  3: { name: 'D加密虚拟机补丁', description: '用于D加密游戏的虚拟机补丁', color: '#9c27b0', group: 'single' },
  4: { name: 'Epic联机补丁', description: '支持Epic平台联机', color: '#e91e63', group: 'online' },
  5: { name: 'Xbox联机补丁', description: '支持Xbox平台联机', color: '#107c10', group: 'online' }
} as const

/**
 * 单机/联机分组类型
 * single = 单机（免Steam、D加密虚拟机）
 * online = 联机（局域网联机、Steam联机、Epic联机、Xbox联机）
 */
export type PatchGroup = 'single' | 'online'

/**
 * 获取补丁所属分组
 * @param patchType - 补丁类型编号
 * @returns 分组类型（single/online），未知类型返回 undefined
 */
export function getPatchGroup(patchType: number): PatchGroup | undefined {
  return PATCH_CATEGORIES[patchType as keyof typeof PATCH_CATEGORIES]?.group
}

/**
 * 判断是否为单机补丁
 * @param patchType - 补丁类型编号
 * @returns 是否属于单机分组
 */
export function isSinglePlayerPatch(patchType: number): boolean {
  return getPatchGroup(patchType) === 'single'
}

/**
 * 判断是否为联机补丁
 * @param patchType - 补丁类型编号
 * @returns 是否属于联机分组
 */
export function isOnlinePatch(patchType: number): boolean {
  return getPatchGroup(patchType) === 'online'
}

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
