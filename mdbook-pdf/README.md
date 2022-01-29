# mdbook-pdf

[开发文档](https://openeuler.feishu.cn/docs/doccnQFaB5OLABMDjhWSE1siX5b#)

用 Rust 编写的 [mdBook](https://github.com/rust-lang/mdBook) 后端，基于[headless chrome](https://github.com/atroche/rust-headless-chrome)和[Chrome开发工具协议](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-printToPDF)生成PDF。

## 用法
确保 Rust 编译环境可用（`cargo build`），然后在此文件夹中运行`cargo build --release`，在`target/release/`中获取可执行文件，并将其放入PATH。

为了使得程序能够正常运行，请确保计算机上安装了 Google Chrome / Chromium / Microsoft Edge，（安装在默认的位置，在当前的PATH中，或配置了二进制文件位置），因为现在自动下载 Chromium 功能还[不可用](https://github.com/atroche/rust-headless-chrome/issues/286).

一个最简单的`book.toml` 示例如下：

```toml
[book]
title = "An Example"

[output.html]

[output.pdf]
```

最后，您可以使用 `mdbook build` 命令生成书籍并获取PDF文件，您的PDF文件将被存放在`book/pdf/output.put`。

## 配置
支持自定义PDF纸张方向、页面缩放比例、纸张宽度和高度、页面边距、生成的PDF页面范围、是否显示页眉和页脚以及自定义其格式等。

查看 [book.toml](test_doc/book.toml#L10-L33) 以了解 `[output.pdf]` 可用配置的详细信息。

## 关于 [headless_chrome](headless_chrome)

由于直接使用上游 [headless_chrome](https://github.com/atroche/rust-headless-chrome) crate 对生成 PDF 不友好，容易导致超时错误。向上游提出了增加管理所有计时器的选项要求，但[上游不同意相关请求，只下放了部分计时器控制权限](https://github.com/atroche/rust-headless-chrome/issues/287)，因而无法满足需求。所以引入上游源代码文件（许可证为 MIT），并应用补丁，将相关PDF打印接口超时设置为5分钟，从而保证功能和编译。
