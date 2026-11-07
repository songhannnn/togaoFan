<script setup lang="ts">
// 进入页面时读取模板，离开字段仍只活在这一页
import { computed, onMounted, ref } from "vue";
// 保存结果用消息提示，删除前用确认框
import { ElMessage, ElMessageBox } from "element-plus";
// 上传组件给出的文件类型
import type { UploadFile, UploadUserFile } from "element-plus";
// 封面卡片上的加号
import { Plus } from "@element-plus/icons-vue";
// 本机模板命令
import {
  deleteCopyTemplate,
  desktopReady,
  errorText,
  listCopyTemplates,
  saveCopyTemplate,
  templateStorePath,
  type CopyTemplate,
} from "../templates";

// 标题
const title = ref("");
// 简介
const summary = ref("");
// 标签
const tags = ref("");
// 哔哩哔哩分区
const partition = ref("");
// 已选视频的文件名，不进入模板
const videoName = ref("还没有选择");
// 已选封面的文件名，不进入模板
const coverName = ref("还没有选择");
// 封面卡片只在这一页预览
const coverFiles = ref<UploadUserFile[]>([]);
// AI 说明默认收起
const showAiNote = ref(false);
// 是不是桌面窗口
const desktop = ref(false);
// 正在读模板
const loading = ref(false);
// 正在保存
const saving = ref(false);
// 读取失败时的说明
const loadError = ref("");
// 模板 JSON 的本机路径
const storePath = ref("");
// 准备保存时填写的名字
const templateName = ref("");
// 当前选中的模板编号
const selectedId = ref<string>();
// 已有模板
const templates = ref<CopyTemplate[]>([]);

// 选中的那一条，用来套用和删除
const selected = computed(() => {
  // 没有编号就没有选中
  if (!selectedId.value) {
    // 未选择
    return null;
  }
  // 按编号查找
  return templates.value.find((item) => item.id === selectedId.value) ?? null;
});

// 视频只记下文件名
function onVideoChange(file: UploadFile): void {
  // 有名字就显示，没有就回到空状态
  videoName.value = file.name || "还没有选择";
}

// 封面只记下文件名，图片预览留在上传组件里
function onCoverChange(file: UploadFile): void {
  // 显示文件名
  coverName.value = file.name || "还没有选择";
}

// 去掉封面后恢复空状态
function onCoverRemove(): void {
  // 封面不在模板里，这里只清界面
  coverName.value = "还没有选择";
}

// 已经有一张封面时，不自动换成下一张
function onCoverExceed(): void {
  // 提示先移除
  ElMessage.info("要更换封面，请先去掉当前这张");
}

// 点开 AI 入口时只展示说明
function openAiNote(): void {
  // 展开说明，不请求模型
  showAiNote.value = true;
}

// 从本机重新读模板列表；保存后的刷新不再挡住整张表单
async function loadTemplates(quiet = false): Promise<void> {
  // 只有第一次进入页面才铺开加载
  if (!quiet) {
    // 开始读
    loading.value = true;
  }
  // 清掉上一次的错误
  loadError.value = "";
  try {
    // 路径和列表一起拿
    storePath.value = await templateStorePath();
    // 读取模板
    templates.value = await listCopyTemplates();
    // 选中的编号如果已经没了，就清掉
    if (selectedId.value && !templates.value.some((item) => item.id === selectedId.value)) {
      // 取消选中
      selectedId.value = undefined;
    }
  } catch (error) {
    // 留下原因
    loadError.value = errorText(error);
  } finally {
    // 结束读取
    loading.value = false;
  }
}

// 把当前标题、分区、标签、简介存成模板
async function saveTemplate(): Promise<void> {
  // 浏览器预览不能写应用数据目录
  if (!desktop.value) {
    // 提示去桌面窗口
    ElMessage.warning("请用桌面窗口打开后再保存模板");
    // 停止
    return;
  }
  // 先看名字
  const name = templateName.value.trim();
  // 没写名字
  if (!name) {
    // 提示
    ElMessage.warning("请先填写模板名字");
    // 停止
    return;
  }
  // 开始保存
  saving.value = true;
  try {
    // 只提交四个文字字段
    const result = await saveCopyTemplate({
      // 模板名字
      name,
      // 标题
      title: title.value,
      // 简介
      summary: summary.value,
      // 标签
      tags: tags.value,
      // 分区
      partition: partition.value,
    });
    // 记住刚保存的名字和编号
    templateName.value = result.template.name;
    // 方便接着套用或删除
    selectedId.value = result.template.id;
    // 同名是更新
    if (result.created) {
      // 新建成功
      ElMessage.success(`已保存模板「${result.template.name}」`);
    } else {
      // 覆盖原来的四个字段
      ElMessage.success(`已更新模板「${result.template.name}」`);
    }
    // 重新读列表，不挡住正在填的表单
    await loadTemplates(true);
  } catch (error) {
    // 显示 Rust 返回的原因
    ElMessage.error(errorText(error));
  } finally {
    // 结束保存
    saving.value = false;
  }
}

