use crate::{
    dto::subscribe_mail_list_dto::SubscribeMailListDTO, request::SubscribeMailListQuery,
    service::crud_service::CrudService, CONTEXT,
};
use axum::extract::{Query};
use axum::response::IntoResponse;
use axum::Json;
use mail2list_common::RespVO;

/**
 *method:/user/list
 *desc:保存
 *author:zhaorunqi
 */
pub async fn list(arg: Option<Query<SubscribeMailListQuery>>) -> impl IntoResponse {
    let arg = arg.unwrap();
    let vo = CONTEXT.subscribe_mail_list_service.list(&arg).await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn save(Json(arg): Json<SubscribeMailListDTO>) -> impl IntoResponse {
    let flag = CONTEXT.subscribe_mail_list_service.save_info(arg).await;
    if flag == true {
        RespVO::from(&"订阅成功".to_string()).resp_json()
    } else {
        RespVO::from(&"用户已订阅".to_string()).resp_json()
    }
}