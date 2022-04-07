use serde::{Deserialize, Serialize};
use crate::entity::sys_entitys::ArchiveMailList;
use validator_derive::Validate;

#[derive(Clone, Debug, Serialize, Deserialize,Validate)]
pub struct ArchiveMailListDTO {
    pub id: Option<i64>,
    pub name: Option<String>,
    #[validate(email)]
    pub from_email: Option<String>,
    pub create_time: Option<String>,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub message_id: Option<String>,
    pub in_reply_to: Option<String>,
    pub reference: Option<String>,
    pub filename: Option<String>,
}
impl_field_name_method!(ArchiveMailListDTO {
    id,
    name,
    from_email,
    create_time,
    subject,
    body,
    message_id,
    in_reply_to,
    reference,
    filename,
});
impl Into<ArchiveMailList> for ArchiveMailListDTO {
    fn into(self) -> ArchiveMailList {
        ArchiveMailList {
            id: self.id,
            name: self.name,
            from_email: self.from_email,
            create_time: self.create_time,
            subject: self.subject,
            body: self.body,
            message_id: self.message_id,
            in_reply_to: self.in_reply_to,
            reference: self.reference,
            filename: self.filename,
        }
    }
}

impl From<ArchiveMailList> for ArchiveMailListDTO {
    fn from(arg: ArchiveMailList) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            from_email: arg.from_email,
            create_time: arg.create_time,
            subject: arg.subject,
            body: arg.body,
            message_id: arg.message_id,
            in_reply_to: arg.in_reply_to,
            reference: arg.reference,
            filename: arg.filename,
        }
    }
}
