// 文案模板只放在本机应用数据目录的一个 JSON 文件里
use serde::{Deserialize, Serialize};
// 读写这个 JSON 文件
use std::fs;
// 拼出 templates.json 的路径
use std::path::{Path, PathBuf};
// 同一进程里避免两次保存交错写坏文件
use std::sync::Mutex;
// 用时间生成不重复的模板编号
use std::time::{SystemTime, UNIX_EPOCH};
// 命令入参，用来拿到应用数据目录
use tauri::AppHandle;
// app_data_dir 挂在 Manager 上
use tauri::Manager;

// 应用数据目录里的文件名，不放视频和封面
const FILE_NAME: &str = "templates.json";
// 整文件读写共用一把锁
static FILE_LOCK: Mutex<()> = Mutex::new(());

// 保存时从前端收下的四个文字字段和模板名字
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveTemplateInput {
    // 模板名字
    pub name: String,
    // 标题
    pub title: String,
    // 简介
    pub summary: String,
    // 标签
    pub tags: String,
    // 哔哩哔哩分区
    pub partition: String,
}

// 写入 JSON 的一条模板
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CopyTemplate {
    // 本机编号
    pub id: String,
    // 模板名字
    pub name: String,
    // 标题
    pub title: String,
    // 简介
    pub summary: String,
    // 标签
    pub tags: String,
    // 哔哩哔哩分区
    pub partition: String,
    // 最近一次保存的毫秒时间，用来把新的排在前面
    #[serde(default)]
    pub updated_at: i64,
}

// 整个 JSON 文件
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TemplateFile {
    // 全部模板
    templates: Vec<CopyTemplate>,
}

// 保存之后告诉界面是新建还是覆盖同名
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SaveTemplateResult {
    // 保存后的那一条
    pub template: CopyTemplate,
    // 真表示新名字，假表示同名更新
    pub created: bool,
}

// 算出本机要写的 JSON 路径，此时还不创建目录
pub fn templates_path(app: &AppHandle) -> Result<PathBuf, String> {
    // Tauri 按标识 com.tougaofang.app 给出的应用数据目录
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|err| format!("读不到应用数据目录：{err}"))?;
    // 固定文件名，避免前端传入别的路径
    Ok(dir.join(FILE_NAME))
}

// 给界面展示文件会写到哪里
#[tauri::command]
pub fn template_store_path(app: AppHandle) -> Result<String, String> {
    // 转成系统路径字符串
    Ok(templates_path(&app)?.to_string_lossy().to_string())
}

// 打开编辑页时读取全部模板
#[tauri::command]
pub fn list_copy_templates(app: AppHandle) -> Result<Vec<CopyTemplate>, String> {
    // 只读这一个固定文件
    list_at(&templates_path(&app)?)
}

// 把当前四个文字字段存成模板
#[tauri::command]
pub fn save_copy_template(
    app: AppHandle,
    input: SaveTemplateInput,
) -> Result<SaveTemplateResult, String> {
    // 视频和封面不在入参里，也不会写进文件
    save_at(&templates_path(&app)?, input)
}

// 按编号删除一条模板
#[tauri::command]
pub fn delete_copy_template(app: AppHandle, id: String) -> Result<(), String> {
    // 编号只用来在 JSON 里查找，不当成路径
    delete_at(&templates_path(&app)?, &id)
}

// 读取模板；文件还不存在时当成空列表
pub fn list_at(path: &Path) -> Result<Vec<CopyTemplate>, String> {
    // 拿到文件锁
    let _guard = lock_file()?;
    // 读出后按最近保存排序
    let mut file = read_templates(path)?;
    // 新的在前
    sort_templates(&mut file.templates);
    // 把列表交给界面
    Ok(file.templates)
}

