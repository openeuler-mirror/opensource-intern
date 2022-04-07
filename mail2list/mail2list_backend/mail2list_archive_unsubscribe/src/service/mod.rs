pub mod crud_service;
pub mod subscribe_mail_list_service;
pub mod archive_mail_list_service;


use self::subscribe_mail_list_service::SubscribeMailListService;
use self::archive_mail_list_service::ArchiveMailListService;


pub struct ServiceContext {
    pub subscribe_mail_list_service: SubscribeMailListService,
    pub archive_mail_list_service: ArchiveMailListService,
}

impl ServiceContext {
    pub fn default() -> Self {
        Self {
            subscribe_mail_list_service: Default::default(),
            archive_mail_list_service: Default::default(),
        }
    }
}
