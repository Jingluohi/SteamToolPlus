import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'

/**
 * Vite配置文件
 * 遵循项目规范：极致体积优化、tree-shaking、资源内联
 */
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
      '@components': resolve(__dirname, 'src/components'),
      '@views': resolve(__dirname, 'src/views'),
      '@store': resolve(__dirname, 'src/store'),
      '@api': resolve(__dirname, 'src/api'),
      '@types': resolve(__dirname, 'src/types'),
      '@utils': resolve(__dirname, 'src/utils'),
      '@assets': resolve(__dirname, 'src/assets'),
      '@styles': resolve(__dirname, 'src/styles')
    }
  },
  build: {
    target: 'es2020',
    minify: 'terser',
    terserOptions: {
      compress: {
        drop_console: true,
        drop_debugger: true,
        pure_funcs: ['console.log', 'console.info', 'console.debug']
      },
      mangle: {
        safari10: true
      }
    },
    rollupOptions: {
      output: {
        manualChunks: {
          'vendor': ['vue', 'vue-router', 'pinia'],
          'tauri': ['@tauri-apps/api', '@tauri-apps/plugin-shell', '@tauri-apps/plugin-dialog', '@tauri-apps/plugin-fs']
        }
      }
    },
    assetsInlineLimit: 4096,
    chunkSizeWarningLimit: 500,
    sourcemap: false
  },
  css: {
    devSourcemap: false
  },
  server: {
    port: 1420,
    strictPort: true,
    host: 'localhost'
  },
  clearScreen: false
})