// 校验后写入；同名覆盖四个字段，不另存一条
pub fn save_at(path: &Path, input: SaveTemplateInput) -> Result<SaveTemplateResult, String> {
    // 先检查字数，不合规则不碰文件
    let input = normalize_input(input)?;
    // 拿到文件锁
    let _guard = lock_file()?;
    // 损坏的文件在这里直接返回，不会被覆盖
    let mut file = read_templates(path)?;
    // 用下标找同名，避免新建时还占着借用
    let found = file
        .templates
        .iter()
        .position(|item| item.name == input.name);
    // 同名就更新，否则新建
    let (template, created) = if let Some(index) = found {
        // 取出同名的那一条
        let item = &mut file.templates[index];
        // 保留原来的编号
        item.title = input.title;
        // 更新简介
        item.summary = input.summary;
        // 更新标签
        item.tags = input.tags;
        // 更新分区
        item.partition = input.partition;
        // 记下这次保存的时间
        item.updated_at = now_millis();
        // 拷贝出来再写文件
        (item.clone(), false)
    } else {
        // 新模板
        let template = CopyTemplate {
            // 本机生成编号
            id: new_id(),
            // 已经去掉首尾空白的名字
            name: input.name,
            // 标题
            title: input.title,
            // 简介
            summary: input.summary,
            // 标签
            tags: input.tags,
            // 分区
            partition: input.partition,
            // 保存时间
            updated_at: now_millis(),
        };
        // 放进列表
        file.templates.push(template.clone());
        // 标记为新建
        (template, true)
    };
    // 写回前按时间排好
    sort_templates(&mut file.templates);
    // 整个文件重写，顺手丢掉视频、封面这类多余字段
    write_templates(path, &file)?;
    // 告诉界面结果
    Ok(SaveTemplateResult { template, created })
}

// 删掉一条；找不到就不动文件
pub fn delete_at(path: &Path, id: &str) -> Result<(), String> {
    // 拿到文件锁
    let _guard = lock_file()?;
    // 先读出来
    let mut file = read_templates(path)?;
    // 记下原来有多少条
    let before = file.templates.len();
    // 按编号过滤
    file.templates.retain(|item| item.id != id);
    // 一条都没少，说明编号不存在
    if file.templates.len() == before {
        // 不改文件
        return Err("没有这个模板".to_string());
    }
    // 写回剩下的模板
    write_templates(path, &file)?;
    // 删除完成
    Ok(())
}

// 锁被写坏时提示稍后再试
fn lock_file() -> Result<std::sync::MutexGuard<'static, ()>, String> {
    // 等待别的保存或删除结束
    FILE_LOCK
        .lock()
        .map_err(|_| "模板文件正忙，请再试一次".to_string())
}

// 新的模板排在前面
fn sort_templates(items: &mut [CopyTemplate]) {
    // 时间相同就按编号，避免每次顺序乱跳
    items.sort_by(|left, right| {
        // 先比保存时间
        right
            .updated_at
            .cmp(&left.updated_at)
            // 再比编号
            .then(right.id.cmp(&left.id))
    });
}

// 去掉首尾空白并限制字数
fn normalize_input(input: SaveTemplateInput) -> Result<SaveTemplateInput, String> {
    // 名字必须能看见
    let name = input.name.trim().to_string();
    // 没写名字就不保存
    if name.is_empty() {
        // 界面可以直接显示这句
        return Err("请先填写模板名字".to_string());
    }
    // 名字太长就拒绝
    check_len("模板名字", &name, 40)?;
    // 标题去掉首尾空白
    let title = input.title.trim().to_string();
    // 标题跟输入框一样最多 80 字
    check_len("标题", &title, 80)?;
    // 简介去掉首尾空白
    let summary = input.summary.trim().to_string();
    // 简介跟计数器一样最多 2000 字
    check_len("简介", &summary, 2000)?;
    // 标签去掉首尾空白，中间的空格保留
    let tags = input.tags.trim().to_string();
    // 标签限制长度
    check_len("标签", &tags, 200)?;
    // 分区去掉首尾空白
    let partition = input.partition.trim().to_string();
    // 分区限制长度
    check_len("分区", &partition, 40)?;
    // 四个字段都通过了
    Ok(SaveTemplateInput {
        // 名字
        name,
        // 标题
        title,
        // 简介
        summary,
        // 标签
        tags,
        // 分区
        partition,
    })
}

// 按字数而不是字节数限制，中文一个字算一个
fn check_len(label: &str, value: &str, max: usize) -> Result<(), String> {
    // 超出就告诉是哪一项
    if value.chars().count() > max {
        // 不截断，让用户自己改
        return Err(format!("{label}最多 {max} 个字"));
    }
    // 长度合格
    Ok(())
}

// 读 JSON；不存在或空白都当成还没有模板
fn read_templates(path: &Path) -> Result<TemplateFile, String> {
    // 第一次保存前文件可以不存在
    if !path.exists() {
        // 空列表
        return Ok(TemplateFile {
            // 还没有模板
            templates: Vec::new(),
        });
    }
    // 读出文本
    let text = fs::read_to_string(path).map_err(|err| format!("模板文件读不出来：{err}"))?;
    // 空文件同样当成没有模板
    if text.trim().is_empty() {
        // 空列表
        return Ok(TemplateFile {
            // 还没有模板
            templates: Vec::new(),
        });
    }
    // 解析失败就停，避免把坏文件覆盖掉
    serde_json::from_str(&text).map_err(|_| "模板文件已损坏，没有改动它。".to_string())
}

