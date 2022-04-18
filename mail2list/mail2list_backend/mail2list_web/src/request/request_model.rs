use serde::{Deserialize, Serialize};

/**
*struct:SysMenuQuery
*desc:菜单查询
*author:zhaorunqi
*email:runqi@isrc.iscas.ac.cn
*/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MailListQuery {
    pub ids: Option<Vec<i64>>,
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubscribeMailListQuery {
    pub id: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArchiveMailListQuery {
    pub ids: Option<Vec<i64>>,
    pub message_id: Option<String>,
    pub name: Option<String>,
}