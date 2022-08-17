# sync4src
## 简介

Rust编写的用于在两个仓库间建立单向同步的小工具。
> Note: 同步时会同步所有分支 (branch) 和标签 (tag)

## 使用
### 查看使用帮助
```shell
cargo run -- --help
```
```text
sync4src 0.1.0
Hengke Sun <sunhengke@sjtu.edu.cn>
A utility to sync between two repositories

USAGE:
    sync4src --config <FILE>

OPTIONS:
    -c, --config <FILE>    Config file path
    -h, --help             Print help information
    -V, --version          Print version information
```


### 进行同步
```shell
cargo run -- -c ./config.yaml
```
```text
sync branch origin/feat/test to target repo
sync branch origin/main to target repo
sync tag 0.0.1 to target repo
sync tag 1.0.0 to target repo
```

其中`-c`或`--config`为必需参数，指定配置文件路径。

配置文件支持YAML格式，内容形如：
```yaml
source:
  url: git@github.com:greenhandatsjtu/sync4src.git
  username: git
  ssh_key: /home/sunhengke/.ssh/id_rsa

target:
  url: https://gitee.com/sunhengke1/sync4src.git
  username: sunhengke1
  password: test123
```
其中，source为源同步仓库的配置，target为目标同步仓库的配置。具体来说：
+ `url`为仓库链接，支持HTTP(S)和SSH
+ `username`为用户名，为可选项，若不提供用户名，`sync4src`会尝试从`url`中读取用户名
+ `ssh_key`和`password`代表两种认证方式，SSH密钥认证或用户名密码认证；若使用SSH密钥认证，用户需要指定SSH密钥的文件路径，若使用用户名密码认证，用户需要提供密码；若两者都不提供，`sync4src`会尝试使用`ssh-agent`进行认证，或者读取`~/.ssh/id_rsa`作为SSH密钥


### 定时同步
使用`crontab`可以实现每小时或每日进行同步。
```shell
crontab -e
```
```text
# 每日
0 0 * * * .../sync4src -c .../config.yaml
# 每小时
0 * * * * .../sync4src -c .../config.yaml
```