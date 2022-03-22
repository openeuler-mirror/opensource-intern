use serde::{Deserialize, Serialize};

use crate::entity::sys_entitys::MailList;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MailListDTO {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub archive: Option<String>,
    pub description: Option<String>,
    pub children:Option<Vec<MailListDTO>>
}
impl_field_name_method!(MailListDTO {
    id,
    name,
    archive,
    email,
    description
});
impl Into<MailList> for MailListDTO {
    fn into(self) -> MailList {
        MailList {
            id: self.id,
            name: self.name,
            description: self.description,
            email: self.email,
            archive: self.archive
        }
    }
}

impl From<MailList> for MailListDTO {
    fn from(arg: MailList) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            email: arg.email,
            archive: arg.archive,
            description: arg.description,
            children: Some(Vec::<MailListDTO>::new()),
        }
    }
}
