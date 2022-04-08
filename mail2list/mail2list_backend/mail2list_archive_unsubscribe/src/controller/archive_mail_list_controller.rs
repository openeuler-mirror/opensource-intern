use crate::CONTEXT;

pub async fn delete(imap_server: &str, mine_email :&str,smtp_server: &str, password :&str, name :&str) {
    CONTEXT.subscribe_mail_list_service.delete(imap_server, mine_email, smtp_server,password, "退订成功","",name).await;
}

pub async fn save(mine_email :&str,smtp_server: &str, password :&str, name :&str) {
    CONTEXT.archive_mail_list_service.save_info(smtp_server, mine_email, password,name).await;
}