use chrono::prelude::*;
use mail2list_archive_unsubscribe::{
    controller::archive_mail_list_controller, controller::subscribe_mail_list_controller,
    MAIL2LIST_CONFIG,
};

extern crate chrono;
extern crate imap;

#[tokio::main]
async fn main() {
    loop {
        tokio::spawn(async {
            email().await;
            true
        })
        .await
        .unwrap();
    }
}

async fn email() {
    let len = MAIL2LIST_CONFIG.email.leave_imap_server.len();
    for i in 0..len {
        let save = archive_mail_list_controller::save(
            &MAIL2LIST_CONFIG.email.archive_mine_email[i],
            &MAIL2LIST_CONFIG.email.archive_imap_server[i],
            &&MAIL2LIST_CONFIG.email.archive_smtp_server[i],
            &MAIL2LIST_CONFIG.email.archive_password[i],
            &MAIL2LIST_CONFIG.email.archive_name[i],
        );
        let delete = subscribe_mail_list_controller::delete(
            &MAIL2LIST_CONFIG.email.leave_imap_server[i],
            &MAIL2LIST_CONFIG.email.leave_email[i],
            &MAIL2LIST_CONFIG.email.leave_smtp_server[i],
            &MAIL2LIST_CONFIG.email.leave_email_password[i],
            &MAIL2LIST_CONFIG.email.leave_name[i],
        );
        futures::join!(save, delete);
    }
}
