extern crate lettre;
extern crate lettre_email;
extern crate mime;

use crate::dto::archive_mail_list_dto::ArchiveMailListDTO;
use crate::entity::sys_entitys::{ArchiveMailList, CommonField, SubscribeMailList};
use crate::request::ArchiveMailListQuery;
use crate::service::crud_service::CrudService;
use crate::RB;
use rbatis::crud::CRUD;
use rbatis::wrapper::Wrapper;

use imap;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{
    message::{Body,Attachment, MultiPart},
    Message, SmtpTransport, Transport,
};
use mailparse::*;
use std::io::Write;
use std::fs;

/**
*struct:ArchiveMailListService
*desc:菜单基础服务
*author:zhaorunqi
*email:runqi@isrc.iscas.ac.cn
*/
pub struct ArchiveMailListService {}

impl ArchiveMailListService {
    /**
     * 转发以及存储
     */
    pub async fn save_info(
        &self,
        smtp_server: &str,
        mine_email: &str,
        password: &str,
        name: &str,
    ) -> imap::error::Result<Option<String>> {
        let client = imap::ClientBuilder::new(smtp_server, 993).native_tls()?;

        // the client we have here is unauthenticated.
        // to do anything useful with the e-mails, we need to log in
        let mut imap_session = client.login(mine_email, password).map_err(|e| e.0)?;

        // we want to fetch the first email in the INBOX mailbox
        let inbox = imap_session.select("INBOX")?;

        // let mut entity: ArchiveMailList = dto.into();
        for i in 0 as u32..inbox.exists {
            // fetch message number 1 in this mailbox, along with its RFC822 field.
            // RFC 822 dictates the format of the body of e-mails
            let messages = imap_session.fetch((inbox.exists - i).to_string(), "RFC822")?;
            let message = if let Some(m) = messages.iter().next() {
                m
            } else {
                return Ok(None);
            };

            // extract the message's body
            let body = message.body().expect("message did not have a body!");
            let body1 = std::str::from_utf8(body)
                .expect("message was not valid utf-8")
                .to_string();
            //println!("{}",body1);

            //此处的话增加两次 以此增加发件人 主题 日期 一次增加内容
            let parsed = parse_mail(body).unwrap();

            //message_id
            let message_id = parsed.headers.get_first_value("Message-ID").unwrap();
            println!("{}", message_id);

            //主题
            let subject = parsed.headers.get_first_value("Subject").unwrap();
            println!("{}", subject);
            let subject1 = subject.clone();
            let subject2 = subject1.clone();

            //from
            let mail = parsed.headers.get_first_value("From").unwrap();
            let pos = mail.rfind("<").unwrap();
            let (_, lst) = mail.split_at(pos + 1);
            let mut from_email = lst.to_string();
            from_email.pop();
            println!("{}", from_email);
            let email = from_email.clone();

            //Date
            let date = dateparse(parsed.headers.get_first_value("Date").unwrap().as_str()).unwrap();
            println!("{}", date);

            //body
            let mut body = dump(&parsed);
            if body.ends_with(">") == true {
                let pos = body.find("<").unwrap();
                let (_, body1) = body.split_at(pos);
                body = body1.to_string();
            }
            let body1 = body.clone();
            let body2 = body.clone();

            println!("{}", body);

            //filename
            //可能存在多个附件 将其名字通过隔一个空格隔开 在前端进行分割拿取
            let mut filename = String::new();
            for subpart in parsed.subparts {
                if subpart.get_content_disposition().disposition == DispositionType::Attachment {
                    // it's an attachment
                    let filename_temp = subpart
                        .get_content_disposition()
                        .params
                        .get("filename")
                        .unwrap()
                        .to_string();
                    filename += &filename_temp;
                    filename += &" ".to_string();
                }
            }
            println!("{}", filename);
            let mut filename1 = Some(filename.clone());
            let filename3 = filename1.clone();
            let temp = filename3.unwrap();
            if temp == "" {
                filename1 = None;
            }

            let filename2 = filename.clone();

            let mut archive_mail_list = ArchiveMailList {
                name: Some(name.to_string()),
                from_email: Some(from_email),
                create_time: Some(date.to_string()),
                subject: Some(subject1),
                message_id: Some(message_id),
                in_reply_to: None,
                reference: None,
                body: Some(body1),
                id: None,
                filename: filename1,
            };

            self.save(&mut archive_mail_list).await;

            println!("{}",filename2);

            //转发 再进行转发的时候 其他都很好实现 但是对于存在多个附件 此时则需要判断一下 另外 其中要转发给的人也需要设置
            let wrapper = RB.new_wrapper().eq("name", name).ne("email", email);
            let detail: Vec<SubscribeMailList> = RB.fetch_list_by_wrapper(wrapper).await.unwrap();
            for data in detail {
                //找到了每个人邮箱 开始进行发送
                let email = data.email.unwrap();
                let mut multipart = MultiPart::alternative_plain_html(
                    String::from("Plaintext version of the body"),
                    String::from(&body2),
                );
                if filename2 != "" {
                    let pos: Vec<&str> = filename2.split(" ").collect();
                    for i in pos.into_iter() {
                        //此处拿到保存在本地的文件
                        if i =="" {
                            break;
                        }
                        let mut content = "E:/openEuler/task/test/".to_string();
                        content += &i;
                        let attachment = fs::read(content).unwrap();
                        let attachment_body = Body::new(attachment);
                        let attachment = Attachment::new(i.to_string())
                            .body(attachment_body, "application/octet-stream".parse().unwrap()); // build `Attachment` here
                        multipart = multipart.singlepart(attachment);
                    }
                }

                let email = Message::builder()
                    .from(mine_email.parse().unwrap())
                    .to(email.parse().unwrap())
                    .subject(&subject2)
                    .multipart(multipart)
                    .unwrap();
                let creds = Credentials::new(mine_email.to_string(), password.to_string());

                // Open a remote connection to gmail
                let mailer = SmtpTransport::relay(smtp_server)
                    .unwrap()
                    .credentials(creds)
                    .build();

                match mailer.send(&email) {
                    Ok(_) => println!("Email sent successfully!"),
                    Err(e) => panic!("Could not send email: {:?}", e),
                }
            }

            imap_session
                .store(format!("{}", message.message), "+FLAGS (\\Deleted)")
                .unwrap();
            imap_session.expunge().unwrap();
        }
        // be nice to the server and log out
        imap_session.logout()?;

        Ok(Some("logout".to_string()))
    }
}

