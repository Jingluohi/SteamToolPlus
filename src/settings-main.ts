import { createApp } from 'vue'
import { createPinia } from 'pinia'
import SettingsWindow from './views/SettingsWindow.vue'

const app = createApp(SettingsWindow)

app.use(createPinia())

app.mount('#app')
