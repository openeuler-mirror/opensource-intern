use std::collections::HashMap;

use crate::dto::mail_list_dto::MailListDTO;
use crate::entity::sys_entitys::{CommonField, MailList};
use crate::request::MailListQuery;
use crate::service::crud_service::CrudService;
use crate::{RB};
use rbatis::wrapper::Wrapper;

/**
*struct:MailListService
*desc:菜单基础服务
*author:zhaorunqi
*email:runqi@isrc.iscas.ac.cn
*/
pub struct MailListService {}

impl MailListService {

    fn build(&self, menus: Vec<MailList>) -> Vec<MailListDTO> {
        let mut result = HashMap::with_capacity(menus.capacity());
        let  data = vec![];
        for x in menus {
            result.insert(x.id.clone().unwrap_or_default(), x);
        }
        data
    }
}
impl Default for MailListService {
    fn default() -> Self {
        MailListService {}
    }
}
impl CrudService<MailList, MailListDTO, MailListQuery> for MailListService {
    fn get_wrapper(arg: &MailListQuery) -> Wrapper {
        let mut wrapper = RB.new_wrapper();
        if let Some(id_list) = &arg.ids {
            wrapper = wrapper.r#in(MailList::id(), id_list);
        }
        wrapper
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut MailList) {
        data.id = common.id;
    }
}
