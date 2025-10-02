import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import Components from "unplugin-vue-components/vite";
import { AntDesignVueResolver } from "unplugin-vue-components/resolvers";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

// 简化版配置 - 用于解决白屏问题
export default defineConfig({
  // 修复 Tauri 生产构建白屏问题
  base: './',
  
  plugins: [
    vue(),
    Components({
      resolvers: [
        AntDesignVueResolver({
          importStyle: false,
        }),
      ],
      dts: false,
    }),
  ],

  // 路径别名配置
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
    },
  },

  // CSS 预处理器配置
  css: {
    preprocessorOptions: {
      less: {
        javascriptEnabled: true,
      },
    },
  },

  clearScreen: false,
  
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },

  // 简化的构建配置 - 禁用代码分割
  build: {
    target: ['es2020', 'edge88'],
    minify: 'esbuild',
    sourcemap: false,
    rollupOptions: {
      output: {
        // 完全禁用代码分割 - 所有代码打包成一个文件
        manualChunks: undefined,
      },
    },
  },
});
