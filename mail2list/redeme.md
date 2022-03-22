# 邮箱管理页面使用

## 运行数据库

安装好openGauss数据库，运行db/mail_list.sql，得到对应的数据结构

## 创建数据

进入mail2list_backend，运行mail2list_clap，其中add代表增加数据，delete代表删除数据。select代表查看所有值。示例如下：

```shell
mail2list_clap.exe add --url <URL> --id <ID> --name <NAME> --email <EMAIL> --archive <ARCHIVE> --description <DESCRIPTION>
```

```shell
mail2list_clap.exe delete --url <URL> --id <ID>
```

```shell
hello-lettre.exe select --url <URL>
```

其中，url是数据库地址，剩下为表中字段，添加自己需要添加的值。

## 运行后端

运行mail2list_web，需要将其中database_url修改成自己的连接。然后运行cargo run 启动后端。

## 运行前端

使用npm install下载对应的包，在使用npm run dev运行程序。