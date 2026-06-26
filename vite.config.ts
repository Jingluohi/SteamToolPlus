import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'

/**
 * Vite配置件
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
        pure_funcs: ['console.log', 'console.info', 'console.debug', 'console.warn', 'console.error'],
        passes: 2,
        hoist_funs: true,
        if_return: true,
        join_vars: true
      },
      mangle: {
        safari10: true
      },
      format: {
        comments: false
      }
    },
    rollupOptions: {
      output: {
        manualChunks(id) {
          if (id.includes('node_modules')) {
            if (/node_modules\/(vue|vue-router|pinia)/.test(id)) {
              return 'vendor'
            }
            if (/node_modules\/(@tauri-apps\/api|@tauri-apps\/plugin-(shell|dialog|fs))/.test(id)) {
              return 'tauri'
            }
          }
          return undefined
        }
      }
    },
    assetsInlineLimit: 4096,
    chunkSizeWarningLimit: 500,
    sourcemap: false,
    reportCompressedSize: false,
    // Tailwind CSS v4 使用 @theme/@custom-variant 等 at-rule，
    // Vite 8 默认使用 lightningcss 压缩 CSS 会报错，改用 esbuild 压缩 CSS
    cssMinify: 'esbuild'
  },
  css: {
    devSourcemap: false
  },
  server: {
    port: 1420,
    strictPort: true,
    host: 'localhost'
  },
  clearScreen: false,
  esbuild: {
    drop: ['console', 'debugger'],
    legalComments: 'none'
  }
})