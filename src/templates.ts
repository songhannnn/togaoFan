// 调用本机命令
import { invoke, isTauri } from "@tauri-apps/api/core";

// 一条文案模板，只有文字字段
export interface CopyTemplate {
  // 本机编号
  id: string;
  // 模板名字
  name: string;
  // 标题
  title: string;
  // 简介
  summary: string;
  // 标签
  tags: string;
  // 哔哩哔哩分区
  partition: string;
  // 最近保存的毫秒时间
  updatedAt: number;
}

// 保存时提交的内容，不含视频和封面
export interface SaveTemplateInput {
  // 模板名字
  name: string;
  // 标题
  title: string;
  // 简介
  summary: string;
  // 标签
  tags: string;
  // 哔哩哔哩分区
  partition: string;
}

// 保存结果，用来区分新建和同名更新
export interface SaveTemplateResult {
  // 保存后的模板
  template: CopyTemplate;
  // 真表示新建
  created: boolean;
}

// 当前是不是 Tauri 桌面窗口
export function desktopReady(): boolean {
  // 浏览器预览写不了项目目录里的模板文件
  return isTauri();
}

// 模板 JSON 的本机路径
export async function templateStorePath(): Promise<string> {
  // 由 Rust 计算，前端不拼路径
  return invoke<string>("template_store_path");
}

// 读取全部模板
export async function listCopyTemplates(): Promise<CopyTemplate[]> {
  // 打开编辑页时调用
  return invoke<CopyTemplate[]>("list_copy_templates");
}

// 保存或更新一条模板
export async function saveCopyTemplate(input: SaveTemplateInput): Promise<SaveTemplateResult> {
  // 参数名跟 Rust 的 input 一致
  return invoke<SaveTemplateResult>("save_copy_template", { input });
}

// 按编号删除
export async function deleteCopyTemplate(id: string): Promise<void> {
  // 参数名跟 Rust 的 id 一致
  return invoke<void>("delete_copy_template", { id });
}

// 把命令失败转成能直接显示的中文
export function errorText(error: unknown): string {
  // Rust 用字符串作为错误时，这里直接就是那句中文
  if (typeof error === "string" && error.trim()) {
    // 去掉首尾空白后显示
    return error.trim();
  }
  // 有些封装会把原因放在 message 里
  if (error && typeof error === "object" && "message" in error) {
    // 取出 message
    const message = (error as { message: unknown }).message;
    // 是非空字符串就用它
    if (typeof message === "string" && message.trim()) {
      // 显示这句
      return message.trim();
    }
  }
  // 标准错误对象
  if (error instanceof Error && error.message.trim()) {
    // 显示错误信息
    return error.message.trim();
  }
  // 没有具体原因时的兜底
  return "模板没有写成，请再试一次";
}
