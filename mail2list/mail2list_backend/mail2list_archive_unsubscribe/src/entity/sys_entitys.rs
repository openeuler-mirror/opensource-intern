#[crud_table(table_name:subscribe_mail_list)]
#[derive(Clone, Debug)]
pub struct SubscribeMailList {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
}
impl_field_name_method!(SubscribeMailList {
    id,
    name,
    email,
    username,
});

#[crud_table(table_name:archive_mail_list)]
#[derive(Clone, Debug)]
pub struct ArchiveMailList {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub from_email: Option<String>,
    pub create_time: Option<String>,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub message_id: Option<String>,
    pub in_reply_to: Option<String>,
    pub reference:Option<String>,
    pub filename: Option<String>,
}
impl_field_name_method!(ArchiveMailList {
    id,
    name,
    from_email,
    create_time,
    subject,
    body,
    message_id,
    in_reply_to,
    reference,
    filename,
});
/**
*struct:CommonField
*desc:所有表的公共字段 CRUD_SERVICE使用
*author:zhaorunqi
*email:runqi@isrc.iscas.ac.cn
*/
#[derive(Clone, Debug)]
pub struct CommonField {
    pub id: Option<i64>,
}