use crate::MAIL2LIST_CONFIG;

use crate::dto::subscribe_mail_list_dto::SubscribeMailListDTO;
use crate::entity::sys_entitys::{CommonField, SubscribeMailList};
use crate::request::SubscribeMailListQuery;
use crate::service::crud_service::CrudService;
use crate::RB;
use mail2list_common::utils::send_email::SendMail;
use rbatis::wrapper::Wrapper;
use imap;
use mailparse::*;

pub struct SubscribeMailListService {}

impl SubscribeMailListService {

    /**
     *退订
     */
    pub async fn delete(&self, imap_server: &str, mine_email :&str,smtp_server: &str, password :&str, subject :&str, body :&str, name :&str) -> imap::error::Result<Option<bool>> {
        let client = imap::ClientBuilder::new(imap_server, 993).native_tls()?;
    let mut imap_session = client
        .login(mine_email, password)
        .map_err(|e| e.0)?;
    imap_session.select("INBOX")?;
    let mut i = 1;
    loop {
        let i1 = i.to_string();
        let messages = imap_session.fetch(i1, "RFC822.HEADER")?;
        let message = if let Some(m) = messages.iter().next() {
            m
        } else {
            return Ok(None);
        };
        let header = message.header().expect("message did not have a subject!");
        let parsed = parse_mail(header).unwrap();
        let mail = parsed.headers.get_first_value("From").unwrap();
        let pos = mail.rfind("<").unwrap();
        let (_, lst) = mail.split_at(pos + 1);
        let mut email = lst.to_string();
        email.pop();
        let email1 = email.clone();
        let flag = self.get_email(email1,name.to_string()).await;
        //找到此用户，则删除
        if flag.is_ok() {
            let email2 = email.clone();
            SendMail::send_email(&email2,&MAIL2LIST_CONFIG.email.leave_email,&MAIL2LIST_CONFIG.email.leave_smtp_server,&MAIL2LIST_CONFIG.email.leave_email_password,subject,body);
            self.del_by_column("email",&email2).await;
            imap_session.store(format!("{}", message.message), "+FLAGS (\\Deleted)").unwrap();
            imap_session.expunge().unwrap();
        }else {
            imap_session.store(format!("{}", message.message), "+FLAGS (\\Deleted)").unwrap();
            imap_session.expunge().unwrap();
        }
        //找不到 则继续遍历 直到遍历完所有邮箱
        i = i + 1;
        if email.is_empty() {
            break;
        }
    }
    // be nice to the server and log out
    imap_session.logout()?;
    Ok(Some(true))
    }
}

impl Default for SubscribeMailListService {
    fn default() -> Self {
        SubscribeMailListService {}
    }
}

impl CrudService<SubscribeMailList, SubscribeMailListDTO, SubscribeMailListQuery>
    for SubscribeMailListService
{
    fn get_wrapper(arg: &SubscribeMailListQuery) -> Wrapper {
        RB.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut SubscribeMailList) {
        data.id = common.id;
    }
}