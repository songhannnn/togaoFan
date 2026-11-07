<script setup lang="ts">
// 设置页这一步只说明授权以后在这里做
</script>

<template>
  <!-- 设置页 -->
  <section class="page">
    <!-- 页头 -->
    <div class="page-row">
      <!-- 页面标题 -->
      <el-text tag="h1" class="page-title">设置</el-text>
      <!-- 范围 -->
      <el-text type="info">这里只放这台电脑上的哔哩哔哩授权。投稿方没有自己的登录账号。</el-text>
    </div>
    <!-- 授权说明 -->
    <el-card shadow="never">
      <!-- 区块标题 -->
      <template #header>哔哩哔哩授权</template>
      <!-- 以后要做的事 -->
      <el-text>哔哩哔哩授权以后在这里做。</el-text>
      <!-- 这一步的边界 -->
      <div>
        <el-text type="info">这一步不接接口，也不在这里填写或保存凭证。</el-text>
      </div>
    </el-card>
  </section>
</template>
