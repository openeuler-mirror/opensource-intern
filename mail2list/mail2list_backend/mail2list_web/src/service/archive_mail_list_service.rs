use std::collections::HashMap;

use crate::dto::archive_mail_list_dto::ArchiveMailListDTO;
use crate::entity::sys_entitys::{CommonField, ArchiveMailList};
use crate::request::ArchiveMailListQuery;
use crate::service::crud_service::CrudService;
use crate::{RB};
use rbatis::wrapper::Wrapper;

/**
*struct:ArchiveMailListService
*desc:菜单基础服务
*author:zhaorunqi
*email:runqi@isrc.iscas.ac.cn
*/
pub struct ArchiveMailListService {}

impl ArchiveMailListService {

    fn build(&self, menus: Vec<ArchiveMailList>) -> Vec<ArchiveMailListDTO> {
        let mut result = HashMap::with_capacity(menus.capacity());
        let  data = vec![];
        for x in menus {
            result.insert(x.id.clone().unwrap_or_default(), x);
        }
        data
    }
}
impl Default for ArchiveMailListService {
    fn default() -> Self {
        ArchiveMailListService {}
    }
}
impl CrudService<ArchiveMailList, ArchiveMailListDTO, ArchiveMailListQuery> for ArchiveMailListService {
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
