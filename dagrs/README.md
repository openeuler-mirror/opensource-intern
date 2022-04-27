# dagrs

本项目是用 Rust 写的 DAG 执行引擎，开发文档请参考：[使用 Rust 编写 DAG 执行引擎](https://openeuler.feishu.cn/docs/doccnVLprAY6vIMv6W1vgfLnfrf)。



## 用法

确保 Rust 编译环境可用（`cargo build`），然后在此文件夹中运行`cargo build --release`，在`target/release/`中获取可执行文件，并将其放入PATH。

本项目面向两个目标群体：

- 普通用户 - 通过 `YAML` 文件对任务进行定义和调度运行。
- 程序员 - 通过实现 `Task Trait` 进行任务的定义和调度运行。



## YAML

此部分是面向普通用户的，即用户并不通过 rust 编程，而是利用 YAML 文件对任务进行描述并调度运行。YAML 文件的一个示例如下：

```yaml
dagrs:
  a:
    name: 任务1
    rely: [b]
    run:
      type: sh
      script: ./test/test.sh
  b:
    name: "任务2"
    run:
      type: deno
      script: print("Hello!")
```

- YAML 文件应该以 `dagrs` 开头。

- `a,b` 是任务的标识符（也可理解为 ID），主要作为标识使用，无具体含义。该字段必须存在且不能重复（否则会覆盖早先定义）。
- `name` 是任务的名称，在后续调度时会输出到 log 中以便用户查看。该字段必须存在，可以重复。
- `rely` 是任务的依赖关系，如 `rely: [b]` 表示 `a` 应该在 `b` **之前**执行（注意不要弄反）。该字段可以省略。
- `run` 是任务的内容定义，包括 `type` 和 `script` 两个子字段。该字段及其子字段必须存在。
  - `type` 是任务的执行方式，当前支持 shell 执行（sh），和 deno 执行（deno）。
  - `script` 是任务的执行内容，可以是具体的命令，也可以是一个文件。



一个稍复杂的例子：

```yaml
dagrs:
  a:
    name: "任务1"
    rely: [b, c]
    run:
      type: sh
      script: echo a
  b:
    name: "任务2"
    rely: [c, f, g]
    run:
      type: sh
      script: echo b
  c:
    name: "任务3"
    rely: [e, g]
    run:
      type: sh
      script: echo c
  d:
    name: "任务4"
    rely: [c, e]
    run:
      type: sh
      script: echo d
  e:
    name: "任务5"
    rely: [h]
    run:
      type: sh
      script: echo e
  f:
    name: "任务6"
    rely: [g]
    run:
      type: deno
      script: Deno.core.print("f\n")
  g:
    name: "任务7"
    rely: [h]
    run:
      type: deno
      script: Deno.core.print("g\n")
  h:
    name: "任务8"
    run:
      type: sh
      script: ./test/test.sh
```



**如何运行？**

在编写好 YAML 文件后，可以通过 cli 进行运行：

```bash
$ ./target/debug/dagrs --help
dagrs 0.1.0
Command Line input

USAGE:
    dagrs [OPTIONS] --filepath <FILEPATH>

OPTIONS:
    -f, --filepath <FILEPATH>    YAML file path
    -h, --help                   Print help information
    -l, --logpath <LOGPATH>      Log file path
    -V, --version                Print version information
```

例如运行上述第一个 YAML 的情况：

```bash
$ ./target/debug/dagrs -f test/test_dag1.yaml
02:50:59 [INFO] [Start] -> 任务1 -> 任务2 -> [End]
02:50:59 [INFO] Executing Task[name: 任务1]
exec sh file success
02:50:59 [INFO] Task[name: 任务1] exec done, success: true, return value: 
02:50:59 [INFO] Executing Task[name: 任务2]
cargo:rerun-if-changed=/Users/wyffeiwhe/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/deno_core-0.121.0/00_primordials.js
cargo:rerun-if-changed=/Users/wyffeiwhe/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/deno_core-0.121.0/01_core.js
cargo:rerun-if-changed=/Users/wyffeiwhe/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/deno_core-0.121.0/02_error.js
02:50:59 [INFO] Task[name: 任务2] exec done, success: false, return value: ReferenceError: print is not defined
    at <anonymous>:1:1
```

可以看到详细的运行情况输出，同时 log 文件可在 `$HOME/.dagrs/dagrs.log$` 下找到（这是默认地址，可以通过 `-l` 选项来自定义。

log 文件记录任务的执行顺序以及执行结果，其内容如下：

```log
02:50:59 [INFO] [Start] -> 任务1 -> 任务2 -> [End]
02:50:59 [INFO] Executing Task[name: 任务1]
02:50:59 [INFO] Task[name: 任务1] exec done, success: true, return value: 
02:50:59 [INFO] Executing Task[name: 任务2]
02:50:59 [INFO] Task[name: 任务2] exec done, success: false, return value: ReferenceError: print is not defined
    at <anonymous>:1:1
```





## TaskTrait

程序员可以通过实现 `TaskTrait` 来更灵活的定义自己的任务。 `TaskTrait` 的定义如下：

```rust
/// Task Trait.
///
/// Any struct implements this trait can be added into dagrs.
pub trait TaskTrait {
    fn run(&self) -> Option<Retval>;
}
```

- `run` 是任务的执行内容，在被调度执行时由 dagrs 调用。
- `Retval` 是任务的返回值，现在还不支持，**请设置为 None**。



程序员实现的 task struct 需要放到 `TaskWrapper` 中进行使用，并通过其提供的 `add_rely` 函数进行依赖设置，具体可见下方的例子。



**如何使用？**

一个例子如下：

```rust
use crate::task::{TaskTrait, TaskWrapper, Retval};
// Set log file path
init_logger(Some("./dagrs.log"));

// Define a task struct
struct T1 {}
impl TaskTrait for T1 {
  fn run(&self) -> Option<Retval> {
    println!("T1!");
    None
  }
}

struct T2 {}
impl TaskTrait for T2 {
  fn run(&self) -> Option<Retval> {
    println!("T2!");
    None
  }
}

// Instance
let mut t1 = TaskWrapper::new(T1 {}, "Task 1");
let mut t2 = TaskWrapper::new(T2 {}, "Task 2");
let t3 = TaskWrapper::new(T1 {}, "Task 3");

// Set up rely
t2.add_relys(&[&t1, &t3]);
t1.add_relys(&[&t3]);

let mut dag = DagEngine::new();
dag.add_task(t1);
dag.add_task(t2);
dag.add_task(t3);

dag.run().unwrap();
```

执行的输出如下：

```bash
03:41:40 [INFO] [Start] -> Task 2 -> Task 1 -> Task 3 -> [End]
03:41:40 [INFO] Executing Task[name: Task 2]
T2!
03:41:40 [INFO] Executing Task[name: Task 1]
T1!
03:41:40 [INFO] Executing Task[name: Task 3]
T1!
```

log 文件如下：

```log
03:41:40 [INFO] [Start] -> Task 2 -> Task 1 -> Task 3 -> [End]
03:41:40 [INFO] Executing Task[name: Task 2]
03:41:40 [INFO] Executing Task[name: Task 1]
03:41:40 [INFO] Executing Task[name: Task 3]
```



**如何运行脚本？**

程序员可以通过 `RunScript` 结构来实现脚本的运行（当然也可以直接在代码里自行运行而不通过该结构体），定义如下：

```rust
pub struct RunScript {
    script: String,
    executor: RunType,
}

pub enum RunType {
    SH,
    DENO,
}
```

`RunScript` 本身提供了 `exec` 函数，故可以将 `TaskTrait` 中的 `run` 函数实现为 `RunScript` 的 `exec` 来实现运行脚本，一个例子如下：

```rust
use crate::task::{RunScript, RunType, TaskTrait, TaskWrapper, Retval};
// Set log file path
init_logger(Some("./dagrs.log"));

struct T {
  run_script: RunScript,
}
// Wrap exec in run
impl TaskTrait for T {
  fn run(&self) -> Option<Retval> {
    Some(self.run_script.exec())
  }
}

let mut t1 = TaskWrapper::new(
  T {
    run_script: RunScript::new("echo T1", RunType::SH),
  },
  "Task 1",
);
let mut t2 = TaskWrapper::new(
  T {
    run_script: RunScript::new("echo T2", RunType::SH),
  },
  "Task 2",
);
let t3 = TaskWrapper::new(
  T {
    run_script: RunScript::new(r#"Deno.core.print("T3\n")"#, RunType::DENO),
  },
  "Task 3",
);

t2.add_relys(&[&t1, &t3]);
t1.add_relys(&[&t3]);

let mut dag = DagEngine::new();
dag.add_task(t1);
dag.add_task(t2);
dag.add_task(t3);

dag.run().unwrap();
```

运行结果为：

```bash
03:47:55 [INFO] [Start] -> Task 2 -> Task 1 -> Task 3 -> [End]
03:47:55 [INFO] Executing Task[name: Task 2]
T2
03:47:55 [INFO] Task[name: Task 2] exec done, success: true, return value: 
03:47:55 [INFO] Executing Task[name: Task 1]
T1
03:47:55 [INFO] Task[name: Task 1] exec done, success: true, return value: 
03:47:55 [INFO] Executing Task[name: Task 3]
cargo:rerun-if-changed=/Users/wyffeiwhe/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/deno_core-0.121.0/00_primordials.js
cargo:rerun-if-changed=/Users/wyffeiwhe/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/deno_core-0.121.0/01_core.js
cargo:rerun-if-changed=/Users/wyffeiwhe/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/deno_core-0.121.0/02_error.js
T3
03:47:55 [INFO] Task[name: Task 3] exec done, success: true, return value: Global { data: 0x7f864600d2a0, isolate_handle: IsolateHandle(IsolateAnnex { isolate: 0x0, isolate_mutex: Mutex { data: (), poisoned: false, .. } }) }
```

log 文件如下：

```log
03:47:55 [INFO] [Start] -> Task 2 -> Task 1 -> Task 3 -> [End]
03:47:55 [INFO] Executing Task[name: Task 2]
03:47:55 [INFO] Task[name: Task 2] exec done, success: true, return value: 
03:47:55 [INFO] Executing Task[name: Task 1]
03:47:55 [INFO] Task[name: Task 1] exec done, success: true, return value: 
03:47:55 [INFO] Executing Task[name: Task 3]
03:47:55 [INFO] Task[name: Task 3] exec done, success: true, return value: Global { data: 0x7f864600d2a0, isolate_handle: IsolateHandle(IsolateAnnex { isolate: 0x0, isolate_mutex: Mutex { data: (), poisoned: false, .. } }) }
```


TODO:
- [ ] 增加 Task 输入输出值功能，定义输入输出值传递的条件
- [ ] 增加 Engine 整体环境变量功能
- [ ] 增加 Task 和 Engine 任务执行成功和失败的功能
- [ ] 优化错误处理
- [ ] 优化 Log