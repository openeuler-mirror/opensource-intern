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
mail2list_clap.exe select --url <URL>
```

其中，url是数据库地址，剩下为表中字段，添加自己需要添加的值。

## 配置环境变量

数据库用户名

export OPENGUASS_USER=xxxxx

数据库密码

export  OPENGUASS_PWD=xxxxx

数据库地址

export OPENGUASS_URL=xxxx

数据库端口

export OPENGUASS_PORT=xxxxxx

数据库名称

export OPENGUASS_DATABASE=xxxx

订阅邮箱邮箱地址（可能包含多个订阅地址，使用数组加入）

export mine_email=[XXXXXX,XXXXXX]

订阅邮箱smtp服务（可能包含多个订阅地址，使用数组加入）

export smtp_server=[xxxxx,xxxxx]

订阅邮箱授权码

export password=[xxxxx,xxxx]

订阅邮箱imap地址

export imap_server=[xxxx,xxxx]

归档邮箱箱邮箱地址

export archive_mine_email=[XXXXXX,XXXXXX]

归档邮箱smtp服务（可能包含多个订阅地址，使用数组加入）

export archive_smtp_server=[xxxxx,xxxxx]

归档邮箱授权码

export archive_password=[xxxxx,xxxx]

归档邮箱名字（对应的是每个归档邮箱所对应列表姓名）

export archive_name=[xxxxx,xxxx]

归档邮箱imap地址

export archive_imap_server=[xxxx,xxxx]

归档邮箱箱邮箱地址

export leave_email=[XXXXXX,XXXXXX]

归档邮箱smtp服务（可能包含多个订阅地址，使用数组加入）

export leave_smtp_server=[xxxxx,xxxxx]

归档邮箱授权码

export leave_email_password=[xxxxx,xxxx]

归档邮箱名字（对应的是每个归档邮箱所对应列表姓名）

export leave_name=[xxxxx,xxxx]

归档邮箱imap地址

export leave_imap_server=[xxxx,xxxx]

**注意：每个数组元素中是一一对应得关系，如mine_email[0]与password[0]对应的是同一个邮箱内容，以此类推**

## 运行后端

1. 进入后端中的包mail2list_web中。
2. 运行mail2list_web，需要将application.yml其中database_url修改成自己的连接。以及对应的邮箱地址进行修改，然后运行cargo run --bin mail2list_web 启动后端。
3. 同时需要运行mail2list_archive_unsubscribe，然后运行cargo run --bin mail2list_archive_unsubscribe启动邮箱订阅归档以及退订功能。



## 运行前端

1. 进入前端对应的vue-mail2list-web中。
2. 使用npm install下载对应的包，在使用npm run dev运行程序。

## 镜像运行

### 运行前端

1. 进入前端对应的vue-mail2list-web中。

2. 运行Dockerfile，构建镜像

3. 运行镜像，使用

   ```dockerfile
   docker run -p <port>:9999 -e mine_email=<mine_email> -e smtp_server=<smtp_server> -e password=<password> -e imap_server=<imap_server> -e archive_mine_email=<archive_mine_email> -e archive_smtp_server=<archive_smtp_server> -e archive_password=<archive_password> -e archive_name=<archive_name> -e archive_imap_server=<archive_imap_server> -e leave_email=<leave_email> -e leave_email_password=<leave_email_password> -e leave_email_password=<leave_email_password> -e leave_smtp_server=<leave_smtp_server> -e leave_imap_server=<leave_imap_server> -e OPENGUASS_USER=<OPENGUASS_USER> -e OPENGUASS_PWD=<OPENGUASS_PWD> -e OPENGUASS_URL=<OPENGUASS_URL> -e OPENGUASS_PORT=<OPENGUASS_PORT> -e OPENGUASS_DATABASE=<OPENGUASS_DATABASE>  <IMAGE>
   ```

   填入对应的参数，即可运行

### 运行后端

1. 进入后端中的包mail2list_web中

2. 运行Dockerfile，构建镜像

3. 运行镜像，使用

   ```dockerfile
   docker run -p <port>:8081 <IMAGE>
   ```

   

