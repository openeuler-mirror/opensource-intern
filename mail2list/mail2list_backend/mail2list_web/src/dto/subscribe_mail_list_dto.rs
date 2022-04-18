use serde::{Deserialize, Serialize};
use crate::entity::sys_entitys::SubscribeMailList;
use validator_derive::Validate;

#[derive(Clone, Debug, Serialize, Deserialize,Validate)]
pub struct SubscribeMailListDTO {
    pub id: Option<i64>,
    pub name: Option<String>,
    #[validate(email)]
    pub user_email: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
}
impl_field_name_method!(SubscribeMailListDTO {
    id,
    name,
    email,
    user_email,
    username
});
impl Into<SubscribeMailList> for SubscribeMailListDTO {
    fn into(self) -> SubscribeMailList {
        SubscribeMailList {
            id: self.id,
            name: self.name,
            email: self.email,
            user_email: self.user_email,
            username: self.username
        }
    }
}

impl From<SubscribeMailList> for SubscribeMailListDTO {
    fn from(arg: SubscribeMailList) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            email: arg.email,
            user_email: arg.user_email,
            username: arg.username,
        }
    }
}
