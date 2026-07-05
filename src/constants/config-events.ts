/**
 * 配置同步事件常量
 * 所有与 Steam 模拟器配置相关的保存/同步事件统一从此文件导入，
 * 避免在组件中硬编码事件名字符串。
 */

export const CONFIG_EVENTS = {
  /** 主配置保存 */
  MAIN_SAVED: 'main-config-saved',
  /** 用户配置保存 */
  USER_SAVED: 'user-config-saved',
  /** 应用/DLC 配置保存 */
  APP_SAVED: 'app-config-saved',
  /** 覆盖层配置保存 */
  OVERLAY_SAVED: 'overlay-config-saved',
  /** 成就配置保存 */
  ACHIEVEMENTS_SAVED: 'achievements-config-saved',
  /** 统计配置保存 */
  STATS_SAVED: 'stats-config-saved',
  /** 物品库存配置保存 */
  ITEMS_SAVED: 'items-config-saved',
  /** 创意工坊模组配置保存 */
  MODS_SAVED: 'mods-config-saved',
  /** 排行榜配置保存 */
  LEADERBOARDS_SAVED: 'leaderboards-config-saved',
  /** 控制器配置保存 */
  CONTROLLER_SAVED: 'controller-config-saved',
  /** 局域网联机配置保存 */
  LAN_SAVED: 'lan-config-saved',
  /** 其他配置保存 */
  OTHER_SAVED: 'other-config-saved',
  /** ColdClientLoader 配置保存 */
  COLDCLIENT_SAVED: 'coldclient-config-saved',
  /** Lobby Connect 配置保存 */
  LOBBY_SAVED: 'lobby-config-saved',
} as const

/** 事件名称类型，用于类型安全的监听与广播 */
export type ConfigEventName = (typeof CONFIG_EVENTS)[keyof typeof CONFIG_EVENTS]
