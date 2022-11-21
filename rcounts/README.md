## 使用 Rust 语言开发相应的代码统计工具，针对单个代码仓库、多代码仓库等进行分类统计，生成 Markdown 等格式的统计报告

#### 运行

1. 运行程序后控制台会输出:
Enter 1 to generate statistics for a single code repository
Enter 2 to perform code aggregate statistics for all warehouses in the organization:
的提示。输入1来统计单个仓库输入2来统计一个组织的所有仓库。
2. 若输入1，程序会直接自动开始在当前目录clone仓库并统计数据，文件夹名称为仓库名。
3. 输入2，控制台会输出:
Enter 1 for gitee, enter 2 for github:
的提示。输入1来选择Gitee的组织仓库，输入2来选择GitHub的组织仓库。
4. 然后控制台会先输出:
Input organization name:
的提示，来让用户输入组织名称。
5. 然后控制台输出:
Input token:
的提示。来让用户输入token。
6. 输入之后，程序会自动在当前目录clone组织仓库，文件夹名称为组织名。
7. 统计完成后，会生成markdown和csv文件来记录统计结果，并在控制台输出结果。