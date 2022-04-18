use crate::entity::sys_entitys::ArchiveMailList;
use rbatis::rbatis::Rbatis;

//查询用户所有菜单
#[py_sql(" select p.id,p.name,p.from_email,p.create_time,p.subject,p.body,p.message_id,p.in_reply_to,p.reference,adf.create_time
    from archive_mail_list p inner join
    archive_mail_list adf on p.reference like CONCAT('%', adf.message_id, '%')
    where adf.message_id = #{message_id}")]
pub async fn get_message_list_by_message_id(rb: &Rbatis,message_id: &str) -> Option<Vec<ArchiveMailList>> {}

#[py_sql("select id,name,from_email,create_time,subject,body,message_id,in_reply_to,reference from archive_mail_list
        where reference is NULL and name=#{name} order by create_time desc")]
        pub async fn get_archive_list(rb: &Rbatis,name: &str) -> Option<Vec<ArchiveMailList>> {}