// 先写临时文件，再换到正式文件名
fn write_templates(path: &Path, file: &TemplateFile) -> Result<(), String> {
    // 应用数据目录可能还没建
    if let Some(parent) = path.parent() {
        // 只创建模板文件自己的目录
        fs::create_dir_all(parent).map_err(|err| format!("无法创建应用数据目录：{err}"))?;
    }
    // 排版后的 JSON
    let mut bytes = serde_json::to_vec_pretty(file)
        .map_err(|err| format!("模板无法整理成文件：{err}"))?;
    // 文件末尾补一个换行
    bytes.push(b'\n');
    // 临时文件和正式文件放在同一目录，方便替换
    let tmp = path.with_extension("json.tmp");
    // 先落临时文件
    fs::write(&tmp, bytes).map_err(|err| format!("模板没有写到本机：{err}"))?;
    // 换到 templates.json
    replace_file(&tmp, path)
}

// Linux 可以直接替换；Windows 上目标已存在时先删再换
fn replace_file(from: &Path, to: &Path) -> Result<(), String> {
    // 先试一次直接替换
    if fs::rename(from, to).is_ok() {
        // 已经换成正式文件
        return Ok(());
    }
    // 目标还在时先删掉
    if to.exists() {
        // 删旧文件
        fs::remove_file(to).map_err(|err| format!("模板没有写到本机：{err}"))?;
    }
    // 再把临时文件换成正式文件
    fs::rename(from, to).map_err(|err| format!("模板没有写到本机：{err}"))
}

// 当前时间的毫秒数
fn now_millis() -> i64 {
    // 取不到时钟时记 0，仍然可以保存
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as i64)
        .unwrap_or(0)
}

// 本机编号，不依赖额外的编号库
fn new_id() -> String {
    // 毫秒时间
    let millis = now_millis();
    // 同一毫秒里再加一，避免两条编号相同
    let seq = SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    // 拼成模板编号
    format!("tpl-{millis}-{seq}")
}

// 编号用的递增计数
static SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

// 只在 cargo test 里跑的文件读写检查
#[cfg(test)]
mod tests {
    // 父模块里的保存和删除
    use super::{delete_at, list_at, save_at, sort_templates, CopyTemplate, SaveTemplateInput};
    // 读回文件文本，确认没有视频和封面
    use std::fs;
    // 每个测试用自己的临时路径
    use std::path::PathBuf;
    // 临时目录名带上时间
    use std::time::{SystemTime, UNIX_EPOCH};

    // 做一条只含文字字段的入参
    fn sample(name: &str) -> SaveTemplateInput {
        // 四个字段都有字
        SaveTemplateInput {
            // 名字
            name: name.to_string(),
            // 标题
            title: "标题".to_string(),
            // 简介
            summary: "简介".to_string(),
            // 标签
            tags: "标签".to_string(),
            // 分区
            partition: "生活".to_string(),
        }
    }

    // 在系统临时目录里造一个不会撞车的 JSON 路径
    fn scratch(name: &str) -> PathBuf {
        // 用纳秒区分并行测试
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        // 测试目录
        let dir = std::env::temp_dir().join(format!("tougao-fang-{name}-{nanos}"));
        // 建目录
        fs::create_dir_all(&dir).unwrap();
        // 正式文件名和正式程序一致
        dir.join("templates.json")
    }

    // 没有文件时列表是空的，并且不把文件造出来
    #[test]
    fn missing_file_lists_empty() {
        // 临时路径
        let path = scratch("empty");
        // 读取
        let items = list_at(&path).unwrap();
        // 没有模板
        assert!(items.is_empty());
        // 只是查看，不应创建文件
        assert!(!path.exists());
    }

    // 保存后能读回，视频和封面不会出现在文件里
    #[test]
    fn save_writes_text_fields_only() {
        // 临时路径
        let path = scratch("save");
        // 保存一条
        let saved = save_at(&path, sample("生活口播")).unwrap();
        // 这是新建
        assert!(saved.created);
        // 读回
        let items = list_at(&path).unwrap();
        // 只有一条
        assert_eq!(items.len(), 1);
        // 名字还在
        assert_eq!(items[0].name, "生活口播");
        // 分区还在
        assert_eq!(items[0].partition, "生活");
        // 文件文本
        let text = fs::read_to_string(&path).unwrap();
        // 没有视频字段
        assert!(!text.contains("\"video\""));
        // 没有封面字段
        assert!(!text.contains("\"cover\""));
    }

