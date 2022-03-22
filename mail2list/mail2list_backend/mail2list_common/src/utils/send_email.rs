extern crate lettre;
extern crate lettre_email;
extern crate mime;

use lettre_email::Email;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};

pub struct SendMail{}

impl SendMail {
    pub fn send_email(email_receiver: &str, mine_email :&str,smtp_server: &str, password :&str, subject :&str, body :&str) {
        let email_receiver = email_receiver;
        let mine_email =  mine_email;
        let smtp_server = smtp_server;
        let password = password; //需要生成应用专用密码
        let email = Email::builder()
        .to(email_receiver)
        .from(mine_email)
        .subject(subject)
        .text(body)
        .build()
        .unwrap();
        let creds = Credentials::new(
            mine_email.to_string(),
            password.to_string(),
        );
        // Open connection to Gmail
        let mut mailer = SmtpClient::new_simple(smtp_server)
        .unwrap()
        .credentials(creds)
        .transport();
        // Send the email
        let result = mailer.send(email.into());
        if result.is_ok() {
            println!("Email sent to ");
            println!("{}", email_receiver.to_string());
        } else {
            println!("Could not send email: {:?}", result);
        }
        print!("{:?}", result);
        mailer.close();
    }
}