pub fn dump(pm: &mailparse::ParsedMail) -> String {
    //接下来是文章内容 文字部分将会存储在body中 进行展示 附件将会保存在云服务器端
    //这里的话我们直接将整个body追加存储在数据中 然后在数据库层面进行判断
    if pm.ctype.mimetype.starts_with("text/") {
        let body = pm.get_body().unwrap();
        return body;
    } else if pm.ctype.mimetype.starts_with("application/") {
        // println!(
        //     "   (Body is binary type {}, {} bytes in length)",
        //     pm.ctype.mimetype,
        //     pm.get_body().unwrap().len()
        // );
        //此处将文件进行存储
        let filename = pm
            .get_content_disposition()
            .params
            .get("filename")
            .unwrap()
            .to_string();
        let data = pm.get_body_raw().unwrap();
        let mut path = "E:/openEuler/task/test/".to_string();
        //此处增加一个随机数 避免文件重合
        path += &filename;
        let mut file = std::fs::File::create(path).expect("create failed");
        //println!("文件创建成功:{:?}",file);
        //写入了文件
        file.write_all(&data).expect("write failed");
        //println!("data written to file");
        //接下来就是存储文件到云端 此处需要服务器 然后通过云服务器地址 展示在前端 此处暂时保留
    } else {
        // println!(
        //     "   (Body is binary type {}, {} bytes in length)",
        //     pm.ctype.mimetype,
        //     pm.get_body().unwrap().len()
        // );
    }
    let mut c = 1;
    let mut all_body = String::new();
    for s in &pm.subparts {
        // println!(">> Subpart {} <<", c);
        let body = dump(s);
        all_body += &body;
        c = c + 1;
    }
    return all_body;
}

impl Default for ArchiveMailListService {
    fn default() -> Self {
        ArchiveMailListService {}
    }
}
impl CrudService<ArchiveMailList, ArchiveMailListDTO, ArchiveMailListQuery>
    for ArchiveMailListService
{
    fn get_wrapper(arg: &ArchiveMailListQuery) -> Wrapper {
        let mut wrapper = RB.new_wrapper();
        if let Some(id_list) = &arg.ids {
            wrapper = wrapper.r#in(ArchiveMailList::id(), id_list);
        }
        wrapper
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut ArchiveMailList) {
        data.id = common.id;
    }
}