    // 同名再次保存只更新原来那条
    #[test]
    fn same_name_updates() {
        // 临时路径
        let path = scratch("update");
        // 第一次
        let first = save_at(&path, sample("生活口播")).unwrap();
        // 改一个字段再存同名
        let mut again = sample("生活口播");
        // 新标题
        again.title = "新标题".to_string();
        // 第二次
        let second = save_at(&path, again).unwrap();
        // 不是新建
        assert!(!second.created);
        // 编号不变
        assert_eq!(first.template.id, second.template.id);
        // 仍然只有一条
        assert_eq!(list_at(&path).unwrap().len(), 1);
        // 标题已换成新的
        assert_eq!(list_at(&path).unwrap()[0].title, "新标题");
    }

    // 删除后列表里没有它
    #[test]
    fn delete_removes_one() {
        // 临时路径
        let path = scratch("delete");
        // 先保存
        let saved = save_at(&path, sample("要删除")).unwrap();
        // 按编号删除
        delete_at(&path, &saved.template.id).unwrap();
        // 列表空了
        assert!(list_at(&path).unwrap().is_empty());
    }

    // 删一个不存在的编号要失败，原文件保持不变
    #[test]
    fn delete_missing_keeps_file() {
        // 临时路径
        let path = scratch("delete-missing");
        // 先有一条
        save_at(&path, sample("留下")).unwrap();
        // 记下原文
        let before = fs::read_to_string(&path).unwrap();
        // 删一个假编号
        let error = delete_at(&path, "missing").unwrap_err();
        // 提示没有这条
        assert_eq!(error, "没有这个模板");
        // 文件内容不变
        assert_eq!(fs::read_to_string(&path).unwrap(), before);
    }

    // 没写名字时不创建文件
    #[test]
    fn empty_name_does_not_create_file() {
        // 临时路径
        let path = scratch("noname");
        // 空白名字
        let error = save_at(&path, sample("   ")).unwrap_err();
        // 提示要填名字
        assert_eq!(error, "请先填写模板名字");
        // 文件不存在
        assert!(!path.exists());
    }

    // 旧文件里如果混进了视频和封面，下次保存会丢掉这两个字段
    #[test]
    fn resave_strips_video_and_cover() {
        // 临时路径
        let path = scratch("strip");
        // 先保证目录在
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        // 故意写进不该有的字段
        fs::write(
            &path,
            r#"{"templates":[{"id":"old","name":"旧","title":"","summary":"","tags":"","partition":"","updatedAt":1,"video":"secret.mp4","cover":"a.png"}]}"#,
        )
        .unwrap();
        // 再存一条新的，会把整个文件按结构重写
        save_at(&path, sample("新")).unwrap();
        // 重写后的文本
        let text = fs::read_to_string(&path).unwrap();
        // 视频路径不该留下
        assert!(!text.contains("secret.mp4"));
        // 封面路径不该留下
        assert!(!text.contains("a.png"));
        // 也不该再有这两个键
        assert!(!text.contains("\"video\""));
        // 封面键也没有
        assert!(!text.contains("\"cover\""));
    }

    // 文件坏了就不要覆盖
    #[test]
    fn corrupt_file_is_not_overwritten() {
        // 临时路径
        let path = scratch("corrupt");
        // 建目录
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        // 写成不是 JSON 的内容
        fs::write(&path, "not-json").unwrap();
        // 保存应失败
        let error = save_at(&path, sample("生活口播")).unwrap_err();
        // 提示损坏
        assert_eq!(error, "模板文件已损坏，没有改动它。");
        // 原文还在
        assert_eq!(fs::read_to_string(&path).unwrap(), "not-json");
    }

    // 排序把最近保存的放在第一条
    #[test]
    fn newest_is_first() {
        // 三条时间不同的模板
        let mut items = vec![
            // 较早
            CopyTemplate {
                id: "a".to_string(),
                name: "甲".to_string(),
                title: String::new(),
                summary: String::new(),
                tags: String::new(),
                partition: String::new(),
                updated_at: 1,
            },
            // 最近
            CopyTemplate {
                id: "b".to_string(),
                name: "乙".to_string(),
                title: String::new(),
                summary: String::new(),
                tags: String::new(),
                partition: String::new(),
                updated_at: 9,
            },
        ];
        // 排序
        sort_templates(&mut items);
        // 第一条是最近的
        assert_eq!(items[0].id, "b");
    }
}
