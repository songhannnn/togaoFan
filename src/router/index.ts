// 创建路由实例，以及桌面窗口里更稳妥的哈希历史
import { createRouter, createWebHashHistory } from "vue-router";
// 首页：草稿和已投记录的空状态
import HomeView from "../views/HomeView.vue";
// 编辑页：五个共用字段和两个平台块
import EditView from "../views/EditView.vue";
// 结果页：这一次投稿结果的空状态
import ResultView from "../views/ResultView.vue";
// 设置页：说明哔哩哔哩授权以后在这里做
import SettingsView from "../views/SettingsView.vue";

// 四个页面，编辑页是单页表单，不是分步向导
export const router = createRouter({
  // 哈希路由在 Tauri 窗口里刷新或直接打开子页面时仍能落到前端
  history: createWebHashHistory(),
  // 页面表
  routes: [
    // 首页
    { path: "/", name: "home", component: HomeView },
    // 编辑页
    { path: "/edit", name: "edit", component: EditView },
    // 结果页
    { path: "/result", name: "result", component: ResultView },
    // 设置页
    { path: "/settings", name: "settings", component: SettingsView },
  ],
});
