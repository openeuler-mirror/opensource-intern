## Pijul v1.0.0-beta.2 RPM 包构建
- 构建与编译cargo项目，需要Rust-1.56.1以上版本工具，否则会编译失败
- 构建与编译项目需要系统安装的xxhash无法通过open Euler（21.09及以前）仓库使用yum直接安装，需要使用网络上的xxhash源码包或open Euler（22.03版本以后）的everything仓库中下载安装。
- 文件目录：

    | pijul\-1\.0\.0~beta\.2\.tar\.gz            | Pijul Rust源码  |
    |:------------------------------------------:|:-------------:|
    | pijul\-1\.0\.0~beta\.2\-1\.0\.src\.rpm     | 已编译二进制源码包     |
    | pijul\-1\.0\.0~beta\.2\-1\.0\.x86\_64\.rpm | Pijul rpm安装包  |
    | pijul\.spec                                | SPEC 文件       |
