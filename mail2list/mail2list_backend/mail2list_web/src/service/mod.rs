pub mod crud_service;
pub mod mail_list_service;

use self::mail_list_service::MailListService;


pub struct ServiceContext {
    pub mail_list_service: MailListService,
}

impl ServiceContext {
    pub fn default() -> Self {
        Self {
            mail_list_service: Default::default(),
        }
    }
}
