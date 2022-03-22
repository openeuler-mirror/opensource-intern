extern crate imap;

use mailparse::*;
use crate::utils::send_email::SendMail;

pub struct ReceiveMail {}

impl ReceiveMail {
    pub fn receive_mail(imap_server: &str, email_receiver: &str, mine_email :&str,smtp_server: &str, password :&str, subject :&str, body :&str) -> imap::error::Result<Option<bool>> {
        let client = imap::ClientBuilder::new(imap_server, 993).native_tls()?;
        // the client we have here is unauthenticated.
        // to do anything useful with the e-mails, we need to log in
        let mut imap_session = client
            .login(mine_email, password)
            .map_err(|e| e.0)?;
        // we want to fetch the first email in the INBOX mailbox
        imap_session.select("INBOX")?;
        // fetch message number 1 in this mailbox, along with its RFC822 field.
        // RFC 822 dictates the format of the body of e-mails
        let mut i = 1;
        let mut flag = false;
        loop {
            let i1 = &i.to_string();
            let messages = imap_session.fetch(i1, "RFC822.HEADER")?;
            let message = if let Some(m) = messages.iter().next() {
                m
            } else {
                return Ok(None);
            };
            // extract the message's body
            let header = message.header().expect("message did not have a subject!");
            let parsed = parse_mail(header).unwrap();
            if parsed.headers.get_first_value("Subject") == Some(subject.to_string()) {
                //找到邮件 发送确认
                SendMail::send_email(email_receiver,mine_email, smtp_server, password,  &subject , body);
                flag = true;
                break;
            } else {
                i = i + 1;
            }
        }
        // be nice to the server and log out
        imap_session.logout()?;
        Ok(Some(flag))
    }
}