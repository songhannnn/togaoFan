<script setup lang="ts">
// 结果页这一步只显示空状态
</script>

<template>
  <!-- 结果页 -->
  <section class="page">
    <!-- 页头 -->
    <div class="page-row">
      <!-- 页面标题 -->
      <el-text tag="h1" class="page-title">结果</el-text>
      <!-- 以后显示各平台成功或失败 -->
      <el-text type="info">投稿完成后，每个平台一行结果会显示在这里。</el-text>
    </div>
    <!-- 空状态 -->
    <el-card shadow="never">
      <!-- 还没有记录 -->
      <el-empty description="还没有投稿结果。" :image-size="72" />
    </el-card>
  </section>
</template>
