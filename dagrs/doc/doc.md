[TOC]

# Overview

DAG Engine 有四个主要文件：

- dag_engine.rs - Engine 的实现，具体功能包括从 YAML 读取、形成图、判断环和输出。
- task.rs - 任务结构的实现。
- graph.rs - 图的实现，用来记录任务之间的依赖关系。
- error_handler.rs - 错误处理，只有最简单的错误输出。



四个文件的关系如下：

```
                    dag_engine
          /             |               \
       /                |                  \
   task.rs           graph.rs        error_handler
      |                 |
error_handler      error_handler
```

