import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import Components from "unplugin-vue-components/vite";
import { AntDesignVueResolver } from "unplugin-vue-components/resolvers";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

// https://vitejs.dev/config/
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

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },

  // 构建优化
  build: {
    target: ['es2020', 'edge88', 'firefox78', 'chrome87', 'safari13'],
    minify: 'esbuild',
    sourcemap: false,
    rollupOptions: {
      output: {
        manualChunks(id) {
          if (id.includes('node_modules')) {
            if (id.includes('ant-design-vue')) {
              const normalizedId = id.replace(/\\/g, '/');
              const antdGroupMap = {
                forms: [
                  'form',
                  'input',
                  'input-number',
                  'select',
                  'checkbox',
                  'radio',
                  'date-picker',
                  'time-picker',
                  'cascader',
                  'tree-select',
                ],
                dataDisplay: [
                  'table',
                  'list',
                  'card',
                  'collapse',
                  'descriptions',
                  'statistic',
                  'timeline',
                  'tree',
                  'tag',
                ],
                navigation: [
                  'menu',
                  'tabs',
                  'breadcrumb',
                  'pagination',
                  'steps',
                  'dropdown',
                ],
                feedback: [
                  'modal',
                  'drawer',
                  'alert',
                  'message',
                  'notification',
                  'popover',
                  'popconfirm',
                  'tooltip',
                  'spin',
                ],
                layout: [
                  'grid',
                  'row',
                  'col',
                  'layout',
                  'space',
                  'divider',
                  'affix',
                  'anchor',
                ],
                utilities: [
                  'config-provider',
                  '_util',
                  'locale-provider',
                  'form-interface',
                  'version',
                ],
              } as const;

              for (const [chunkName, modules] of Object.entries(antdGroupMap)) {
                if (modules.some((mod) => normalizedId.includes(`ant-design-vue/es/${mod}`))) {
                  return `antd-${chunkName}`;
                }
              }

              return 'antd-misc';
            }
            if (id.includes('@ant-design/icons-vue')) {
              return 'antd-icons';
            }
            if (id.includes('vue')) {
              return 'vue-vendor';
            }
          }
          return undefined;
        },
      },
    },
  },
});
