use crate::dao::mapper::{get_message_list_by_message_id,get_archive_list};
use crate::dto::archive_mail_list_dto::ArchiveMailListDTO;
use crate::entity::sys_entitys::{CommonField, ArchiveMailList};
use crate::request::ArchiveMailListQuery;
use crate::service::crud_service::CrudService;
use crate::{RB};
use rbatis::wrapper::Wrapper;
use mail2list_common::error::Result;

/**
*struct:ArchiveMailListService
*desc:菜单基础服务
*author:zhaorunqi
*email:runqi@isrc.iscas.ac.cn
*/
pub struct ArchiveMailListService {}

impl ArchiveMailListService {

    pub async fn get_message_list_by_message_id(&self,message_id: String) -> Result<Vec<ArchiveMailListDTO>> {
        let one = self.get_message_id(message_id.clone()).await.unwrap();
        let result = get_message_list_by_message_id(&RB,message_id.as_str()).await.unwrap();
        let mut vec = vec![];
        vec.push(ArchiveMailListDTO::from(one));
        for r in result.unwrap() {
            let r1 = r.clone();
            vec.push(ArchiveMailListDTO::from(r));
            let message_id = r1.message_id.unwrap();
            let result = get_message_list_by_message_id(&RB,message_id.as_str()).await.unwrap();
            for r2 in result.unwrap() {
                vec.push(ArchiveMailListDTO::from(r2));
            }
        }
        Ok(vec)
    }

    pub async fn list_archive(&self, name: String) -> Result<Vec<ArchiveMailListDTO>> {
        let result = get_archive_list(&RB,name.as_str()).await.unwrap();
        let mut vec = vec![];
        for r in result.unwrap() {
            vec.push(ArchiveMailListDTO::from(r));
        }
        Ok(vec)
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
