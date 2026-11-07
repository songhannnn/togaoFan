<script setup lang="ts">
// 新建只打开编辑页，不在首页列模板
import { useRouter } from "vue-router";

// 当前应用的路由器
const router = useRouter();

// 进入空白编辑页，模板在编辑页底部再选
function goCreate(): void {
  // 切换到编辑页
  void router.push("/edit");
}
</script>

<template>
  <!-- 首页 -->
  <section class="page">
    <!-- 标题和新建 -->
    <el-row class="page-row" justify="space-between" align="middle">
      <!-- 标题组 -->
      <div>
        <!-- 页面标题 -->
        <el-text tag="h1" class="page-title">首页</el-text>
        <!-- 草稿还没有落盘 -->
        <el-text type="info">草稿和已投记录以后留在这台电脑上。现在还没有数据。</el-text>
      </div>
      <!-- 打开编辑页 -->
      <el-button type="primary" @click="goCreate">新建</el-button>
    </el-row>

    <!-- 两块空状态 -->
    <el-space direction="vertical" fill :size="16" style="width: 100%">
      <!-- 草稿 -->
      <el-card shadow="never">
        <!-- 区块标题 -->
        <template #header>草稿</template>
        <!-- 空状态 -->
        <el-empty description="还没有草稿。" :image-size="72" />
      </el-card>
      <!-- 已投记录 -->
      <el-card shadow="never">
        <!-- 区块标题 -->
        <template #header>已投记录</template>
        <!-- 空状态 -->
        <el-empty description="还没有投稿记录。" :image-size="72" />
      </el-card>
    </el-space>
  </section>
</template>
