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
     * 公共列表查询方法
     */
    async fn list(&self, arg: &Params) -> Result<Vec<Dto>> {
        //构建查询条件
        let wrapper = Self::get_wrapper(arg);
        //执行查询
        let list: Vec<Entity> = RB.fetch_list_by_wrapper(wrapper).await?;
        let mut vos = vec![];
        //将Entity实体转换成 Vo对象 返回
        for x in list {
            vos.push(Dto::from(x));
        }
        Ok(vos)
    }

    async fn list_archive(&self, arg: &Params, name: &str) -> Result<Vec<Dto>> {
        //构建查询条件
        let wrapper = Self::get_wrapper(arg).eq("name",name).is_null("reference").order_by(false,&["create_time"]);
        //执行查询
        let list: Vec<Entity> = RB.fetch_list_by_wrapper(wrapper).await?;
        let mut vos = vec![];
        //将Entity实体转换成 Vo对象 返回
        for x in list {
            vos.push(Dto::from(x));
        }
        Ok(vos)
    }
    /**
     * 根据id更新实体
     */
    async fn update_by_id(&self, id: String, mut data: &Entity) {
        let wrapper = RB.new_wrapper().eq("id", id);
        RB.update_by_wrapper(
            &mut data,
            wrapper,
            &[Skip::Column("id"), Skip::Column("create_date")],
        )
        .await;
    }
    /**
     * 根据id查询条件查询单个值
     */
    async fn get(&self, id: String) -> Result<Dto> {
        let wrapper = RB.new_wrapper().eq("id", id);
        let detail: Entity = RB.fetch_by_wrapper(wrapper).await?;
        let vo = Dto::from(detail);
        return Ok(vo);
    }

    async fn get_message_id(&self, message_id: String) -> Result<Dto> {
        let wrapper = RB.new_wrapper().eq("message_id", message_id);
        let detail: Entity = RB.fetch_by_wrapper(wrapper).await?;
        let vo = Dto::from(detail);
        return Ok(vo);
    }


    async fn get_email(&self, email: String, name: String) -> Result<Dto> {
        let wrapper = RB.new_wrapper().eq("name", name).eq("email",email);
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
    /**
     * 批量删除实体 逻辑删除
     */
    async fn del_batch(&self, ids: &Vec<u64>) {
        RB.remove_batch_by_column::<Entity, _>("id", ids).await;
    }
}
