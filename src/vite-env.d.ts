// 让 TypeScript 认识 Vite 注入的静态资源类型
/// <reference types="vite/client" />
// 让 vue-tsc 认识全局注册的 Element Plus 组件
/// <reference types="element-plus/global" />

// 声明 Vue 单文件组件可以被导入
declare module "*.vue" {
  // 引入 Vue 组件的类型工具
  import type { DefineComponent } from "vue";
  // 单个 .vue 文件对应的组件类型
  const component: DefineComponent<{}, {}, any>;
  // 默认导出该组件
  export default component;
}
