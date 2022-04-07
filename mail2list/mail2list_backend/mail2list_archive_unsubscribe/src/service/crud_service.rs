use crate::entity::sys_entitys::CommonField;
use crate::{RB, REQUEST_CONTEXT};
use async_trait::async_trait;
use mail2list_common::error::Result;
use rbatis::crud::{CRUDTable, Skip, CRUD};
use rbatis::wrapper::Wrapper;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::convert::From;
/**
 *struct:CrudService
 *desc:orm基础CRUD实现
 *author:zhaorunqi
 *email:runqi@isrc.iscas.ac.cn
 */
#[async_trait]
pub trait CrudService<Entity, Dto, Params>: Sync + Send
where
    Entity: CRUDTable + DeserializeOwned,
    Dto: From<Entity> + Send + Sync + Serialize,
    Params: Send + Sync + Serialize,
{
    /**
     * 获取查询条件Wrapper
     * 子类实现
     */
    fn get_wrapper(arg: &Params) -> Wrapper;
    /**设置公共的字段保存方法*/
    fn set_save_common_fields(&self, common: CommonField, data: &mut Entity);


    /**
     * 根据id查询条件查询单个值
     */
    async fn get(&self, id: String) -> Result<Dto> {
        let wrapper = RB.new_wrapper().eq("id", id);
        let detail: Entity = RB.fetch_by_wrapper(wrapper).await?;
        let vo = Dto::from(detail);
        return Ok(vo);
    }

    async fn get_user_email(&self, user_email: String, name: String) -> Result<Dto> {
        let wrapper = RB.new_wrapper().eq("name", name).eq("user_email",user_email);
        let detail: Entity = RB.fetch_by_wrapper(wrapper).await?;
        let vo = Dto::from(detail);
        return Ok(vo);
    }

    async fn get_email_archive(&self, email: String, name: String) -> Result<Dto> {
        let wrapper = RB.new_wrapper().eq("name", name).ne("email",email);
        let detail: Entity = RB.fetch_by_wrapper(wrapper).await?;
        let vo = Dto::from(detail);
        return Ok(vo);
    }
    /**
     * 保存实体
     */
    async fn save(&self, data: &mut Entity) -> Result<i64> {
        /*设置创建人*/

        let tls = REQUEST_CONTEXT.clone();
        let creator = if let Some(a) = tls.get() { a.uid } else { 0 };
        /*设置公共字段*/
        self.set_save_common_fields(
            CommonField {
                id: Some(0),
            },
            data,
        );
        let result = RB.save(data, &[Skip::Column("id")]).await?;
        return Ok(0);
    }

    // async fn save_archive(&self, subject: String, message_id:String, crate_time:String, from_email:String) -> Result<i64> {
    //     /*设置创建人*/

    //     // let tls = REQUEST_CONTEXT.clone();
    //     // let creator = if let Some(a) = tls.get() { a.uid } else { 0 };
    //     // /*设置公共字段*/
    //     // self.set_save_common_fields(
    //     //     CommonField {
    //     //         id: Some(0),
    //     //     },
    //     //     data,
    //     // );
    //     let data = ArchiveMailList{
    //         subject:Some(subject),
    //     };
    //     let result = RB.save(data, &[Skip::Column("id")]).await?;
    //     return Ok(0);
    // }

    /**
     * 批量保存实体
     */
    async fn save_batch(&self, mut list: &Vec<Entity>) {
        RB.save_batch(&mut list, &[Skip::Column("id")]).await;
    }
    /**
     * 删除实体 逻辑删除
     */
    async fn del(&self, id: &String) {
        RB.remove_by_column::<Entity, _>("id", id).await;
    }
    /**
     * 根据字段实体
     */
    async fn del_by_column(&self, column: &str, column_value: &str) {
        RB.remove_by_column::<Entity, _>(column, column_value).await;
    }

    async fn del_user_email(&self, user_email: String, email: String) {
        let wrapper = RB.new_wrapper().eq("user_email", user_email).eq("email",email);
        RB.remove_by_wrapper::<Entity>(wrapper).await;
    }
    /**
     * 批量删除实体 逻辑删除
     */
    async fn del_batch(&self, ids: &Vec<u64>) {
        RB.remove_batch_by_column::<Entity, _>("id", ids).await;
    }
}
