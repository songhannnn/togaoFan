// 文案模板的本机读写命令
mod templates;

// 示例命令仍留在这里，界面没有调用它
#[tauri::command]
fn greet(name: &str) -> String {
    // 原模板里的问候，不参与模板存储
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 桌面程序入口
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 组装窗口和命令
    tauri::Builder::default()
        // 原来的打开链接插件
        .plugin(tauri_plugin_opener::init())
        // 注册问候和四条模板命令
        .invoke_handler(tauri::generate_handler![
            greet,
            templates::template_store_path,
            templates::list_copy_templates,
            templates::save_copy_template,
            templates::delete_copy_template
        ])
        // 按 tauri.conf.json 启动
        .run(tauri::generate_context!())
        // 启动失败时停下来
        .expect("error while running tauri application");
}
