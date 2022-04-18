use crate::MAIL2LIST_CONFIG;

use crate::dto::subscribe_mail_list_dto::SubscribeMailListDTO;
use crate::entity::sys_entitys::{CommonField, SubscribeMailList};
use crate::request::SubscribeMailListQuery;
use crate::service::crud_service::CrudService;
use crate::RB;
use mail2list_common::utils::{receive_mail::ReceiveMail, send_email::SendMail, search::Search};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rbatis::wrapper::Wrapper;

/**
*struct:SubscribeMailListService
*desc:菜单基础服务
*author:zhaorunqi
*email:runqi@isrc.iscas.ac.cn
*/
pub struct SubscribeMailListService {}

impl SubscribeMailListService {
    /**
     * 保存用户
     */
    pub async fn save_info(&self, dto: SubscribeMailListDTO) -> bool {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        let user_email = dto.user_email.clone();
        let email1 = user_email.unwrap();
        let email2 = email1.clone();
        let email = dto.email.clone().unwrap();
        let index =  Search::search(MAIL2LIST_CONFIG.email.mine_email.clone(),email);
        let username = dto.username.clone();
        let username1 = String::new();
        match username {
            Some(username1) => println!("username is true"),
            None => println!("username is empty"),
        };
        let name = dto.name.clone();
        let name1 = name.unwrap();
        let mut subject = "confirm+".to_string();
        subject += &rand_string;
        let mut body = "邮箱订阅确认\n您好，这是openeuler.org的邮件列表服务。\n我们收到来自以下邮箱的注册请求：\n\r".to_string();
        body += &email2;
        body += &"\n在开始使用本站的邮件列表服务之前，请确认这是您的邮箱。你可以通过回复此消息来进行确认，请保持回信的主题不变。\n若您不想注册该邮箱，请忽略此消息。若您怀疑自己被恶意订阅了该列表或有其他任何疑问，请联系：\n\r".to_string();
        body += &MAIL2LIST_CONFIG.email.mine_email[index];

        let mut success_body = "欢迎使用".to_string();
        success_body += &name1;
        success_body += &"邮件列表！\n要通过此列表交流，请发送电子邮件至:\n\r".to_string();
        success_body += &MAIL2LIST_CONFIG.email.mine_email[index];
        success_body += &"\n\n取消订阅或调整选项请发送电子邮件至:\n\r".to_string();
        success_body += &MAIL2LIST_CONFIG.email.leave_email[index];

        let flag = self.get_email(email2, name1).await;
        if flag.is_err() {
            let mut entity: SubscribeMailList = dto.into();
            //发送邮箱
            SendMail::send_email(
                email1.as_str(),
                MAIL2LIST_CONFIG.email.mine_email[index].as_str(),
                MAIL2LIST_CONFIG.email.smtp_server[index].as_str(),
                MAIL2LIST_CONFIG.email.password[index].as_str(),
                subject.as_str(),
                body.as_str(),
            );
            //接收邮件 此处要判断是否接收到 要是没有收到则一直等待 直到收到 此时我们可以启动其他服务
            loop {
                let flag = ReceiveMail::receive_mail(
                    MAIL2LIST_CONFIG.email.imap_server[index].as_str(),
                    email1.as_str(),
                    MAIL2LIST_CONFIG.email.mine_email[index].as_str(),
                    MAIL2LIST_CONFIG.email.smtp_server[index].as_str(),
                    MAIL2LIST_CONFIG.email.password[index].as_str(),
                    subject.as_str(),
                    success_body.as_str(),
                );
                match flag {
                    Ok(flag) if flag == true => {
                        break;
                    }
                    Ok(flag) => {}
                    Err(err) => panic!("Problem receive the mail: {:?}", err),
                };
            }
            let id = self.save(&mut entity).await;
            // id.unwrap()
            return true;
        }

        return false;
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