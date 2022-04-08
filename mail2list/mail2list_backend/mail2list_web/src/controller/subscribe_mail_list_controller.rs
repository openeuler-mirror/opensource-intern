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
// pub async fn save(Json(arg): Json<SysUserDTO>) -> impl IntoResponse {
//     let context = CONTAINER.get::<ServiceContext>();
//     let user = arg;
//     if let Err(e) = user.validate() {
//         return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
//     }

//     context.sys_user_service.save_info(user).await;

//     return RespVO::from(&"保存成功".to_string()).resp_json();
// }

pub async fn list(arg: Option<Query<SubscribeMailListQuery>>) -> impl IntoResponse {
    let arg = arg.unwrap();
    let vo = CONTEXT.subscribe_mail_list_service.list(&arg).await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn save(Json(arg): Json<SubscribeMailListDTO>) -> impl IntoResponse {
    let flag = CONTEXT.subscribe_mail_list_service.save_info(arg).await;
    if flag == true {
        RespVO::from(&"邮件已发送，请查看邮箱并回复".to_string()).resp_json()
    } else {
        RespVO::from(&"用户已订阅".to_string()).resp_json()
    }
}

pub async fn delete(imap_server: &str, mine_email :&str,smtp_server: &str, password :&str, name :&str) {
    CONTEXT.subscribe_mail_list_service.delete(imap_server, mine_email, smtp_server,password, "退订成功","",name).await;
}