// 一键填进标题、分区、标签、简介
function applyTemplate(): void {
  // 还没选
  if (!selected.value) {
    // 提示
    ElMessage.warning("请先选择一个模板");
    // 停止
    return;
  }
  // 填标题
  title.value = selected.value.title;
  // 填分区
  partition.value = selected.value.partition;
  // 填标签
  tags.value = selected.value.tags;
  // 填简介
  summary.value = selected.value.summary;
  // 视频和封面保持原样
  ElMessage.success(`已填入模板「${selected.value.name}」。视频和封面没有改动。`);
}

// 删除选中的模板
async function removeTemplate(): Promise<void> {
  // 还没选
  if (!selected.value) {
    // 提示
    ElMessage.warning("请先选择一个模板");
    // 停止
    return;
  }
  // 记下编号和名字，确认框期间列表可能变化
  const id = selected.value.id;
  // 名字用在确认文案里
  const name = selected.value.name;
  try {
    // 确认后再删
    await ElMessageBox.confirm(`删除模板「${name}」？删除后不能恢复。`, "删除模板", {
      // 确认按钮
      confirmButtonText: "删除",
      // 取消按钮
      cancelButtonText: "取消",
      // 警告样式
      type: "warning",
    });
  } catch {
    // 取消时组件会抛出，这里当作不删除
    return;
  }
  try {
    // 从本机文件里去掉
    await deleteCopyTemplate(id);
    // 如果删的就是当前选中，清掉选择
    if (selectedId.value === id) {
      // 取消选中
      selectedId.value = undefined;
    }
    // 提示已删除
    ElMessage.success(`已删除模板「${name}」`);
    // 刷新列表，不挡住正在填的表单
    await loadTemplates(true);
  } catch (error) {
    // 显示原因
    ElMessage.error(errorText(error));
  }
}

// 打开编辑页就读一次本机模板
onMounted(() => {
  // 判断是不是桌面窗口
  desktop.value = desktopReady();
  // 浏览器里不调用本机命令
  if (!desktop.value) {
    // 停在说明上
    return;
  }
  // 读取模板
  void loadTemplates();
});
</script>

