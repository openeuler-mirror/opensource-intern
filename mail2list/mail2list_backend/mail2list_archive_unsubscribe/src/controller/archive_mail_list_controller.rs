use crate::CONTEXT;
pub async fn save(mine_email :&str,imap_server: &str, smtp_server: &str,password :&str, name :&str) {
    CONTEXT.archive_mail_list_service.save_info(imap_server, smtp_server,mine_email, password,name).await;
}