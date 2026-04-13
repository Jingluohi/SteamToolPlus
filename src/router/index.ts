import { createRouter, createWebHistory } from 'vue-router'
import GameList from '../views/GameList.vue'
import GameDetail from '../views/GameDetail.vue'
import GameDownload from '../views/GameDownload.vue'
import SteamPatchInject from '../views/SteamPatchInject.vue'
import SettingsWindow from '../views/SettingsWindow.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'all-games',
      component: GameList,
      meta: { title: '全部游戏', category: 'all' }
    },
    {
      path: '/no-steam',
      name: 'no-steam',
      component: GameList,
      meta: { title: '免 Steam 启动', category: 'no-steam' }
    },
    {
      path: '/lan-multiplayer',
      name: 'lan-multiplayer',
      component: GameList,
      meta: { title: '局域网联机', category: 'lan-multiplayer' }
    },
    {
      path: '/steam-multiplayer',
      name: 'steam-multiplayer',
      component: GameList,
      meta: { title: 'Steam 联机', category: 'steam-multiplayer' }
    },
    {
      path: '/denuvo-vm',
      name: 'denuvo-vm',
      component: GameList,
      meta: { title: 'D 加密虚拟机', category: 'denuvo-vm' }
    },
    {
      path: '/game/:id',
      name: 'game-detail',
      component: GameDetail,
      meta: { title: '游戏详情' }
    },
    {
      path: '/game-download',
      name: 'game-download',
      component: GameDownload,
      meta: { title: '游戏本体下载' }
    },
    {
      path: '/steam-patch-inject',
      name: 'steam-patch-inject',
      component: SteamPatchInject,
      meta: { title: '免 Steam 补丁注入' }
    },
    {
      path: '/settings',
      name: 'settings',
      component: SettingsWindow,
      meta: { title: '设置' }
    }
  ]
})

export default router
