use crate::MAIL2LIST_CONFIG;

use crate::dto::subscribe_mail_list_dto::SubscribeMailListDTO;
use crate::entity::sys_entitys::{CommonField, SubscribeMailList};
use crate::request::SubscribeMailListQuery;
use crate::service::crud_service::CrudService;
use crate::RB;
use mail2list_common::utils::{search::Search, send_email::SendMail};
use mailparse::*;
use rbatis::wrapper::Wrapper;

use mail2list_common::utils::timeout::Timeout;
use std::time::Duration;
use native_tls::TlsConnector;
use std::error::Error;

pub struct SubscribeMailListService {}

impl SubscribeMailListService {
    /**
     *退订
     */
    pub async fn delete(
        &self,
        imap_server: &str,
        mine_email: &str,
        smtp_server: &str,
        password: &str,
        subject: &str,
        body: &str,
        name: &str,
    ) -> Result<Option<bool>,Box<dyn Error>> {
        let timeout = Duration::from_secs(5);
        let tls = TlsConnector::builder().build()?;
        let client = Timeout::connect_all_timeout((imap_server, 993), imap_server, &tls, timeout)?;
        let mut imap_session = client.login(mine_email, password).map_err(|e| e.0)?;
        let inbox = imap_session.select("INBOX")?;
        let unseen = inbox.unseen;
        match unseen {
            Some(len) => {
                for i in 0 as u32..len {
                    let messages =
                        imap_session.fetch((inbox.exists - i).to_string(), "RFC822.HEADER")?;
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
                    let mut user_email = lst.to_string();
                    user_email.pop();
                    let user_email1 = user_email.clone();
                    let flag = self.get_user_email(user_email1, name.to_string()).await;
                    //找到此用户，则删除
                    if flag.is_ok() {
                        let user_email2 = user_email.clone();
                        let email = flag.unwrap().email.unwrap();
                        let index = Search::search(
                            MAIL2LIST_CONFIG.email.mine_email.clone(),
                            email.clone(),
                        );
                        SendMail::send_email(
                            &user_email2,
                            &MAIL2LIST_CONFIG.email.leave_email[index],
                            &MAIL2LIST_CONFIG.email.leave_smtp_server[index],
                            &MAIL2LIST_CONFIG.email.leave_email_password[index],
                            subject,
                            body,
                        );
                        self.del_user_email(user_email2, email.clone()).await;
                        imap_session
                            .store(format!("{}", message.message), "+FLAGS (\\Seen)")
                            .unwrap();
                        imap_session.expunge().unwrap();
                    } else {
                        imap_session
                            .store(format!("{}", message.message), "+FLAGS (\\Seen)")
                            .unwrap();
                        imap_session.expunge().unwrap();
                    }
                    // //找不到 则继续遍历 直到遍历完所有邮箱
                    // if user_email.is_empty() {
                    //     break;
                    // }
                }
            }
            None => {}
        };

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
