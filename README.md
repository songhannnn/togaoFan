# 投稿方

Windows 桌面程序的第一版空窗口。打开后可以在首页、编辑、结果、设置之间切换。

窗口里有首页、编辑、结果、设置。草稿和投稿记录还不会写入本机。编辑页底部可以把标题、分区、标签和简介存成模板，写在这台电脑的应用数据目录里。视频和封面不会进模板。哔哩哔哩投稿、抖音请求、AI 写文案都还没有接上。

## 在 Windows 上先安装

1. Node.js（安装后命令行里能用 `npm`）
2. Rust（用 rustup 安装稳定版）
3. Visual Studio C++ 生成工具，勾选「使用 C++ 的桌面开发」

系统若是较旧的 Windows 10，还需要 WebView2 运行时。Windows 11 一般已经带了。详细说明见 Tauri 的先决条件文档：https://tauri.app/start/prerequisites/

## 开发

在这个目录打开终端：

```
npm install
npm run tauri dev
```

## 打出 exe

依赖装好之后：

```
npm run tauri build
```

请在 Windows 上执行。安装包会出现在 `src-tauri/target/release/bundle/`。

密钥不要写进源码，也不要写进这个说明。设置页这一步不填写凭证。

## 文案模板

编辑页填好标题、分区、标签、简介后，在底部写一个模板名字，点「保存模板」。关掉窗口再打开，模板还在。选择已有模板后点「套用模板」，这四个字段会被填上。视频和封面保持不动。点「删除」会去掉这一条。同名再次保存会更新原来的模板。

文件在 Tauri 的应用数据目录，名字是 `templates.json`。Windows 上一般是：

`C:\Users\<用户名>\AppData\Roaming\com.tougaofang.app\templates.json`

编辑页底部也会写出当前这台电脑上的实际路径。

这一版改了 Rust 命令，并新增了 Element Plus。请重新执行 `npm install`，然后重新运行 `npm run tauri dev`。只改界面热更新不够，Rust 命令要重启后才进窗口。
