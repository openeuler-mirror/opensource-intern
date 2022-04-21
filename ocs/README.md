# openEuler Cheat Sheet

## 运行

前端需要 `yarn install` 添加依赖包
然后可以使用 `yarn dev` 进行预览

需要使用 `yarn build` 进行前端构建
然后在后端运行 `cargo run`
后端构建需要使用 `cargo build --release`

## 贡献软件包

在 `site/docs/02.软件包/`目录下创建 markdown 文件。
文件名为 `序号.xxx.md` xxx 为软件包名

可以用 h2 标题创建一个 描述。

这里我们用到一个组件 ，需要提供操作系统的名称，和安装命令。

```md
<Pkg osName="Fedora" bashCode="dnf install conda"/>
```

参考 `01.sudo.md` 文件

## 文档开发

使用 vuepress 构建和 vdoing 主题。参考 vuepress markdown 和 vdoing 书写规范。侧边栏需要修改 config.js。

在更新文档后运行 `yarn build` 构建前端界面
