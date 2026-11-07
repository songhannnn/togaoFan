<script setup lang="ts">
// 浅色窄导航。左侧四个入口用 Element Plus 菜单。
// 当前路由，用来点亮左侧对应的一项
import { useRoute, RouterView } from "vue-router";
// 只用通用图标，不用哔哩哔哩的商标
import { EditPen, House, Setting, Tickets } from "@element-plus/icons-vue";

// 当前页面路径
const route = useRoute();
</script>

<template>
  <!-- 左窄导航，右白底内容；没有顶栏，也没有账号 -->
  <el-container class="shell">
    <!-- 窄侧栏，只有四个自己的页面 -->
    <el-aside class="sidebar" width="200px">
      <!-- 产品名 -->
      <p class="brand">投稿方</p>
      <!-- 菜单按路径切换页面 -->
      <el-menu
        class="sidebar-menu"
        :default-active="route.path"
        router
        :ellipsis="false"
      >
        <!-- 首页 -->
        <el-menu-item index="/">
          <!-- 图标 -->
          <el-icon><House /></el-icon>
          <!-- 名称 -->
          <span>首页</span>
        </el-menu-item>
        <!-- 编辑 -->
        <el-menu-item index="/edit">
          <!-- 图标 -->
          <el-icon><EditPen /></el-icon>
          <!-- 名称 -->
          <span>编辑</span>
        </el-menu-item>
        <!-- 结果 -->
        <el-menu-item index="/result">
          <!-- 图标 -->
          <el-icon><Tickets /></el-icon>
          <!-- 名称 -->
          <span>结果</span>
        </el-menu-item>
        <!-- 设置 -->
        <el-menu-item index="/settings">
          <!-- 图标 -->
          <el-icon><Setting /></el-icon>
          <!-- 名称 -->
          <span>设置</span>
        </el-menu-item>
      </el-menu>
    </el-aside>
    <!-- 右侧当前页面 -->
    <el-main class="content">
      <!-- 路由匹配到的页面 -->
      <RouterView />
    </el-main>
  </el-container>
</template>
