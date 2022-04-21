#!/bin/bash
# 命令后加入 & ，保持程序后台持续运行
 ./mail2list_web &
 ./mail2list_archive_unsubscribe &
# 死循环，保持docker前台运行
while [[ true ]]; do
    sleep 1
done