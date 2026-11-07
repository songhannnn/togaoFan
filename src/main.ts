// 创建 Vue 应用
import { createApp } from "vue";
// 组件库
import ElementPlus from "element-plus";
// 确定、取消等文案用中文
import zhCn from "element-plus/es/locale/lang/zh-cn";
// 组件库自己的样式，按钮和输入框不再手写皮肤
import "element-plus/dist/index.css";
// 根组件：窄侧栏和当前页面
import App from "./App.vue";
// 首页、编辑、结果、设置
import { router } from "./router";
// 只保留窗口高度和侧栏这层布局
import "./styles/app.css";

// 以根组件创建应用
const app = createApp(App);
// 四个页面的路由
app.use(router);
// 中文语言；控件用默认尺寸，贴近投稿页的密度
app.use(ElementPlus, { locale: zhCn, size: "default" });
// 挂到页面上的 #app
app.mount("#app");
