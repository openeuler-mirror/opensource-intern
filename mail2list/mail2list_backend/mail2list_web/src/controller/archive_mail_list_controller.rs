use crate::REQUEST_CONTEXT;
use crate::{service::crud_service::CrudService, CONTEXT};
use axum::extract::{Path};
use axum::response::IntoResponse;

use mail2list_common::RespVO;
// use validator::Validate;


/**
 *method:/user/list
 *desc:用户查询
 *author:zhaorunqi
 */
pub async fn list(Path(name): Path<String>) -> impl IntoResponse {
    let vo = CONTEXT.archive_mail_list_service.list_archive(name).await;
    RespVO::from_result(&vo).resp_json()
}
/**
 *method:/menu/getList/:id
 *desc:查询
 *author:zhaorunqi
 */
pub async fn get_by_message_id(Path(message_id): Path<String>) -> impl IntoResponse {
    let vo = CONTEXT.archive_mail_list_service.get_message_list_by_message_id(message_id).await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn info() -> impl IntoResponse {
    let tls = REQUEST_CONTEXT.clone();
    let uid = if let Some(a) = tls.get() { a.uid } else { 0 };
    let vo = CONTEXT.archive_mail_list_service.get(uid.to_string()).await.unwrap();
    RespVO::from(&vo).resp_json()
}