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
x = "https://gitee.com/src-openeuler/openjdk-11/raw/master/openjdk-11.spec"
y = "https://src.fedoraproject.org/rpms/java-11-openjdk/raw/rawhide/f/java-11-openjdk.spec"

[[addresses]]
name = "e2fsprogs"
x = "https://gitee.com/src-openeuler/e2fsprogs/raw/master/e2fsprogs.spec"
y = "https://src.fedoraproject.org/rpms/e2fsprogs/raw/rawhide/f/e2fsprogs.spec"

[[addresses]]
name = "bluez "
x = "https://gitee.com/src-openeuler/bluez/raw/master/bluez.spec"
y = "https://src.fedoraproject.org/rpms/bluez/raw/rawhide/f/bluez.spec"

[[addresses]]
name = "swtpm"
x = "https://gitee.com/src-openeuler/swtpm/raw/master/swtpm.spec"
y = "https://src.fedoraproject.org/rpms/swtpm/raw/rawhide/f/swtpm.spec"

[[addresses]]
name = "python-tomli"
x = "https://gitee.com/src-openeuler/python-tomli/raw/master/python-tomli.spec"
y = "https://src.fedoraproject.org/rpms/python-tomli/raw/rawhide/f/python-tomli.spec"

[[addresses]]
name = "firebird"
x = "https://gitee.com/src-openeuler/firebird/raw/master/firebird.spec"
y = "https://src.fedoraproject.org/rpms/firebird/raw/rawhide/f/firebird.spec"

[[addresses]]
name = "anaconda"
x = "https://gitee.com/src-openeuler/anaconda/raw/master/anaconda.spec"
y = "https://src.fedoraproject.org/rpms/anaconda/raw/rawhide/f/anaconda.spec"

[[addresses]]
name = "pango"
x = "https://gitee.com/src-openeuler/pango/raw/master/pango.spec"
y = "https://src.fedoraproject.org/rpms/pango/raw/rawhide/f/pango.spec"

[[addresses]]
name = "redland"
x = "https://gitee.com/src-openeuler/redland/raw/master/redland.spec"
y = "https://src.fedoraproject.org/rpms/redland/raw/rawhide/f/redland.spec"

[[addresses]]
name = "glib2"
x = "https://gitee.com/src-openeuler/glib2/raw/master/glib2.spec"
y = "https://src.fedoraproject.org/rpms/glib2/raw/rawhide/f/glib2.spec"
```

输出结果：
```bash
(base) ➜  release git:(master) ./specdiff -t false -r ../tmp  test.toml
report written successfully in ../tmp/openjdk-11-specdiff-2022-03-10 20:41:16.md
diff-ratio for openjdk-11 is: 0.49667898
report written successfully in ../tmp/e2fsprogs-specdiff-2022-03-10 20:41:19.md
diff-ratio for e2fsprogs is: 0.10695971
report written successfully in ../tmp/bluez -specdiff-2022-03-10 20:41:21.md
diff-ratio for bluez  is: 0.109042555
report written successfully in ../tmp/swtpm-specdiff-2022-03-10 20:41:23.md
diff-ratio for swtpm is: 0.61728394
report written successfully in ../tmp/python-tomli-specdiff-2022-03-10 20:41:24.md
diff-ratio for python-tomli is: 0.5371429
report written successfully in ../tmp/firebird-specdiff-2022-03-10 20:41:25.md
diff-ratio for firebird is: 0.14468086
report written successfully in ../tmp/anaconda-specdiff-2022-03-10 20:41:27.md
diff-ratio for anaconda is: 0.04122439
report written successfully in ../tmp/pango-specdiff-2022-03-10 20:41:30.md
diff-ratio for pango is: 0.05135135
report written successfully in ../tmp/redland-specdiff-2022-03-10 20:41:32.md
diff-ratio for redland is: 0.21333334
report written successfully in ../tmp/glib2-specdiff-2022-03-10 20:41:33.md
diff-ratio for glib2 is: 0.2825279
The avg_ratio is: 0.26002258
```