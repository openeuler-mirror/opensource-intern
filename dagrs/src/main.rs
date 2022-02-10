/*!
本项目是用 Rust 写的 DAG 执行引擎，开发文档请参考：[使用 Rust 编写 DAG 执行引擎](https://openeuler.feishu.cn/docs/doccnVLprAY6vIMv6W1vgfLnfrf)。

## 用法

确保 Rust 编译环境可用（`cargo build`），然后在此文件夹中运行`cargo build --release`，在`target/release/`中获取可执行文件，并将其放入PATH。

命令行使用方式为：

```bash
$ dagrs -h
dagrs 0.1.0

USAGE:
    dagrs --filepath <FILEPATH>

OPTIONS:
    -f, --filepath <FILEPATH>    YAML file path
    -h, --help                   Print help information
    -V, --version                Print version information
```

例如:

```bash
$ dagrs -f test/test_dag.yaml
[Start] -> d -> a -> b -> f -> c -> g -> e -> h -> [End]
```



## YAML 定义

YAML 定义 DAG 通过如下一个例子来说明：

```YAML
dagrs:
    a:
        name: "任务1"
        rely: [b, c]
    b:
        name: "任务2"
        rely: [c]
    c:
        name: "任务3"
        # empty if no rely
```

- 这里有三个任务：a，b 和 c。这里的 a，b，c 并不是 `name` ，而是标识任务的标识，可以认为是 ID。
- 例如 a 指向 b 和 c，表示 a 在 b，c 之前执行，写做 `rely: [b, c]` 。
- **注意，重复的 ID 会覆盖之前的任务定义。**

最终形成的图如下，那么一个可行的执行顺序是： `A->B->C->D->E`

<img src="https://tva1.sinaimg.cn/large/008i3skNgy1gz3qe2or2qj30mg0ky0td.jpg" alt="img" style="zoom:33%;" />

具体的任务定义可以后续随需要进行扩展。
 */

extern crate bimap;
extern crate clap;
extern crate yaml_rust;

mod dag_engine;
mod error_handler;
mod graph;
pub mod task;

use clap::Parser;
use dag_engine::DagEngine;
use task::Task;

#[derive(Parser)]
#[clap(version)]
/// Command Line input
struct Args {
    /// YAML file path
    #[clap(short, long)]
    filepath: String,
}

fn main() {
    let args = Args::parse();
    let mut dagrs: DagEngine<Task> = DagEngine::new();

    if let Err(e) = dagrs.run(&args.filepath) {
        println!("[Error] {}", e);
    }
}

#[test]
fn test_dag() {
    let res = DagEngine::<Task>::new().run("test/test_dag.yaml").unwrap();
    assert_eq!(res, true)
}

#[test]
fn test_loop() {
    let res = DagEngine::<Task>::new().run("test/test_loop1.yaml").unwrap();
    assert_eq!(res, false)
}

#[test]
fn test_complex_loop() {
    let res = DagEngine::<Task>::new().run("test/test_loop2.yaml").unwrap();
    assert_eq!(res, false)
}

#[test]
fn test_format_error1() {
    use error_handler::{DagError, FormatErrorMark};
    let res = DagEngine::<Task>::new().run("test/test_error1.yaml");
    assert_eq!(
        res,
        Err(DagError::format_error("a".into(), FormatErrorMark::NoName))
    );
}

#[test]
fn test_format_error2() {
    use error_handler::{DagError, FormatErrorMark};
    let res = DagEngine::<Task>::new().run("test/test_error2.yaml");
    assert_eq!(
        res,
        Err(DagError::format_error("".into(), FormatErrorMark::StartWordError))
    );
}


#[test]
fn test_rely_error() {
    use error_handler::{DagError, FormatErrorMark};
    let res = DagEngine::<Task>::new().run("test/test_error3.yaml");
    assert_eq!(
        res,
        Err(DagError::format_error("a".into(), FormatErrorMark::RelyIDIllegal))
    );
}