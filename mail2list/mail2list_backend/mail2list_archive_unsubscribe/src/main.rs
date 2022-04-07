use mail2list_archive_unsubscribe::{
    controller::subscribe_mail_list_controller,
    controller::archive_mail_list_controller,
    MAIL2LIST_CONFIG,
};




/**
 *method:main
 *desc:退订以及归档 需要长时间自动访问 因此单独提取出来一个程序
 *author:zhaorunqi
 *email:348040933QQ.com
 */
#[tokio::main]
async fn main() {
    //此处直接开始监控删除并且一直监控
    //单独提出来一个程序 单独运行 并且如果找到的话记得删除邮件
    loop{
        //subscribe_mail_list_controller::delete(&MAIL2LIST_CONFIG.email.leave_smtp_server,&MAIL2LIST_CONFIG.email.leave_email,&MAIL2LIST_CONFIG.email.leave_smtp_server,&MAIL2LIST_CONFIG.email.leave_email_password, &MAIL2LIST_CONFIG.email.leave_name).await;
        archive_mail_list_controller::save(&MAIL2LIST_CONFIG.email.leave_email,&MAIL2LIST_CONFIG.email.leave_smtp_server,&MAIL2LIST_CONFIG.email.leave_email_password, &MAIL2LIST_CONFIG.email.leave_name).await;
    }
}
