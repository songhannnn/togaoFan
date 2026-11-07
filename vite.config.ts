// 引入 Vite 配置函数
import { defineConfig } from "vite";
// 引入 Vue 单文件组件插件
import vue from "@vitejs/plugin-vue";
// 读取进程环境，供 Tauri 指定开发主机；未安装 Node 类型包时按官方模板忽略下一行
// @ts-expect-error type error without @types/node package
import process from "node:process";

// Tauri 开发时可能传入的主机地址
const host = process.env.TAURI_DEV_HOST;

// 导出开发服务器和构建都使用的配置
export default defineConfig(() => ({
  // 启用 Vue 插件
  plugins: [vue()],
  // 不清屏，避免盖住 Rust 侧的编译输出
  clearScreen: false,
  // 开发服务器，端口与 tauri.conf.json 的 devUrl 对齐
  server: {
    // Tauri 固定访问 1420
    port: 1420,
    // 端口被占用时直接失败，避免窗口连到别的服务
    strictPort: true,
    // 有外部主机时监听它，否则用 Vite 默认主机
    host: host || false,
    // 远程开发时把热更新指到该主机
    hmr: host
      ? {
          // 热更新走 WebSocket
          protocol: "ws",
          // 热更新主机
          host,
          // 热更新端口
          port: 1421,
        }
      : undefined,
    // 文件监听范围
    watch: {
      // 不监听 Rust 工程，避免无意义刷新
      ignored: ["**/src-tauri/**"],
    },
  },
}));
