use crate::controller::{
    mail_list_controller,
    subscribe_mail_list_controller,
    archive_mail_list_controller,
};
use axum::{
    routing::{get},
    Router,
};
pub fn routers() -> Router {
    Router::new()
        .route("/menu/list", get(mail_list_controller::list))
        .route(
            "/menu/getListById/:id",
            get(mail_list_controller::get_by_id)
        )
        .route(
            "/menu/subscribe",
            get(subscribe_mail_list_controller::list).post(subscribe_mail_list_controller::save)
        )
        .route("/archive/list", get(archive_mail_list_controller::list))
}
