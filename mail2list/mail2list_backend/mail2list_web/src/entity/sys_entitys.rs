/**
*struct:SysUser
*desc:后台用户表
*author:zhaorunqi
*email:runqi@isrc.iscas.ac.cn
*/

/**
*struct:MailList
*desc:邮箱列表表
*author:zhaorunqi
*email:runqi@isrc.iscas.ac.cn
*/
#[crud_table(table_name:mail_list)]
#[derive(Clone, Debug)]
pub struct MailList {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub archive: Option<String>,
    pub description: Option<String>
}
impl_field_name_method!(MailList {
    id,
    name,
    email,
    archive,
    description
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