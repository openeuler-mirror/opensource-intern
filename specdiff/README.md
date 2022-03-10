# specdiff

本项目是使用 rust 编写的简易 spec 文件比较程序，支持 toml 格式的配置文件输入。允许在控制台输出以及生成 diff 报告，输出报告格式为 markdown。开发文档请参考：[使用 Rust 编写比较 RPM Spec 工具](https://openeuler.feishu.cn/docs/doccnppT2EFvz9AoOmtfkZjAKjc)




## 编译 & 用法

确保 Rust 编译环境可用（`cargo build`），然后在此文件夹中运行`cargo build --release`，在`target/release/`中获取可执行文件，并将其放入PATH。

通过执行 `specdiff -h` 我们可以获取帮助。specdiff 命令行输入格式如下：

```
specdiff [OPTIONS] <CONFIG PATH>
```
### TOML 配置文件
我们需要一个 TOML 格式的配置文件地址 CONFIG PATH。

一个 配置文件示例如下，其中包括软件名字 name, 输出报告名称 out_name, 软件不同的spec文件地址 x 和 y (都是 String 类型)。除了 `out_name` 以外其他字段必填。多组软件对比放在其他 `[[addresses]]` 下即可。
```toml
[[addresses]]
name = "fpaste"  # 软件名 
out_name = "fpaste_34" # 自定义输出报告文件名，可以为空。默认输出报告文件名为：<name>-specdiff-<date>.md
x = "https://src.fedoraproject.org/rpms/fpaste/raw/rawhide/f/fpaste.spec"
y = "https://src.fedoraproject.org/rpms/fpaste/raw/f34/f/fpaste.spec"
```

此外，程序会通过上述地址下载两个 spec 文件，默认存放在 `/tmp/specdiff/download` 目录下，下载目录可通过命令行参数修改。

### 命令行参数
```
OPTIONS:
    -h, --help
            Print help information

    -r, --report-out-path <REPORT_OUT_PATH>
            specdiff 输出的对比报告存放的路径, 默认为当前目录

    -s, --spec-save-path <SPEC_SAVE_PATH>
            指定下载 spec 文件的保存目录, 默认为 /tmp/specdiff/download/

    -t, --terminal-out <TERMINAL_OUT>
            是否在控制台输出 diff 内容, 默认为 true

    -V, --version
            Print version information
```

## 一个例子
执行命令的实例：
```bash
$ ./target/debug/specdiff -t false test.toml
```
test.toml
```toml
[[addresses]]
name = "openjdk-11"
x = "https://gitee.com/src-openeuler/openjdk-11/raw/openEuler-22.03-LTS-LoongArch/openjdk-11.spec"
y = "https://gitee.com/src-openeuler/openjdk-11/raw/master/openjdk-11.spec"

[[addresses]]
name = "e2fsprogs"
x = "https://gitee.com/src-openeuler/e2fsprogs/raw/master/e2fsprogs.spec"
y = "https://gitee.com/src-openeuler/e2fsprogs/raw/openEuler-20.09/e2fsprogs.spec"

[[addresses]]
name = "obs_meta "
x = "https://gitee.com/src-openeuler/obs_meta/raw/master/master/openEuler:Mainline:Bak/openEuler-latest-release/openEuler-latest-release.spec"
y = "https://gitee.com/src-openeuler/obs_meta/raw/revert-merge-1126-master/master/openEuler:Mainline:Bak/openEuler-latest-release/openEuler-latest-release.spec"

[[addresses]]
name = "docker"
x = "https://gitee.com/src-openeuler/docker/raw/master/docker.spec"
y = "https://gitee.com/src-openeuler/docker/raw/openEuler-22.03-LTS/docker.spec"

[[addresses]]
name = "python-tomli"
x = "https://gitee.com/src-openeuler/python-tomli/raw/master/python-tomli.spec"
y = "https://gitee.com/src-openeuler/python-tomli/raw/sync-pr3-pr_2-to-openEuler-22.03-LTS/python-tomli.spec"

[[addresses]]
name = "a2jmidid"
x = "https://src.fedoraproject.org/rpms/a2jmidid/raw/rawhide/f/a2jmidid.spec"
y = "https://src.fedoraproject.org/rpms/a2jmidid/raw/f23/f/a2jmidid.spec"

[[addresses]]
name = "qpwgraph"
x = "https://src.fedoraproject.org/rpms/qpwgraph/raw/rawhide/f/qpwgraph.spec"
y = "https://src.fedoraproject.org/rpms/qpwgraph/f34/rawhide/f/qpwgraph.spec"

[[addresses]]
name = "c4log"
x = "https://src.fedoraproject.org/rpms/c4log/raw/rawhide/f/c4log.spec"
y = "https://src.fedoraproject.org/rpms/c4log/f34/rawhide/f/c4log.spec"

[[addresses]]
name = "apptainer"
x = "https://src.fedoraproject.org/rpms/apptainer/raw/rawhide/f/apptainer.spec"
y = "https://src.fedoraproject.org/rpms/apptainer/f35/rawhide/f/apptainer.spec"

[[addresses]]
name = "bitcoin-core-selinux"
x = "https://src.fedoraproject.org/rpms/bitcoin-core-selinux/raw/rawhide/f/bitcoin-core-selinux.spec"
y = "https://src.fedoraproject.org/rpms/bitcoin-core-selinux/f34/rawhide/f/bitcoin-core-selinux.spec"
```

输出结果：
```
(base) ➜  release git:(master) ✗ ./specdiff -t false  test.toml    
report written successfully in ./openjdk-11-specdiff-2022-03-10 17:27:30.md
diff-ratio for openjdk-11 is: 0.9831144
report written successfully in ./e2fsprogs-specdiff-2022-03-10 17:27:31.md
diff-ratio for e2fsprogs is: 0.877551
report written successfully in ./obs_meta -specdiff-2022-03-10 17:27:31.md
diff-ratio for obs_meta  is: 1
report written successfully in ./docker-specdiff-2022-03-10 17:27:32.md
diff-ratio for docker is: 0.95555556
report written successfully in ./python-tomli-specdiff-2022-03-10 17:27:32.md
diff-ratio for python-tomli is: 1
report written successfully in ./a2jmidid-specdiff-2022-03-10 17:27:33.md
diff-ratio for a2jmidid is: 0.6576271
report written successfully in ./qpwgraph-specdiff-2022-03-10 17:27:35.md
diff-ratio for qpwgraph is: 0
report written successfully in ./c4log-specdiff-2022-03-10 17:27:36.md
diff-ratio for c4log is: 0
report written successfully in ./apptainer-specdiff-2022-03-10 17:27:38.md
diff-ratio for apptainer is: 0
report written successfully in ./bitcoin-core-selinux-specdiff-2022-03-10 17:27:39.md
diff-ratio for bitcoin-core-selinux is: 0
The avg_ratio is: 0.5473848
```