<template>
  <!-- 编辑页：字段顺序按投稿页，模板只在这一页 -->
  <section class="page">
    <!-- 页头 -->
    <div class="page-row">
      <!-- 页面标题 -->
      <el-text tag="h1" class="page-title">编辑</el-text>
      <!-- 这一页保存什么 -->
      <el-text type="info">
        星号是投稿时要填的项。底部可把标题、分区、标签和简介存成模板。视频和封面不会写进模板。
      </el-text>
    </div>

    <!-- 白底表单 -->
    <el-card shadow="never" v-loading="loading">
      <!-- 从上到下：视频、封面、标题、分区、标签、简介 -->
      <el-form
        class="edit-form"
        label-position="left"
        label-width="92px"
        require-asterisk-position="left"
        @submit.prevent
      >
        <!-- 视频 -->
        <el-form-item label="视频">
          <!-- 只选择文件，不上传到服务器 -->
          <el-upload
            drag
            action="#"
            :auto-upload="false"
            :show-file-list="false"
            accept="video/*"
            style="width: 100%"
            :on-change="onVideoChange"
          >
            <!-- 选择说明 -->
            <div class="el-upload__text">选择视频</div>
            <!-- 当前文件名 -->
            <div class="el-upload__tip">{{ videoName }}</div>
          </el-upload>
        </el-form-item>

        <!-- 封面，投稿页上这一项带星号 -->
        <el-form-item label="封面" required>
          <!-- 一张封面的卡片 -->
          <el-upload
            v-model:file-list="coverFiles"
            list-type="picture-card"
            action="#"
            :auto-upload="false"
            :limit="1"
            accept="image/*"
            :on-change="onCoverChange"
            :on-remove="onCoverRemove"
            :on-exceed="onCoverExceed"
          >
            <!-- 空卡片上的加号 -->
            <el-icon><Plus /></el-icon>
          </el-upload>
        </el-form-item>

        <!-- 标题 -->
        <el-form-item label="标题" required>
          <!-- 单行，右下角计数 -->
          <el-input
            v-model="title"
            maxlength="80"
            show-word-limit
            clearable
            placeholder="填写标题"
          />
        </el-form-item>

        <!-- 分区 -->
        <el-form-item label="分区" required>
          <!-- 不拉分区列表，只记用户写下的文字 -->
          <el-input
            v-model="partition"
            maxlength="40"
            show-word-limit
            clearable
            placeholder="例如：生活"
          />
        </el-form-item>

        <!-- 标签 -->
        <el-form-item label="标签" required>
          <!-- 单行 -->
          <el-input
            v-model="tags"
            maxlength="200"
            clearable
            placeholder="多个标签用空格或逗号分开"
          />
        </el-form-item>

        <!-- 简介，投稿页上这项没有星号 -->
        <el-form-item label="简介">
          <!-- 多行，右下角计数 -->
          <el-input
            v-model="summary"
            type="textarea"
            :rows="8"
            maxlength="2000"
            show-word-limit
            placeholder="填写简介"
          />
        </el-form-item>
      </el-form>

      <!-- 底部：保存模板、套用模板。这里不做存草稿 -->
      <el-space
        direction="vertical"
        alignment="center"
        :size="16"
        style="width: 100%; margin-top: 28px"
      >
        <!-- 不在桌面窗口时的说明 -->
        <el-alert
          v-if="!desktop"
          type="warning"
          :closable="false"
          show-icon
          title="请用桌面窗口打开。浏览器预览不能把模板写进本机数据目录。"
        />
        <!-- 读取失败 -->
        <el-alert v-if="loadError" type="error" :closable="false" show-icon :title="loadError" />
        <!-- 名字、选择、删除 -->
        <el-space wrap alignment="center" :size="12">
          <!-- 模板名字 -->
          <el-input
            v-model="templateName"
            maxlength="40"
            show-word-limit
            clearable
            placeholder="模板名字，例如：生活区口播"
            style="width: 280px"
            :disabled="!desktop"
          />
          <!-- 已有模板 -->
          <el-select
            v-model="selectedId"
            placeholder="选择已有模板"
            clearable
            filterable
            style="width: 220px"
            :disabled="!desktop"
          >
            <!-- 每一条 -->
            <el-option
              v-for="item in templates"
              :key="item.id"
              :label="item.name"
              :value="item.id"
            />
            <!-- 还没有模板 -->
            <template #empty>
              <!-- 空状态 -->
              <el-empty description="还没有模板" :image-size="56" />
            </template>
          </el-select>
          <!-- 删除选中的模板 -->
          <el-button type="danger" link :disabled="!desktop || !selected" @click="removeTemplate">
            删除
          </el-button>
        </el-space>
        <!-- 两个主操作，位置对应投稿页底部的一对按钮 -->
        <el-space :size="16">
          <!-- 保存当前四个文字字段 -->
          <el-button :loading="saving" :disabled="!desktop" @click="saveTemplate">保存模板</el-button>
          <!-- 一键填入 -->
          <el-button type="primary" :disabled="!desktop || !selected" @click="applyTemplate">
            套用模板
          </el-button>
        </el-space>
        <!-- 选中后说明会替换哪些字段 -->
        <el-text v-if="selected" type="info" size="small">
          套用「{{ selected.name }}」会替换标题、分区、标签和简介，不会改动视频和封面。
        </el-text>
        <!-- 同名规则 -->
        <el-text type="info" size="small">同名会更新原来的模板，不会另存一份。</el-text>
        <!-- 本机文件位置 -->
        <el-text v-if="storePath" type="info" size="small" tag="p">模板文件：{{ storePath }}</el-text>
      </el-space>
    </el-card>

    <!-- 抖音和文案仍留在表单下面，不放进左侧菜单 -->
    <el-card shadow="never" style="margin-top: 16px">
      <!-- 平台名和状态 -->
      <template #header>
        <!-- 标题行 -->
        <el-space>
          <!-- 平台名 -->
          <span>抖音</span>
          <!-- 固定状态 -->
          <el-tag type="warning">未接通</el-tag>
        </el-space>
      </template>
      <!-- 占位说明 -->
      <el-text type="info">看得见这一块。不能勾选投稿，也不会发出网络请求。</el-text>
      <!-- 文案入口 -->
      <div style="margin-top: 12px">
        <!-- 只展开说明 -->
        <el-button @click="openAiNote">AI 写文案</el-button>
      </div>
      <!-- 点开后的说明 -->
      <el-alert
        v-if="showAiNote"
        style="margin-top: 12px"
        type="info"
        :closable="false"
        show-icon
        title="以后再做。第一版仍由人手填写标题、简介和标签。这里不连接模型，也不发送请求。"
      />
    </el-card>
  </section>
</template>
